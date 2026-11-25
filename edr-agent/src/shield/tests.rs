use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use openssl::pkey::{PKey, Private};
use openssl::sign::Signer;

use super::*;
use crate::config::{AutoApply, AutoPromote};
use crate::pb_engine::{PlaybookHit, ShieldTemplate, SlotHitSummary};

struct FakeClock {
    unix: AtomicU64,
}

impl FakeClock {
    fn at(unix: u64) -> Arc<Self> {
        Arc::new(Self { unix: AtomicU64::new(unix) })
    }
    fn advance(&self, secs: u64) {
        self.unix.fetch_add(secs, Ordering::Relaxed);
    }
}

impl Clock for Arc<FakeClock> {
    fn unix(&self) -> u64 {
        self.unix.load(Ordering::Relaxed)
    }
    // Boot time runs alongside, starting at boot 0 = unix 1_000_000.
    fn boot_ns(&self) -> u64 {
        (self.unix() - 1_000_000) * 1_000_000_000
    }
}

impl RuleMaps for Arc<MemMaps> {
    fn put(&self, kind: ShieldKind, key: &[u8], rule: &ShieldRule) -> Result<(), String> {
        (**self).put(kind, key, rule)
    }
    fn get(&self, kind: ShieldKind, key: &[u8]) -> Option<ShieldRule> {
        (**self).get(kind, key)
    }
    fn delete(&self, kind: ShieldKind, key: &[u8]) -> Result<(), String> {
        (**self).delete(kind, key)
    }
    fn keys(&self, kind: ShieldKind) -> Vec<Vec<u8>> {
        (**self).keys(kind)
    }
    fn set_slot(&self, slot: u32, value: u64) -> Result<(), String> {
        (**self).set_slot(slot, value)
    }
    fn hits(&self, num: u32) -> u64 {
        (**self).hits(num)
    }
    fn reset_hits(&self, num: u32) -> Result<(), String> {
        (**self).reset_hits(num)
    }
}

const NOW: u64 = 1_700_000_000;

struct Rig {
    m: ShieldManager,
    maps: Arc<MemMaps>,
    clock: Arc<FakeClock>,
    key: PKey<Private>,
    dir: PathBuf,
}

fn temp(tag: &str) -> PathBuf {
    static N: AtomicU64 = AtomicU64::new(0);
    let d = std::env::temp_dir().join(format!("edr_shield_{tag}_{}_{}", std::process::id(), N.fetch_add(1, Ordering::Relaxed)));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn policy() -> ShieldPolicy {
    ShieldPolicy { max_lifetime_sec: 7200, max_active: 3, ..Default::default() }
}

fn rig_with(tag: &str, p: ShieldPolicy, never: NeverBlock, dir: Option<PathBuf>) -> Rig {
    let key = PKey::generate_ed25519().unwrap();
    let pem = key.public_key_to_pem().unwrap();
    let dir = dir.unwrap_or_else(|| temp(tag));
    let maps = Arc::new(MemMaps::default());
    let clock = FakeClock::at(NOW);
    let m = ShieldManager::new(
        p,
        never,
        Box::new(maps.clone()),
        Box::new(clock.clone()),
        pem,
        &dir.join("state"),
        &dir.join("shields_audit.jsonl"),
        "kernel",
    );
    Rig { m, maps, clock, key, dir }
}

fn rig(tag: &str) -> Rig {
    rig_with(tag, policy(), NeverBlock::default(), None)
}

fn sign(key: &PKey<Private>, text: &str) -> String {
    let mut s = Signer::new_without_digest(key).unwrap();
    hex::encode(s.sign_oneshot_to_vec(text.as_bytes()).unwrap())
}

fn conn_doc(id: &str, serial: u64, cidr: &str, port: Option<u16>, life: u64) -> ShieldDoc {
    ShieldDoc {
        id: id.into(),
        serial,
        kind: ShieldKind::ConnectDeny,
        target: ShieldMatch { cidr: Some(cidr.into()), port, ..Default::default() },
        scope: ShieldScope::default(),
        mode: ShieldMode::Enforce,
        not_before: NOW,
        expires_at: NOW + life,
        reason: Some("test".into()),
        incident_id: None,
    }
}

impl Rig {
    fn post(&self, doc: &ShieldDoc) -> Outcome {
        let text = document_text(doc);
        self.m.apply_signed(&text, &sign(&self.key, &text))
    }
    fn rule(&self, kind: ShieldKind, key: &[u8]) -> Option<ShieldRule> {
        self.maps.get(kind, key)
    }
    fn slot(&self, slot: u32) -> u64 {
        self.maps.slots.lock().unwrap().get(&slot).copied().unwrap_or(0)
    }
    fn audit_actions(&self) -> Vec<String> {
        std::fs::read_to_string(self.dir.join("shields_audit.jsonl"))
            .unwrap_or_default()
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).unwrap()["action"].as_str().unwrap().to_string())
            .collect()
    }
}

