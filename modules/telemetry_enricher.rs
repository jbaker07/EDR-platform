// edr-agent/forensic/telemetry_enricher.rs

use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use lazy_static::lazy_static;
use std::sync::Mutex;

/// Struct for an enriched telemetry payload
#[derive(Debug, Clone)]
pub struct EnrichedTelemetry {
    pub base: Value,
    pub semantic_tags: Vec<String>,
    pub mitre_ids: Vec<String>,
    pub enriched_fields: HashMap<String, String>,
}

lazy_static! {
    static ref TAG_DB: Mutex<Value> = Mutex::new(load_semantic_tags());
}

/// Load semantic_tags.json once into memory
fn load_semantic_tags() -> Value {
    let path = "ontology/semantic_tags.json";
    let content = fs::read_to_string(path).unwrap_or_else(|_| "{}".into());
    serde_json::from_str(&content).unwrap_or(Value::Null)
}

/// Public entry to enrich any telemetry JSON blob
pub fn enrich_telemetry(input: &Value) -> EnrichedTelemetry {
    let mut tags = vec![];
    let mut mitre_ids = vec![];
    let mut enriched = HashMap::new();
    let tag_db = TAG_DB.lock().unwrap();

    if let Value::Object(ref db) = *tag_db {
        for (tag, props) in db {
            if let Some(Value::Object(p)) = props.as_object() {
                if let Some(Value::Array(matchers)) = p.get("match") {
                    for matcher in matchers {
                        if let Some(Value::String(field)) = matcher.get("field") {
                            if let Some(Value::String(value)) = matcher.get("value") {
                                if let Some(Value::String(actual)) = input.get(field).and_then(|v| v.as_str().map(String::from)) {
                                    if actual.contains(value) {
                                        tags.push(tag.to_string());
                                        if let Some(Value::String(mitre)) = p.get("mitre") {
                                            mitre_ids.push(mitre.to_string());
                                        }
                                        if let Some(Value::String(risk)) = p.get("risk_level") {
                                            enriched.insert("risk_hint".into(), risk.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    EnrichedTelemetry {
        base: input.clone(),
        semantic_tags: tags,
        mitre_ids,
        enriched_fields: enriched,
    }
}
