use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SemanticTagInfo {
    pub tag: String,
    pub description: String,
    pub category: String,
    pub mitre_tactic: Option<String>,
    pub mitre_technique: Option<String>,
    pub risk_score: f64,
}

#[derive(Debug)]
pub struct OntologyMap {
    pub tags: HashMap<String, SemanticTagInfo>,
}

impl OntologyMap {
    pub fn load_from_files(semantic_tag_path: &str, risk_weight_path: &str) -> Self {
        let tag_json = fs::read_to_string(Path::new(semantic_tag_path))
            .expect("[ontology_mapper] Failed to read semantic_tags.json");

        let weight_json = fs::read_to_string(Path::new(risk_weight_path))
            .expect("[ontology_mapper] Failed to read tag_risk_weights.json");

        let mut tag_map: HashMap<String, SemanticTagInfo> =
            serde_json::from_str(&tag_json).expect("Failed to parse semantic_tags.json");

        let risk_weights: HashMap<String, f64> =
            serde_json::from_str(&weight_json).expect("Failed to parse tag_risk_weights.json");

        for (tag, weight) in risk_weights.iter() {
            if let Some(tag_info) = tag_map.get_mut(tag) {
                tag_info.risk_score = *weight;
            }
        }

        OntologyMap { tags: tag_map }
    }

    pub fn enrich_tag(&self, tag: &str) -> Option<&SemanticTagInfo> {
        self.tags.get(tag)
    }

    pub fn enrich_tags(&self, tag_list: Vec<String>) -> Vec<SemanticTagInfo> {
        tag_list
            .iter()
            .filter_map(|t| self.enrich_tag(t).cloned())
            .collect()
    }
}

/// Convenience loader (singleton-like)
static mut ONTOLOGY_INSTANCE: Option<OntologyMap> = None;

pub fn init_ontology_mapper(sem_path: &str, risk_path: &str) {
    let ontology = OntologyMap::load_from_files(sem_path, risk_path);
    unsafe {
        ONTOLOGY_INSTANCE = Some(ontology);
    }
}

pub fn map_tags(tags: Vec<String>) -> Vec<SemanticTagInfo> {
    unsafe {
        if let Some(ref mapper) = ONTOLOGY_INSTANCE {
            return mapper.enrich_tags(tags);
        } else {
            eprintln!("[ontology_mapper] Ontology not initialized.");
            vec![]
        }
    }
}