fn key_for(cidr: &str, port: u16) -> Vec<u8> {
    let n = masked(&Net::parse(cidr).unwrap());
    abi::conn_key(&n.addr, n.prefix, port).to_vec()
}

#[test]
fn a_signed_shield_starts_in_monitor_with_its_deadline_in_boot_time() {
    let r = rig("monitor");
    let o = r.post(&conn_doc("c2", 1, "203.0.113.10", Some(8000), 3600));
    assert_eq!(o.status, 201, "{:?}", o.body);
    assert_eq!(o.body["shield"]["mode"], "monitor");
    assert_eq!(o.body["shield"]["signed_mode"], "enforce");
    let row = r.rule(ShieldKind::ConnectDeny, &key_for("203.0.113.10", 8000)).unwrap();
    assert_eq!(row.mode, abi::MODE_MONITOR);
    assert_eq!(row.kind, abi::KIND_CONNECT);
    // The clock's boot time is (unix - 1_000_000) s, so the deadline is NOW + 3600 on it.
    assert_eq!(row.expires_ns, (NOW + 3600 - 1_000_000) * 1_000_000_000);
    assert_eq!(row.not_before_ns, 0);
    assert_eq!(r.slot(abi::SCFG_CONN_ROWS), 1);
    assert_eq!(r.slot(abi::SCFG_EXEC_ROWS), 0);
    // State and audit files are owner only.
    use std::os::unix::fs::PermissionsExt;
    for f in [r.m.journal_path(), r.dir.join("shields_audit.jsonl")] {
        assert_eq!(std::fs::metadata(&f).unwrap().permissions().mode() & 0o777, 0o600, "{}", f.display());
    }
    assert_eq!(r.audit_actions(), vec!["apply"]);
}

