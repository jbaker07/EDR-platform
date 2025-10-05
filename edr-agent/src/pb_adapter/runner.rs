use std::sync::atomic::Ordering;
use std::sync::Arc;

use tokio::sync::mpsc::Receiver;

use crate::metrics;
use crate::pb_introspect;
use crate::pb_wiring;
use crate::utils::time::now_ts;

use super::fact::AdapterFact;
use super::with_engine;

pub async fn run_adapter(mut rx: Receiver<AdapterFact>, _metrics: Arc<metrics::Metrics>) {
    while let Some(fact) = rx.recv().await {
        metrics::facts_in().fetch_add(1, Ordering::Relaxed);

        let Some(record) = super::to_pb_record(&fact) else {
            log::warn!(
                "pb_adapter: failed to map fact kind={} detector={}",
                fact.kind,
                fact.detector
            );
            continue;
        };

        if let Some(engine) = with_engine() {
            match engine.lock() {
                Ok(mut eng) => {
                    let mut records = Vec::with_capacity(1);
                    records.push(record);
                    let now = now_ts();
                    let out = pb_wiring::ingest_batch(&mut *eng, &records, now);
                    for pending in out.pending {
                        pb_introspect::add_or_update_pending(pending);
                    }
                    for hit in out.hits {
                        pb_introspect::push_hit(hit);
                    }
                }
                Err(err) => {
                    log::warn!("pb_adapter: engine lock poisoned: {err}");
                }
            }
        } else {
            log::warn!("pb_adapter: engine handle not initialized");
        }
    }
}
