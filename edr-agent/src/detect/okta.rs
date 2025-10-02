use std::{fs, io::Write, time::Duration};

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use log::{info, warn};
use reqwest::header::HeaderMap as ReqHeaderMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::Receiver;
use tokio::time::sleep;

use crate::detect::{registry::IntegrationsRegistry, types::*};

#[derive(Debug, Deserialize, Clone)]
struct OktaEvent {
    #[serde(default)]
    uuid: String,
    #[serde(default)]
    eventType: String,
    #[serde(default)]
    displayMessage: String,
    #[serde(default)]
    severity: String, // INFO|WARN|ERROR
    #[serde(default)]
    published: Option<DateTime<Utc>>,
    #[serde(default)]
    actor: serde_json::Value,
    #[serde(default)]
    target: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    client: Option<serde_json::Value>,
    #[serde(default)]
    outcome: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
struct CredsOkta {
    base_url: String,
    api_token: String,
}

const CURSOR_FILE_PREFIX: &str = "state/okta_cursor_"; // + <id>.json

#[derive(Debug, Serialize, Deserialize, Default)]
struct Cursor {
    since: Option<DateTime<Utc>>,
    next_url: Option<String>,
}

fn cursor_path(id: &str) -> String {
    format!("{}{}.json", CURSOR_FILE_PREFIX, id)
}

fn load_cursor(id: &str) -> Result<Cursor> {
    let p = cursor_path(id);
    if !std::path::Path::new(&p).exists() {
        return Ok(Cursor::default());
    }
    let s = fs::read_to_string(&p)?;
    Ok(serde_json::from_str::<Cursor>(&s)?)
}

fn save_cursor(id: &str, c: &Cursor) -> Result<()> {
    let _ = fs::create_dir_all("state");
    let p = cursor_path(id);
    let blob = serde_json::to_string_pretty(c)?;
    fs::write(p, blob)?;
    Ok(())
}

fn normalize_base(base: &str) -> String {
    let base = base.trim_end_matches('/');
    if base.starts_with("http://") || base.starts_with("https://") {
        base.to_string()
    } else {
        format!("https://{}", base)
    }
}

fn parse_next_link(headers: &ReqHeaderMap) -> Option<String> {
    // Okta uses RFC5988 Link header: <url>; rel="next"
    let link = headers.get("Link")?.to_str().ok()?;
    for part in link.split(',') {
        let p = part.trim();
        if p.contains("rel=\"next\"") {
            let start = p.find('<')?;
            let end = p.find('>')?;
            return Some(p[start + 1..end].to_string());
        }
    }
    None
}

async fn fetch_page(
    client: &Client,
    url: &str,
    token: &str,
) -> Result<(Vec<OktaEvent>, Option<String>)> {
    let resp = client
        .get(url)
        .header("Authorization", format!("SSWS {}", token))
        .header("Accept", "application/json")
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!("okta http {} at {}", resp.status(), url));
    }

    let headers = resp.headers().clone();
    let body = resp.json::<serde_json::Value>().await?;
    let mut events: Vec<OktaEvent> = Vec::new();

    if let Some(arr) = body.as_array() {
        for v in arr {
            match serde_json::from_value::<OktaEvent>(v.clone()) {
                Ok(ev) => events.push(ev),
                Err(e) => warn!("okta: failed to decode event: {:?}", e),
            }
        }
    }

    let next = parse_next_link(&headers);
    Ok((events, next))
}

fn mitre_hints(event_type: &str) -> Vec<String> {
    match event_type {
        "user.session.start" => vec!["T1078".into()], // Valid Accounts
        "system.mfa.factor.verify" | "user.mfa.factor.verify" => vec!["T1110".into()],
        "application.oauth2.as.token.issue" => vec!["T1550".into()],
        _ => vec![],
    }
}

async fn emit_alert_http(_internal_base: &str, alert: &NormalizedAlert) -> Result<()> {
    let _ = fs::create_dir_all("logs");
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/alerts.jsonl")?;
    let line = serde_json::to_string(alert)?;
    f.write_all(line.as_bytes())?;
    f.write_all(b"\n")?;
    Ok(())
}

