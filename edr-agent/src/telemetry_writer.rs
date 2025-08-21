use crate::telemetry_types::{TelemetryEvent, MemoryAnomalyType};
use crate::telemetry::TelemetryRecord;
use crate::utils::time::now_ts;

use std::collections::HashMap;
use std::env;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Default limits and paths; can be overridden by env:
/// - EDR_TELEMETRY_DIR (default: telemetry_output)
/// - EDR_TELEMETRY_MAX_BUFFER (default: 10000)
const DEFAULT_DIR: &str = "telemetry_output";
const DEFAULT_MAX_BUFFER: usize = 10_000;
const DROP_FRACTION_ON_PRESSURE: f32 = 0.10; // drop oldest 10% when full

pub struct TelemetryWriter {
    buffer: Mutex<Vec<TelemetryRecord>>,
    dir: PathBuf,
    max_buffer: usize,
}

impl TelemetryWriter {
    /// Create a writer using defaults or env overrides.
    pub fn new() -> Self {
        let dir = PathBuf::from(env::var("EDR_TELEMETRY_DIR").unwrap_or_else(|_| DEFAULT_DIR.into()));
        // Best effort to create before first flush; we still try in flush_to_disk too.
        let _ = create_dir_all(&dir);

        let max_buffer = env::var("EDR_TELEMETRY_MAX_BUFFER")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(DEFAULT_MAX_BUFFER);

        TelemetryWriter {
            buffer: Mutex::new(Vec::new()),
            dir,
            max_buffer,
        }
    }

    /// Append a single record into the in-memory buffer (backpressure aware).
    pub fn append(&self, event: TelemetryRecord) {
        if let Ok(mut buffer) = self.buffer.lock() {
            Self::maybe_apply_backpressure(&mut buffer, self.max_buffer);
            buffer.push(event);
        }
    }

    /// Append a batch of records.
    pub fn append_batch(&self, mut records: Vec<TelemetryRecord>) {
        if let Ok(mut buffer) = self.buffer.lock() {
            // If the incoming batch is larger than capacity, keep tail end (most recent).
            if records.len() > self.max_buffer {
                let keep = self.max_buffer;
                records = records.split_off(records.len().saturating_sub(keep));
            }
            // Backpressure to make room if needed.
            let needed = records.len();
            if buffer.len() + needed > self.max_buffer {
                let over = buffer.len() + needed - self.max_buffer;
                let drop_n = over.max((self.max_buffer as f32 * DROP_FRACTION_ON_PRESSURE).ceil() as usize);
                if drop_n > 0 && drop_n <= buffer.len() {
                    buffer.drain(0..drop_n);
                } else if drop_n > buffer.len() {
                    buffer.clear();
                }
            }
            buffer.extend(records);
        }
    }

    /// Historical alias; identical to `append`.
    pub fn write_record(&self, record: TelemetryRecord) {
        self.append(record);
    }

    /// Another alias used by some modules.
    pub fn send(&self, record: TelemetryRecord) {
        self.append(record);
    }