#[test]
fn tampered_replayed_overlong_and_badly_formed_documents_are_refused() {
    let r = rig("refuse");
    let doc = conn_doc("c2", 5, "203.0.113.10", None, 600);
    let text = document_text(&doc);
    let sig = sign(&r.key, &text);
    // One changed byte: the address.
    let tampered = text.replace("203.0.113.10", "203.0.113.11");
    let o = r.m.apply_signed(&tampered, &sig);
    assert_eq!((o.status, o.body["refused"].as_str()), (403, Some("bad_signature")));
    // Signed by another key.
    let other = PKey::generate_ed25519().unwrap();
    assert_eq!(r.m.apply_signed(&text, &sign(&other, &text)).status, 403);
    // The real one applies, and the same document again is a replay.
    assert_eq!(r.m.apply_signed(&text, &sig).status, 201);
    let o = r.m.apply_signed(&text, &sig);
    assert_eq!((o.status, o.body["refused"].as_str()), (409, Some("replay")));
    // An older serial is a replay too, even after the shield is removed.
    assert_eq!(r.m.remove("c2", "done").status, 200);
    let o = r.post(&conn_doc("c2", 4, "203.0.113.10", None, 600));
    assert_eq!(o.body["refused"], "replay");
    let o = r.post(&conn_doc("c2", 5, "203.0.113.10", None, 600));
    assert_eq!(o.body["refused"], "replay");
    assert_eq!(r.post(&conn_doc("c2", 6, "203.0.113.10", None, 600)).status, 201);
    // Over the policy's 7200 s.
    let o = r.post(&conn_doc("long", 1, "198.51.100.0/24", None, 7201));
    assert_eq!((o.status, o.body["refused"].as_str()), (403, Some("lifetime")));
    // A future start counts from not_before, not from now.
    let mut later = conn_doc("later", 1, "198.51.100.0/24", None, 0);
    later.not_before = NOW + 100_000;
    later.expires_at = NOW + 100_000 + 7200;
    assert_eq!(r.post(&later).status, 201);
    // Already expired, every address, no reason, a file match on a connect shield.
    let mut past = conn_doc("past", 1, "192.0.2.1", None, 10);
    past.expires_at = NOW - 1;
    assert_eq!(r.post(&past).body["refused"], "expired");
    assert_eq!(r.post(&conn_doc("all", 1, "0.0.0.0/0", None, 10)).body["refused"], "bad_match");
    let mut noreason = conn_doc("nr", 1, "192.0.2.1", None, 10);
    noreason.reason = None;
    assert_eq!(r.post(&noreason).status, 400);
    let mut mixed = conn_doc("mixed", 1, "192.0.2.1", None, 10);
    mixed.target.ino = Some(5);
    assert_eq!(r.post(&mixed).body["refused"], "bad_match");
    let mut badid = conn_doc("no spaces", 1, "192.0.2.1", None, 10);
    badid.id = "no spaces".into();
    assert_eq!(r.post(&badid).status, 400);
    // An unknown field in the document.
    let extra = document_text(&conn_doc("x", 1, "192.0.2.1", None, 10)).replace("\"serial\"", "\"color\": 1, \"serial\"");
    assert_eq!(r.m.apply_signed(&extra, &sign(&r.key, &extra)).body["refused"], "bad_document");
}

#[test]
fn never_block_targets_are_refused() {
    let d = temp("never");
    let sshd = d.join("sshd");
    std::fs::write(&sshd, b"x").unwrap();
    let never = NeverBlock::build(
        &NeverBlockPolicy { paths: vec![sshd.display().to_string()], cidrs: vec!["10.9.0.0/16".into()] },
        &[],
        &["198.51.100.5".parse().unwrap()],
    );
    let r = rig_with("never", policy(), never, Some(d.clone()));
    let meta = std::fs::metadata(&sshd).unwrap();
    let mut doc = conn_doc("x", 1, "192.0.2.1", None, 60);
    doc.kind = ShieldKind::ExecDeny;
    doc.target = ShieldMatch { path: Some("/whatever".into()), dev: Some(meta.dev()), ino: Some(meta.ino()), ..Default::default() };
    let o = r.post(&doc);
    assert_eq!((o.status, o.body["refused"].as_str()), (403, Some("never_block")), "{:?}", o.body);
    // pid 1 and the agent itself are on the built in list.
    let me = std::fs::metadata("/proc/self/exe").unwrap();
    doc.serial = 2;
    doc.target = ShieldMatch { dev: Some(me.dev()), ino: Some(me.ino()), ..Default::default() };
    let o = r.post(&doc);
    assert!(o.body["error"].as_str().unwrap().contains("the agent"), "{:?}", o.body);
    // Loopback, the backend, a network that covers the backend, and the policy's networks.
    for (i, cidr) in ["127.0.0.1", "198.51.100.5", "198.51.100.0/24", "10.9.4.0/24", "::1", "0.0.0.0/1"].iter().enumerate() {
        let o = r.post(&conn_doc(&format!("n{i}"), 1, cidr, None, 60));
        assert_eq!(o.status, 403, "{cidr}: {:?}", o.body);
    }
    assert_eq!(r.post(&conn_doc("ok", 1, "203.0.113.0/24", None, 60)).status, 201);
}

