use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use regex::Regex;

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::replay_writer::store_replay_event;
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::write_telemetry_record;
use crate::trust_hook::{submit_trust_event, TrustEvent};
use crate::utils::time::now_ts;

static ATTEMPT_LOG: LazyLock<Mutex<HashMap<String, VecDeque<SystemTime>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
pub static PASSWORD_SPRAY_STATIC: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// Per-user and per-IP cooldowns to avoid event floods
static USER_LAST_ALERT: LazyLock<Mutex<HashMap<String, SystemTime>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static IP_LAST_ALERT: LazyLock<Mutex<HashMap<String, SystemTime>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

const TIME_WINDOW: Duration = Duration::from_secs(60);
const DEFAULT_ATTEMPT_THRESHOLD: usize = 5;
const COOLDOWN: Duration = Duration::from_secs(300); // 5 minutes
const MAX_LOG_SIZE: usize = 1000;

/// Resolve threshold from env or default
fn attempt_threshold() -> usize {
    env::var("EDR_SPRAY_THRESHOLD")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&n| n > 0)
        .unwrap_or(DEFAULT_ATTEMPT_THRESHOLD)
}

/// Evict oldest usernames (by their newest attempt time) to bound memory
fn evict_if_oversize(log: &mut HashMap<String, VecDeque<SystemTime>>) {
    if log.len() <= MAX_LOG_SIZE {
        return;
    }
    // Collect (username, newest_ts)
    let mut users: Vec<(String, SystemTime)> = log
        .iter()
        .map(|(u, q)| {
            (
                u.clone(),
                q.back().copied().unwrap_or(SystemTime::UNIX_EPOCH),
            )
        })
        .collect();
    users.sort_by_key(|(_, ts)| *ts); // oldest first
    let to_remove = log.len().saturating_sub(MAX_LOG_SIZE);
    for (u, _) in users.into_iter().take(to_remove) {
        log.remove(&u);
    }
}

/// Logs a login attempt and returns a TelemetryOutput if a spray is detected
pub fn log_login_attempt(raw_username: &str) -> Option<TelemetryOutput> {
    let now = SystemTime::now();
    let username = raw_username.trim().to_lowercase();
    let threshold = attempt_threshold();

    let mut log = ATTEMPT_LOG.lock().unwrap();

    // IMPORTANT: ensure no outstanding entry-borrow when we might need to reborrow `log`
    evict_if_oversize(&mut log);

    // Insert + prune per-user window (holds a &mut borrow only within this scope)
    {
        let entry = log.entry(username.clone()).or_insert_with(VecDeque::new);
        entry.push_back(now);
        while let Some(front) = entry.front() {
            if now.duration_since(*front).unwrap_or(Duration::ZERO) > TIME_WINDOW {
                entry.pop_front();
            } else {
                break;
            }
        }
        // entry borrow ends here
    }

    // Re-borrow the entry after eviction/prune
    let entry = log.entry(username.clone()).or_insert_with(VecDeque::new);

    // Cooldown check (per user)
    let mut last = USER_LAST_ALERT.lock().unwrap();
    let on_cooldown = last
        .get(&username)
        .and_then(|t| now.duration_since(*t).ok())
        .map(|d| d < COOLDOWN)
        .unwrap_or(false);

    if entry.len() >= threshold && !on_cooldown {
        last.insert(username.clone(), now);

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
        data.insert(
            "soc_note".into(),
            "High-rate login attempts detected".into(),
        );
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
        let _ = store_replay_event(data.clone());

        *PASSWORD_SPRAY_STATIC.lock().unwrap() = true;

        // Optional: clear to avoid immediate re-trigger on next attempt
        entry.clear();

        return Some(TelemetryOutput {
            category: "auth".into(),
            signal: "password_spray".into(),
            confidence: 0.95,
            data,
        });
    }

    None
}

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
