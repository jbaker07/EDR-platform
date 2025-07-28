use crate::gnn::graph_manager::prune_telemetry_graph;
use std::time::Duration;
use log::info;

pub fn run_pruning_cycle() {
    let ttl_hours = 72;
    let ttl_secs = ttl_hours * 3600;

    info!("[PRUNER] Starting telemetry graph pruning cycle (TTL: {} hours)", ttl_hours);
    match prune_telemetry_graph(ttl_secs) {
        Ok(pruned) => {
            info!("[PRUNER] Completed pruning: {} stale nodes removed.", pruned);
        },
        Err(e) => {
            eprintln!("[PRUNER] Failed to prune telemetry graph: {:?}", e);
        }
    }
}