#[test]
fn shield_numbers_are_reused_and_their_counters_start_at_zero() {
    let r = rig("numbers");
    assert_eq!(r.post(&conn_doc("a", 1, "192.0.2.1", None, 60)).status, 201);
    assert_eq!(r.post(&conn_doc("b", 1, "192.0.2.2", None, 60)).status, 201);
    assert_eq!((r.m.by_num(1).unwrap().doc.id.as_str(), r.m.by_num(2).unwrap().doc.id.as_str()), ("a", "b"));
    r.maps.hits.lock().unwrap().insert(1, 99);
    assert_eq!(r.m.remove("a", "").body["hits"], 99);
    // The next shield takes number 1 again, with its counter zeroed.
    assert_eq!(r.post(&conn_doc("c", 1, "192.0.2.3", None, 60)).status, 201);
    assert_eq!(r.m.by_num(1).unwrap().doc.id, "c");
    assert_eq!(r.maps.hits(1), 0);
}

#[test]
fn the_maximum_conflicts_and_replacement() {
    let r = rig("max");
    for (i, c) in ["192.0.2.1", "192.0.2.2", "192.0.2.3"].iter().enumerate() {
        assert_eq!(r.post(&conn_doc(&format!("s{i}"), 1, c, None, 60)).status, 201);
    }
    let o = r.post(&conn_doc("s3", 1, "192.0.2.4", None, 60));
    assert_eq!((o.status, o.body["refused"].as_str()), (429, Some("max_active")));
    // The same target under another id is a conflict, the same id with a higher
    // serial replaces it, and the count does not grow.
    assert_eq!(r.m.remove("s2", "").status, 200);
    let o = r.post(&conn_doc("other", 1, "192.0.2.1", None, 60));
    assert_eq!(o.body["refused"], "conflict");
    let o = r.post(&conn_doc("s0", 2, "192.0.2.9", Some(53), 60));
    assert_eq!(o.status, 201);
    assert_eq!(o.body["replaced"], true);
    assert!(r.rule(ShieldKind::ConnectDeny, &key_for("192.0.2.1", 0)).is_none(), "the old target's row is gone");
    assert!(r.rule(ShieldKind::ConnectDeny, &key_for("192.0.2.9", 53)).is_some());
    assert_eq!(r.m.active_count(), 2);
    assert_eq!(r.slot(abi::SCFG_CONN_ROWS), 2);
}

#[test]
fn promote_needs_a_document_signed_for_enforce() {
    let r = rig("promote");
    let mut mon = conn_doc("mon", 1, "192.0.2.7", None, 600);
    mon.mode = ShieldMode::Monitor;
    assert_eq!(r.post(&mon).status, 201);
    let o = r.m.promote("mon", "looks fine");
    assert_eq!((o.status, o.body["refused"].as_str()), (409, Some("monitor_only")));
    assert_eq!(r.post(&conn_doc("enf", 1, "192.0.2.8", None, 600)).status, 201);
    // Hits counted in monitor mode survive the promotion.
    let k = key_for("192.0.2.8", 0);
    let num = r.rule(ShieldKind::ConnectDeny, &k).unwrap().id;
    r.maps.hits.lock().unwrap().insert(num, 7);
    let o = r.m.promote("enf", "trial clean");
    assert_eq!(o.status, 200, "{:?}", o.body);
    assert_eq!(o.body["shield"]["mode"], "enforce");
    assert_eq!(o.body["shield"]["hits"], 7);
    let row = r.rule(ShieldKind::ConnectDeny, &k).unwrap();
    assert_eq!((row.mode, row.id), (abi::MODE_ENFORCE, num));
    assert_eq!(r.m.promote("enf", "").body["unchanged"], true);
    assert_eq!(r.m.promote("nope", "").status, 404);
    assert!(r.audit_actions().contains(&"promote".to_string()));
}

