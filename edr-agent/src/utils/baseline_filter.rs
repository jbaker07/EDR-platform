use crate::services::trust_vector::TrustVector;
use std::collections::HashSet;

/// Indicates whether an alert or telemetry signal should be suppressed.
#[derive(Debug)]
pub enum SuppressionDecision {
    SuppressReasonably(String),
    DoNotSuppress(String),
}

/// A loaded known baseline triple: path:hash:uid
pub fn load_known_safe_fingerprints(path: &str) -> HashSet<String> {
    std::fs::read_to_string(path)
        .map(|content| {
            content
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.trim().to_string())
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default()
}

/// Core suppression logic based on context
pub fn should_suppress_signal(
    path: &str,
    hash: Option<&String>,
    uid: u32,
    tags: &[String],
    cmdline: &str,
    trust_vector: &TrustVector,
    known_safe_set: &HashSet<String>,
) -> SuppressionDecision {
    // === Immediate No-Suppress Conditions ===
    let critical_tags = [
        "evasion::inject",
        "memory::rwx",
        "persistence::suspicious_autostart",
        "gnn_escalate",
        "file::high_entropy",
        "signal::override",
    ];
    if tags.iter().any(|t| critical_tags.iter().any(|ct| t.contains(ct))) {
        return SuppressionDecision::DoNotSuppress("Critical or override tag present".into());
    }

    // === Known Good Baseline ===
    if let Some(h) = hash {
        let triple = format!("{path}:{h}:{uid}");
        if known_safe_set.contains(&triple) {
            return SuppressionDecision::SuppressReasonably("Fingerprint in baseline set".into());
        }
    }

    // === Trust Vector Evaluation ===
    let trust_high = trust_vector.dimensions.get("process").cloned().unwrap_or(0.0) >= 85.0
        && trust_vector.dimensions.get("memory").cloned().unwrap_or(0.0) >= 85.0
        && trust_vector.dimensions.get("file").cloned().unwrap_or(0.0) >= 85.0;

    // === Path Heuristics ===
    let known_safe_paths = [
        "/usr/bin/",
        "/usr/lib/",
        "/lib/systemd/system/",
        "/opt/firefox/",
        "/usr/share/",
    ];
    let path_heuristically_safe = known_safe_paths.iter().any(|prefix| path.starts_with(prefix));

    let uid_safe = uid >= 1000 && uid < 60000;
    let tags_present = !tags.is_empty();
    let cmdline_sane = !cmdline.contains("--inject") && !cmdline.contains("base64");

    // === Suppression Decision Matrix ===
    if path_heuristically_safe && uid_safe && trust_high && !tags_present && cmdline_sane {
        SuppressionDecision::SuppressReasonably("Heuristic + trust + no flags".into())
    } else {
        SuppressionDecision::DoNotSuppress("Failed one or more conditions".into())
    }
}
