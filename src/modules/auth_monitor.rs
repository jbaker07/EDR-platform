use std::{
    collections::{HashMap, HashSet},
    env,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use lazy_static::lazy_static;
use serde::Serialize;

use crate::{
    gnn_hook::{push_to_gnn_vector_log, submit_to_gnn},
    logger::log,
    modules::{geo_ip_anomaly, mfa_bypass, password_spray}, // kept for compatibility if referenced elsewhere
    modules::replay_writer::store_replay_event,
    telemetry::TelemetryRecord,
    telemetry_types::TelemetryOutput,
    telemetry_writer::write_telemetry_record,
    trust_hook::{submit_trust_event, TrustEvent},
    utils::time::now_ts,
};
use libc::{getpid, getppid, getuid};

static AUTH_MONITOR_STARTED: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Clone, Debug)]
pub struct AuthAnomalyAlert {
    pub timestamp: u64,
    pub user: String,
    pub alert_type: String,
    pub details: String,
}

lazy_static! {
    // (user, success, ip, geo, ts)
    static ref LOGIN_LOGS: std::sync::Mutex<Vec<(String, bool, String, String, u64)>> =
        std::sync::Mutex::new(Vec::new());

    // (user, alert_type) -> last_emit_ts (cooldown gate to avoid noisy repeats)
    static ref ALERT_LAST_EMIT: std::sync::Mutex<HashMap<(String, String), u64>> =
        std::sync::Mutex::new(HashMap::new());
}

// --------- Config helpers (env-tunable) ---------

fn env_f32(key: &str, default: f32) -> f32 {
    env::var(key)
        .ok()
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(default)
}

fn env_u64(key: &str, default: u64) -> u64 {
    env::var(key)
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(default)
}

// Default weights & controls (can be overridden via env):
// EDR_AUTH_W_SPRAY, EDR_AUTH_W_MFA, EDR_AUTH_W_GEO
// EDR_AUTH_ALERT_COOLDOWN (seconds), EDR_AUTH_LOG_RETENTION (seconds)
fn weights() -> (f32, f32, f32) {
    (
        env_f32("EDR_AUTH_W_SPRAY", 3.5),
        env_f32("EDR_AUTH_W_MFA", 2.5),
        env_f32("EDR_AUTH_W_GEO", 3.0),
    )
}

fn cooldown_secs() -> u64 {
    env_u64("EDR_AUTH_ALERT_COOLDOWN", 600) // 10m default
}

fn log_retention_secs() -> u64 {
    env_u64("EDR_AUTH_LOG_RETENTION", 6 * 3600) // 6h default
}

// --------- Public API ---------

pub fn log_auth_attempt(user: &str, success: bool, ip: &str, geo: &str) {
    let mut logs = LOGIN_LOGS.lock().unwrap();
    logs.push((user.to_string(), success, ip.to_string(), geo.to_string(), now_ts()));

    // Trim by size
    if logs.len() > 10_000 {
        let len = logs.len();
        logs.drain(0..len.saturating_sub(5_000));
    }
}

