use std::collections::HashMap;
use std::fs;
use serde::Deserialize;
use once_cell::sync::Lazy;
use std::sync::RwLock;

#[derive(Debug, Clone, Deserialize)]
pub struct OntologyEntry {
    pub description: String,
    pub category: String,
    pub kill_chain_phase: String,
    pub examples: Option<Vec<String>>,
    pub version: Option<String>,
    pub threat_level: Option<String>
}

static ONTOLOGY_MAP: Lazy<RwLock<HashMap<String, OntologyEntry>>> = Lazy::new(|| {
    let path = "ontology/semantic_tags.json";
    let data = fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
    let parsed: HashMap<String, OntologyEntry> = serde_json::from_str(&data).unwrap_or_default();
    RwLock::new(parsed)
});

/// Returns the full ontology record for a tag
pub fn get_ontology_entry(tag: &str) -> Option<OntologyEntry> {
    ONTOLOGY_MAP.read().unwrap().get(tag).cloned()
}

/// Returns description or fallback text
pub fn get_tag_description(tag: &str) -> String {
    get_ontology_entry(tag)
        .map(|e| e.description)
        .unwrap_or_else(|| "Unknown or undocumented tag".to_string())
}