pub async fn run_okta_connector(
    cfg: IntegrationConfig,
    internal_base: String,
    mut cancel: Receiver<()>,
    reg: IntegrationsRegistry,
) {
    // Parse creds
    let creds: CredsOkta = match serde_json::from_value(cfg.creds.clone()) {
        Ok(c) => c,
        Err(e) => {
            reg.update_status(
                &cfg.id,
                IntegrationStatus::Error,
                Some(format!("bad creds: {e}")),
                None,
                None,
            );
            return;
        }
    };

    let client = Client::builder()
        .user_agent("edr-detect/0.1")
        .build()
        .expect("client");

    // Cursor / polling
    let mut cur = load_cursor(&cfg.id).unwrap_or_default();
    if cur.since.is_none() {
        cur.since = Some(Utc::now() - chrono::Duration::minutes(10));
    }
    let mut next_url = cur.next_url.clone();
    let mut since = cur.since;

    let poll_every: u64 = std::env::var("OKTA_POLL_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);

    let mut backoff_secs: u64 = 1;

    reg.update_status(&cfg.id, IntegrationStatus::Running, None, None, None);
    info!("okta connector started id={} kind={}", cfg.id, cfg.kind);

    loop {
        let mut tick = sleep(Duration::from_secs(poll_every));
        tokio::pin!(tick); // required for tokio::select! with Sleep

        tokio::select! {
            _ = &mut cancel => {
                info!("okta cancelled id={}", cfg.id);
                break;
            }
            _ = &mut tick => {}
        }

        let mut seen: u64 = 0;
        let mut emitted: u64 = 0;
        let mut last_ts: Option<DateTime<Utc>> = None;

        let base = normalize_base(&creds.base_url);
        let mut page_url = match next_url.clone() {
            Some(u) => u,
            None => {
                let mut url = format!("{}/api/v1/logs?limit=1000", base);
                if let Some(s) = since {
                    url.push_str(&format!("&since={}", s.to_rfc3339()));
                }
                url
            }
        };

        // paginate within the same poll tick
        loop {
            match fetch_page(&client, &page_url, &creds.api_token).await {
                Ok((events, next)) => {
                    seen += events.len() as u64;

                    for ev in events {
                        let sev = match ev.severity.as_str() {
                            "ERROR" => 0.9,
                            "WARN" => 0.6,
                            _ => 0.3,
                        };

                        let title = if ev.eventType.is_empty() {
                            "Okta Event".to_string()
                        } else {
                            ev.eventType.clone()
                        };
                        let short = if ev.displayMessage.is_empty() {
                            ev.eventType.clone()
                        } else {
                            ev.displayMessage.clone()
                        };

                        let entities = serde_json::json!({
                            "actor": ev.actor,
                            "target": ev.target
                        });
                        let attrs = serde_json::json!({
                            "eventType": ev.eventType,
                            "client": ev.client,
                            "outcome": ev.outcome,
                            "okta_uuid": ev.uuid
                        });

                        let ts = ev.published.unwrap_or_else(|| Utc::now());
                        last_ts = Some(last_ts.map_or(ts, |old| old.max(ts)));

                        let alert = NormalizedAlert {
                            id: format!("{}:{}", cfg.id, ev.uuid),
                            ts,
                            source: "okta".into(),
                            connector_id: cfg.id.clone(),
                            severity: sev,
                            title,
                            short_why: short,
                            entities,
                            attributes: attrs,
                            mitre: mitre_hints(ev.eventType.as_str()),
                        };

                        if let Err(e) = emit_alert_http(&internal_base, &alert).await {
                            warn!("emit alert failed: {:?}", e);
                        } else {
                            emitted += 1;
                        }
                    }

                    if let Some(nu) = next {
                        page_url = nu;
                        continue; // next page
                    } else {
                        next_url = None;
                        break; // done with pagination this tick
                    }
                }
                Err(e) => {
                    warn!("okta fetch failed; backing off (id={}): {:?}", cfg.id, e);
                    sleep(Duration::from_secs(backoff_secs.min(60))).await;
                    backoff_secs = (backoff_secs * 2).min(120);
                    break; // retry next tick
                }
            }
        }

        if let Some(ts) = last_ts {
            since = Some(ts);
            let _ = save_cursor(
                &cfg.id,
                &Cursor {
                    since,
                    next_url: None,
                },
            );
        }

        reg.update_status(
            &cfg.id,
            IntegrationStatus::Running,
            None,
            Some(Utc::now()),
            Some((seen, emitted)),
        );

        backoff_secs = 1; // reset after success
    }
}
