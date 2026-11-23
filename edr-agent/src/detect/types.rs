use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value};

/// Normalized alert the detector registry scores (pipeline.rs builds it from telemetry)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NormalizedAlert {
    pub id: String,
    pub ts: DateTime<Utc>,
    pub source: String,
    pub connector_id: String,
    pub severity: f32,
    pub title: String,
    pub short_why: String,
    pub entities: serde_json::Value,
    pub attributes: serde_json::Value,
    pub mitre: Vec<String>,
}

/// Normalized *event* used by local rule engine (built from Telemetry)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NormalizedEvent {
    pub category: Option<String>,
    pub event: Option<String>,
    pub exe: Option<String>,
    pub cmdline: Option<String>,
    pub cwd: Option<String>,
    /// free-form attributes (everything else)
    pub attrs: JsonMap<String, Value>,
    /// tag -> "1" style presence map
    pub tags: HashMap<String, String>,
}

impl NormalizedEvent {
    /// Lossy converter: build a NormalizedEvent from a Telemetry record.
    /// We use serde_json to be resilient to small struct shape changes.
    pub fn from_telemetry(t: &crate::telemetry::Telemetry) -> Self {
        let v = serde_json::to_value(t).unwrap_or(Value::Null);
        let obj = v.as_object();

        let get_str = |k: &str| -> Option<String> {
            obj.and_then(|m| m.get(k))
                .and_then(|x| x.as_str())
                .map(|s| s.to_string())
        };

        let category = get_str("category").or_else(|| get_str("cat"));
        let event = get_str("event").or_else(|| get_str("evt"));
        let exe = get_str("exe").or_else(|| get_str("binary_path"));
        let cmdline = get_str("cmdline").or_else(|| get_str("command_line"));
        let cwd = get_str("cwd");

        let mut tags: HashMap<String, String> = HashMap::new();
        if let Some(arr) = obj.and_then(|m| m.get("tags")).and_then(|v| v.as_array()) {
            for s in arr.iter().filter_map(|x| x.as_str()) {
                tags.insert(s.to_string(), "1".into());
            }
        }

        let mut attrs: JsonMap<String, Value> = JsonMap::new();
        if let Some(m) = obj {
            for (k, vv) in m {
                if matches!(
                    k.as_str(),
                    "category"
                        | "cat"
                        | "event"
                        | "evt"
                        | "exe"
                        | "binary_path"
                        | "cmdline"
                        | "command_line"
                        | "cwd"
                        | "tags"
                ) {
                    continue;
                }
                attrs.insert(k.clone(), vv.clone());
            }
        }

        Self {
            category,
            event,
            exe,
            cmdline,
            cwd,
            attrs,
            tags,
        }
    }
}
