// edr-agent/src/modules/geo_ip_anomaly.rs

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::RwLock;
use std::time::{Duration, SystemTime};
use std::net::Ipv4Addr;
use serde::{Deserialize, Serialize};
use crate::telemetry_writer::write_telemetry_record;
use crate::gnn_hook::push_to_gnn_vector_log;
use std::sync::Mutex;
use crate::trust_hook::{submit_trust_event, TrustEvent};
use crate::modules::replay_writer::store_replay_event;
use crate::utils::time::now_ts;
use crate::telemetry_types::TelemetryOutput;
use chrono::Utc;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::services::trust_vector::{update_trust_vector, TrustVector};
use crate::services::trust_decay_audit::log_decay_reason;

lazy_static! {
    static ref GEO_ANOMALY_FOUND: AtomicBool = AtomicBool::new(false);
    static ref GEO_LOGS: Mutex<Vec<(String, String, u64)>> = Mutex::new(Vec::new()); // (user, geo, timestamp)
}

#[derive(Debug, Clone)]
pub struct LoginAttempt {
    pub user: String,
    pub country: String,
    pub timestamp: SystemTime,
    pub ip: String,
    pub device_fingerprint: Option<String>,
    pub sso_id: Option<String>,
    pub mfa_success: bool,

    // Optional telemetry for trust_event context
    pub pid: Option<i32>,
    pub ppid: Option<i32>,
    pub uid: Option<u32>,
}


#[derive(Debug, Clone)]
pub struct LoginHistory {
    pub country: String,
    pub timestamp: SystemTime,
    pub device_fingerprint: Option<String>,
    pub trust_score: f64,
    pub sso_id: Option<String>,
}

lazy_static! {
    static ref HISTORY: RwLock<HashMap<String, LoginHistory>> = RwLock::new(HashMap::new());
    static ref WHITELIST: RwLock<HashMap<String, Vec<String>>> = RwLock::new(HashMap::new());
}

fn is_vpn_or_anonymized(ip: &IpAddr) -> bool {
    let suspicious_ranges = vec!["185.", "45.129", "172.67", "104.244", "198.18", "192.42"];
    suspicious_ranges.iter().any(|prefix| ip.to_string().starts_with(prefix))
}

fn is_whitelisted(user: &str, country: &str) -> bool {
    if let Some(allowed) = WHITELIST.read().ok().and_then(|wl| wl.get(user).cloned()) {
        return allowed.contains(&country.to_string());
    }
    false
}

fn get_role_multiplier(user: &str) -> f64 {
    if user == "admin" || user.contains("ciso") || user.contains("finance") {
        1.5
    } else {
        1.0
    }
}

fn log_geo_ip_anomaly(user: &str, country: &str) {
    println!("Geo-IP anomaly detected: user={}, country={}", user, country);
}


