// edr-agent/forensic/phase_coordinator.rs
use crate::decay_engine::TrustDecayManager;
use crate::telemetry_pruner::run_pruning_cycle;
use crate::trust_state_writer::{load_last_snapshot, persist_snapshot_periodically};
use crate::trust_escalation::check_and_escalate;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_phase_coordinator() {
    let trust_manager = Arc::new(Mutex::new(TrustDecayManager::new()));

    // Attempt to restore last saved trust state
    if let Some(saved) = load_last_snapshot() {
        let mut tm = trust_manager.lock().unwrap();
        for (id, state) in saved {
            tm.update_score(&id, state.last_score, state.decay_half_life);
        }
    }

    // Start periodic snapshot writer in background
    persist_snapshot_periodically(Arc::clone(&trust_manager));

    // Decay + prune + trigger loop
    thread::spawn(move || loop {
        {
            let mut tm = trust_manager.lock().unwrap();
            tm.prune_expired(3600 * 48); // 48h max age
            let ids: Vec<String> = tm.export_trust_state().keys().cloned().collect();
            for id in ids {
                if let Some(score) = tm.get_decayed_score(&id) {
                    check_and_escalate(&id, score); // Apply thresholds, replay, etc.
                }
            }
        }

        run_pruning_cycle(); // Trim GNN and telemetry graph
        thread::sleep(Duration::from_secs(90)); // Main wire-up loop
    });
}
