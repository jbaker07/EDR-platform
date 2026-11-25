//! Playbook hits propose shields. A playbook names shield templates (see
//! ShieldTemplate in pb_engine.rs). When it fires, each template becomes an
//! unsigned shield document built from the hit's evidence: a reverse shell gives
//! connect_deny of its destination for the shell's cgroup, a dropped payload
//! that ran gives exec_deny of that file. The document goes on the incident, ready
//! for an operator to sign and POST to /shields. The agent applies it by itself
//! only when the signed policy lists the kind under shields.auto_apply.

use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use super::{document_text, ShieldDoc, ShieldKind, ShieldMatch, ShieldMode, ShieldScope};
use crate::pb_engine::{PlaybookHit, ShieldTemplate};

pub const CGROUP_ROOT: &str = "/sys/fs/cgroup";

#[derive(Clone, Debug)]
pub struct Proposal {
    pub doc: ShieldDoc,
    pub document: String,
    pub summary: String,
    /// Why the scope or the target is not what the template asked for.
    pub notes: Vec<String>,
}

impl Proposal {
    pub fn json(&self) -> Value {
        json!({
            "id": self.doc.id,
            "kind": self.doc.kind.as_str(),
            "summary": self.summary,
            "notes": self.notes,
            "expires_at": self.doc.expires_at,
            "document": self.document,
            "sign_with": "scripts/policy_signer.py sign <file with the document>",
        })
    }
}

/// The cgroup v2 directory of a process, from /proc/<pid>/cgroup ("0::/path").
/// None when the process is gone or sits outside this cgroup namespace.
pub fn cgroup_dir_of(proc_root: &Path, cgroup_root: &Path, pid: u32) -> Option<PathBuf> {
    let text = std::fs::read_to_string(proc_root.join(pid.to_string()).join("cgroup")).ok()?;
    let rel = text.lines().find_map(|l| l.strip_prefix("0::"))?.trim();
    if rel.contains("..") {
        return None;
    }
    let dir = cgroup_root.join(rel.trim_start_matches('/'));
    dir.join("cgroup.procs").exists().then_some(dir)
}

fn short_hash(parts: &[&str]) -> String {
    let mut h = Sha256::new();
    for p in parts {
        h.update(p.as_bytes());
        h.update(b"|");
    }
    hex::encode(h.finalize())[..8].to_string()
}

/// The shield id for a proposal: playbook, kind and target, so a repeat of the
/// same attack proposes the same id with a higher serial.
fn proposal_id(playbook: &str, kind: ShieldKind, target: &str) -> String {
    let pb: String = playbook.trim_start_matches("pb_").chars().take(36).collect();
    let k = match kind {
        ShieldKind::ExecDeny => "exec",
        ShieldKind::FileDeny => "file",
        ShieldKind::ConnectDeny => "conn",
    };
    format!("pb.{pb}.{k}.{}", short_hash(&[playbook, k, target]))
}

/// The proposals of one playbook hit. `now` is Unix seconds and also the serial,
/// so a later proposal for the same id always has a higher one.
pub fn from_hit(hit: &PlaybookHit, incident_id: &str, now: u64, proc_root: &Path, cgroup_root: &Path) -> Vec<Proposal> {
    hit.shield_templates
        .iter()
        .filter_map(|t| from_template(hit, t, incident_id, now, proc_root, cgroup_root))
        .collect()
}

fn from_template(
    hit: &PlaybookHit,
    t: &ShieldTemplate,
    incident_id: &str,
    now: u64,
    proc_root: &Path,
    cgroup_root: &Path,
) -> Option<Proposal> {
    let kind = ShieldKind::parse(&t.kind)?;
    let slot = hit.slots.get(&t.slot)?;
    let field = |k: &str| slot.fields.get(k).map(String::as_str).filter(|v| !v.is_empty());
    let mut notes = Vec::new();

    let (target, label) = match kind {
        ShieldKind::ConnectDeny => {
            let ip: std::net::IpAddr = field("dst_ip")?.parse().ok()?;
            let port: Option<u16> = if t.with_port { field("dst_port").and_then(|p| p.parse().ok()).filter(|p| *p != 0) } else { None };
            let label = match port {
                Some(p) => format!("{ip} port {p}"),
                None => format!("{ip} any port"),
            };
            (ShieldMatch { cidr: Some(ip.to_string()), port, ..Default::default() }, label)
        }
        ShieldKind::ExecDeny | ShieldKind::FileDeny => {
            let path = if kind == ShieldKind::FileDeny { field("path").or_else(|| field("exe")) } else { field("exe") }?;
            if let Some(re) = &t.path_regex {
                if !Regex::new(re).map(|r| r.is_match(path)).unwrap_or(false) {
                    return None;
                }
            }
            // Named by device and inode, read now. The signed document then names
            // that exact file, whatever path points at it later.
            let meta = std::fs::metadata(path).ok()?;
            (
                ShieldMatch { path: Some(path.to_string()), dev: Some(meta.dev()), ino: Some(meta.ino()), ..Default::default() },
                path.to_string(),
            )
        }
    };

    let scope = match &t.scope_slot {
        None => ShieldScope::default(),
        Some(s) => {
            let pid: Option<u32> = hit.slots.get(s).and_then(|h| h.fields.get("pid")).and_then(|p| p.parse().ok());
            match pid.and_then(|p| cgroup_dir_of(proc_root, cgroup_root, p)) {
                Some(dir) => ShieldScope { cgroup: Some(dir.display().to_string()) },
                None => {
                    notes.push(format!("the process of slot {s} is gone or outside this cgroup namespace, so the scope is the whole host"));
                    ShieldScope::default()
                }
            }
        }
    };

    let doc = ShieldDoc {
        id: proposal_id(&hit.playbook_id, kind, &label),
        serial: now,
        kind,
        target,
        scope: scope.clone(),
        mode: ShieldMode::Enforce,
        not_before: now,
        expires_at: now + t.ttl_sec,
        reason: Some(format!("proposed by playbook {} from slot {}", hit.playbook_id, t.slot)),
        incident_id: Some(incident_id.to_string()),
    };
    let summary = format!(
        "{} {label} for {}, {} s",
        kind.as_str(),
        scope.cgroup.as_deref().unwrap_or("the whole host"),
        t.ttl_sec
    );
    Some(Proposal { document: document_text(&doc), doc, summary, notes })
}
