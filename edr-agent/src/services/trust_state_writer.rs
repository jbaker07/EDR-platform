// edr-agent/src/services/trust_state_writer.rs

use std::collections::HashMap;
use std::fs::{create_dir_all, File, read_dir};
use std::io::{Write, BufReader, BufRead};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Serialize, Deserialize};

use crate::services::decay_engine::{TrustState, TrustDecayManager};
use crate::gnn_hook::push_gnn_event;

const TRUST_STATE_PATH: &str = "/var/log/edr/trust_cache/";

/// Serializable wrapper for disk storage
#[derive(Debug, Serialize, Deserialize)]
struct PersistedTrustCache {
    exported_at: i64,
    entries: HashMap<String, TrustState>,
}

/// ✅ Background initialization log
pub fn start_trust_state_writer() {
    println!("✅ Trust state writer started.");
}

/// ✅ Per-endpoint trust persistence (used in real-time scoring)
pub fn persist_trust_state(endpoint_id: &str, score: f64) -> std::io::Result<()> {
    create_dir_all(TRUST_STATE_PATH)?;

    let filename = format!(
        "trust_{}_{}.json",
        endpoint_id,
        Utc::now().format("%Y%m%d_%H%M%S")
    );
    let full_path = PathBuf::from(TRUST_STATE_PATH).join(filename);

    let state = TrustState {
        last_observed: Utc::now().timestamp() as u64,
        last_score: score.clamp(0.0, 1.0),
        decay_half_life: 3600, // Default 1-hour half-life or make configurable
    };

    let json = serde_json::to_string_pretty(&state)?;
    let mut file = File::create(full_path)?;
    file.write_all(json.as_bytes())?;

    println!("[📝 Trust Writer] Wrote trust score for {}", endpoint_id);

    // ✅ Push to GNN as a time-series trust update
    push_gnn_event(endpoint_id, score, "trust_tick");

    Ok(())
}

/// ✅ Forensic snapshot tied to specific event or reason
pub fn write_trust_state_snapshot(
    endpoint_id: &str,
    trust_state: &TrustState,
    tags: &[String],
    reason: &str,
) -> std::io::Result<()> {
    create_dir_all(TRUST_STATE_PATH)?;

    let clean_reason = reason.replace(' ', "_").to_lowercase();
    let filename = format!(
        "trust_snapshot_{}_{}_{}.json",
        endpoint_id,
        Utc::now().format("%Y%m%d_%H%M%S"),
        clean_reason
    );

    let full_path = PathBuf::from(TRUST_STATE_PATH).join(filename);
    let snapshot = serde_json::to_string_pretty(&trust_state)?;
    let mut file = File::create(full_path)?;
    file.write_all(snapshot.as_bytes())?;

    println!(
        "[📸 Trust Snapshot] Captured snapshot for {} due to: {}",
        endpoint_id, reason
    );

    // Optionally push to GNN (e.g., root cause breadcrumb or marker)
    push_gnn_event(endpoint_id, trust_state.last_score, &format!("snapshot::{reason}"));

    Ok(())
}

/// ✅ Batch persistence for full trust cache snapshots (used in background thread)
pub fn persist_trust_snapshot(cache: &HashMap<String, TrustState>) -> std::io::Result<()> {
    create_dir_all(TRUST_STATE_PATH)?;

    let filename = format!(
        "trust_state_{}.json.gz",
        Utc::now().format("%Y%m%d_%H%M%S")
    );
    let full_path = PathBuf::from(TRUST_STATE_PATH).join(filename);

    let snapshot = PersistedTrustCache {
        exported_at: Utc::now().timestamp(),
        entries: cache.clone(),
    };

    let json = serde_json::to_vec(&snapshot)?;
    let file = File::create(full_path)?;
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder.write_all(&json)?;
    encoder.finish()?;

    println!(
        "[📝 Trust Writer] Snapshot written with {} entries.",
        snapshot.entries.len()
    );

    Ok(())
}

/// ✅ Background task for periodic persistence every 5 minutes
pub fn start_periodic_writer(mut manager: TrustDecayManager) {
    thread::spawn(move || loop {
        let snapshot = manager.export_trust_state();
        if let Err(e) = persist_trust_snapshot(&snapshot) {
            eprintln!("[⚠️ Trust Writer] Failed to persist trust state: {}", e);
        }
        thread::sleep(Duration::from_secs(300));
    });
}

/// ✅ Load latest trust cache snapshot on startup
pub fn load_latest_trust_state() -> Option<HashMap<String, TrustState>> {
    let dir = PathBuf::from(TRUST_STATE_PATH);
    if let Ok(entries) = read_dir(&dir) {
        let mut files: Vec<_> = entries
            .flatten()
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "gz"))
            .collect();

        files.sort_by_key(|f| f.path());
        if let Some(latest) = files.last() {
            let file = File::open(latest.path()).ok()?;
            let mut decoder = flate2::read::GzDecoder::new(file);
            let reader = BufReader::new(&mut decoder);
            let snapshot: PersistedTrustCache = serde_json::from_reader(reader).ok()?;
            return Some(snapshot.entries);
        }
    }
    None
}

/// ✅ Load trust state from a flat JSON file (fallback)
pub fn load_trust_state(path: &str) -> HashMap<String, TrustState> {
    let file = File::open(path);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            match serde_json::from_reader(reader) {
                Ok(map) => map,
                Err(e) => {
                    eprintln!(
                        "[⚠️ Trust Writer] Failed to parse trust state from {}: {}",
                        path, e
                    );
                    HashMap::new()
                }
            }
        }
        Err(e) => {
            eprintln!(
                "[⚠️ Trust Writer] Could not open trust state file {}: {}",
                path, e
            );
            HashMap::new()
        }
    }
}
