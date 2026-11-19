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
use serde_json::json;
use std::convert::Infallible;
use std::path::Path;
use tokio::sync::broadcast;

use crate::{pb_adapter, pb_engine, pb_introspect};

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

// Re-reads the playbook directory ($PB_DIR or ./playbooks) and swaps the result
// into the engine main.rs registered with pb_adapter. Bad files are skipped and
// listed in the response. If the directory cannot be read, the current set stays.
pub async fn reload() -> (StatusCode, Json<serde_json::Value>) {
    let Some(engine) = pb_adapter::with_engine() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"ok": false, "reloaded": false, "error": "playbook engine not initialized"})),
        );
    };
    let dir = pb_engine::playbook_dir();
    let task_dir = dir.clone();
    // File reads and YAML parsing run off the async worker threads.
    let res = tokio::task::spawn_blocking(move || {
        let path = Path::new(&task_dir);
        let report = pb_engine::reload_into(&engine, path)?;
        report.log(path);
        Ok::<_, anyhow::Error>(report)
    })
    .await;

    match res {
        Ok(Ok(report)) => {
            let skipped: Vec<_> = report
                .skipped
                .iter()
                .map(|(file, reason)| json!({"file": file, "reason": reason}))
                .collect();
            let warnings: Vec<_> = report
                .warnings
                .iter()
                .map(|(id, warning)| json!({"playbook": id, "warning": warning}))
                .collect();
            (
                StatusCode::OK,
                Json(json!({
                    "ok": true,
                    "reloaded": true,
                    "dir": dir,
                    "loaded": report.loaded,
                    "skipped": skipped,
                    "warnings": warnings,
                    "ignored": report.ignored,
                })),
            )
        }
        Ok(Err(e)) => {
            eprintln!("playbooks: reload from {dir} failed, keeping the current set: {e:#}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"ok": false, "reloaded": false, "dir": dir, "error": format!("{e:#}")})),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"ok": false, "reloaded": false, "error": format!("reload task failed: {e}")})),
        ),
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pb_engine::PlaybookEngine;
    use std::sync::{Arc, Mutex};

    // cargo test runs from the crate root, so ./playbooks is the shipped set.
    #[tokio::test]
    async fn reload_endpoint_replaces_the_engine_playbooks() {
        pb_adapter::set_engine_handle(Arc::new(Mutex::new(PlaybookEngine::empty())));
        let handle = pb_adapter::with_engine().expect("engine handle");
        *handle.lock().unwrap() = PlaybookEngine::empty();

        let (status, Json(body)) = reload().await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let loaded = body["loaded"].as_array().expect("loaded list").len();
        assert!(loaded > 0, "{body}");
        assert_eq!(handle.lock().unwrap().playbooks.len(), loaded);
        assert_eq!(
            loaded + body["skipped"].as_array().unwrap().len(),
            PlaybookEngine::load_dir_report(pb_engine::playbook_dir())
                .map(|(_, r)| r.loaded.len() + r.skipped.len())
                .unwrap()
        );
    }
}
