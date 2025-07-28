use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;
use std::thread;
use crate::telemetry::TelemetryRecord;
use crate::trust_hook::submit_trust_event;
use crate::gnn_hook::submit_to_gnn;
use crate::utils::time::now_ts;

pub fn start_auth_pipe_listener() {
    thread::spawn(|| {
        let pipe_path = "/opt/edr-agent/auth_pipe";
        if !Path::new(pipe_path).exists() {
            println!("[⚠️ PAM Hook] Pipe not found: {pipe_path}");
            return;
        }

        loop {
            let file = match OpenOptions::new().read(true).open(pipe_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("[❌ PAM Hook] Error opening pipe: {:?}", e);
                    thread::sleep(std::time::Duration::from_secs(5));
                    continue;
                }
            };

            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                if let Ok(event): Result<serde_json::Value, _> = serde_json::from_str(&line) {
                    let user = event["user"].as_str().unwrap_or("").to_string();
                    let result = event["result"].as_str().unwrap_or("0") == "1";
                    let risk = if result { 0.5 } else { 3.5 };

                    let record = TelemetryRecord {
                        pid: 0,
                        ppid: 0,
                        uid: 0,
                        binary_path: "/usr/bin/login".to_string(),
                        command_line: format!("PAM Auth: {}", user),
                        cwd: "/tmp".into(),
                        env_vars: Some(vec![]),
                        risk_score: Some((risk * 10.0) as u32),
                        tags: vec!["pam_auth".into(), if result { "success".into() } else { "failure".into() }],
                        timestamp: now_ts(),
                    };

                    submit_to_gnn(&record);
                    submit_trust_event(crate::trust_hook::TrustEvent {
                        signal_type: "pam_auth".into(),
                        score: if result { 0.3 } else { 7.0 },
                        tags: vec![user],
                        timestamp: now_ts(),
                        description: format!("PAM auth event: {} ({})", user, if result { "success" } else { "fail" }),
                    });
                }
            }

            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