pub fn detect_geo_anomaly(
    login: LoginAttempt,
    trust_vector: &mut TrustVector,
) -> Result<(), Box<dyn std::error::Error>> {
    if is_whitelisted(&login.user, &login.country) {
        return Ok(());
    }

    let mut history = HISTORY.write().map_err(|e| format!("Lock error: {e}"))?;
    let prev_entry = history.get(&login.user).cloned();
    let now = now_ts();

    if let Some(previous) = prev_entry {
        let country_changed = previous.country != login.country;
        let time_diff = login.timestamp.duration_since(previous.timestamp).unwrap_or(Duration::ZERO);
        let time_short = time_diff < Duration::from_secs(900);

        let device_mismatch = match (&login.device_fingerprint, &previous.device_fingerprint) {
            (Some(current), Some(prev)) => current != prev,
            _ => false,
        };

        let parsed_ip: IpAddr = login.ip.parse().unwrap_or_else(|_| IpAddr::from([0, 0, 0, 0]));
        let from_vpn = is_vpn_or_anonymized(&parsed_ip);

        let sso_drift = match (&login.sso_id, &previous.sso_id) {
            (Some(current), Some(prev)) => current != prev,
            _ => false,
        };

        let anomaly = country_changed && time_short && device_mismatch;

        if anomaly || from_vpn || sso_drift {
            let mut risk_score = 88.0;
            if from_vpn { risk_score += 6.0; }
            if !login.mfa_success { risk_score += 4.0; }
            if sso_drift { risk_score += 5.0; }

            let multiplier = get_role_multiplier(&login.user);
            risk_score *= multiplier;
            let bounded_score = (100.0 - risk_score).clamp(0.0, 100.0);

            let trust_event = TrustEvent {
                timestamp: now,
                pid: login.pid.unwrap_or(0),
                ppid: login.ppid.unwrap_or(0),
                uid: login.uid.unwrap_or(0),
                binary_path: "unknown".into(),
                command_line: "unknown".into(),
                cwd: "unknown".into(),
                anomaly_type: "GeoIPMismatch".into(),
                component: "auth".into(),
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("user".into(), login.user.clone());
                    meta.insert("previous_country".into(), previous.country.clone());
                    meta.insert("current_country".into(), login.country.clone());
                    meta.insert("vpn".into(), from_vpn.to_string());
                    meta.insert("device_mismatch".into(), device_mismatch.to_string());
                    meta.insert("sso_drift".into(), sso_drift.to_string());
                    meta.insert("role_multiplier".into(), multiplier.to_string());
                    meta.insert("computed_risk".into(), risk_score.to_string());
                    meta
                },
                risk_score: risk_score as f32,
                source_module: "geo_ip_anomaly".into(),
                decay_context: Some("geo_location".into()),
                module: Some("geo_ip_anomaly".into()),
                signal: Some("geo_ip_anomaly".into()),
                signal_type: Some("auth_behavioral".into()),
                score: Some(bounded_score as f32),
                raw_score: Some(risk_score as f32),
                tags: Some(vec![
                    "geo_mismatch".into(),
                    login.country.clone(),
                    if from_vpn { "vpn_detected".into() } else { "no_vpn".into() },
                    if sso_drift { "sso_drift".into() } else { "sso_stable".into() }
                ]),
                description: Some(format!(
                    "GeoIP anomaly: user={} from={} (prev={}) device_mismatch={} vpn={} sso_drift={}",
                    login.user,
                    login.country,
                    previous.country,
                    device_mismatch,
                    from_vpn,
                    sso_drift
                )),
            };

            submit_trust_event(trust_event);

            let mut data = HashMap::new();
            data.insert("user".into(), login.user.clone());
            data.insert("country".into(), login.country.clone());
            data.insert("vpn".into(), from_vpn.to_string());
            data.insert("device_mismatch".into(), device_mismatch.to_string());
            data.insert("sso_drift".into(), sso_drift.to_string());
            data.insert("mfa_success".into(), login.mfa_success.to_string());
            data.insert("event_type".into(), "geo_ip_anomaly".into());
            data.insert("timestamp".into(), now.to_string());
            data.insert("category".into(), "auth".into());
            data.insert("signal".into(), "geo_ip_anomaly".into());
            data.insert("replay_tag".into(), "geo_ip_anomaly_detected".into());
            data.insert("gnn_escalate".into(), "true".into());
            data.insert("role_multiplier".into(), multiplier.to_string());
            data.insert("computed_risk".into(), risk_score.to_string());

            let output = TelemetryOutput {
                category: "auth".into(),
                signal: "geo_ip_anomaly".into(),
                confidence: 0.95,
                data: data.clone(),
            };

            write_telemetry_record(data.clone());
            push_to_gnn_vector_log(data.clone());
            store_replay_event(data.clone())?;
            crate::gnn_hook::push_metadata_to_gnn_vector_log(data.clone());
            log_geo_ip_anomaly(&login.user, &login.country);

            let decay_factor = if anomaly && sso_drift && from_vpn {
                0.85
            } else if anomaly || sso_drift {
                0.9
            } else {
                0.95
            };

            let new_trust = previous.trust_score * decay_factor;

            update_trust_vector(trust_vector, "auth_trust", -risk_score);
            log_decay_reason("geo_ip_anomaly", &login.user, -risk_score, "auth");

            history.insert(
                login.user.clone(),
                LoginHistory {
                    country: login.country.clone(),
                    timestamp: login.timestamp,
                    device_fingerprint: login.device_fingerprint.clone(),
                    trust_score: new_trust,
                    sso_id: login.sso_id.clone(),
                },
            );
        } else {
            history.insert(
                login.user.clone(),
                LoginHistory {
                    country: login.country.clone(),
                    timestamp: login.timestamp,
                    device_fingerprint: login.device_fingerprint.clone(),
                    trust_score: previous.trust_score,
                    sso_id: login.sso_id.clone(),
                },
            );
        }
    } else {
        history.insert(
            login.user.clone(),
            LoginHistory {
                country: login.country.clone(),
                timestamp: login.timestamp,
                device_fingerprint: login.device_fingerprint.clone(),
                trust_score: 100.0,
                sso_id: login.sso_id.clone(),
            },
        );
    }

    Ok(())
}


pub fn scan_geo_ip_activity() -> Vec<TelemetryOutput> {
    if GEO_ANOMALY_FOUND.load(Ordering::SeqCst) {
        GEO_ANOMALY_FOUND.store(false, Ordering::SeqCst);
        return vec![];
    }

    let timestamp = now_ts();

    let trust_event = TrustEvent {
        timestamp,
        pid: 0,
        ppid: 0,
        uid: 0,
        binary_path: "unknown".into(),
        command_line: "no_anomalous_login".into(),
        cwd: "/".into(),
        anomaly_type: "GeoIPStable".into(),
        component: "auth".into(),
        metadata: HashMap::from([
            ("status".into(), "no_geo_anomaly".into()),
            ("timestamp".into(), timestamp.to_string()),
        ]),
        risk_score: -1.0, // gentle trust reinforcement
        source_module: "geo_ip_anomaly".into(),
        decay_context: Some("geo_location".into()),
        module: Some("geo_ip_anomaly".into()),
        signal: Some("geo_ip_no_anomaly".into()),
        signal_type: Some("auth_behavioral".into()),
        score: Some(1.0),
        raw_score: Some(0.0),
        tags: Some(vec!["geo_stable".into()]),
        description: Some("No geo-IP anomaly detected during passive scan".into()),
    };

    submit_trust_event(trust_event);

    let mut data = HashMap::new();
    data.insert("event_type".into(), "geo_ip_monitor".into());
    data.insert("status".into(), "no_geo_anomaly".into());
    data.insert("timestamp".into(), timestamp.to_string());

    vec![TelemetryOutput {
        category: "auth".into(),
        signal: "geo_ip_no_anomaly".into(),
        confidence: 0.0,
        data,
    }]
}

