use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::process::Command;
use std::sync::{OnceLock, atomic::{AtomicBool, Ordering}};

use crate::telemetry_types::TelemetryOutput;
use crate::utils::time::now_ts;
use crate::telemetry_writer::write_telemetry_record;
use crate::trust_hook::{TrustEvent, submit_trust_event, generate_feature_vector};
use crate::gnn_hook::push_to_gnn_vector_log;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub username: String,
    pub terminal: String,
    pub host: String,
    pub login_time: String, // epoch seconds as string
}

pub static SCAN_USER_TRACKER: OnceLock<AtomicBool> = OnceLock::new();

/// Normalize a TTY/PTS string for comparison.
fn normalize_tty(s: &str) -> String {
    s.trim_start_matches("/dev/")
        .trim_start_matches("tty")
        .trim()
        .to_string()
}

/// Attempt to extract (pid, ppid, uid) for a user session tied to a terminal.
pub fn get_session_process_info(username: &str, terminal: &str) -> (i32, i32, u32) {
    // 1) Prefer `ps` to correlate controlling ttys for interactive sessions.
    if let Ok(output) = Command::new("ps").args(&["-eo", "pid,ppid,uid,tty,user,comm"]).output() {
        if output.status.success() {
            let reader = BufReader::new(&output.stdout[..]);
            let term_norm = normalize_tty(terminal);

            for line in reader.lines().flatten() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 6 {
                    continue;
                }

                let pid = parts[0].parse::<i32>().unwrap_or(-1);
                let ppid = parts[1].parse::<i32>().unwrap_or(-1);
                let uid = parts[2].parse::<u32>().unwrap_or(0);
                let tty = parts[3];
                let user = parts[4];

                if user == username {
                    let tty_norm = normalize_tty(tty);
                    if tty_norm == term_norm || tty.contains(&term_norm) || term_norm.contains(&tty_norm) {
                        return (pid, ppid, uid);
                    }
                }
            }
        }
    }

    // 2) Fallback: scan /proc for GUI/daemon-attached sessions.
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let pid_path = entry.path();
            let pid_str = entry.file_name().to_string_lossy().to_string();
            if !pid_str.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }

            // Read environ (null-separated). We don’t require valid UTF-8 for every byte.
            let environ_path = pid_path.join("environ");
            if let Ok(bytes) = fs::read(&environ_path) {
                let env_txt = String::from_utf8_lossy(&bytes);
                if env_txt.contains(&format!("USER={}", username))
                    && (env_txt.contains("XDG_SESSION_TYPE=wayland")
                        || env_txt.contains("XDG_SESSION_TYPE=x11")
                        || env_txt.contains("XDG_SEAT=seat0"))
                {
                    let status_path = pid_path.join("status");
                    if let Ok(status) = fs::read_to_string(&status_path) {
                        let mut uid = 0_u32;
                        let mut ppid = -1_i32;
                        for line in status.lines() {
                            if let Some(rest) = line.strip_prefix("Uid:") {
                                uid = rest.split_whitespace().nth(0).and_then(|n| n.parse().ok()).unwrap_or(0);
                            } else if let Some(rest) = line.strip_prefix("PPid:") {
                                ppid = rest.split_whitespace().nth(0).and_then(|n| n.parse().ok()).unwrap_or(-1);
                            }
                        }
                        let pid = pid_str.parse::<i32>().unwrap_or(-1);
                        return (pid, ppid, uid);
                    }
                }
            }

            // Additional heuristic: look for display/session processes
            let cmdline_path = pid_path.join("cmdline");
            if let Ok(bytes) = fs::read(&cmdline_path) {
                let cmdline = String::from_utf8_lossy(&bytes);
                if cmdline.contains("Xorg")
                    || cmdline.contains("gdm-session-worker")
                    || cmdline.contains("loginctl")
                {
                    let status_path = pid_path.join("status");
                    if let Ok(status) = fs::read_to_string(&status_path) {
                        let mut uid = 0_u32;
                        let mut ppid = -1_i32;
                        for line in status.lines() {
                            if let Some(rest) = line.strip_prefix("Uid:") {
                                uid = rest.split_whitespace().nth(0).and_then(|n| n.parse().ok()).unwrap_or(0);
                            } else if let Some(rest) = line.strip_prefix("PPid:") {
                                ppid = rest.split_whitespace().nth(0).and_then(|n| n.parse().ok()).unwrap_or(-1);
                            }
                        }
                        if uid != 0 {
                            let pid = pid_str.parse::<i32>().unwrap_or(-1);
                            return (pid, ppid, uid);
                        }
                    }
                }
            }
        }
    }

    (-1, -1, 0) // final fallback
}

