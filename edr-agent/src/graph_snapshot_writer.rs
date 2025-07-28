// src/graph_snapshot_writer.rs

use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn store_graph_snapshot(session_id: &str, reasons: &[String], trust_score: f64) {
    let snapshot_path = format!("/var/log/edr/graph_snapshots/{}.snapshot", session_id);

    if let Ok(mut file) = File::create(&snapshot_path) {
        let _ = writeln!(file, "Session ID: {}", session_id);
        let _ = writeln!(file, "Trust Score: {:.2}", trust_score);
        let _ = writeln!(file, "Reasons:");
        for reason in reasons {
            let _ = writeln!(file, "  - {}", reason);
        }
    } else {
        eprintln!("[ERROR] Could not create snapshot file: {}", snapshot_path);
    }
}

