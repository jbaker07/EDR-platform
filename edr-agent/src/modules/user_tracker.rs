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
    pub login_time: String,
}
use chrono::TimeZone;
use std::convert::TryInto;
pub static SCAN_USER_TRACKER: OnceLock<AtomicBool> = OnceLock::new();

pub fn get_session_process_info(username: &str, terminal: &str) -> (i32, i32, u32) {
    // First attempt: use `ps` output for terminal match (for real TTYs)
    if let Ok(output) = Command::new("ps").args(&["-eo", "pid,ppid,uid,tty,user,comm"]).output() {
        if output.status.success() {
            let reader = BufReader::new(&output.stdout[..]);

            for line in reader.lines().flatten() {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if parts.len() >= 6 {
                    let pid = parts[0].parse::<i32>().unwrap_or(-1);
                    let ppid = parts[1].parse::<i32>().unwrap_or(-1);
                    let uid = parts[2].parse::<u32>().unwrap_or(0);
                    let tty = parts[3];
                    let user = parts[4];

                    if user == username {
                        let tty_norm = tty.trim_start_matches("/dev/").trim_start_matches("tty").trim();
                        let term_norm = terminal.trim_start_matches("/dev/").trim_start_matches("tty").trim();

                        if tty_norm == term_norm || tty.contains(term_norm) || term_norm.contains(tty_norm) {
                            return (pid, ppid, uid);
                        }
                    }
                }
            }
        }
    }

    // Fallback: check /proc/* manually for GUI session markers (seat0 etc)
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let pid_path = entry.path();
            let pid_str = entry.file_name().to_string_lossy().to_string();
            if !pid_str.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }

            let environ_path = pid_path.join("environ");
            if let Ok(mut file) = fs::File::open(&environ_path) {
                let mut buf = String::new();
                let _ = file.read_to_string(&mut buf);
                if buf.contains(&format!("USER={}", username)) &&
                   (buf.contains(&format!("XDG_SEAT={}", terminal)) || buf.contains("XDG_SESSION_TYPE=wayland") || buf.contains("XDG_SESSION_TYPE=x11"))
                {
                    let status_path = pid_path.join("status");
                    if let Ok(status) = fs::read_to_string(&status_path) {
                        let mut uid = 0;
                        let mut ppid = -1;
                        for line in status.lines() {
                            if line.starts_with("Uid:") {
                                uid = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
                            }
                            if line.starts_with("PPid:") {
                                ppid = line.split_whitespace().nth(1).unwrap_or("-1").parse().unwrap_or(-1);
                            }
                        }
                        let pid = pid_str.parse::<i32>().unwrap_or(-1);
                        return (pid, ppid, uid);
                    }
                }
            }

            // Additional fallback: check cmdline for session-related processes
            let cmdline_path = pid_path.join("cmdline");
            if let Ok(mut f) = fs::File::open(&cmdline_path) {
                let mut content = String::new();
                let _ = f.read_to_string(&mut content);
                if content.contains("Xorg") || content.contains("gdm-session-worker") || content.contains("loginctl") {
                    let status_path = pid_path.join("status");
                    if let Ok(status) = fs::read_to_string(&status_path) {
                        let mut uid = 0;
                        let mut ppid = -1;
                        let mut proc_user = String::new();
                        for line in status.lines() {
                            if line.starts_with("Uid:") {
                                uid = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
                            }
                            if line.starts_with("PPid:") {
                                ppid = line.split_whitespace().nth(1).unwrap_or("-1").parse().unwrap_or(-1);
                            }
                            if line.starts_with("Name:") {
                                proc_user = line.split_whitespace().nth(1).unwrap_or("").to_string();
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

    (-1, -1, 0) // full fallback
}


pub fn get_logged_in_users() -> Vec<UserSession> {
    use chrono::{NaiveTime, NaiveDate, Local, Datelike, Timelike};
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
                if parts.len() < 3 {
                    continue;
                }

                let username = parts[0].to_string();
                let terminal = parts[1].to_string();
                let (pid, ppid, uid) = get_session_process_info(&username, &terminal);
                let login_day = parts.get(2).copied().unwrap_or_default();
                let login_time = parts.get(3).copied().unwrap_or_default();
                let host = parts
                    .iter()
                    .find(|p| p.starts_with('(') && p.ends_with(')'))
                    .map(|s| s.trim_matches(&['(', ')'][..]).to_string())
                    .unwrap_or_else(|| "localhost".to_string());

                let now = Local::now();
                let full_login_time_str = format!("{} {}", login_day, login_time);
                let mut login_ts = now_ts();

                // Try to parse login time if available
                if let (Ok(day), Ok(time)) = (
                    NaiveDate::parse_from_str(&format!("{} {}", now.year(), login_day), "%Y-%m-%d"),
                    NaiveTime::parse_from_str(login_time, "%H:%M"),
                ) {
                    login_ts = Local
                    .from_local_datetime(&day.and_time(time))
                    .unwrap()
                    .timestamp()
                    .try_into()
                    .unwrap(); // Convert i64 → u64

                }

                let session_id = {
                    let mut hasher = DefaultHasher::new();
                    format!("{}-{}", username, terminal).hash(&mut hasher);
                    hasher.finish().to_string()
                };

                let suspicious_tty = !terminal.starts_with("tty") && !terminal.starts_with("pts");
                let session_age = now_ts() - login_ts;

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
                    if suspicious_tty { 0.3 } else { 0.1 },
                    session_age,
                    2.0,
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
                "[!] `who` exited with non-zero status: {}",
                output.status.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            eprintln!("[!] Failed to execute `who`: {}", e);
        }
    }

    sessions
}

pub fn scan_user_sessions() -> Vec<TelemetryOutput> {
    use chrono::{Local, NaiveDateTime};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut outputs = Vec::new();
    let sessions = get_logged_in_users();
    let now = now_ts();

    for session in sessions {
        // Compute session_id
        let mut hasher = DefaultHasher::new();
        format!("{}-{}", session.username, session.terminal).hash(&mut hasher);
        let session_id = hasher.finish().to_string();

        // Parse session age
        let login_ts = session.login_time.parse::<i64>().unwrap_or(now.try_into().unwrap());
        let session_age = now.saturating_sub(login_ts.try_into().unwrap());
        let suspicious_tty = !session.terminal.starts_with("tty") && !session.terminal.starts_with("pts");

        let mut data = HashMap::new();
        data.insert("username".into(), session.username.clone());
        data.insert("terminal".into(), session.terminal.clone());
        data.insert("host".into(), session.host.clone());
        data.insert("login_time".into(), session.login_time.clone());
        data.insert("timestamp".into(), now.to_string());
        data.insert("replay_tag".into(), "user_login_session".into());
        data.insert("source".into(), "scan_user_sessions".into());
        data.insert("soc_note".into(), format!(
            "User '{}' active on terminal '{}' (host: {})",
            session.username, session.terminal, session.host
        ));
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
