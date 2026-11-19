use std::sync::atomic::Ordering;
use std::sync::Arc;

use tokio::sync::mpsc::Receiver;

use crate::metrics;
use crate::pb_wiring;

use super::fact::AdapterFact;
use super::with_engine;

/// Feed every adapter fact to the engine as it arrives. A completed playbook goes to
/// pb_wiring::publish, which logs it, shows it under /playbooks/hits and queues it for
/// the batch loop to turn into an incident.
pub async fn run_adapter(mut rx: Receiver<AdapterFact>, _metrics: Arc<metrics::Metrics>) {
    while let Some(fact) = rx.recv().await {
        metrics::facts_in().fetch_add(1, Ordering::Relaxed);

        let facts = pb_wiring::map_adapter_fact(&fact);
        if facts.is_empty() {
            continue;
        }
        let Some(engine) = with_engine() else {
            log::warn!("pb_adapter: engine handle not initialized");
            continue;
        };
        let hits = match engine.lock() {
            Ok(mut eng) => facts
                .into_iter()
                .flat_map(|f| eng.ingest_fact(f))
                .collect::<Vec<_>>(),
            Err(err) => {
                log::warn!("pb_adapter: engine lock poisoned: {err}");
                continue;
            }
        };
        for hit in hits {
            pb_wiring::publish(hit);
        }
    }
}
