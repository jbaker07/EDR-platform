// src/utils/baseline_filter.rs (drop-in improved version)

use crate::services::trust_vector::{TrustDim, TrustVector};
use std::collections::HashSet;

/// Indicates whether an alert or telemetry signal should be suppressed.
#[derive(Debug)]
pub enum SuppressionDecision {
    SuppressReasonably(String),
    DoNotSuppress(String),
}

/// Loads known-safe fingerprints. Each line may be:
///   - "path:hash:uid"
///   - "path,hash,uid"   (CSV tolerated)
///   - Empty lines or lines starting with '#' are ignored.
pub fn load_known_safe_fingerprints(path: &str) -> HashSet<String> {
    std::fs::read_to_string(path)
        .map(|content| {
            content
                .lines()
                .filter_map(|line| {
                    let s = line.trim();
                    if s.is_empty() || s.starts_with('#') {
                        return None;
                    }
                    // Accept CSV "a,b,c" -> "a:b:c"
                    if s.contains(',') && !s.contains(':') {
                        let mut parts = s.split(',').map(|x| x.trim());
                        let p = parts.next()?;
                        let h = parts.next()?;
                        let u = parts.next()?;
                        return Some(format!("{p}:{h}:{u}"));
                    }
                    Some(s.to_string())
                })
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default()
}

/// Core suppression logic based on context
///
/// IMPORTANT: TrustVector values are in [0.0, 1.0].
pub fn should_suppress_signal(
    path: &str,
    hash: Option<&String>,
    uid: u32,
    tags: &[String],
    cmdline: &str,
    trust_vector: &TrustVector,
    known_safe_set: &HashSet<String>,
) -> SuppressionDecision {
    // ---------------- Immediate Do-Not-Suppress gates ----------------
    if has_critical_tag(tags) {
        return SuppressionDecision::DoNotSuppress("Critical/override tag present".into());
    }
    if is_ephemeral_path(path) {
        return SuppressionDecision::DoNotSuppress("Ephemeral path (tmp/shm)".into());
    }
    if is_cmdline_suspicious(cmdline) {
        return SuppressionDecision::DoNotSuppress("Command line contains suspicious patterns".into());
    }

    // ---------------- Known-good fingerprint match -------------------
    if let Some(h) = hash {
        let triple = format!("{path}:{h}:{uid}");
        if known_safe_set.contains(&triple) {
            return SuppressionDecision::SuppressReasonably("Fingerprint triple matches baseline".into());
        }
    }

    // ---------------- Trust-based evaluation (0.0..1.0) --------------
    // Consider a signal suppressible when high trust is maintained in the most
    // relevant dimensions and no suspicious hints are present.
    let mem_ok = dim_ge(trust_vector, TrustDim::Memory, 0.85);
    let proc_ok = dim_ge(trust_vector, TrustDim::Process, 0.85);
    let fs_ok = dim_ge(trust_vector, TrustDim::FileSys, 0.85);

    // Slightly lower threshold for supporting dimensions:
    let net_ok = dim_ge(trust_vector, TrustDim::Network, 0.80);
    let priv_ok = dim_ge(trust_vector, TrustDim::Privilege, 0.80);

    // If overall is strong and there are no explicitly suspicious tags,
    // we can consider suppression *when* the path looks standard and UID is non-root.
    let no_suspicious_tags = !tags.iter().any(|t| tag_suspicious_hint(t));

    let path_looks_standard = is_standard_system_path(path);
    let is_unpriv_uid = uid >= 1000 && uid < 60000;

    // ---------------- Decision matrix --------------------------------
    if path_looks_standard
        && is_unpriv_uid
        && no_suspicious_tags
        && (mem_ok && proc_ok && fs_ok)
        && (net_ok || priv_ok)
    {
        return SuppressionDecision::SuppressReasonably(
            "High trust across core dims, standard path, unprivileged UID, no suspicious tags".into(),
        );
    }

    // Additional soft allow: known-safe path prefixes with strong trust
    if path_looks_standard && (mem_ok && proc_ok) && no_suspicious_tags {
        return SuppressionDecision::SuppressReasonably(
            "Strong trust and standard path with no suspicious tags".into(),
        );
    }

    // Default to not suppressing to avoid missed detections.
    SuppressionDecision::DoNotSuppress("One or more safety conditions not met".into())
}

// ---------------- helpers ----------------

#[inline]
fn dim_ge(tv: &TrustVector, dim: TrustDim, thr: f32) -> bool {
    // TrustVector.v ∈ [0,1].
    tv.v[dim.idx()] >= thr
}

fn has_critical_tag(tags: &[String]) -> bool {
    // Any of these (or substrings) should hard-block suppression.
    // Keep lowercase compare for safety.
    let crit: &[&str] = &[
        "gnn_escalate",
        "signal::override",
        "evasion::inject",
        "memory::rwx",
        "persistence::suspicious_autostart",
        "file::high_entropy",
        "forensic_replay",
    ];
    let tags_lc = tags.iter().map(|t| t.to_ascii_lowercase()).collect::<Vec<_>>();
    tags_lc.iter().any(|t| crit.iter().any(|c| t.contains(c)))
}

fn tag_suspicious_hint(tag: &str) -> bool {
    // Softer hints that nudge us away from suppression
    let t = tag.to_ascii_lowercase();
    t.contains("inject")
        || t.contains("cred_dump")
        || t.contains("persistence")
        || t.contains("dns_tunnel")
        || t.contains("proxy")
        || t.contains("exfil")
        || t.contains("ransom")
        || t.contains("mfa_bypass")
}

fn is_standard_system_path(path: &str) -> bool {
    // Common system/library locations (not foolproof)
    const SAFE_PREFIXES: &[&str] = &[
        "/usr/bin/",
        "/usr/lib/",
        "/usr/lib64/",
        "/lib/",
        "/lib64/",
        "/lib/systemd/system/",
        "/usr/share/",
        "/opt/",
        "/sbin/",
        "/usr/sbin/",
    ];
    SAFE_PREFIXES.iter().any(|p| path.starts_with(p))
}

fn is_ephemeral_path(path: &str) -> bool {
    const EPHEMERAL: &[&str] = &["/tmp/", "/var/tmp/", "/dev/shm/"];
    EPHEMERAL.iter().any(|p| path.starts_with(p))
}

fn is_cmdline_suspicious(cmdline: &str) -> bool {
    // Lightweight heuristics avoiding overreach; keep false positives low.
    // Examples: explicit inject flags, suspicious base64 execution chains, curl|sh piping.
    let lc = cmdline.to_ascii_lowercase();
    lc.contains("--inject")
        || lc.contains(" ptrace ")
        || lc.contains("LD_PRELOAD=")
        || lc.contains("base64 -d")
        || lc.contains("bash -c")
        || (lc.contains("curl ") && lc.contains("| sh"))
        || (lc.contains("wget ") && lc.contains("| sh"))
}