#[test]
fn sweep_expires_rows_and_auto_promotes_after_the_trial() {
    let p = ShieldPolicy {
        auto_promote: AutoPromote { kinds: vec!["connect_deny".into()], trial_sec: 60 },
        ..policy()
    };
    let r = rig_with("sweep", p, NeverBlock::default(), None);
    assert_eq!(r.post(&conn_doc("short", 1, "192.0.2.10", None, 30)).status, 201);
    assert_eq!(r.post(&conn_doc("trial", 1, "192.0.2.11", None, 600)).status, 201);
    let mut file = conn_doc("f", 1, "", None, 600);
    file.kind = ShieldKind::FileDeny;
    file.target = ShieldMatch { path: Some("/tmp/x".into()), dev: Some(0x801), ino: Some(42), ..Default::default() };
    assert_eq!(r.post(&file).status, 201);
    r.clock.advance(31);
    r.m.sweep();
    assert!(r.rule(ShieldKind::ConnectDeny, &key_for("192.0.2.10", 0)).is_none());
    assert_eq!(r.m.active_count(), 2);
    let trial = key_for("192.0.2.11", 0);
    assert_eq!(r.rule(ShieldKind::ConnectDeny, &trial).unwrap().mode, abi::MODE_MONITOR);
    r.clock.advance(30);
    r.m.sweep();
    assert_eq!(r.rule(ShieldKind::ConnectDeny, &trial).unwrap().mode, abi::MODE_ENFORCE);
    // file_deny is not on the auto_promote list.
    let fk = abi::file_key(&ExeId::from_stat(0x801, 42)).to_vec();
    assert_eq!(r.rule(ShieldKind::FileDeny, &fk).unwrap().mode, abi::MODE_MONITOR);
    let actions = r.audit_actions();
    assert!(actions.contains(&"expire".into()) && actions.contains(&"auto_promote".into()), "{actions:?}");
}

#[test]
fn auto_apply_needs_the_policy_and_caps_the_lifetime() {
    let r = rig("auto");
    let o = r.m.apply_auto(conn_doc("pb.x", 1, "203.0.113.10", Some(4444), 3600), "pb_webshell");
    assert_eq!((o.status, o.body["refused"].as_str()), (403, Some("auto_apply_not_allowed")));
    let p = ShieldPolicy { auto_apply: AutoApply { kinds: vec!["connect_deny".into()], max_lifetime_sec: 600 }, ..policy() };
    let r = rig_with("auto2", p, NeverBlock::default(), None);
    let o = r.m.apply_auto(conn_doc("pb.x", 1, "203.0.113.10", Some(4444), 3600), "pb_webshell");
    assert_eq!(o.status, 201, "{:?}", o.body);
    assert_eq!(o.body["shield"]["expires_at"], NOW + 600);
    assert_eq!(o.body["shield"]["source"], "auto:pb_webshell");
    assert_eq!(o.body["shield"]["mode"], "monitor", "auto applied shields start in monitor too");
}

#[test]
fn restore_keeps_kernel_rows_rewrites_missing_ones_and_drops_strays() {
    let d = temp("restore");
    let r = rig_with("restore", policy(), NeverBlock::default(), Some(d.clone()));
    assert_eq!(r.post(&conn_doc("a", 1, "192.0.2.20", None, 600)).status, 201);
    assert_eq!(r.post(&conn_doc("b", 1, "192.0.2.21", None, 600)).status, 201);
    // A restart with the same pinned maps: a keeps its hits, b's row was lost,
    // and a stray row nobody owns is in the map.
    let ka = key_for("192.0.2.20", 0);
    let num_a = r.rule(ShieldKind::ConnectDeny, &ka).unwrap().id;
    r.maps.hits.lock().unwrap().insert(num_a, 3);
    r.maps.delete(ShieldKind::ConnectDeny, &key_for("192.0.2.21", 0)).unwrap();
    r.maps.put(ShieldKind::ConnectDeny, &key_for("192.0.2.99", 0), &ShieldRule::default()).unwrap();
    let m2 = ShieldManager::new(policy(), NeverBlock::default(), Box::new(r.maps.clone()), Box::new(r.clock.clone()), Vec::new(), &d.join("state"), &d.join("a.jsonl"), "kernel");
    let notes = m2.restore();
    assert_eq!(m2.active_count(), 2, "{notes:?}");
    assert_eq!(r.maps.hits(num_a), 3, "the pinned counters keep their hits");
    assert!(r.rule(ShieldKind::ConnectDeny, &key_for("192.0.2.21", 0)).is_some());
    assert!(r.rule(ShieldKind::ConnectDeny, &key_for("192.0.2.99", 0)).is_none());
    // The serials survive too: a replay after the restart is refused.
    let text = document_text(&conn_doc("a", 1, "192.0.2.20", None, 600));
    let pem = r.key.public_key_to_pem().unwrap();
    m2.set_pubkey(pem);
    assert_eq!(m2.apply_signed(&text, &sign(&r.key, &text)).body["refused"], "replay");
}

