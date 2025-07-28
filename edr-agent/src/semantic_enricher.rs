use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticTag {
    pub id: String,
    pub description: String,
    pub match_keywords: Vec<String>,
    pub match_fields: Vec<String>,
    pub risk_weight: f64,
}

/// Main enricher engine
pub struct SemanticEnricher {
    pub tags: Vec<SemanticTag>,
}

impl SemanticEnricher {
    pub fn load_from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read semantic_tags.json");
        let tags: Vec<SemanticTag> = serde_json::from_str(&content).expect("Invalid semantic_tags format");
        Self { tags }
    }

    /// Given a telemetry record (e.g., field_name -> value), return matching tags
    pub fn enrich(&self, telemetry: &HashMap<String, String>) -> Vec<String> {
        let mut matched = Vec::new();

        for tag in &self.tags {
            for field in &tag.match_fields {
                if let Some(value) = telemetry.get(field) {
                    for keyword in &tag.match_keywords {
                        if value.to_lowercase().contains(&keyword.to_lowercase()) {
                            matched.push(tag.id.clone());
                            break;
                        }
                    }
                }
            }
        }

        matched
    }
}
