use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
    
};

use lazy_static::lazy_static;
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use crate::{
    gnn_hook::{push_to_gnn_vector_log, submit_to_gnn},
    logger::log,
    modules::{geo_ip_anomaly, mfa_bypass, password_spray},
    telemetry::TelemetryRecord,
    telemetry_types::TelemetryOutput,
    telemetry_writer::write_telemetry_record,
    trust_hook::{submit_trust_event, TrustEvent},
    utils::time::now_ts,
};
use libc::{getpid, getppid, getuid};
use crate::modules::replay_writer::store_replay_event;

static AUTH_MONITOR_STARTED: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Clone, Debug)]
pub struct AuthAnomalyAlert {
    pub timestamp: u64,
    pub user: String,
    pub alert_type: String,
    pub details: String,
}

lazy_static! {
    static ref LOGIN_LOGS: std::sync::Mutex<Vec<(String, bool, String, String, u64)>> =
        std::sync::Mutex::new(Vec::new());
}

pub fn log_auth_attempt(user: &str, success: bool, ip: &str, geo: &str) {
    let mut logs = LOGIN_LOGS.lock().unwrap();
    logs.push((user.to_string(), success, ip.to_string(), geo.to_string(), now_ts()));

    if logs.len() > 10_000 {
        let len = logs.len();
        logs.drain(0..len.saturating_sub(5000));
    }
}

pub fn start_auth_monitor() {
    thread::spawn(|| loop {
        let mut risk_score = 0.0f32;
        let mut alerts = Vec::new();

        if let Some(alert) = check_password_spray() {
            println!("[Password Spray] {:?}", alert);
            risk_score += 3.5;
            alerts.push(alert);
        }

        if let Some(alert) = check_mfa_bypass() {
            println!("[MFA Bypass] {:?}", alert);
            risk_score += 2.5;
            alerts.push(alert);
        }

        if let Some(alert) = check_geo_ip_anomaly() {
            println!("[Geo/IP Anomaly] {:?}", alert);
            risk_score += 3.0;
            alerts.push(alert);
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

            write_telemetry_record(data.clone());
            push_to_gnn_vector_log(data.clone());
            crate::gnn_hook::push_metadata_to_gnn_vector_log(data.clone());
            store_replay_event(data.clone());

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
        }

        thread::sleep(Duration::from_secs(60));
    });
}

pub fn check_password_spray() -> Option<AuthAnomalyAlert> {
    let logs = LOGIN_LOGS.lock().unwrap();
    let now = now_ts();
    let mut failure_map = HashMap::new();

    for (user, success, _, _, ts) in logs.iter() {
        if !success && now - *ts < 600 {
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

    for (user, success, _, geo, ts) in logs.iter() {
        if *success && geo == "unknown" && now - *ts < 300 {
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

    for (user, success, _, geo, ts) in logs.iter() {
        if *success && now - *ts < 3600 {
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

pub fn scan_auth_activity() -> Vec<TelemetryOutput> {
    if !AUTH_MONITOR_STARTED.swap(true, Ordering::SeqCst) {
        start_auth_monitor();
    }

    let timestamp = now_ts();

    let pid = unsafe { getpid() } as i32;
    let ppid = unsafe { getppid() } as i32;
    let uid = unsafe { getuid() } as u32;
    let cwd = env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("/"))
        .display()
        .to_string();

    let binary_path = "/usr/bin/journalctl"; // Update this if your monitor uses something else

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