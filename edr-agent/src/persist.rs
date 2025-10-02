use anyhow::{Context, Result};
use std::fs::{create_dir_all, metadata, read_dir, remove_file, rename, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::telemetry::TelemetryRecord;
use crate::trust_hook::TrustEvent;

const ALERTS_PATH: &str = "logs/alerts.jsonl";
const LOGS_DIR: &str = "logs";
const MAX_BYTES: u64 = 200 * 1024 * 1024; // rotate at ~200MB
const KEEP_ROTATED: usize = 10; // keep last 10 rotated files

pub fn persist<T>(events: &[TrustEvent], _records: &[TelemetryRecord], _data: &T) -> Result<()> {
    if events.is_empty() {
        return Ok(());
    }

    // Ensure dirs exist
    create_dir_all(LOGS_DIR).context("creating logs/ directory")?;
    // Rotate if the active file is too large
    rotate_if_needed(ALERTS_PATH).ok(); // best-effort; don't fail persistence on rotation issues
                                        // Housekeep older rotated files (best-effort)
    prune_rotated(LOGS_DIR).ok();

    // Append
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(ALERTS_PATH)
        .with_context(|| format!("opening {}", ALERTS_PATH))?;

    let mut w = BufWriter::new(file);

    for ev in events {
        // We don't rely on TrustEvent implementing Serialize or specific fields.
        // Store a minimal, schema-stable JSON line with a debug dump.
        let obj = serde_json::json!({
            "ts": chrono::Utc::now().timestamp(),
            "source": "trust_engine",
            "event_debug": format!("{:?}", ev),
        });
        writeln!(w, "{}", obj).context("writing alert line")?;
    }

    w.flush().ok(); // best-effort
    Ok(())
}

fn rotate_if_needed(path: &str) -> Result<()> {
    if let Ok(md) = metadata(path) {
        if md.len() >= MAX_BYTES {
            // e.g., logs/alerts-20250101T120102.jsonl
            let ts = chrono::Utc::now().format("%Y%m%dT%H%M%S");
            let rotated = format!("{}/alerts-{}.jsonl", LOGS_DIR, ts);

            // Try to rename (fast). If it fails, fall back to truncation.
            if rename(path, &rotated).is_err() {
                // Truncate active file if rename failed (e.g., cross-device weirdness)
                let _ = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(path)?;
            }
        }
    }
    Ok(())
}

fn prune_rotated(dir: &str) -> Result<()> {
    // Gather rotated files that match alerts-*.jsonl
    let mut files: Vec<_> = read_dir(dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            if let Ok(ft) = e.file_type() {
                ft.is_file()
            } else {
                false
            }
        })
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.starts_with("alerts-") && name.ends_with(".jsonl")
        })
        .map(|e| {
            let path = e.path();
            let mtime = path
                .metadata()
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| t.elapsed().ok())
                .map(|el| el.as_secs())
                .unwrap_or(u64::MAX);
            (mtime, path)
        })
        .collect();

    // Sort by age (newest first => smallest elapsed)
    files.sort_by_key(|(elapsed, _)| *elapsed);

    // Remove oldest beyond KEEP_ROTATED
    if files.len() > KEEP_ROTATED {
        for (_, p) in files.into_iter().skip(KEEP_ROTATED) {
            let _ = remove_file(&p);
        }
    }
    Ok(())
}
