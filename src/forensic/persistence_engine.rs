use std::fs;
use std::thread;
use std::time::Duration;
use std::path::{PathBuf};
use chrono::Utc;
use crate::trust_hook;
use crate::replay_writer::{generate_replay_event, persist_replay_event};
use crate::semantic_tags::get_tag_risk_weight;

pub fn start_persistence_engine() {
    thread::spawn(|| {
        let raw_paths = vec![
            "/etc/rc.local",
            "/etc/systemd/system/",
            "/etc/init.d/",
            "~/.bashrc",
        ];

        let expanded_paths: Vec<PathBuf> = raw_paths.into_iter()
            .map(|p| shellexpand::tilde(p).to_string())
            .map(PathBuf::from)
            .collect();

        loop {
            for path in &expanded_paths {
                if let Ok(metadata) = fs::metadata(path) {
                    if metadata.is_file() || metadata.is_dir() {
                        let trust_score = get_tag_risk_weight("persistence_mechanism_detected")
                            .unwrap_or(0.3);

                        let trust_payload = trust_hook::generate_trust_payload(
                            trust_score,
                            70000,
                            20.0,
                        );

                        println!(
                            "[🛑 Persistence Engine] Persistence marker found at: {} | Score: {}",
                            path.display(),
                            trust_payload.get("trust_score").unwrap_or(&"N/A".into())
                        );

                        if let Some(event) = generate_replay_event(
                            "persistence_monitor",
                            "localhost",
                            &serde_json::json!({
                                "path": path.display().to_string(),
                                "timestamp": Utc::now().timestamp(),
                                "semantic_tag": "persistence_mechanism_detected",
                                "trust_score": trust_payload
                            }),
                        ) {
                            let _ = persist_replay_event(&event);
                        }
                    }
                }
            }

            thread::sleep(Duration::from_secs(120));
        }
    });
}
