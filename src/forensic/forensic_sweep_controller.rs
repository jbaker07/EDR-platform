use std::{thread, time::Duration};
use chrono::{Timelike, Utc};
use log::{error, info};

use crate::forensic::*;
use crate::modes::forensic::run_all_forensic_modules;
use crate::trust_hook::report_trust_payload;
use crate::utils::{get_hostname, post_json};

const DAILY_SWEEP_HOUR: u32 = 3; // 3 AM
const SWEEP_INTERVAL_SECONDS: u64 = 3600; // hourly check

/// Launches continuous background thread to run forensic sweeps
pub fn start_forensic_sweep_controller() {
    thread::spawn(move || {
        loop {
            let now = Utc::now();
            let hour = now.hour();

            if hour == DAILY_SWEEP_HOUR || should_predictive_sweep() {
                info!("[sweep_controller] Initiating forensic sweep at {}", now);

                // Run all forensic checks (file, auth, memory, syscall, etc.)
                match run_all_forensic_modules() {
                    Ok(telemetry) => {
                        if let Err(e) = post_json("http://localhost:8000/api/forensics/upload", &telemetry) {
                            error!("[sweep_controller] Failed to post forensic telemetry: {:?}", e);
                        }

                        if let Ok(trust_payload) = report_trust_payload(&telemetry) {
                            if let Err(e) = post_json("http://localhost:8000/api/trust/update", &trust_payload) {
                                error!("[sweep_controller] Failed to post trust payload: {:?}", e);
                            }
                        } else {
                            error!("[sweep_controller] Failed to build trust payload.");
                        }
                    }
                    Err(e) => {
                        error!("[sweep_controller] Forensic module run failed: {:?}", e);
                    }
                }
            }

            thread::sleep(Duration::from_secs(SWEEP_INTERVAL_SECONDS));
        }
    });
}

/// Optional predictive trigger — hook in GNN trust fluctuations, anomalies, etc.
fn should_predictive_sweep() -> bool {
    // Placeholder: add hooks for volatile trust scores, alert thresholds, etc.
    false
}