#[test]
fn a_new_policy_removes_never_block_shields_and_caps_lifetimes() {
    let r = rig("repolicy");
    assert_eq!(r.post(&conn_doc("keep", 1, "192.0.2.30", None, 7000)).status, 201);
    assert_eq!(r.post(&conn_doc("gone", 1, "10.20.0.0/16", None, 600)).status, 201);
    let p = ShieldPolicy { max_lifetime_sec: 1000, ..policy() };
    let never = NeverBlock::build(&NeverBlockPolicy { paths: vec![], cidrs: vec!["10.20.5.0/24".into()] }, &[], &[]);
    let notes = r.m.apply_policy(p, never);
    assert_eq!(r.m.active_count(), 1, "{notes:?}");
    let row = r.rule(ShieldKind::ConnectDeny, &key_for("192.0.2.30", 0)).unwrap();
    assert_eq!(row.expires_ns, (NOW + 1000 - 1_000_000) * 1_000_000_000);
    let list = r.m.list();
    assert_eq!(list["shields"][0]["expires_at"], NOW + 1000);
    let actions = r.audit_actions();
    assert!(actions.contains(&"removed_by_policy".into()) && actions.contains(&"capped_by_policy".into()), "{actions:?}");
}

#[test]
fn a_detect_only_agent_refuses_new_shields() {
    let d = temp("none");
    let m = ShieldManager::new(policy(), NeverBlock::default(), Box::new(MemMaps::default()), Box::new(SystemClock), Vec::new(), &d, &d.join("a.jsonl"), "none");
    let o = m.apply_auto(conn_doc("x", 1, "192.0.2.1", None, 60), "pb");
    assert_eq!(o.status, 403, "auto apply is checked first");
    let text = document_text(&conn_doc("x", 1, "192.0.2.1", None, 60));
    assert_eq!(m.apply_signed(&text, "00").status, 403);
    assert_eq!(m.list()["enforcement"], "none");
}

