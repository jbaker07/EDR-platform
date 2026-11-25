//! Live signed policy reload. A thread checks policy.json and policy.json.sig once
//! a second. When either changed, the new bytes must verify with the pinned public
//! key and carry a serial above the running one (config::judge_reload). A policy
//! that passes is applied to the running agent (the guard modes and allowlists in
//! one bank switch, the shield limits, the response actions, masking and the log
//! level), recorded as the new anti-rollback floor, and written to the audit log.
//! A refused one is written to the audit log and becomes a high severity incident,
//! because a bad signature or an older serial is what tampering looks like. The
//! running policy stays in force either way.

use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::config::{judge_reload, signature_path, LoadedPolicy, PolicyStore, Reload};

pub type ApplyFn = Box<dyn Fn(&LoadedPolicy) -> Vec<String> + Send>;

#[derive(Default)]
struct Status {
    serial: u64,
    sha256: String,
    source: String,
    applied_at: u64,
    applied_total: u64,
    refused_total: u64,
    last: Value,
}

static STATUS: OnceLock<Arc<Mutex<Status>>> = OnceLock::new();

fn status() -> &'static Arc<Mutex<Status>> {
    STATUS.get_or_init(|| Arc::new(Mutex::new(Status::default())))
}

fn sha(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

/// GET /policy/status.
pub fn status_json() -> Value {
    let s = status().lock().unwrap_or_else(|e| e.into_inner());
    json!({
        "serial": s.serial,
        "sha256": s.sha256,
        "source": s.source,
        "applied_at": s.applied_at,
        "reloads_applied_total": s.applied_total,
        "reloads_refused_total": s.refused_total,
        "last": s.last,
    })
}

/// Records the policy the agent started with.
pub fn started(p: &LoadedPolicy) {
    let mut s = status().lock().unwrap_or_else(|e| e.into_inner());
    s.serial = p.policy.serial;
    s.sha256 = sha(&p.bytes);
    s.source = p.source.display().to_string();
    s.applied_at = crate::utils::time::now_ts();
    s.last = json!({ "action": "start", "serial": p.policy.serial });
}

/// Size, change time and inode of both files: a change in any means a new read.
fn fingerprint(policy: &Path) -> Vec<(u64, i64, i64, u64)> {
    [policy.to_path_buf(), signature_path(policy)]
        .iter()
        .map(|p| {
            std::fs::metadata(p)
                .map(|m| (m.len(), m.mtime(), m.mtime_nsec(), m.ino()))
                .unwrap_or((0, 0, 0, 0))
        })
        .collect()
}

fn audit(path: &Path, line: Value) {
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    use std::os::unix::fs::OpenOptionsExt;
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
    {
        let _ = writeln!(f, "{line}");
    }
}

/// The incident a refused policy raises.
pub fn refusal_alert(path: &Path, why: &str, serial: Option<u64>) -> (crate::guard::Finding, Value) {
    let ts = crate::utils::time::now_ts();
    let reason = format!("policy {} refused: {why}", path.display());
    let tags = vec!["guard".to_string(), "guard:policy".to_string(), "T1562.001".to_string()];
    let signal = "guard::policy::refused".to_string();
    let mut data = std::collections::HashMap::new();
    data.insert("source".into(), "edr_guard".into());
    data.insert("timestamp".into(), ts.to_string());
    data.insert("guard".into(), "policy".into());
    data.insert("action".into(), "refused".into());
    data.insert("target".into(), path.display().to_string());
    data.insert("mitre".into(), "T1562.001".into());
    data.insert("tags".into(), tags.join(","));
    data.insert("signal".into(), signal.clone());
    data.insert("reason".into(), reason.clone());
    let pid = std::process::id();
    let finding = crate::guard::build_finding(signal, pid, 0, 80.0, tags, reason.clone(), data);
    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
    let alert = json!({
        "id": format!("policy:refused:{}:{ts}", serial.unwrap_or(0)),
        "source": "guard",
        "title": format!("Policy refused: {}", path.display()),
        "ts": ts,
        "severity": "high",
        "risk": 0.8,
        "technique": "T1562.001",
        "findings": [{ "label": "T1562.001" }],
        "event": { "exe": "policy", "pid": pid.to_string(), "host": host },
        "details": { "guard": "policy", "action": "refused", "mode": "enforce", "target": path.display().to_string(), "serial": serial, "reason": reason },
        "rationale": [reason],
    });
    (finding, alert)
}

/// The serial a refused file claims, when it parses at all. For the audit only.
fn claimed_serial(bytes: &[u8]) -> Option<u64> {
    serde_json::from_slice::<Value>(bytes).ok()?.pointer("/policy/serial")?.as_u64()
}

pub struct Watch {
    pub policy_path: PathBuf,
    pub pubkey_pem: Vec<u8>,
    pub store: PolicyStore,
    pub audit_path: PathBuf,
    pub allow_unsigned: bool,
    pub every: Duration,
}

impl Watch {
    /// One look at the files. Public so the tests can drive it.
    pub fn check(&self, running: &mut LoadedPolicy, seen: &mut Vec<(u64, i64, i64, u64)>, apply: &ApplyFn) -> Option<Value> {
        let now = fingerprint(&self.policy_path);
        if &now == seen {
            return None;
        }
        // Let a writer finish both files, then read them as they are.
        std::thread::sleep(Duration::from_millis(300));
        *seen = fingerprint(&self.policy_path);
        let bytes = std::fs::read(&self.policy_path).unwrap_or_default();
        let sig = std::fs::read_to_string(signature_path(&self.policy_path)).unwrap_or_default();
        let ts = crate::utils::time::now_ts();
        let line = match judge_reload(running, &bytes, &sig, &self.pubkey_pem, self.allow_unsigned, &self.policy_path) {
            Reload::Unchanged => return None,
            Reload::Refused(why) => {
                let serial = claimed_serial(&bytes);
                {
                    let mut s = status().lock().unwrap_or_else(|e| e.into_inner());
                    s.refused_total += 1;
                    s.last = json!({ "action": "refuse", "ts": ts, "claimed_serial": serial, "why": why });
                }
                log::warn!("policy: reload refused: {why}");
                eprintln!("⚠️ policy reload refused: {why}");
                let (finding, alert) = refusal_alert(&self.policy_path, &why, serial);
                crate::guard::emit_with_alert(finding, alert);
                json!({ "ts": ts, "action": "refuse", "running_serial": running.policy.serial, "claimed_serial": serial, "sha256": sha(&bytes), "why": why })
            }
            Reload::Apply(p) => {
                let notes = apply(&p);
                if let Err(e) = self.store.record(&p) {
                    log::warn!("policy: could not record serial {}: {e}", p.policy.serial);
                }
                let from = running.policy.serial;
                *running = *p;
                {
                    let mut s = status().lock().unwrap_or_else(|e| e.into_inner());
                    s.serial = running.policy.serial;
                    s.sha256 = sha(&running.bytes);
                    s.source = running.source.display().to_string();
                    s.applied_at = ts;
                    s.applied_total += 1;
                    s.last = json!({ "action": "apply", "ts": ts, "from_serial": from, "serial": running.policy.serial, "notes": notes });
                }
                log::warn!("policy: applied serial {} (was {from})", running.policy.serial);
                eprintln!("policy: applied serial {} (was {from})", running.policy.serial);
                json!({ "ts": ts, "action": "apply", "from_serial": from, "serial": running.policy.serial, "sha256": sha(&running.bytes), "notes": notes })
            }
        };
        audit(&self.audit_path, line.clone());
        Some(line)
    }

    pub fn spawn(self, mut running: LoadedPolicy, apply: ApplyFn) -> std::io::Result<std::thread::JoinHandle<()>> {
        std::thread::Builder::new().name("policy_watch".into()).spawn(move || {
            let mut seen = fingerprint(&self.policy_path);
            loop {
                std::thread::sleep(self.every);
                let _ = self.check(&mut running, &mut seen, &apply);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openssl::pkey::PKey;
    use openssl::sign::Signer;

    fn sign(key: &PKey<openssl::pkey::Private>, bytes: &[u8]) -> String {
        let mut s = Signer::new_without_digest(key).unwrap();
        hex::encode(s.sign_oneshot_to_vec(bytes).unwrap())
    }

    fn policy(serial: u64, guard: &str) -> String {
        format!(r#"{{"policy":{{"serial":{serial},"guard_mode":"{guard}","redact_command_lines":true,"response_actions":["kill_proc"],"management_ports":[22],"logging":{{"level":"info"}}}}}}"#)
    }

    #[test]
    fn the_watcher_applies_newer_policies_and_refuses_rollback_and_forgery() {
        let key = PKey::generate_ed25519().unwrap();
        let pem = key.public_key_to_pem().unwrap();
        let d = std::env::temp_dir().join(format!("policy_watch_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        let path = d.join("policy.json");
        let write = |text: &str, sig: &str| {
            std::fs::write(&path, text).unwrap();
            std::fs::write(signature_path(&path), sig).unwrap();
        };
        let p2 = policy(2, "monitor");
        write(&p2, &sign(&key, p2.as_bytes()));
        let mut running = LoadedPolicy {
            policy: crate::config::parse_policy_bytes(p2.as_bytes()).unwrap(),
            bytes: p2.clone().into_bytes(),
            sig: sign(&key, p2.as_bytes()),
            source: path.clone(),
        };
        let w = Watch {
            policy_path: path.clone(),
            pubkey_pem: pem,
            store: PolicyStore::new(&d.join("state")),
            audit_path: d.join("policy_audit.jsonl"),
            allow_unsigned: false,
            every: Duration::from_millis(10),
        };
        let applied = Arc::new(Mutex::new(Vec::new()));
        let a2 = applied.clone();
        let apply: ApplyFn = Box::new(move |p| {
            a2.lock().unwrap().push(p.policy.serial);
            vec![format!("guard_mode {}", p.policy.guard_mode)]
        });
        let mut seen = fingerprint(&path);
        assert!(w.check(&mut running, &mut seen, &apply).is_none(), "nothing changed");

        let p3 = policy(3, "enforce");
        write(&p3, &sign(&key, p3.as_bytes()));
        let line = w.check(&mut running, &mut seen, &apply).unwrap();
        assert_eq!(line["action"], "apply");
        assert_eq!(running.policy.serial, 3);
        assert_eq!(w.store.floor(), 3);

        let p1 = policy(1, "off");
        write(&p1, &sign(&key, p1.as_bytes()));
        let line = w.check(&mut running, &mut seen, &apply).unwrap();
        assert_eq!(line["action"], "refuse");
        assert_eq!(line["claimed_serial"], 1);
        assert_eq!(running.policy.serial, 3, "the running policy stays");

        let p9 = policy(9, "off");
        let other = PKey::generate_ed25519().unwrap();
        write(&p9, &sign(&other, p9.as_bytes()));
        let line = w.check(&mut running, &mut seen, &apply).unwrap();
        assert_eq!(line["action"], "refuse");
        assert!(line["why"].as_str().unwrap().contains("signature"));
        assert_eq!(*applied.lock().unwrap(), vec![3]);
        let audit = std::fs::read_to_string(d.join("policy_audit.jsonl")).unwrap();
        assert_eq!(audit.lines().count(), 3);
        let _ = std::fs::remove_dir_all(&d);
    }
}
