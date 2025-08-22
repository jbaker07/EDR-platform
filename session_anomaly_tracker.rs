use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct SessionStats {
    pub session_id: String,
    pub hostname: String,
    pub baseline_tags: HashSet<String>,
    pub current_tags: HashSet<String>,
    pub anomaly_score: f64,
    pub timestamp: DateTime<Utc>,
}

impl SessionStats {
    pub fn new(session_id: &str, hostname: &str) -> Self {
        SessionStats {
            session_id: session_id.to_string(),
            hostname: hostname.to_string(),
            baseline_tags: HashSet::new(),
            current_tags: HashSet::new(),
            anomaly_score: 0.0,
            timestamp: Utc::now(),
        }
    }

    pub fn update_tags(&mut self, new_tags: Vec<String>) {
        self.current_tags = new_tags.into_iter().collect();
        self.timestamp = Utc::now();
        self.calculate_anomaly_score();
    }

    fn calculate_anomaly_score(&mut self) {
        let unexpected: HashSet<_> = self.current_tags
            .difference(&self.baseline_tags)
            .collect();

        let unexpected_count = unexpected.len() as f64;
        let baseline_size = self.baseline_tags.len().max(1) as f64;

        // Anomaly = ratio of new/unexpected behaviors
        self.anomaly_score = unexpected_count / baseline_size;
    }

    pub fn is_anomalous(&self, threshold: f64) -> bool {
        self.anomaly_score >= threshold
    }
}

/// Manages per-session anomaly tracking
pub struct SessionAnomalyTracker {
    sessions: HashMap<String, SessionStats>,
    threshold: f64,
}

impl SessionAnomalyTracker {
    pub fn new(threshold: f64) -> Self {
        SessionAnomalyTracker {
            sessions: HashMap::new(),
            threshold,
        }
    }

    pub fn register_session(&mut self, session_id: &str, hostname: &str, baseline_tags: Vec<String>) {
        let mut stats = SessionStats::new(session_id, hostname);
        stats.baseline_tags = baseline_tags.into_iter().collect();
        self.sessions.insert(session_id.to_string(), stats);
    }

    pub fn update_session(&mut self, session_id: &str, new_tags: Vec<String>) -> Option<&SessionStats> {
        if let Some(stats) = self.sessions.get_mut(session_id) {
            stats.update_tags(new_tags);
            Some(stats)
        } else {
            None
        }
    }

    pub fn get_anomalies(&self) -> Vec<&SessionStats> {
        self.sessions
            .values()
            .filter(|s| s.is_anomalous(self.threshold))
            .collect()
    }
}
