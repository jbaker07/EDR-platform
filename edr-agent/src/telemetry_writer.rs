use crate::telemetry_types::{TelemetryEvent, MemoryAnomalyType};
use crate::telemetry::TelemetryRecord;
use crate::utils::time::now_ts;

use std::sync::{Arc, Mutex};
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::time::{Duration, SystemTime};
use std::thread;
use std::collections::HashMap;

pub struct TelemetryWriter {
    buffer: Mutex<Vec<TelemetryRecord>>,
}

impl TelemetryWriter {
    pub fn new() -> Self {
        create_dir_all("telemetry_output").ok();
        TelemetryWriter {
            buffer: Mutex::new(Vec::new()),
        }
    }

    pub fn append(&self, event: TelemetryRecord) {
        if let Ok(mut buffer) = self.buffer.lock() {
            buffer.push(event);
        }
    }

    pub fn append_batch(&self, records: Vec<TelemetryRecord>) {
        if let Ok(mut buffer) = self.buffer.lock() {
            buffer.extend(records);
        }
    }

    pub fn write_record(&self, record: TelemetryRecord) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.push(record);
    }

    pub fn send(&self, record: TelemetryRecord) {
        self.append(record);
    }

    pub fn start_periodic_flush(self: Arc<Self>) {
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(30));
            self.flush_to_disk();
        });
    }

    pub fn flush_to_disk(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        if buffer.is_empty() {
            return;
        }

        let timestamp = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let filename = format!("telemetry_output/telemetry_{}.json", timestamp);
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&filename) {
            for event in buffer.drain(..) {
                if let Ok(json) = serde_json::to_string(&event) {
                    let _ = writeln!(file, "{}", json);
                }
            }
        }

        println!("[📝 Telemetry] Flushed events to {}", filename);
    }

    /// Flush adapter used in main.rs
    pub fn flush(&mut self) -> std::io::Result<()> {
        // Flush is deferred to flush_to_disk
        Ok(())
    }

    /// Optional builder for modules needing structured record creation
    pub fn build(data: HashMap<String, String>, risk_score: Option<u32>, tags: Vec<String>) -> TelemetryRecord {
        TelemetryRecord {
            timestamp: now_ts(),
            pid: data.get("pid").and_then(|s| s.parse().ok()).unwrap_or(0),
            ppid: data.get("ppid").and_then(|s| s.parse().ok()).unwrap_or(0),
            uid: data.get("uid").and_then(|s| s.parse().ok()).unwrap_or(0),
            binary_path: data.get("binary_path").cloned().unwrap_or_default(),
            command_line: data.get("command_line").cloned().unwrap_or_default(),
            cwd: data.get("cwd").cloned().unwrap_or_default(),
            env_vars: None,
            tags,
            risk_score,
        }
    }
}

/// Pushes a high-risk memory anomaly directly to disk (independent of buffer)
pub fn push_memory_telemetry(
    pid: i32,
    ppid: i32,
    uid: u32,
    binary_path: &str,
    command_line: &str,
    cwd: &str,
    anomaly_type: MemoryAnomalyType,
    description: String,
) -> Result<(), std::io::Error> {
    let record = TelemetryRecord {
        timestamp: now_ts(),
        pid,
        ppid,
        uid,
        binary_path: binary_path.to_string(),
        command_line: command_line.to_string(),
        cwd: cwd.to_string(),
        env_vars: None,
        risk_score: Some(85),
        tags: vec![
            format!("anomaly:{:?}", anomaly_type),
            format!("desc:{}", description),
        ],
    };

    let serialized = serde_json::to_string(&record)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("telemetry_log.jsonl")?;

    writeln!(file, "{}", serialized)?;
    Ok(())
}

/// Debug-style fallback printer for basic telemetry maps
pub fn write_telemetry_record(record: HashMap<String, String>) {
    println!("📝 Telemetry record: {:?}", record);
}