/// Try to parse the login timestamp from `who` tokens in several common formats.
fn parse_who_login_ts(tokens: &[&str]) -> Option<u64> {
    use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};

    // Common layouts seen in `who`:
    // 1) user pts/0 2025-08-19 12:34 (host)
    // 2) user pts/0 Aug 19 12:34 (host)
    // 3) user tty1  2025-08-19 12:34
    // We’ll scan tokens for a date + time pair.

    // Build candidates from index 2 onward
    let year = Local::now().year();

    // Helper: parse YYYY-MM-DD HH:MM
    let try_iso_pair = |d: &str, t: &str| -> Option<u64> {
        NaiveDate::parse_from_str(d, "%Y-%m-%d")
            .ok()
            .and_then(|nd| NaiveTime::parse_from_str(t, "%H:%M").ok().map(|nt| (nd, nt)))
            .and_then(|(nd, nt)| {
                let ldt = NaiveDateTime::new(nd, nt);
                Local.from_local_datetime(&ldt).single().map(|dt| dt.timestamp() as u64)
            })
    };

    // Helper: parse Mon DD HH:MM with current year
    let try_mon_pair = |m: &str, d: &str, t: &str| -> Option<u64> {
        // e.g., "Aug", "19", "12:03"
        let date_str = format!("{} {} {}", year, m, d);
        NaiveDate::parse_from_str(&date_str, "%Y %b %d")
            .ok()
            .and_then(|nd| NaiveTime::parse_from_str(t, "%H:%M").ok().map(|nt| (nd, nt)))
            .and_then(|(nd, nt)| {
                let ldt = NaiveDateTime::new(nd, nt);
                Local.from_local_datetime(&ldt).single().map(|dt| dt.timestamp() as u64)
            })
    };

    let n = tokens.len();
    for i in 2..n {
        // ISO date + time
        if i + 1 < n && tokens[i].len() == 10 && tokens[i + 1].len() >= 4 && tokens[i].contains('-') && tokens[i + 1].contains(':') {
            if let Some(ts) = try_iso_pair(tokens[i], tokens[i + 1]) {
                return Some(ts);
            }
        }
        // Mon DD HH:MM
        if i + 2 < n && tokens[i].len() == 3 && tokens[i + 1].chars().all(|c| c.is_ascii_digit()) && tokens[i + 2].contains(':') {
            if let Some(ts) = try_mon_pair(tokens[i], tokens[i + 1], tokens[i + 2]) {
                return Some(ts);
            }
        }
    }

    None
}

pub fn get_logged_in_users() -> Vec<UserSession> {
    use chrono::{Datelike, Local};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    SCAN_USER_TRACKER.get_or_init(|| AtomicBool::new(true));
    let mut sessions = Vec::new();

    let output = Command::new("who").output();

    match output {
        Ok(output) if output.status.success() => {
            let reader = BufReader::new(&output.stdout[..]);

            for line in reader.lines().flatten() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 {
                    continue;
                }

                let username = parts[0].to_string();
                let terminal = parts[1].to_string();

                // Host in parentheses if present
                let host = parts
                    .iter()
                    .find(|p| p.starts_with('(') && p.ends_with(')'))
                    .map(|s| s.trim_matches(&['(', ')'][..]).to_string())
                    .unwrap_or_else(|| "localhost".to_string());

                // Parse login timestamp (fallback to now)
                let login_ts = parse_who_login_ts(&parts).unwrap_or_else(now_ts);

                // Session ID (stable-ish)
                let session_id = {
                    let mut hasher = DefaultHasher::new();
                    format!("{}-{}", username, terminal).hash(&mut hasher);
                    hasher.finish().to_string()
                };

                let (pid, ppid, uid) = get_session_process_info(&username, &terminal);
                let suspicious_tty = !terminal.starts_with("tty") && !terminal.starts_with("pts");
                let session_age = now_ts().saturating_sub(login_ts);

                let mut data = HashMap::new();
                data.insert("username".into(), username.clone());
                data.insert("terminal".into(), terminal.clone());
                data.insert("host".into(), host.clone());
                data.insert("pid".into(), pid.to_string());
                data.insert("ppid".into(), ppid.to_string());
                data.insert("uid".into(), uid.to_string());
                data.insert("login_time".into(), login_ts.to_string());
                data.insert("timestamp".into(), now_ts().to_string());
                data.insert("replay_tag".into(), "user_login".into());
                data.insert("source".into(), "get_logged_in_users".into());
                data.insert("session_id".into(), session_id.clone());
                data.insert("session_age_seconds".into(), session_age.to_string());
                data.insert("suspicious_tty".into(), suspicious_tty.to_string());

                let features = generate_feature_vector(
                    if suspicious_tty { 0.3 } else { 0.1 }, // cpu-ish
                    session_age,                              // mem placeholder
                    2.0,                                      // risk-ish
                );
                data.insert("features".into(), format!("{:?}", features));

                push_to_gnn_vector_log(data.clone());

                submit_trust_event(TrustEvent {
                    timestamp: now_ts(),
                    pid,
                    ppid,
                    uid,
                    binary_path: "/usr/bin/who".into(),
                    command_line: format!("who → {}", username),
                    cwd: "/".into(),
                    anomaly_type: "user_logged_in".into(),
                    component: "user_tracker".into(),
                    description: Some(format!(
                        "User '{}' logged in via terminal '{}' (suspicious: {})",
                        username, terminal, suspicious_tty
                    )),
                    metadata: data.clone(),
                    risk_score: if suspicious_tty { 4.0 } else { 2.0 },
                    source_module: "user_tracker".into(),
                    decay_context: Some("user_login".into()),
                    module: Some("user_tracker".into()),
                    signal: Some("user_login_detected".into()),
                    signal_type: Some("auth".into()),
                    score: Some(2.0),
                    raw_score: Some(2.0),
                    tags: Some(vec![
                        "user".into(),
                        "login".into(),
                        terminal.clone(),
                        if suspicious_tty { "suspicious_tty".into() } else { "tty".into() },
                    ]),
                });

                let _ = write_telemetry_record(data);

                sessions.push(UserSession {
                    username,
                    terminal,
                    host,
                    login_time: login_ts.to_string(),
                });
            }
        }
        Ok(output) => {
            eprintln!(
                "[user_tracker] `who` exited with non-zero status: {}",
                output.status.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            eprintln!("[user_tracker] Failed to execute `who`: {}", e);
        }
    }

    sessions
}