pub fn start_auth_monitor() {
    // Idempotent: ensure we only spawn once regardless of call site
    if AUTH_MONITOR_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    thread::spawn(|| loop {
        // Periodic pruning (time-based)
        prune_old_login_logs();

        let (w_spray, w_mfa, w_geo) = weights();
        let mut risk_score = 0.0f32;
        let mut alerts = Vec::new();

        if let Some(alert) = check_password_spray() {
            if should_emit(&alert.user, &alert.alert_type, alert.timestamp) {
                println!("[Password Spray] {:?}", alert);
                risk_score += w_spray;
                alerts.push(alert);
            }
        }

        if let Some(alert) = check_mfa_bypass() {
            if should_emit(&alert.user, &alert.alert_type, alert.timestamp) {
                println!("[MFA Bypass] {:?}", alert);
                risk_score += w_mfa;
                alerts.push(alert);
            }
        }

        if let Some(alert) = check_geo_ip_anomaly() {
            if should_emit(&alert.user, &alert.alert_type, alert.timestamp) {
                println!("[Geo/IP Anomaly] {:?}", alert);
                risk_score += w_geo;
                alerts.push(alert);
            }
        }

        for alert in alerts {
            let timestamp = alert.timestamp;
            let confidence = (risk_score / 10.0f32).min(1.0f32);

            let mut data = HashMap::new();
            data.insert("timestamp".into(), timestamp.to_string());
            data.insert("user".into(), alert.user.clone());
            data.insert("alert_type".into(), alert.alert_type.clone());
            data.insert("details".into(), alert.details.clone());
            data.insert("event_type".into(), "auth_anomaly".into());
            data.insert("signal".into(), "auth_anomaly".into());
            data.insert("category".into(), "auth".into());
            data.insert("confidence".into(), format!("{:.2}", confidence));
            data.insert("replay_tag".into(), "auth_anomaly_detected".into());
            data.insert("gnn_escalate".into(), "true".into());

            let telemetry_output = TelemetryOutput {
                category: "auth".into(),
                signal: "auth_anomaly".into(),
                confidence,
                data: data.clone(),
            };

            // sinks
            write_telemetry_record(data.clone());
            push_to_gnn_vector_log(data.clone());
            crate::gnn_hook::push_metadata_to_gnn_vector_log(data.clone());
            store_replay_event(data.clone());

            // trust event
            let trust_event = TrustEvent {
                timestamp,
                pid: -1,
                ppid: -1,
                uid: 0,
                binary_path: "unknown".into(),
                command_line: alert.details.clone(),
                cwd: "/".into(),
                anomaly_type: alert.alert_type.clone(),
                component: "auth".into(),
                metadata: data.clone(),
                risk_score,
                source_module: "auth_monitor".into(),
                decay_context: Some("auth_behavior".into()),
                module: Some("auth_monitor".into()),
                signal: Some("auth_anomaly".into()),
                signal_type: Some("auth_behavioral".into()),
                score: Some((100.0 - risk_score).max(0.0)),
                raw_score: Some(risk_score),
                tags: Some(vec![alert.alert_type.clone(), "auth".into()]),
                description: Some(alert.details.clone()),
            };
            submit_trust_event(trust_event);

            // telemetry record (for submit_to_gnn)
            let record = TelemetryRecord {
                timestamp,
                pid: -1,
                ppid: -1,
                uid: 0,
                binary_path: "unknown".into(),
                command_line: alert.details.clone(),
                cwd: "/".into(),
                env_vars: None,
                tags: vec![alert.alert_type.clone()],
                risk_score: Some((risk_score * 10.0) as u32),
            };
            submit_to_gnn(&record);

            // mark cooldown for (user, alert_type)
            note_emitted(&alert.user, &alert.alert_type, timestamp);
        }

        thread::sleep(Duration::from_secs(60));
    });
}

// --------- Heuristics ---------

pub fn check_password_spray() -> Option<AuthAnomalyAlert> {
    let logs = LOGIN_LOGS.lock().unwrap();
    let now = now_ts();
    let mut failure_map: HashMap<String, u32> = HashMap::new();

    for (user, success, _ip, _geo, ts) in logs.iter() {
        if !success && now.saturating_sub(*ts) < 600 {
            *failure_map.entry(user.clone()).or_insert(0) += 1;
        }
    }

    for (user, count) in failure_map {
        if count >= 5 {
            return Some(AuthAnomalyAlert {
                timestamp: now,
                user,
                alert_type: "Password Spray".into(),
                details: format!("{} failures in 10 minutes", count),
            });
        }
    }
    None
}

pub fn check_mfa_bypass() -> Option<AuthAnomalyAlert> {
    let logs = LOGIN_LOGS.lock().unwrap();
    let now = now_ts();

    for (user, success, _ip, geo, ts) in logs.iter() {
        if *success && geo.eq_ignore_ascii_case("unknown") && now.saturating_sub(*ts) < 300 {
            return Some(AuthAnomalyAlert {
                timestamp: now,
                user: user.clone(),
                alert_type: "MFA Bypass".into(),
                details: "Login success with no geo/IP resolution".into(),
            });
        }
    }
    None
}

