// edr-agent/replay/replay_trigger.rs

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::session_trust_curve::SessionTrustCurve;
use crate::services::vault_exporter::export_to_vault;

use lazy_static::lazy_static;


/// ✅ Static list of sensitive tags that always trigger replay
fn sensitive_tags_set() -> HashSet<&'static str> {
    [
        "privilege_escalation",
        "lolbin_usage",
        "root_logon",
        "cloud_exfil",
        "suspicious_network",
        "persistence_mechanism",
        "mfa_bypass",
        "malicious_usb",
    ]
    .iter()
    .copied()
    .collect()
}

/// ✅ Returns true if trust score is low or tags indicate high-risk behavior
pub fn should_trigger_replay(trust_score: f64, tags: &[String]) -> bool {
    let sensitive_tags = sensitive_tags_set();
    let tag_match = tags.iter().any(|t| sensitive_tags.contains(t.as_str()));
    trust_score < 30.0 || tag_match
}

/// ✅ If a trigger condition is met, initiates secure vault export for later forensic replay
pub fn handle_replay_trigger(
    session_id: &str,
    trust_score: f64,
    tags: &[String],
    session_logs: &[String],
) {
    if should_trigger_replay(trust_score, tags) {
        println!("[ALERT] Triggering forensic replay for session: {}", session_id);
        if let Err(e) = export_to_vault(session_id, session_logs) {
            eprintln!("[ERROR] Vault export failed: {}", e);
        }
    }
}

lazy_static::lazy_static! {
    static ref SESSION_CURVES: Arc<Mutex<HashMap<String, SessionTrustCurve>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

/// ✅ Tracks trust decay over time and checks for replay escalation
pub fn update_session_trust(session_id: &str, new_trust: f64, role: &str) -> bool {
    let mut map = SESSION_CURVES.lock().unwrap();

    let curve = map
        .entry(session_id.to_string())
        .or_insert_with(|| SessionTrustCurve::new(session_id, role));

    curve.update(new_trust, "trust_update");

    if curve.escalation_triggered() {
        println!(
            "[!] Trust decay triggered replay escalation for session: {}",
            session_id
        );
        return true;
    }

    false
}
/// ✅ Central dispatcher to check if replay should be triggered based on trust and behavior
pub fn trigger_replay_if_needed(
    session_id: &str,
    trust_score: f64,
    tags: &[String],
    session_logs: &[String],
    role: &str,
) {
    let escalation = update_session_trust(session_id, trust_score, role);
    if escalation || should_trigger_replay(trust_score, tags) {
        handle_replay_trigger(session_id, trust_score, tags, session_logs);
    }
}


