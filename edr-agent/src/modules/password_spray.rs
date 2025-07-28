use std::collections::{HashMap, VecDeque, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Mutex, LazyLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use regex::Regex;

use crate::trust_hook::{submit_trust_event, TrustEvent};
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::telemetry_writer::write_telemetry_record;
use crate::telemetry_types::TelemetryOutput;
use crate::utils::time::now_ts;
use crate::modules::replay_writer::store_replay_event;
static ATTEMPT_LOG: LazyLock<Mutex<HashMap<String, VecDeque<SystemTime>>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

pub static PASSWORD_SPRAY_STATIC: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

const TIME_WINDOW: Duration = Duration::from_secs(60);
const ATTEMPT_THRESHOLD: usize = 5;
const MAX_LOG_SIZE: usize = 1000;
/// Logs a login attempt and returns a TelemetryOutput if a spray is detected
pub fn log_login_attempt(raw_username: &str) -> Option<TelemetryOutput> {
    let now = SystemTime::now();
    let username = raw_username.trim().to_lowercase();

    let mut log = ATTEMPT_LOG.lock().unwrap();

    if log.len() > MAX_LOG_SIZE {
        log.retain(|_, v| !v.is_empty());
    }

    let entry = log.entry(username.clone()).or_insert_with(VecDeque::new);
    entry.push_back(now);

    while let Some(front) = entry.front() {
        if now.duration_since(*front).unwrap_or(Duration::ZERO) > TIME_WINDOW {
            entry.pop_front();
        } else {
            break;
        }
    }

    if entry.len() >= ATTEMPT_THRESHOLD {
        let ts = now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let msg = format!(
            "Password spray detected for user '{}': {} attempts in 60 seconds",
            username,
            entry.len()
        );

        let mut data = HashMap::new();
        data.insert("username".into(), username.clone());
        data.insert("attempt_count".into(), entry.len().to_string());
        data.insert("timestamp".into(), ts.to_string());
        data.insert("summary".into(), msg.clone());
        data.insert("replay_tag".into(), "password_spray".into());
        data.insert("soc_note".into(), "High-rate login attempts detected".into());
        data.insert("gnn_escalate".into(), "true".into());

        let event = TrustEvent {
            timestamp: ts,
            pid: -1,
            ppid: -1,
            uid: u32::MAX,
            binary_path: "login_monitor".into(),
            command_line: format!("login attempt for '{}'", username),
            cwd: "/var/log/auth.log".into(),
            anomaly_type: "password_spray".into(),
            component: "auth::spray".into(),
            metadata: data.clone(),
            risk_score: 95.0,
            source_module: "password_spray.rs".into(),
            decay_context: Some("brute_force_window".into()),
            module: Some("auth".into()),
            signal: Some("password_spray".into()),
            signal_type: Some("auth".into()),
            score: Some(95.0),
            raw_score: Some(95.0),
            tags: Some(vec![
                "password_spray".into(),
                "brute_force".into(),
                "auth_abuse".into(),
            ]),
            description: Some(msg.clone()),
        };

        submit_trust_event(event);
        write_telemetry_record(data.clone());
        push_to_gnn_vector_log(data.clone());
        store_replay_event(data.clone());

        *PASSWORD_SPRAY_STATIC.lock().unwrap() = true;

        return Some(TelemetryOutput {
            category: "auth".into(),
            signal: "password_spray".into(),
            confidence: 0.95,
            data,
        });
    }

    None
}

/// Passively scans /var/log/auth.log for password spray indicators
pub fn scan_password_sprays() -> Vec<TelemetryOutput> {
    let mut ip_user_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut outputs = Vec::new();
    let now = now_ts();

    let re = Regex::new(r"Failed password for (invalid user )?(\S+) from (\d+\.\d+\.\d+\.\d+)").unwrap();

    if let Ok(file) = File::open("/var/log/auth.log") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if let Some(caps) = re.captures(&line) {
                let username = caps.get(2).map_or("", |m| m.as_str()).to_lowercase();
                let ip = caps.get(3).map_or("", |m| m.as_str()).to_string();
                ip_user_map.entry(ip).or_default().push(username);
            }
        }
    }

    for (ip, users) in ip_user_map.iter() {
        let unique_users: HashSet<_> = users.iter().collect();
        if unique_users.len() >= 5 {
            let msg = format!(
                "Password spray detected from {} targeting {} users",
                ip,
                unique_users.len()
            );

            let mut data = HashMap::new();
            data.insert("source_ip".into(), ip.clone());
            data.insert("unique_usernames".into(), unique_users.len().to_string());
            data.insert("summary".into(), msg.clone());
            data.insert("timestamp".into(), now.to_string());
            data.insert("replay_tag".into(), "password_spray".into());
            data.insert("soc_note".into(), "Detected multiple failed logins from one IP".into());
            data.insert("gnn_escalate".into(), "true".into());

            let event = TrustEvent {
                timestamp: now,
                pid: -1,
                ppid: -1,
                uid: u32::MAX,
                binary_path: "/usr/sbin/sshd".into(),
                command_line: "sshd -D".into(),
                cwd: "/var/log".into(),
                anomaly_type: "password_spray".into(),
                component: "auth::spray".into(),
                metadata: data.clone(),
                risk_score: 90.0,
                source_module: "password_spray.rs".into(),
                decay_context: Some("multi_user_attempt".into()),
                module: Some("auth".into()),
                signal: Some("password_spray_log".into()),
                signal_type: Some("auth".into()),
                score: Some(90.0),
                raw_score: Some(90.0),
                tags: Some(vec![
                    "auth_abuse".into(),
                    "logon_bruteforce".into(),
                    "log_analysis".into(),
                ]),
                description: Some(msg.clone()),
            };

            submit_trust_event(event);
            write_telemetry_record(data.clone());
            push_to_gnn_vector_log(data.clone());

            *PASSWORD_SPRAY_STATIC.lock().unwrap() = true;

            outputs.push(TelemetryOutput {
                category: "auth".into(),
                signal: "password_spray_log".into(),
                confidence: 0.9,
                data,
            });
        }
    }

    outputs
}
