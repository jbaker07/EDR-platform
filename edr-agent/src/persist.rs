use anyhow::Result;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;

use crate::telemetry::TelemetryRecord;
use crate::trust_hook::TrustEvent;

pub fn persist<T>(events: &[TrustEvent], _records: &[TelemetryRecord], _data: &T) -> Result<()> {
    // Ensure state dir exists
    create_dir_all("state")?;

    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open("state/alerts.jsonl")?;

    for ev in events {
        // Build a compact, schema-stable JSON object from known fields.
        // We avoid requiring Serialize on TrustEvent by constructing a JSON Value manually.
        let obj = serde_json::json!({
            "ts": ev.timestamp,                    // u64
            "pid": ev.pid,                         // i32
            "ppid": ev.ppid,                       // i32
            "uid": ev.uid,                         // u32
            "anomaly_type": ev.anomaly_type,       // String
            "component": ev.component,             // String
            "risk_score": ev.risk_score,           // f32
            "binary_path": ev.binary_path,         // String
            "command_line": ev.command_line,       // String
            "cwd": ev.cwd,                         // String
            "description": ev.description,         // Option<String>
            "tags": ev.tags,                       // Option<Vec<String>>
            "metadata": ev.metadata,               // HashMap<String,String>
            // optional extras if present in your struct:
            "signal": ev.signal,                   // Option<String>
            "signal_type": ev.signal_type,         // Option<String>
            "module": ev.module,                   // Option<String>
            "decay_context": ev.decay_context,     // Option<String>
            "score": ev.score,                     // Option<f32>
            "raw_score": ev.raw_score,             // Option<f32>
            "source_module": ev.source_module,     // String
        });

        writeln!(f, "{}", obj.to_string())?;
    }

    Ok(())
}