pub fn check_geo_ip_anomaly() -> Option<AuthAnomalyAlert> {
    let logs = LOGIN_LOGS.lock().unwrap();
    let now = now_ts();
    let mut geo_map: HashMap<String, Vec<String>> = HashMap::new();

    for (user, success, _ip, geo, ts) in logs.iter() {
        if *success && now.saturating_sub(*ts) < 3600 {
            geo_map.entry(user.clone()).or_default().push(geo.clone());
        }
    }

    for (user, geos) in geo_map {
        let unique: HashSet<_> = geos.into_iter().collect();
        if unique.len() > 2 {
            return Some(AuthAnomalyAlert {
                timestamp: now,
                user,
                alert_type: "Geo/IP Anomaly".into(),
                details: format!("Multiple locations detected: {:?}", unique),
            });
        }
    }
    None
}

// --------- Passive scan bridge ---------

pub fn scan_auth_activity() -> Vec<TelemetryOutput> {
    if !AUTH_MONITOR_STARTED.load(Ordering::SeqCst) {
        start_auth_monitor(); // safe, idempotent
    }

    let timestamp = now_ts();

    let pid = unsafe { getpid() } as i32;
    let ppid = unsafe { getppid() } as i32;
    let uid = unsafe { getuid() } as u32;
    let cwd = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("/"))
        .display()
        .to_string();

    let binary_path = "/usr/bin/journalctl"; // keep existing value

    let trust_event = TrustEvent {
        timestamp,
        pid,
        ppid,
        uid,
        binary_path: binary_path.into(),
        command_line: "heartbeat".into(),
        cwd,
        anomaly_type: "NoAuthAnomaly".into(),
        component: "auth_monitor".into(),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("event_type".into(), "auth_heartbeat".into());
            meta.insert("status".into(), "no_anomalies".into());
            meta.insert("source".into(), "auth_monitor".into());
            meta.insert("timestamp".into(), timestamp.to_string());
            meta
        },
        risk_score: 0.0,
        source_module: "auth_monitor".into(),
        decay_context: Some("auth_idle".into()),
        module: Some("auth_monitor".into()),
        signal: Some("auth_idle".into()),
        signal_type: Some("auth_behavioral".into()),
        score: Some(100.0),
        raw_score: Some(0.0),
        tags: Some(vec!["heartbeat".into(), "auth".into()]),
        description: Some("Auth module heartbeat – no anomalies detected".into()),
    };

    submit_trust_event(trust_event);

    let mut data = HashMap::new();
    data.insert("event_type".into(), "auth_monitor_active".into());
    data.insert("status".into(), "no_auth_anomalies".into());
    data.insert("timestamp".into(), timestamp.to_string());

    vec![TelemetryOutput {
        category: "auth".into(),
        signal: "auth_monitor_active".into(),
        confidence: 0.0,
        data,
    }]
}

// --------- Internals ---------

fn prune_old_login_logs() {
    let cutoff = now_ts().saturating_sub(log_retention_secs());
    let mut logs = LOGIN_LOGS.lock().unwrap();
    if logs.is_empty() {
        return;
    }
    logs.retain(|(_u, _s, _ip, _g, ts)| *ts >= cutoff);
    // size guard remains in log_auth_attempt()
}

fn should_emit(user: &str, alert_type: &str, now: u64) -> bool {
    let mut map = ALERT_LAST_EMIT.lock().unwrap();
    let k = (user.to_string(), alert_type.to_string());
    let last = map.get(&k).copied().unwrap_or(0);
    if now.saturating_sub(last) >= cooldown_secs() {
        true
    } else {
        false
    }
}

fn note_emitted(user: &str, alert_type: &str, now: u64) {
    let mut map = ALERT_LAST_EMIT.lock().unwrap();
    map.insert((user.to_string(), alert_type.to_string()), now);
}
