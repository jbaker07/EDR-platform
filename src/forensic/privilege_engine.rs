use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::thread;
use std::time::Duration;
use chrono::Utc;
use regex::Regex;
use crate::trust_hook;
use crate::replay_writer::{generate_replay_event, persist_replay_event};
use crate::semantic_tags::get_tag_risk_weight; // ← assumes semantic_tags.rs exposes this

pub fn start_privilege_engine() {
    thread::spawn(|| {
        let mut last_offset = 0;
        let log_path = "/var/log/auth.log";
        let sudo_re = Regex::new(r"sudo: (\w+) : TTY=(\S+) ; .* COMMAND=(.*)").unwrap();

        loop {
            if let Ok(mut file) = File::open(log_path) {
                let _ = file.seek(SeekFrom::Start(last_offset));
                let reader = BufReader::new(&file);

                for line in reader.lines().flatten() {
                    last_offset += line.len() as u64 + 1;

                    if let Some(caps) = sudo_re.captures(&line) {
                        let user = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
                        let tty = caps.get(2).map(|m| m.as_str()).unwrap_or("unknown");
                        let command = caps.get(3).map(|m| m.as_str()).unwrap_or("unknown");

                        // Enrich with semantic tag
                        let tag = "privilege_escalation_attempt";
                        let risk_weight = get_tag_risk_weight(tag).unwrap_or(0.7);

                        let trust_payload = trust_hook::generate_trust_payload(
                            risk_weight,
                            120_000,
                            30.0,
                        );

                        println!(
                            "[🛡️ Privilege Engine] sudo by user '{}' on '{}' → cmd: {}, score: {}",
                            user,
                            tty,
                            command,
                            trust_payload.get("trust_score").unwrap_or(&"N/A".into())
                        );

                        if let Some(event) = generate_replay_event(
                            "privilege_monitor",
                            "localhost",
                            &serde_json::json!({
                                "event": "sudo usage",
                                "user": user,
                                "tty": tty,
                                "command": command,
                                "timestamp": Utc::now().timestamp(),
                                "semantic_tag": tag,
                                "trust_score": trust_payload
                            }),
                        ) {
                            let _ = persist_replay_event(&event);
                        }
                    }
                }
            }

            thread::sleep(Duration::from_secs(45));
        }
    });
}
