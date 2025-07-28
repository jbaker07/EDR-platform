mod logger;
mod config;
mod telemetry;
pub mod telemetry_writer;
pub mod telemetry_types;
pub mod services;
mod trust_hook;
mod gnn_hook;
pub mod utils;
pub mod modules;
pub mod relay;
pub mod forensic;
pub mod adaptive_threshold_engine;
pub mod trust_digest_engine;
pub mod score_reason;
pub mod session_trust_curve;
use crate::telemetry_writer::TelemetryWriter;
use crate::logger::init_logger;
use crate::config::load_and_verify_policy;
use crate::telemetry::{get_current_telemetry_snapshot, TelemetryRecord, run_sideeffect_monitors_and_collect};
use services::ontology_mapper::SemanticTagInfo;
use crate::telemetry_types::TelemetryOutput;
use crate::utils::time::now_ts;
use crate::services::trust_engine_final::{evaluate_and_dispatch_trust_score, TelemetryData, TrustResult};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Convert TelemetryOutput into TelemetryRecord
impl std::convert::TryFrom<TelemetryOutput> for TelemetryRecord {
    type Error = String;

    fn try_from(output: TelemetryOutput) -> Result<Self, Self::Error> {
        Ok(TelemetryRecord {
            timestamp: now_ts(),
            pid: 0,
            ppid: 0,
            uid: 0,
            binary_path: output.data.get("binary_path").cloned().unwrap_or_default(),
            command_line: output.data.get("command_line").cloned().unwrap_or_default(),
            cwd: output.data.get("cwd").cloned().unwrap_or_default(),
            env_vars: None,
            tags: vec![output.signal],
            risk_score: Some((output.confidence * 100.0) as u32),
        })
    }
}


fn main() {
    init_logger();

    let _policy = load_and_verify_policy("policy.json")
        .expect("❌ Failed to load or verify policy");

    let writer = Arc::new(Mutex::new(TelemetryWriter::new()));

    {
        let flush_writer = writer.clone();
        thread::spawn(move || loop {
            std::thread::sleep(Duration::from_secs(30));
            if let Ok(mut locked) = flush_writer.lock() {
                let _ = locked.flush();
            }
        });
    }

    loop {
        let mut records: Vec<TelemetryRecord> = get_current_telemetry_snapshot(writer.clone());

        let side_effects = run_sideeffect_monitors_and_collect();
        for output in side_effects {
            println!("📥 Passive Signal: {:?}", output);

            if let Ok(record) = TelemetryRecord::try_from(output) {
                records.push(record);
            } else {
                println!("⚠️ Failed to convert TelemetryOutput to TelemetryRecord");
            }
        }

        if records.is_empty() {
            println!("⚠️  No telemetry records collected this cycle.");
        } else {
            for rec in &records {
                println!("🟡 Raw Telemetry: {:?}", rec);

                let telemetry_data = TelemetryData::from_record(rec);
                let result: TrustResult = evaluate_and_dispatch_trust_score(&telemetry_data);
                println!("✅ Trust Result: {:?}", result);
            }
        }

        writer.clone().lock().unwrap().append_batch(records);
        std::thread::sleep(Duration::from_secs(30));
    }
}
