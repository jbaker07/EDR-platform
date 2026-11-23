use std::collections::{HashMap, HashSet};
use std::process::Command;

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::write_telemetry_record;
use crate::trust_hook::{generate_feature_vector, submit_trust_event, TrustEvent};
use crate::utils::time::now_ts;
use anyhow::{Context, Result};

/// Parse current USB devices via `lsblk -o NAME,TRAN`.
/// Returns a set of normalized "name,tran" lines that include "usb" transport.
/// Robust to headers and odd spacing.
fn fetch_usb_devices() -> Result<HashSet<String>> {
    let output = Command::new("lsblk")
        .arg("-o")
        .arg("NAME,TRAN")
        .output()
        .context("failed to execute lsblk")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut current = HashSet::new();

    for line in stdout.lines() {
        // skip header if present
        let lower = line.to_lowercase();
        if lower.contains("name") && lower.contains("tran") {
            continue;
        }
        // we consider a line USB if it contains "usb" anywhere in TRAN/line
        if lower.contains("usb") {
            // normalize whitespace
            let norm = line.split_whitespace().collect::<Vec<_>>().join(" ");
            if !norm.is_empty() {
                current.insert(norm);
            }
        }
    }

    Ok(current)
}

/// Passive fallback scan for USB devices using `lsblk`.
/// This reuses the same parsing logic as the real-time loop.
pub fn scan_usb_state() -> Vec<TelemetryOutput> {
    let mut results = Vec::new();

    match fetch_usb_devices() {
        Ok(devs) => {
            for dev in devs {
                let ts = now_ts();

                let features = generate_feature_vector(0.3, 5000, 3.0);
                let mut data = HashMap::new();
                data.insert("device_info".into(), dev.clone());
                data.insert("timestamp".into(), ts.to_string());
                data.insert("replay_tag".into(), "usb_inserted".into());
                data.insert(
                    "soc_note".into(),
                    "USB device seen during passive scan".into(),
                );
                data.insert("source".into(), "scan_usb_state".into());
                data.insert("features".into(), format!("{:?}", features));

                // Push to GNN
                push_to_gnn_vector_log(data.clone());

                // Submit TrustEvent
                submit_trust_event(TrustEvent {
                    timestamp: ts,
                    pid: 0,
                    ppid: 0,
                    uid: 0,
                    binary_path: "/dev/null".into(),
                    command_line: "scan_usb_state".into(),
                    cwd: "/".into(),
                    anomaly_type: "usb_device_seen".into(),
                    component: "usb_scanner".into(),
                    metadata: data.clone(),
                    risk_score: 3.0,
                    source_module: "usb_scanner".into(),
                    decay_context: Some("usb_scan".into()),
                    module: Some("usb_scanner".into()),
                    signal: Some("usb_device_seen".into()),
                    signal_type: Some("usb".into()),
                    score: Some(3.0),
                    raw_score: Some(3.0),
                    tags: Some(vec!["usb".into(), "passive".into()]),
                    description: Some("USB device detected during passive scan.".into()),
                });

                // Write to telemetry record
                let _ = write_telemetry_record(data.clone());

                results.push(TelemetryOutput {
                    category: "usb".into(),
                    signal: "usb_device_seen".into(),
                    confidence: 0.4,
                    data,
                });
            }
        }
        Err(e) => {
            eprintln!("❌ scan_usb_state: {}", e);
        }
    }

    results
}
