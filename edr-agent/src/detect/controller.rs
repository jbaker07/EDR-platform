// src/detect/controller.rs
use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::detect::{registry::IntegrationsRegistry, types::*};

#[derive(Clone)]
pub struct DetectState {
    pub reg: std::sync::Arc<IntegrationsRegistry>,
    pub internal_base: String,
}

#[derive(Debug, Serialize)]
struct CatalogResp {
    items: Vec<IntegrationSpec>,
}

pub fn router(state: DetectState) -> Router {
    Router::new()
        .route("/catalog", get(get_catalog))
        .route("/", get(list_integrations).post(create_integration))
        .route("/:id", delete(delete_integration))
        .with_state(state)
}

async fn get_catalog() -> Json<CatalogResp> {
    let items = vec![
        IntegrationSpec {
            id: "okta_syslog".into(),
            name: "Okta System Log".into(),
            vendor: "Okta".into(),
            description: "Pulls org System Log via /api/v1/logs".into(),
            required_fields: ["base_url", "api_token"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        IntegrationSpec {
            id: "github_audit".into(),
            name: "GitHub Audit".into(),
            vendor: "GitHub".into(),
            description: "Org audit/security log".into(),
            required_fields: ["org", "token"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        IntegrationSpec {
            id: "duo_auth".into(),
            name: "Duo Auth Logs".into(),
            vendor: "Duo".into(),
            description: "Admin API".into(),
            required_fields: ["api_host", "ikey", "skey"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        IntegrationSpec {
            id: "auth0_logs".into(),
            name: "Auth0 Logs".into(),
            vendor: "Auth0".into(),
            description: "Tenant events".into(),
            required_fields: ["domain", "client_id", "client_secret"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        IntegrationSpec {
            id: "zeek_suricata".into(),
            name: "Zeek/Suricata".into(),
            vendor: "Zeek/Suricata".into(),
            description: "Tail local JSON logs".into(),
            required_fields: ["path"].into_iter().map(String::from).collect(),
        },
        IntegrationSpec {
            id: "windows_eventlog".into(),
            name: "Windows Event Log".into(),
            vendor: "Microsoft".into(),
            description: "Local winevt".into(),
            required_fields: ["channels"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        IntegrationSpec {
            id: "pf_opnsense_syslog".into(),
            name: "pfSense/OPNsense syslog".into(),
            vendor: "pf/OPNsense".into(),
            description: "Tail syslog file".into(),
            required_fields: ["path"].into_iter().map(String::from).collect(),
        },
    ];
    Json(CatalogResp { items })
}

#[derive(Debug, Deserialize)]
struct CreateReq {
    kind: String,
    name: String,
    creds: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct CreateResp {
    id: String,
    status: IntegrationStatus,
}

async fn create_integration(
    State(st): State<DetectState>,
    Json(req): Json<CreateReq>,
) -> Json<CreateResp> {
    let cfg = IntegrationConfig::new(&req.kind, &req.name, req.creds);
    let id = cfg.id.clone();
    st.reg.add(cfg).expect("persist add");
    st.reg
        .spawn(&id, st.internal_base.clone())
        .expect("spawn");
    Json(CreateResp {
        id,
        status: IntegrationStatus::Starting,
    })
}

#[derive(Debug, Serialize)]
struct ListItem {
    cfg: IntegrationConfig,
    rt: IntegrationRuntime,
}

async fn list_integrations(State(st): State<DetectState>) -> Json<Vec<ListItem>> {
    let v = st
        .reg
        .list()
        .into_iter()
        .map(|(cfg, rt)| ListItem { cfg, rt })
        .collect();
    Json(v)
}

async fn delete_integration(
    State(st): State<DetectState>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    st.reg.remove(&id).expect("remove");
    Json(serde_json::json!({ "ok": true }))
}
