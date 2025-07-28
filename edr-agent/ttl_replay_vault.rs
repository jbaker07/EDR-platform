use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplayEntry {
    pub session_id: String,
    pub timestamp: u64,
    pub trust_score: f32,
    pub tags: Vec<String>,
    pub replay_data: String,
    pub flagged: bool,
}

pub struct TTLReplayVault {
    vault_path: PathBuf,
    ttl_secs: u64,
    critical_tags: Vec<String>,
    trust_threshold: f32,
}

impl TTLReplayVault {
    pub fn new<P: Into<PathBuf>>(
        vault_path: P,
        ttl_secs: u64,
        critical_tags: Vec<String>,
        trust_threshold: f32,
    ) -> Self {
        Self {
            vault_path: vault_path.into(),
            ttl_secs,
            critical_tags,
            trust_threshold,
        }
    }

    pub fn store(&self, entry: ReplayEntry) -> Result<(), String> {
        let file_name = format!("{}_{}.json", entry.session_id, entry.timestamp);
        let file_path = self.vault_path.join(file_name);

        let json = serde_json::to_string(&entry).map_err(|e| e.to_string())?;
        fs::create_dir_all(&self.vault_path).map_err(|e| e.to_string())?;
        let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
        file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn clean_expired(&self) -> Result<(), String> {
        let now = current_ts();
        let entries = fs::read_dir(&self.vault_path).map_err(|e| e.to_string())?;

        for entry in entries {
            if let Ok(file) = entry {
                let path = file.path();
                if path.is_file() {
                    let contents = fs::read_to_string(&path).unwrap_or_default();
                    if let Ok(replay) = serde_json::from_str::<ReplayEntry>(&contents) {
                        let expired = now > (replay.timestamp + self.ttl_secs);
                        let exempt = replay.flagged
                            || replay.trust_score < self.trust_threshold
                            || !replay
                                .tags
                                .iter()
                                .filter(|t| self.critical_tags.contains(t))
                                .collect::<Vec<_>>()
                                .is_empty();

                        if expired && !exempt {
                            let _ = fs::remove_file(&path);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn current_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
