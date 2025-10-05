use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterFact {
    pub ts: i64,
    pub host: String,
    pub detector: String,
    pub kind: String,
    pub tags: Vec<String>,
    pub proc_key: Option<String>,
    pub socket_key: Option<String>,
    pub file_key: Option<String>,
    pub user_key: Option<String>,
    pub container_id: Option<String>,
    pub meta: serde_json::Value,
}