pub fn scan_user_sessions() -> Vec<TelemetryOutput> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut outputs = Vec::new();
    let sessions = get_logged_in_users();
    let now = now_ts();

    for session in sessions {
        // Session ID
        let mut hasher = DefaultHasher::new();
        format!("{}-{}", session.username, session.terminal).hash(&mut hasher);
        let session_id = hasher.finish().to_string();

        // Session age from stored epoch string
        let login_ts_u64 = session
            .login_time
            .parse::<u64>()
            .unwrap_or_else(now_ts);
        let session_age = now.saturating_sub(login_ts_u64);
        let suspicious_tty = !session.terminal.starts_with("tty") && !session.terminal.starts_with("pts");

        let mut data = HashMap::new();
        data.insert("username".into(), session.username.clone());
        data.insert("terminal".into(), session.terminal.clone());
        data.insert("host".into(), session.host.clone());
        data.insert("login_time".into(), session.login_time.clone());
        data.insert("timestamp".into(), now.to_string());
        data.insert("replay_tag".into(), "user_login_session".into());
        data.insert("source".into(), "scan_user_sessions".into());
        data.insert(
            "soc_note".into(),
            format!(
                "User '{}' active on terminal '{}' (host: {})",
                session.username, session.terminal, session.host
            ),
        );
        data.insert("session_id".into(), session_id.clone());
        data.insert("session_age_seconds".into(), session_age.to_string());
        data.insert("suspicious_tty".into(), suspicious_tty.to_string());

        let features = generate_feature_vector(
            if suspicious_tty { 0.3 } else { 0.1 },
            session_age,
            1.5,
        );
        data.insert("features".into(), format!("{:?}", features));

        push_to_gnn_vector_log(data.clone());

        submit_trust_event(TrustEvent {
            timestamp: now,
            pid: 0,
            ppid: 0,
            uid: 0,
            binary_path: "/usr/bin/who".into(),
            command_line: format!("who session → {}", session.username),
            cwd: "/".into(),
            anomaly_type: "user_session_observed".into(),
            component: "user_tracker".into(),
            description: Some(format!(
                "User '{}' observed on terminal '{}' (suspicious: {})",
                session.username, session.terminal, suspicious_tty
            )),
            metadata: data.clone(),
            risk_score: if suspicious_tty { 2.5 } else { 1.5 },
            source_module: "user_tracker".into(),
            decay_context: Some("user_session".into()),
            module: Some("user_tracker".into()),
            signal: Some("session_seen".into()),
            signal_type: Some("auth".into()),
            score: Some(1.5),
            raw_score: Some(1.5),
            tags: Some(vec![
                "user".into(),
                "session".into(),
                "login".into(),
                if suspicious_tty { "suspicious_tty".into() } else { "tty".into() },
            ]),
        });

        let _ = write_telemetry_record(data.clone());

        outputs.push(TelemetryOutput {
            category: "user".into(),
            signal: "session_seen".into(),
            confidence: if suspicious_tty { 0.6 } else { 0.3 },
            data,
        });
    }

    outputs
}
