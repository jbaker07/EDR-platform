use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, SystemTime};

use regex::Regex;

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::replay_writer::store_replay_event;
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::write_telemetry_record;
use crate::trust_hook::{submit_trust_event, TrustEvent};
use crate::utils::time::now_ts;

/// Per-IP cooldown to avoid event floods
static IP_LAST_ALERT: LazyLock<Mutex<HashMap<String, SystemTime>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

const COOLDOWN: Duration = Duration::from_secs(300); // 5 minutes

/// Passively scans system auth logs for password spray indicators
pub fn scan_password_sprays() -> Vec<TelemetryOutput> {
    let mut ip_user_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut outputs = Vec::new();
    let now = now_ts();

    // Ubuntu/Debian & RHEL/CentOS
    let candidates = ["/var/log/auth.log", "/var/log/secure"];

    // Regex 1: OpenSSH "Failed password for (invalid user )?<user> from <ip>"
    let re_failed =
        Regex::new(r"Failed password for (?:invalid user )?(\S+) from ([0-9]{1,3}(?:\.[0-9]{1,3}){3}|[A-Fa-f0-9:]+)")
            .unwrap();

    // Regex 2: PAM style lines (captures rhost=<ip> user=<name>)
    let re_pam =
        Regex::new(r"pam_unix\([^)]+\): authentication failure.*rhost=([0-9]{1,3}(?:\.[0-9]{1,3}){3}|[A-Fa-f0-9:]+).*user=(\S+)")
            .unwrap();

    for path in candidates {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                if let Some(caps) = re_failed.captures(&line) {
                    let user = caps.get(1).map(|m| m.as_str()).unwrap_or("").to_lowercase();
                    let ip = caps.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
                    if !user.is_empty() && !ip.is_empty() {
                        ip_user_map.entry(ip).or_default().push(user);
                    }
                    continue;
                }
                if let Some(caps) = re_pam.captures(&line) {
                    let ip = caps.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                    let user = caps.get(2).map(|m| m.as_str()).unwrap_or("").to_lowercase();
                    if !user.is_empty() && !ip.is_empty() {
                        ip_user_map.entry(ip).or_default().push(user);
                    }
                }
            }
        }
    }

    // Per-IP cooldown to avoid repeated emits every scan
    let mut ip_last = IP_LAST_ALERT.lock().unwrap();

    for (ip, users) in ip_user_map.iter() {
        let unique_users: HashSet<_> = users.iter().collect();
        if unique_users.len() >= 5 {
            let now_st = SystemTime::now();
            let on_cooldown = ip_last
                .get(ip)
                .and_then(|t| now_st.duration_since(*t).ok())
                .map(|d| d < COOLDOWN)
                .unwrap_or(false);
            if on_cooldown {
                continue;
            }
            ip_last.insert(ip.clone(), now_st);

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
            data.insert(
                "soc_note".into(),
                "Detected multiple failed logins from one IP".into(),
            );
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
            let _ = store_replay_event(data.clone());

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
