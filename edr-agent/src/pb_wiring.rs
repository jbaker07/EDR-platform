// src/pb_wiring.rs
// Wire TelemetryRecord -> PlaybookEngine Facts + batch ingest shim.

use std::collections::HashMap;

use crate::pb_engine::{Fact, FactKind, PlaybookEngine, PlaybookHit as EngineHit};
use crate::pb_introspect::{PendingEvidence, PlaybookHit as IntrospectHit};
use crate::telemetry::TelemetryRecord;

/// Simple context wrapper if you want to own a PB engine outside main.
pub struct PbCtx {
    pub engine: PlaybookEngine,
}

impl PbCtx {
    pub fn new(dir: &str) -> Self {
        // Ensure the directory exists so load_dir can succeed even if empty.
        let _ = std::fs::create_dir_all(dir);

        let engine = match PlaybookEngine::load_dir(dir) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("pb_wiring: failed to load playbooks from '{dir}': {e}");
                // Fallback: use an empty temp dir so the engine is valid but has no playbooks.
                let tmp = std::env::temp_dir().join("pb_empty");
                let _ = std::fs::create_dir_all(&tmp);
                PlaybookEngine::load_dir(&tmp)
                    .expect("pb_wiring: failed to build empty PlaybookEngine")
            }
        };
        Self { engine }
    }
}

/// Output container returned by `ingest_batch`.
#[derive(Default)]
pub struct BatchOut {
    pub pending: Vec<PendingEvidence>,
    pub hits: Vec<IntrospectHit>,
}

/// Convert an engine-level hit into an introspection hit.
/// NOTE: `pb_engine::PlaybookHit` does NOT expose `id`, `filled_slots`, or `missing_slots`.
/// We synthesize a stable-ish ID and provide minimal tags/rationale using known fields.
fn to_introspect_hit(h: EngineHit) -> IntrospectHit {
    // synthesize id from stable bits we know exist
    let id = format!("pb:{}:{}:{}", h.playbook_id, h.host, h.ts);

    // use playbook_name as a visible tag; severity for rationale
    let tag = h.playbook_name.clone();
    let sev = format!("{:?}", h.severity).to_lowercase();
    let rationale = vec![format!("{} {} on {}", h.playbook_name, sev, h.host)];

    IntrospectHit {
        id,
        playbook_id: h.playbook_id,
        ts: h.ts,
        filled_slots: vec![],     // engine hit doesn't expose slots here
        missing_slots: vec![],    // ^
        tags: vec![tag],
        rationale,
    }
}

/// Batch ingest: map each TelemetryRecord -> one or more Facts and feed them
/// to the PlaybookEngine. Any completed playbooks (hits) are collected and
/// returned. We currently do not synthesize "pending" here; IOC/YARA wiring
/// already pushes pending evidence elsewhere.
pub fn ingest_batch(
    engine: &mut PlaybookEngine,
    records: &[TelemetryRecord],
    _now: u64,
) -> BatchOut {
    let mut out = BatchOut::default();

    for r in records {
        for fact in facts_from_record(r) {
            if let Some(hit) = engine.ingest_fact(fact) {
                out.hits.push(to_introspect_hit(hit));
            }
        }
    }
    out
}

