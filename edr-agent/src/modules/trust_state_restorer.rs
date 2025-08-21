//! trust_state_restorer.rs
//!
//! Persist and restore per-endpoint TrustVectors with decay and conservative merging.
//! - Atomic snapshot writes (tmp + rename)
//! - Backward-compatible load
//! - Applies decay based on saved_at → now
//! - Merges into TRUST_VECTOR_GLOBAL using `merge_min`
//!
//! Typical usage on startup:
//!     let _ = restore_trust_state_from("/var/lib/edr/trust_state.json", 6.0 * 3600.0);
//!
//! Periodic snapshot (e.g., every few minutes):
//!     let _ = snapshot_trust_state_to("/var/lib/edr/trust_state.json");

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::services::trust_vector::{TrustVector, TRUST_VECTOR_GLOBAL, TRUST_DIM_CT};
use crate::utils::time::now_ts;

/// On-disk snapshot for a single endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrustVectorSnapshot {
    /// Trust values in [0,1]
    v: [f32; TRUST_DIM_CT],
    /// Optional tag echoes for diagnostics
    #[serde(default)]
    tags: Vec<String>,
    /// When this snapshot was written (secs since epoch)
    saved_at: u64,
}

/// On-disk file format (versioned).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrustStateFile {
    version: u32,
    saved_at: u64,
    endpoints: HashMap<String, TrustVectorSnapshot>,
}

impl Default for TrustStateFile {
    fn default() -> Self {
        Self {
            version: 1,
            saved_at: now_ts(),
            endpoints: HashMap::new(),
        }
    }
}

/// Restore global trust state from `path`.
/// Applies decay using `half_life_s` (seconds) between `saved_at` and now.
/// Returns number of endpoints restored on success.
///
/// If the file does not exist, returns Ok(0).
pub fn restore_trust_state_from(path: &str, half_life_s: f32) -> std::io::Result<usize> {
    let p = Path::new(path);
    if !p.exists() {
        return Ok(0);
    }

    let mut buf = String::new();
    File::open(p)?.read_to_string(&mut buf)?;
    if buf.trim().is_empty() {
        return Ok(0);
    }

    // Primary format
    let parsed: Result<TrustStateFile, _> = serde_json::from_str(&buf);

    // Backward-compat shim: older dumps might just be endpoint -> { v, tags, saved_at }
    let state = match parsed {
        Ok(v) => v,
        Err(_) => {
            let legacy: HashMap<String, TrustVectorSnapshot> =
                serde_json::from_str(&buf).map_err(io_err)?;
            TrustStateFile {
                version: 1,
                saved_at: now_ts(),
                endpoints: legacy,
            }
        }
    };

    let now = now_ts() as f32;
    let mut restored = 0usize;

    if let Ok(mut global) = TRUST_VECTOR_GLOBAL.lock() {
        for (endpoint, snap) in state.endpoints.iter() {
            let mut tv = TrustVector::new();
            tv.v = sanitize_vec(&snap.v);
            tv.tags = snap.tags.clone();
            tv.recompute_cache();

            // Decay deficits toward 1.0 based on wall time since saved_at
            let dt_s = (now_ts().saturating_sub(snap.saved_at)) as f32;
            if half_life_s > 0.0 && dt_s > 0.0 {
                tv.apply_decay(dt_s, half_life_s);
            }

            // Merge conservatively with any existing value (min per-dimension)
            if let Some(existing) = global.get_mut(endpoint) {
                existing.merge_min(&tv);
                existing.recompute_cache();
            } else {
                global.insert(endpoint.clone(), tv);
            }

            restored += 1;
        }
    }

    Ok(restored)
}

/// Snapshot the current `TRUST_VECTOR_GLOBAL` map to `path` atomically.
/// Writes to `<path>.tmp` then renames over `<path>`.
pub fn snapshot_trust_state_to(path: &str) -> std::io::Result<()> {
    let parent = Path::new(path).parent().map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from("."));
    fs::create_dir_all(&parent)?;

    let mut file_model = TrustStateFile::default();
    file_model.version = 1;
    file_model.saved_at = now_ts();

    if let Ok(global) = TRUST_VECTOR_GLOBAL.lock() {
        for (endpoint, tv) in global.iter() {
            // Clamp just in case; we skip per-dimension timestamps in snapshots
            let v = sanitize_vec(&tv.v);
            file_model.endpoints.insert(
                endpoint.clone(),
                TrustVectorSnapshot {
                    v,
                    tags: tv.tags.clone(),
                    saved_at: file_model.saved_at,
                },
            );
        }
    }

    let tmp_path = format!("{}.tmp", path);
    {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&tmp_path)?;
        let payload = serde_json::to_vec_pretty(&file_model).map_err(io_err)?;
        f.write_all(&payload)?;
        f.sync_all()?; // durability before rename
    }

    // Atomic replace
    fs::rename(&tmp_path, path)?;
    Ok(())
}

/// Convenience helper to get a clone of an endpoint's TrustVector.
/// Returns a default (fully trusted) vector if missing.
pub fn get_endpoint_trust(endpoint_id: &str) -> TrustVector {
    TRUST_VECTOR_GLOBAL
        .lock()
        .ok()
        .and_then(|m| m.get(endpoint_id).cloned())
        .unwrap_or_default()
}

/// Convenience helper to mutate an endpoint's TrustVector atomically.
/// Creates a default vector if not present.
pub fn with_endpoint_trust<F, T>(endpoint_id: &str, f: F) -> Option<T>
where
    F: FnOnce(&mut TrustVector) -> T,
{
    if let Ok(mut m) = TRUST_VECTOR_GLOBAL.lock() {
        let tv = m.entry(endpoint_id.to_string()).or_insert_with(TrustVector::new);
        let out = f(tv);
        tv.recompute_cache();
        Some(out)
    } else {
        None
    }
}

/// Clamp all values into [0,1] to avoid drift from bad inputs.
fn sanitize_vec(v: &[f32; TRUST_DIM_CT]) -> [f32; TRUST_DIM_CT] {
    let mut out = *v;
    for x in &mut out {
        *x = x.clamp(0.0, 1.0);
    }
    out
}

fn io_err<E: std::error::Error + Send + Sync + 'static>(e: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, e)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn snapshot_and_restore_roundtrip() {
        let tmp = "/tmp/trust_state_restorer_test.json";
        {
            let _ = fs::remove_file(tmp);
        }

        // Seed global with two endpoints
        with_endpoint_trust("hostA", |tv| {
            tv.penalty(crate::services::trust_vector::TrustDim::Memory, 0.2);
        });
        with_endpoint_trust("hostB", |tv| {
            tv.penalty(crate::services::trust_vector::TrustDim::Network, 0.1);
        });

        snapshot_trust_state_to(tmp).unwrap();

        // Nuke global and reload
        if let Ok(mut m) = TRUST_VECTOR_GLOBAL.lock() {
            m.clear();
        }
        let restored = restore_trust_state_from(tmp, 0.0).unwrap();
        assert_eq!(restored, 2);

        let a = get_endpoint_trust("hostA");
        let b = get_endpoint_trust("hostB");
        assert!(a.deficit_by(crate::services::trust_vector::TrustDim::Memory) > 0.0);
        assert!(b.deficit_by(crate::services::trust_vector::TrustDim::Network) > 0.0);
    }
}
