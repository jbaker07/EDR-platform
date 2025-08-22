use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::services::session_revalidator::SessionData;

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedTrustState {
    pub sessions: HashMap<String, SessionData>,
}

pub fn load_trust_state(path: &str) -> HashMap<String, SessionData> {
    if Path::new(path).exists() {
        match fs::read_to_string(path) {
            Ok(content) => {
                if let Ok(saved) = serde_json::from_str::<SavedTrustState>(&content) {
                    return saved.sessions;
                } else {
                    eprintln!("[restorer] Failed to parse trust state.");
                }
            }
            Err(e) => eprintln!("[restorer] Could not read trust file: {}", e),
        }
    } else {
        eprintln!("[restorer] No existing trust snapshot found.");
    }

    HashMap::new()
}
