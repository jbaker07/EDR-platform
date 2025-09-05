use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum IntegrationStatus {
    Disabled,
    Starting,
    Running,
    Error,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollStats {
    pub last_poll: Option<DateTime<Utc>>,
    pub events_seen: u64,
    pub events_emitted: u64,
}
impl Default for PollStats {
    fn default() -> Self {
        Self { last_poll: None, events_seen: 0, events_emitted: 0 }
    }
}

/// Catalog entry for the UI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntegrationSpec {
    pub id: String,
    pub name: String,
    pub vendor: String,
    pub description: String,
    pub required_fields: Vec<String>,
}

/// Persisted config for a connector instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub last_ok: Option<DateTime<Utc>>,
    pub creds: serde_json::Value,
}
impl IntegrationConfig {
    pub fn new(kind: &str, name: &str, creds: serde_json::Value) -> Self {
        Self {
            id: format!("{}-{}", kind, Uuid::new_v4()),
            kind: kind.to_string(),
            name: name.to_string(),
            enabled: true,
            created_at: Utc::now(),
            last_ok: None,
            creds,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntegrationRuntime {
    pub status: IntegrationStatus,
    pub last_error: Option<String>,
    pub poll: PollStats,
}
impl Default for IntegrationRuntime {
    fn default() -> Self {
        Self { status: IntegrationStatus::Disabled, last_error: None, poll: PollStats::default() }
    }
}

/// Normalized alert used by SaaS integrations (Okta, GitHub, …)
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
    pub event:    Option<String>,
    pub exe:      Option<String>,
    pub cmdline:  Option<String>,
    pub cwd:      Option<String>,
    /// free-form attributes (everything else)
    pub attrs: JsonMap<String, Value>,
    /// tag -> "1" style presence map
    pub tags: HashMap<String, String>,
}

impl NormalizedEvent {
    /// Lossy converter: build a NormalizedEvent from your Telemetry record.
    /// We use serde_json to be resilient to small struct shape changes.
    pub fn from_telemetry(t: &crate::telemetry::Telemetry) -> Self {
        let v = serde_json::to_value(t).unwrap_or(Value::Null);
        let obj = v.as_object();

        let get_str = |k: &str| -> Option<String> {
            obj.and_then(|m| m.get(k)).and_then(|x| x.as_str()).map(|s| s.to_string())
        };

        let category = get_str("category").or_else(|| get_str("cat"));
        let event    = get_str("event").or_else(|| get_str("evt"));
        let exe      = get_str("exe").or_else(|| get_str("binary_path"));
        let cmdline  = get_str("cmdline").or_else(|| get_str("command_line"));
        let cwd      = get_str("cwd");

        let mut tags: HashMap<String, String> = HashMap::new();
        if let Some(arr) = obj.and_then(|m| m.get("tags")).and_then(|v| v.as_array()) {
            for s in arr.iter().filter_map(|x| x.as_str()) {
                tags.insert(s.to_string(), "1".into());
            }
        }

        let mut attrs: JsonMap<String, Value> = JsonMap::new();
        if let Some(m) = obj {
            for (k, vv) in m {
                if matches!(k.as_str(),
                    "category" | "cat" |
                    "event" | "evt" |
                    "exe" | "binary_path" |
                    "cmdline" | "command_line" |
                    "cwd" | "tags") {
                    continue;
                }
                attrs.insert(k.clone(), vv.clone());
            }
        }

        Self { category, event, exe, cmdline, cwd, attrs, tags }
    }
}
