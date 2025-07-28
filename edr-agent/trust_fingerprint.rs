use sha2::{Sha256, Digest};
use std::collections::HashMap;

/// Generates a trust fingerprint from key telemetry and tag fields.
/// This is used for trust delta detection, fingerprint comparison, etc.
pub fn generate_fingerprint(
    hostname: &str,
    pid: u32,
    trust_score: f64,
    gnn_score: f64,
    tags: &[String],
    telemetry: &HashMap<String, String>,
) -> String {
    let mut hasher = Sha256::new();

    hasher.update(hostname.as_bytes());
    hasher.update(pid.to_le_bytes());
    hasher.update(trust_score.to_le_bytes());
    hasher.update(gnn_score.to_le_bytes());
    for tag in tags {
        hasher.update(tag.as_bytes());
    }
    for (k, v) in telemetry {
        hasher.update(k.as_bytes());
        hasher.update(v.as_bytes());
    }

    let hash = hasher.finalize();
    format!("{:x}", hash)
}
