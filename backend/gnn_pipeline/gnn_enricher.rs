use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::semantic::ontology_loader::{get_ontology_entry};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GnnNode {
    pub id: String,
    pub node_type: String,
    pub attributes: HashMap<String, String>,
    pub tags: Vec<String>,
    pub enriched_fields: HashMap<String, String>
}

pub fn enrich_node_with_ontology(mut node: GnnNode) -> GnnNode {
    let mut enrichment = HashMap::new();

    for tag in &node.tags {
        if let Some(entry) = get_ontology_entry(tag) {
            enrichment.insert(format!("{}_description", tag), entry.description);
            enrichment.insert(format!("{}_category", tag), entry.category);
            enrichment.insert(format!("{}_kill_chain", tag), entry.kill_chain_phase);
            if let Some(level) = entry.threat_level {
                enrichment.insert(format!("{}_threat_level", tag), level);
            }
        }
    }

    node.enriched_fields = enrichment;
    node
}
