use crate::services::trust_vector::TrustVector;

#[derive(Debug)]
pub enum SuppressionDecision {
    SuppressReasonably(String),
    DoNotSuppress(String),
}

/// Helper to safely extract a trust dimension and convert to f32
fn get_trust_score(trust_vector: &TrustVector, key: &str) -> f32 {
    trust_vector
        .dimensions
        .get(key)
        .cloned()
        .unwrap_or(0.0f64) as f32
}

/// Determines whether a signal should be suppressed based on trust, tags, path, and user
pub fn should_suppress(
    path: &str,
    hash: Option<&String>,
    uid: u32,
    tags: &[String],
    cmdline: &str,
    trust_vector: &TrustVector,
) -> SuppressionDecision {
    // Strong override: never suppress if any critical tag is present
    let critical_tags = [
        "evasion::inject",
        "memory::rwx",
        "persistence::suspicious_autostart",
        "gnn_escalate",
        "file::high_entropy",
    ];
    if tags.iter().any(|t| critical_tags.iter().any(|ct| t.contains(ct))) {
        return SuppressionDecision::DoNotSuppress("Critical tag present".to_string());
    }

    // Extract relevant trust dimensions
    let trust_ok = get_trust_score(trust_vector, "process") >= 80.0
        && get_trust_score(trust_vector, "memory") >= 80.0
        && get_trust_score(trust_vector, "file") >= 80.0;

    // Heuristic checks
    let known_safe_paths = [
        "/usr/bin/",
        "/usr/lib/",
        "/bin/",
        "/opt/firefox/",
    ];
    let path_safe = known_safe_paths.iter().any(|p| path.starts_with(p));
    let uid_safe = uid > 1000;
    let has_tags = !tags.is_empty();

    // Suppression logic
    if path_safe && uid_safe && trust_ok && !has_tags {
        SuppressionDecision::SuppressReasonably("Known path + high trust + no tags".to_string())
    } else {
        SuppressionDecision::DoNotSuppress("One or more unsafe conditions present".to_string())
    }
}

