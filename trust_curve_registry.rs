use std::collections::HashMap;
use std::sync::Mutex;
use crate::services::session_trust_curve::SessionTrustCurve;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRUST_CURVE_REGISTRY: Mutex<HashMap<String, SessionTrustCurve>> = Mutex::new(HashMap::new());
}

/// Fetch or initialize a trust curve for a session
pub fn get_or_create_curve(session_id: &str, role: &str) -> SessionTrustCurve {
    let mut registry = TRUST_CURVE_REGISTRY.lock().unwrap();
    registry.entry(session_id.to_string())
        .or_insert_with(|| SessionTrustCurve::new(session_id, role))
        .clone()
}

/// Update trust score and log trend
pub fn update_curve(session_id: &str, role: &str, score: f64, event_type: &str) {
    let mut registry = TRUST_CURVE_REGISTRY.lock().unwrap();
    let curve = registry.entry(session_id.to_string())
        .or_insert_with(|| SessionTrustCurve::new(session_id, role));
    curve.update(score, event_type);
}
