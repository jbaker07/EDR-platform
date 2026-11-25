use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crate::metrics;
use crate::pb_wiring;

use super::fact::AdapterFact;
use super::queue::{FactQueue, BATCH};
use super::with_engine;

/// Starts the engine thread. It is an OS thread of its own, not a task on the tokio
/// runtime, so the blocking file I/O on the runtime's workers cannot starve it.
pub fn spawn_adapter_thread(queue: Arc<FactQueue<AdapterFact>>) -> std::io::Result<()> {
    queue.set_consumer_alive(true);
    let q = queue.clone();
    let spawned = std::thread::Builder::new()
        .name("pb-adapter".into())
        .spawn(move || run_adapter(&q));
    if spawned.is_err() {
        queue.set_consumer_alive(false);
    }
    spawned.map(|_| ())
}

/// Feed adapter facts to the engine in batches: one mapper lock and one engine lock per
/// batch of up to BATCH facts. A completed playbook goes to pb_wiring::publish, which
/// logs it, shows it under /playbooks/hits and queues it for the batch loop to turn into
/// an incident. Every 5 s, between batches, time moves on: TCP connections open a
/// minute or more become open_flow facts (so a long-lived connection is seen while it is
/// still open), and tick drops slot hits and counted events that aged out, so a chain
/// that started long ago cannot complete late.
pub fn run_adapter(queue: &FactQueue<AdapterFact>) {
    let mut last_tick = std::time::Instant::now();
    loop {
        let batch = queue.pop_batch(BATCH, Duration::from_secs(1));
        if !batch.is_empty() {
            process_batch(&batch);
        }
        if last_tick.elapsed() >= Duration::from_secs(5) {
            last_tick = std::time::Instant::now();
            advance_time(crate::utils::time::now_ts());
        }
    }
}

/// The open_flow sweep and the engine tick at `now`.
pub fn advance_time(now: u64) {
    let flows = pb_wiring::sweep_flows(now);
    let Some(engine) = with_engine() else {
        return;
    };
    let hits = match engine.lock() {
        Ok(mut eng) => {
            let hits: Vec<_> = flows.into_iter().flat_map(|f| eng.ingest_fact(f)).collect();
            eng.tick(now);
            hits
        }
        Err(_) => return,
    };
    for hit in hits {
        pb_wiring::publish(hit);
    }
}

/// One batch through the mapper and the engine. Returns how many playbooks fired.
pub fn process_batch(batch: &[AdapterFact]) -> usize {
    metrics::facts_in().fetch_add(batch.len() as u64, Ordering::Relaxed);
    let facts = pb_wiring::map_adapter_facts(batch);
    if facts.is_empty() {
        return 0;
    }
    let Some(engine) = with_engine() else {
        log::warn!("pb_adapter: engine handle not initialized");
        return 0;
    };
    let hits = match engine.lock() {
        Ok(mut eng) => facts
            .into_iter()
            .flat_map(|f| eng.ingest_fact(f))
            .collect::<Vec<_>>(),
        Err(err) => {
            log::warn!("pb_adapter: engine lock poisoned: {err}");
            return 0;
        }
    };
    let n = hits.len();
    for hit in hits {
        pb_wiring::publish(hit);
    }
    n
}