    /// Spawns a background thread to flush every 30s (unchanged signature).
    pub fn start_periodic_flush(self: Arc<Self>) {
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(30));
            self.flush_to_disk();
        });
    }

    /// Flush buffered `TelemetryRecord`s to a JSONL file.
    pub fn flush_to_disk(&self) {
        let mut buffer = match self.buffer.lock() {
            Ok(guard) => guard,
            Err(poison) => {
                eprintln!("⚠️ [TelemetryWriter] Buffer mutex poisoned; recovering.");
                poison.into_inner()
            }
        };
        if buffer.is_empty() {
            return;
        }

        if let Err(e) = create_dir_all(&self.dir) {
            eprintln!("❌ [TelemetryWriter] Failed to ensure output dir {:?}: {e}", self.dir);
            return;
        }

        let filename = self.make_file_name("telemetry");
        if let Err(e) = Self::write_jsonl(&filename, buffer.drain(..)) {
            eprintln!("❌ [TelemetryWriter] Flush failed to {:?}: {e}", filename);
        } else {
            println!("[📝 Telemetry] Flushed events to {:?}", filename);
        }
    }

    /// No-op for API compatibility; actual work happens in `flush_to_disk`.
    pub fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    /// Builder for modules needing structured record creation (unchanged).
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

    // ---------------- internal helpers ----------------

    fn maybe_apply_backpressure(buf: &mut Vec<TelemetryRecord>, max: usize) {
        if buf.len() >= max {
            let drop_n = ((buf.len() as f32) * DROP_FRACTION_ON_PRESSURE).ceil() as usize;
            if drop_n > 0 && drop_n <= buf.len() {
                buf.drain(0..drop_n.min(buf.len()));
                eprintln!(
                    "⚠️ [TelemetryWriter] Buffer at capacity; dropped {} oldest events (len now {}).",
                    drop_n,
                    buf.len()
                );
            }
        }
    }

    fn make_file_name(&self, prefix: &str) -> PathBuf {
        // Keep timestamp-based naming for full backward-compat.
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        self.dir.join(format!("{prefix}_{ts}.jsonl"))
    }

    fn write_jsonl<I>(path: &Path, iter: I) -> std::io::Result<()>
    where
        I: IntoIterator<Item = TelemetryRecord>,
    {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        let mut writer = BufWriter::new(file);
        for event in iter {
            let line = serde_json::to_string(&event).unwrap_or_else(|e| {
                // Fallback minimal line if serialization fails (should be rare).
                format!(r#"{{"error":"serialize","msg":"{}","ts":{}}}"#, e, now_ts())
            });
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
        }
        writer.flush()?;
        Ok(())
    }

    /// Lightweight helper to persist simple key/value “telemetry records”
    /// (used widely across modules). Writes JSONL alongside the structured logs.
    fn write_kv_jsonl(path: &Path, map: &HashMap<String, String>) -> std::io::Result<()> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        let mut writer = BufWriter::new(file);
        let line = serde_json::to_string(map).unwrap_or_else(|e| {
            format!(r#"{{"error":"kv_serialize","msg":"{}","ts":{}}}"#, e, now_ts())
        });
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }
}

impl Drop for TelemetryWriter {
    fn drop(&mut self) {
        // Best effort final flush on shutdown.
        if let Ok(mut buf) = self.buffer.lock() {
            if buf.is_empty() {
                return;
            }
            if let Err(e) = create_dir_all(&self.dir) {
                eprintln!("❌ [TelemetryWriter] Drop flush: failed to ensure dir {:?}: {e}", self.dir);
                return;
            }
            let filename = self.make_file_name("telemetry_final");
            if let Err(e) = TelemetryWriter::write_jsonl(&filename, buf.drain(..)) {
                eprintln!("❌ [TelemetryWriter] Drop flush failed to {:?}: {e}", filename);
            } else {
                println!("[📝 Telemetry] Final flush to {:?}", filename);
            }
        }
    }
}

/// Pushes a high-risk memory anomaly directly to disk (independent of buffer).
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

    // Maintain backward-compat target file and also mirror into output dir.
    let serialized = serde_json::to_string(&record)?;
    {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("telemetry_log.jsonl")?;
        writeln!(file, "{}", serialized)?;
    }

    // Best-effort mirror into structured dir—don’t fail the call if this errors.
    if let Err(e) = (|| -> std::io::Result<()> {
        create_dir_all(DEFAULT_DIR)?;
        let path = Path::new(DEFAULT_DIR).join("memory_anomalies.jsonl");
        let mut writer = BufWriter::new(OpenOptions::new().create(true).append(true).open(path)?);
        writer.write_all(serialized.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    })() {
        eprintln!("⚠️ [TelemetryWriter] Mirror memory log failed: {e}");
    }

    Ok(())
}

/// Debug-style fallback printer for basic telemetry maps.
/// Now also writes a KV JSONL file next to structured telemetry.
pub fn write_telemetry_record(record: HashMap<String, String>) {
    println!("📝 Telemetry record: {:?}", record);

    if let Err(e) = (|| -> std::io::Result<()> {
        create_dir_all(DEFAULT_DIR)?;
        let path = Path::new(DEFAULT_DIR).join("kv_telemetry.jsonl");
        TelemetryWriter::write_kv_jsonl(&path, &record)
    })() {
        eprintln!("⚠️ [TelemetryWriter] write_telemetry_record persist failed: {e}");
    }
}