#[test]
fn kernel_reports_become_alerts_with_shield_target_and_mode() {
    let r = rig("event");
    let mut doc = conn_doc("c2", 1, "203.0.113.10", Some(8000), 600);
    doc.incident_id = Some("inc-7".into());
    assert_eq!(r.post(&doc).status, 201);
    let a = r.m.by_num(1).unwrap();
    let e = ShieldEvent {
        timestamp_ns: 1,
        exe_ino: 9,
        target_ino: 0,
        cgroup_id: 5,
        exe_dev: 1,
        tgid: 4242,
        tid: 4242,
        uid: 33,
        shield_id: 1,
        target_dev: 0,
        port: 8000,
        kind: abi::KIND_CONNECT,
        action: abi::ACTION_BLOCKED,
        family: 4,
        addr: [203, 0, 113, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        comm: "bash".into(),
        exe_name: "bash".into(),
        target: String::new(),
    };
    let (f, alert) = shield_finding(&e, Some(&a), Some("/usr/bin/bash")).unwrap();
    assert_eq!(f.output.signal, "shield::connect_deny::blocked");
    assert_eq!(alert["source"], "shield");
    assert_eq!(alert["severity"], "high");
    assert_eq!(alert["details"]["target"], "203.0.113.10:8000");
    assert_eq!(alert["details"]["shield"], "c2");
    assert_eq!(alert["details"]["incident_id"], "inc-7");
    assert_eq!(alert["details"]["mode"], "enforce");
    let mut e2 = e.clone();
    e2.action = abi::ACTION_WOULD_BLOCK;
    let (_, alert) = shield_finding(&e2, Some(&a), None).unwrap();
    assert_eq!((alert["details"]["mode"].as_str(), alert["severity"].as_str()), (Some("monitor"), Some("medium")));
    e2.kind = 9;
    assert!(shield_finding(&e2, None, None).is_none());
}

fn hit(slots: Vec<(&str, Vec<(&str, &str)>)>, templates: Vec<ShieldTemplate>) -> PlaybookHit {
    PlaybookHit {
        playbook_id: "pb_webshell_reverse_shell_linux".into(),
        playbook_name: "Webshell".into(),
        severity: "high".into(),
        host: "h".into(),
        ts: NOW,
        tags: vec![],
        rationale: vec![],
        slots: slots
            .into_iter()
            .map(|(id, f)| {
                (
                    id.to_string(),
                    SlotHitSummary { ts: NOW, tags: vec![], fields: f.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect::<HashMap<_, _>>() },
                )
            })
            .collect(),
        filled_slots: vec![],
        missing_optional: vec![],
        primary_ttp: Some("T1505.003".into()),
        techniques: vec![],
        narrative: None,
        context: vec![],
        left_out: vec![],
        ground_truth: false,
        shield_templates: templates,
    }
}

fn template(kind: &str, slot: &str, scope: Option<&str>) -> ShieldTemplate {
    ShieldTemplate { kind: kind.into(), slot: slot.into(), scope_slot: scope.map(str::to_string), ttl_sec: 3600, with_port: true, path_regex: None }
}

#[test]
fn a_reverse_shell_proposes_connect_deny_for_the_shells_cgroup() {
    let d = temp("propose");
    let proc_root = d.join("proc");
    let cg_root = d.join("cgroup");
    std::fs::create_dir_all(proc_root.join("4242")).unwrap();
    std::fs::write(proc_root.join("4242/cgroup"), "0::/system.slice/nginx.service\n").unwrap();
    std::fs::create_dir_all(cg_root.join("system.slice/nginx.service")).unwrap();
    std::fs::write(cg_root.join("system.slice/nginx.service/cgroup.procs"), "").unwrap();
    let h = hit(
        vec![
            ("web_to_shell_exec", vec![("pid", "4242"), ("exe", "/usr/bin/bash")]),
            ("outbound_rev_c2", vec![("dst_ip", "203.0.113.10"), ("dst_port", "4444"), ("pid", "4242")]),
        ],
        vec![template("connect_deny", "outbound_rev_c2", Some("web_to_shell_exec"))],
    );
    let props = propose::from_hit(&h, "inc-1", NOW, &proc_root, &cg_root);
    assert_eq!(props.len(), 1);
    let p = &props[0];
    assert_eq!(p.doc.kind, ShieldKind::ConnectDeny);
    assert_eq!(p.doc.target.cidr.as_deref(), Some("203.0.113.10"));
    assert_eq!(p.doc.target.port, Some(4444));
    assert_eq!(p.doc.scope.cgroup.as_deref(), Some(cg_root.join("system.slice/nginx.service").to_str().unwrap()));
    assert_eq!((p.doc.serial, p.doc.expires_at, p.doc.mode), (NOW, NOW + 3600, ShieldMode::Enforce));
    assert_eq!(p.doc.incident_id.as_deref(), Some("inc-1"));
    assert!(valid_id(&p.doc.id), "{}", p.doc.id);
    // The document text is exactly what an operator signs, and it parses back.
    assert_eq!(parse_document(&p.document).unwrap(), p.doc);
    // The same attack later proposes the same id with a higher serial.
    let again = propose::from_hit(&h, "inc-1", NOW + 5, &proc_root, &cg_root);
    assert_eq!(again[0].doc.id, p.doc.id);
    assert!(again[0].doc.serial > p.doc.serial);
    // With the shell gone the scope widens to the host, and the proposal says so.
    std::fs::remove_file(proc_root.join("4242/cgroup")).unwrap();
    let gone = propose::from_hit(&h, "inc-1", NOW, &proc_root, &cg_root);
    assert!(gone[0].doc.scope.cgroup.is_none());
    assert!(gone[0].notes[0].contains("whole host"));
}

#[test]
fn a_dropped_payload_proposes_exec_deny_by_inode_and_the_regex_filters() {
    let d = temp("payload");
    let payload = d.join("payload");
    std::fs::write(&payload, b"#!/bin/sh\n").unwrap();
    let path = payload.display().to_string();
    let h = hit(
        vec![("child_exec", vec![("pid", "1"), ("exe", path.as_str())])],
        vec![ShieldTemplate { path_regex: Some(format!("^{}/", d.display())), ..template("exec_deny", "child_exec", None) }],
    );
    let props = propose::from_hit(&h, "inc-2", NOW, Path::new("/proc"), Path::new(propose::CGROUP_ROOT));
    assert_eq!(props.len(), 1);
    let meta = std::fs::metadata(&payload).unwrap();
    assert_eq!(props[0].doc.target.ino, Some(meta.ino()));
    assert_eq!(props[0].doc.target.dev, Some(meta.dev()));
    assert_eq!(props[0].doc.target.path.as_deref(), Some(path.as_str()));
    // A packaged binary outside the regex gets no proposal.
    let h2 = hit(
        vec![("child_exec", vec![("exe", "/usr/bin/curl")])],
        vec![ShieldTemplate { path_regex: Some("^/(tmp|dev/shm|var/tmp)/".into()), ..template("exec_deny", "child_exec", None) }],
    );
    assert!(propose::from_hit(&h2, "i", NOW, Path::new("/proc"), Path::new(propose::CGROUP_ROOT)).is_empty());
}

/// The document format both sides use: scripts/shieldctl.py writes these keys and
/// tests/test_shield_tools.py checks it against the same file.
#[test]
fn the_shared_fixture_document_parses_and_round_trips() {
    let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/shield_doc.json")).unwrap();
    let doc = parse_document(&text).unwrap();
    assert_eq!(doc.kind, ShieldKind::ConnectDeny);
    assert_eq!((doc.target.cidr.as_deref(), doc.target.port), (Some("203.0.113.10"), Some(4444)));
    assert_eq!(doc.mode, ShieldMode::Enforce);
    assert_eq!(doc.expires_at - doc.not_before, 3600);
    assert!(valid_id(&doc.id));
    assert_eq!(parse_document(&document_text(&doc)).unwrap(), doc);
}

#[test]
fn the_journal_survives_a_crash_and_folds_into_the_snapshot() {
    let d = temp("journal");
    let r = rig_with("journal", policy(), NeverBlock::default(), Some(d.clone()));
    assert_eq!(r.post(&conn_doc("j1", 1, "192.0.2.40", None, 600)).status, 201);
    assert_eq!(r.post(&conn_doc("j2", 1, "192.0.2.41", None, 600)).status, 201);
    assert_eq!(r.m.promote("j1", "").status, 200);
    assert_eq!(r.m.remove("j2", "").status, 200);
    // No snapshot was written since, only journal lines: put, put, put, del.
    let journal = std::fs::read_to_string(r.m.journal_path()).unwrap();
    assert_eq!(journal.lines().count(), 4, "{journal}");
    // A torn last line, as a crash in the middle of a write leaves it.
    std::fs::write(r.m.journal_path(), format!("{journal}{{\"op\":\"put\",\"act")).unwrap();
    let m2 = ShieldManager::new(policy(), NeverBlock::default(), Box::new(r.maps.clone()), Box::new(r.clock.clone()), r.key.public_key_to_pem().unwrap(), &d.join("state"), &d.join("a.jsonl"), "kernel");
    let notes = m2.restore();
    assert_eq!(m2.active_count(), 1, "{notes:?}");
    assert_eq!(m2.list()["shields"][0]["mode"], "enforce", "the promotion was journaled");
    assert!(notes.iter().any(|n| n.contains("torn")), "{notes:?}");
    // j2 was removed but its serial is still known: replaying its document fails.
    let text = document_text(&conn_doc("j2", 1, "192.0.2.41", None, 600));
    assert_eq!(m2.apply_signed(&text, &sign(&r.key, &text)).body["refused"], "replay");
    // The restore folded the journal into the snapshot.
    assert!(std::fs::metadata(m2.shields_path()).unwrap().len() > 0);
}