/// Map a TelemetryRecord to one or more Facts understood by PlaybookEngine.
pub fn facts_from_record(r: &TelemetryRecord) -> Vec<Fact> {
    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
    let user = if r.uid == 0 {
        "root".into()
    } else {
        format!("uid:{}", r.uid)
    };

    let mut out = Vec::<Fact>::new();

    // Exec fact (when we have process info).
    if !r.binary_path.is_empty() || !r.command_line.is_empty() {
        let mut fields = HashMap::new();
        if !r.binary_path.is_empty() {
            fields.insert("exe".into(), r.binary_path.clone());
        }
        if !r.command_line.is_empty() {
            fields.insert("cmd".into(), r.command_line.clone());
        }
        fields.insert("user".into(), user.clone());
        fields.insert("host".into(), host.clone());

        out.push(Fact {
            ts: r.timestamp,
            kind: FactKind::Exec,
            host: host.clone(),
            user: Some(user.clone()),
            exe: if !r.binary_path.is_empty() {
                Some(r.binary_path.clone())
            } else {
                None
            },
            cmd: if !r.command_line.is_empty() {
                Some(r.command_line.clone())
            } else {
                None
            },
            fields,
        });
    }

    // FileIO (heuristic based on tags/cwd).
    let is_write = r.tags.iter().any(|t| t.contains("write"));
    let is_file = r
        .tags
        .iter()
        .any(|t| t.starts_with("file_") || t.contains("open") || t.contains("write"));
    if is_file {
        let op = if is_write { "write" } else { "open" };
        let path = if !r.cwd.is_empty() {
            r.cwd.clone()
        } else {
            "-".into()
        };

        let mut fields = HashMap::new();
        fields.insert("op".into(), op.into());
        fields.insert("path".into(), path.clone());
        fields.insert("topdir".into(), topdir_of(&path));
        if !r.binary_path.is_empty() {
            fields.insert("exe".into(), r.binary_path.clone());
        }
        if !r.command_line.is_empty() {
            fields.insert("cmd".into(), r.command_line.clone());
        }
        fields.insert("user".into(), user.clone());
        fields.insert("host".into(), host.clone());

        out.push(Fact {
            ts: r.timestamp,
            kind: FactKind::FileIO,
            host: host.clone(),
            user: Some(user.clone()),
            exe: if !r.binary_path.is_empty() {
                Some(r.binary_path.clone())
            } else {
                None
            },
            cmd: if !r.command_line.is_empty() {
                Some(r.command_line.clone())
            } else {
                None
            },
            fields,
        });
    }

    // NetConn (heuristic from tags/cmdline).
    let is_net = r
        .tags
        .iter()
        .any(|t| t.contains("net_") || t.contains("dns") || t.contains("connect"));
    if is_net {
        let ip_port = r
            .command_line
            .split_whitespace()
            .find(|s| s.contains(':') && s.chars().any(|c| c.is_ascii_digit()))
            .unwrap_or("-");
        let (daddr, dport) = split_ip_port(ip_port);

        let mut fields = HashMap::new();
        fields.insert("daddr".into(), daddr.clone());
        if let Some(p) = dport {
            fields.insert("dport".into(), p.to_string());
        }
        fields.insert("proto".into(), "tcp".into());
        if !r.binary_path.is_empty() {
            fields.insert("exe".into(), r.binary_path.clone());
        }
        if !r.command_line.is_empty() {
            fields.insert("cmd".into(), r.command_line.clone());
        }
        fields.insert("user".into(), user.clone());
        fields.insert("host".into(), host.clone());

        out.push(Fact {
            ts: r.timestamp,
            kind: FactKind::NetConn,
            host: host.clone(),
            user: Some(user.clone()),
            exe: if !r.binary_path.is_empty() {
                Some(r.binary_path.clone())
            } else {
                None
            },
            cmd: if !r.command_line.is_empty() {
                Some(r.command_line.clone())
            } else {
                None
            },
            fields,
        });
    }

    out
}

fn split_ip_port(s: &str) -> (String, Option<u16>) {
    if let Some((a, b)) = s.rsplit_once(':') {
        if let Ok(p) = b.parse::<u16>() {
            return (a.to_string(), Some(p));
        }
        return (a.to_string(), None);
    }
    (s.to_string(), None)
}

fn topdir_of(path: &str) -> String {
    let p = path.trim();
    if !p.starts_with('/') {
        return "-".into();
    }
    let mut parts = p.split('/').filter(|x| !x.is_empty());
    if let Some(first) = parts.next() {
        format!("/{}", first)
    } else {
        "-".into()
    }
}
