use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageMode {
    LightDelta,
    FullCapture,
    VaultOnly,
    EscalatedReplay,
}

#[derive(Debug, Clone)]
pub struct StorageProfile {
    pub current_mode: StorageMode,
    pub role: String,
    pub endpoint_type: String,
    pub is_offline: bool,
    pub trust_score: f64,
    pub recent_tags: Vec<String>,
}

impl StorageProfile {
    pub fn new(role: &str, endpoint_type: &str, is_offline: bool) -> Self {
        Self {
            current_mode: StorageMode::LightDelta,
            role: role.to_string(),
            endpoint_type: endpoint_type.to_string(),
            is_offline,
            trust_score: 1.0,
            recent_tags: vec![],
        }
    }

    pub fn update_context(&mut self, trust_score: f64, recent_tags: Vec<String>) {
        self.trust_score = trust_score;
        self.recent_tags = recent_tags;
        self.recalculate_storage_mode();
    }

    fn recalculate_storage_mode(&mut self) {
        let sensitive_roles = vec!["sysadmin", "developer", "it"];
        let critical_tags = vec![
            "persistence_attempt",
            "privilege_escalation",
            "remote_shell",
            "unusual_script_exec",
        ];

        if self.is_offline {
            self.current_mode = StorageMode::VaultOnly;
            return;
        }

        if trust_score_low(self.trust_score) || self.recent_tags.iter().any(|t| critical_tags.contains(&t.as_str())) {
            self.current_mode = StorageMode::EscalatedReplay;
        } else if sensitive_roles.contains(&self.role.as_str()) || self.endpoint_type == "server" {
            self.current_mode = StorageMode::FullCapture;
        } else {
            self.current_mode = StorageMode::LightDelta;
        }
    }

    pub fn get_current_mode(&self) -> StorageMode {
        self.current_mode.clone()
    }
}

fn trust_score_low(score: f64) -> bool {
    score < 0.4
}
