use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;

/// A single trust rule from the config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustRule {
    pub id: String,
    pub description: String,
    pub required_tags: Vec<String>,
    pub trust_penalty: f64,
    pub escalate: bool,
    pub replay_trigger: bool,
    pub trigger_once: bool,
}

/// Runtime structure to track matched rules
#[derive(Default)]
pub struct RuleMatchResult {
    pub total_penalty: f64,
    pub triggered_rules: Vec<String>,
    pub escalate: bool,
    pub trigger_replay: bool,
}

/// Rule engine state
pub struct TrustRuleMatcher {
    pub rules: Vec<TrustRule>,
    pub already_triggered: HashMap<String, bool>,
}

impl TrustRuleMatcher {
    pub fn load_from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to load rule file");
        let rules: Vec<TrustRule> = serde_json::from_str(&content).expect("Invalid rule format");
        Self {
            rules,
            already_triggered: HashMap::new(),
        }
    }

    /// Match tags against rule set
    pub fn evaluate(&mut self, tags: &[String]) -> RuleMatchResult {
        let mut result = RuleMatchResult::default();

        for rule in &self.rules {
            if rule.required_tags.iter().all(|t| tags.contains(t)) {
                if rule.trigger_once && self.already_triggered.contains_key(&rule.id) {
                    continue;
                }

                result.total_penalty += rule.trust_penalty;
                if rule.escalate {
                    result.escalate = true;
                }
                if rule.replay_trigger {
                    result.trigger_replay = true;
                }
                result.triggered_rules.push(rule.id.clone());

                if rule.trigger_once {
                    self.already_triggered.insert(rule.id.clone(), true);
                }
            }
        }

        result
    }
}
