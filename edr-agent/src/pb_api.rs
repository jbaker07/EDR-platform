// src/pb_api.rs
// Axum HTTP and SSE endpoints to expose playbook state.
// Mount these routes in main.rs:
//   .route("/playbooks/pending", get(pb_api::pending))
//   .route("/playbooks/hits", get(pb_api::hits))
//   .route("/playbooks/reload", post(pb_api::reload))
//   .route("/playbooks/stream", get(pb_api::stream))
//
// At startup, create a broadcast channel and pass the Sender to pb_introspect::set_sse_sender().

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::sse::{Event, Sse},
    Json,
};
use futures_util::stream::unfold;
use serde::Deserialize;
use std::convert::Infallible;
use tokio::sync::broadcast;

use crate::pb_introspect;

#[derive(Clone)]
pub struct PbApiState {
    pub tx: broadcast::Sender<String>,
}

pub async fn pending() -> Json<serde_json::Value> {
    let items = pb_introspect::get_pending();
    Json(serde_json::json!({ "items": items }))
}

#[derive(Deserialize)]
pub struct HitsQ {
    pub limit: Option<usize>,
}

pub async fn hits(Query(q): Query<HitsQ>) -> Json<serde_json::Value> {
    let limit = q.limit.unwrap_or(200);
    let items = pb_introspect::get_hits(limit);
    Json(serde_json::json!({ "items": items }))
}

// Stub: wire your loader to actually rebuild the in-memory bundle.
// For now we just respond 202 to indicate the request is accepted.
pub async fn reload() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({"ok": true, "reloaded": true})),
    )
}

pub async fn stream(
    State(st): State<PbApiState>,
) -> Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>> {
    let rx = st.tx.subscribe();
    let stream = unfold(rx, |mut rx| async move {
        match rx.recv().await {
            Ok(line) => Some((Ok(Event::default().data(line)), rx)),
            Err(_) => None,
        }
    });
    Sse::new(stream)
}
