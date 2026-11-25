// src/pb_engine.rs
// Playbook engine. Loads YAML playbooks, matches facts against their slots, and fires a
// playbook when every required slot has a live hit inside the playbook window and no
// forbidden slot has one.
//
// Fact kinds: Exec, FileIO, NetFlow (net_flow sensor), Net (tcp_state sensor), Auth (sshd
// lines in the auth log), RuleHit (rules.json alerts and guard decisions), HostPerf (CPU
// samples from /proc) and VendorAlert. pb_wiring.rs says which fields each one carries.
//
// Slot conditions use eq, ne (or neq), gt, gte, lt, lte, contains, not_contains,
// contains_any, regex, not_regex, in, not_in, any_in, in_cidr, not_in_cidr, exists and
// in_slot, and `any_of: [...]` holds when one of its conditions does. A condition on a
// field the fact does not carry is false, except `exists: false`. in_slot compares a field
// with the same field (or `slot.field`) of another slot's live hit, so two steps can be
// tied to one process.
//
// A slot whose conditions test `__agg.*` fields is a counting slot: its other conditions
// pick which events count, and the counts over the aggregate window are attached to the
// event that just arrived before the slot is checked (see "Aggregates" below).
//
// Time is Unix seconds everywhere (pb_adapter::clock converts kernel stamps). Facts carry
// their event time. tick(now) runs from the batch loop every 5 s and drops slot hits and
// counted events that aged out, even when no new event arrives.

use anyhow::{anyhow, Result};
use regex::Regex;
use serde::Deserialize;
use serde_yaml::Value as Yaml;
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Mirror a subset of the DecisionEngine to create Signals without circular deps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetectorKind {
    Playbook,
}

#[derive(Debug, Clone)]
pub struct Signal {
    pub detector: DetectorKind,
    pub score: f32,
    pub ts: u64,
    pub exe: Option<String>,
    pub user: Option<String>,
    pub host: Option<String>,
    pub cmd_prefix: Option<String>,
    pub parent: Option<String>,
    pub signer: Option<String>,
    pub hash: Option<String>,
    pub pid: Option<i32>,
    pub tags: Vec<String>,
}

// ---- Fact model (normalized telemetry snippets) ----

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FactKind {
    Exec,
    FileIO,
    /// net_flow: connect() destinations and bytes passed to send, sendto and sendmsg.
    NetFlow,
    /// tcp_state: TCP connections opening, closing and staying open.
    Net,
    /// sshd logins and failures read from the auth log.
    Auth,
    /// A rules.json rule that matched, or a guard decision.
    RuleHit,
    /// CPU samples for the host and for busy processes.
    HostPerf,
    VendorAlert,
}

#[derive(Debug, Clone)]
pub struct Fact {
    pub ts: u64,
    pub kind: FactKind,
    pub host: String,
    pub user: Option<String>,
    pub exe: Option<String>,
    pub cmd: Option<String>,
    pub fields: HashMap<String, String>,
}

impl Fact {
    pub fn get_str(&self, k: &str) -> Option<&str> {
        if k == "exe" {
            return self.exe.as_deref();
        }
        if k == "cmd" {
            return self.cmd.as_deref();
        }
        if k == "user" {
            return self.user.as_deref();
        }
        if k == "host" {
            return Some(&self.host);
        }
        self.fields.get(k).map(|s| s.as_str())
    }
    pub fn get_f64(&self, k: &str) -> Option<f64> {
        self.get_str(k).and_then(|v| v.parse::<f64>().ok())
    }
}

// ---- YAML types ----

/// One condition. Either field, op and value, or `any_of` with a list of conditions.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PredicateSpec {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub op: String,
    #[serde(default)]
    pub value: Yaml,
    #[serde(default)]
    pub any_of: Vec<PredicateSpec>,
    /// Compiled at load for regex and not_regex.
    #[serde(skip)]
    re: Option<Regex>,
}

impl PredicateSpec {
    fn is_any_of(&self) -> bool {
        !self.any_of.is_empty()
    }

    /// True when this condition or one inside its any_of needs another slot's hit.
    fn needs_slot(&self) -> bool {
        self.op == "in_slot" || self.any_of.iter().any(PredicateSpec::needs_slot)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SlotSpec {
    pub slot_id: String,
    pub slot_ttl_sec: u64,
    pub fact: String,
    // YAML key is "where" → map to field `where_`
    #[serde(default, rename = "where")]
    pub where_: Vec<PredicateSpec>,
    #[serde(default)]
    pub derive_tags: Vec<String>, // applied when this slot matches
    /// A note for readers of the playbook. Not used for matching.
    #[serde(default)]
    pub notes: Option<String>,
    /// Parsed from `fact` at load.
    #[serde(skip)]
    kind: Option<FactKind>,
    /// True when a condition uses in_slot, so the slot is checked again against the
    /// playbook's live hits.
    #[serde(skip)]
    needs_slot: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VendorMapEntry {
    pub product: String,
    pub alert_type: String,
    pub fills_slot: String,
}

/// An order or timing rule between two slots of one playbook.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinkSpec {
    pub from: String,
    pub to: String,
    /// same_host, same_user, same_pid or same_exe. A side with no value passes.
    #[serde(default)]
    pub requires: Vec<String>,
    /// `to` was hit at or after `from`, at most this many seconds later.
    #[serde(default)]
    pub follows_within_sec: Option<u64>,
    /// Either order, at most this many seconds apart.
    #[serde(default)]
    pub within_sec: Option<u64>,
}

// Keys a link's `requires` list may use.
const LINK_REQUIRES: &[&str] = &["same_host", "same_user", "same_pid", "same_exe"];

impl LinkSpec {
    fn describe(&self) -> String {
        match (self.follows_within_sec, self.within_sec) {
            (Some(n), _) => format!("{} then {} within {n} s", self.from, self.to),
            (_, Some(n)) => format!("{} and {} within {n} s", self.from, self.to),
            _ => format!("{} and {}", self.from, self.to),
        }
    }

    /// True when the two hits keep this link.
    fn holds(&self, from: &SlotHit, to: &SlotHit) -> bool {
        let timed = match (self.follows_within_sec, self.within_sec) {
            (Some(n), _) => to.ts >= from.ts && to.ts - from.ts <= n,
            (_, Some(n)) => from.ts.abs_diff(to.ts) <= n,
            _ => true,
        };
        timed
            && self.requires.iter().all(|r| {
                let field = match r.as_str() {
                    "same_host" => "host",
                    "same_user" => "user",
                    "same_pid" => "pid",
                    "same_exe" => "exe",
                    _ => return true,
                };
                match (from.fact.get_str(field), to.fact.get_str(field)) {
                    (Some(a), Some(b)) => a == b,
                    _ => true,
                }
            })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExplainSpec {
    /// One technique id or a list. The first is the primary technique.
    #[serde(default, rename = "primary_ttp", deserialize_with = "one_or_many")]
    pub ttps: Vec<String>,
    pub narrative: Option<String>,
    /// Evidence lines for the hit. `{{slot.field}}` is the field of that slot's hit, and
    /// `{{#if slot}}...{{/if}}` keeps the text only when the slot has a hit.
    #[serde(default)]
    pub steps_template: Vec<String>,
}

impl ExplainSpec {
    pub fn primary_ttp(&self) -> Option<String> {
        self.ttps.first().cloned()
    }
}

fn one_or_many<'de, D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Vec<String>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(String),
        Many(Vec<String>),
    }
    Ok(match Option::<OneOrMany>::deserialize(d)? {
        None => Vec::new(),
        Some(OneOrMany::One(s)) => vec![s],
        Some(OneOrMany::Many(v)) => v,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlaybookSpec {
    pub id: String,
    pub version: Option<String>,
    pub name: String,
    pub severity: String,
    pub description: Option<String>,
    pub window_sec: u64,
    pub cooldown_sec: u64,
    // Defaults to empty so the loader can say why a file has none.
    // validate_playbook rejects an empty list.
    #[serde(default)]
    pub required: Vec<SlotSpec>,
    #[serde(default)]
    pub optional: Vec<SlotSpec>,
    #[serde(default)]
    pub forbidden: Vec<SlotSpec>,
    #[serde(default)]
    pub vendor_map: Vec<VendorMapEntry>,
    #[serde(default)]
    pub explain: Option<ExplainSpec>,
    /// Order and timing rules between slots. See LinkSpec.
    #[serde(default)]
    pub links: Vec<LinkSpec>,
    /// Fact values that split evidence into separate chains, such as [host, user].
    /// host is always one of them. A fact with no value for a key joins the chain
    /// with "-" for it.
    #[serde(default)]
    pub context_keys: Vec<String>,
    /// Write each hit of this playbook to the ground truth file as a labelled attack.
    #[serde(default)]
    pub emit_ground_truth: bool,
    /// Runtime shields a hit proposes. See ShieldTemplate.
    #[serde(default)]
    pub shield_templates: Vec<ShieldTemplate>,
}

/// A runtime shield a playbook hit proposes (src/shield/propose.rs). The target
/// comes from the fact that filled `slot`: dst_ip and dst_port for connect_deny,
/// exe for exec_deny and file_deny. The shield is limited to the cgroup of the
/// process that filled `scope_slot`, or the whole host without one.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShieldTemplate {
    pub kind: String,
    pub slot: String,
    #[serde(default)]
    pub scope_slot: Option<String>,
    pub ttl_sec: u64,
    /// connect_deny only: block the one port (default) or every port of the address.
    #[serde(default = "default_true")]
    pub with_port: bool,
    /// exec_deny and file_deny only: propose a shield only for a path that matches.
    #[serde(default)]
    pub path_regex: Option<String>,
}

fn default_true() -> bool {
    true
}

pub const SHIELD_KINDS: &[&str] = &["exec_deny", "file_deny", "connect_deny"];

impl PlaybookSpec {
    fn is_required(&self, slot_id: &str) -> bool {
        self.required.iter().any(|s| s.slot_id == slot_id)
    }

    /// The chain a fact belongs to: the host, then each other context key's value.
    fn context_of(&self, f: &Fact) -> String {
        let mut key = f.host.clone();
        for k in self.context_keys.iter().filter(|k| k.as_str() != "host") {
            key.push('|');
            key.push_str(f.get_str(k).filter(|v| !v.is_empty()).unwrap_or("-"));
        }
        key
    }

    /// The context key and value pairs of a chain key made by context_of.
    fn context_pairs(&self, key: &str) -> Vec<(String, String)> {
        let mut parts = key.split('|');
        let mut out = vec![("host".to_string(), parts.next().unwrap_or("").to_string())];
        for k in self.context_keys.iter().filter(|k| k.as_str() != "host") {
            out.push((k.clone(), parts.next().unwrap_or("-").to_string()));
        }
        out
    }
}


// ---- Runtime state ----

#[derive(Debug, Clone)]
struct SlotHit {
    ts: u64,
    tags: Vec<String>,
    fact: Fact,
}

/// Evidence for one playbook on one host: the newest hit per slot. Forbidden slots are
/// kept here too, so a live forbidden hit can veto the playbook.
#[derive(Debug, Default)]
struct Hypothesis {
    hits: HashMap<String, SlotHit>, // slot_id -> newest hit
}

impl Hypothesis {
    /// Drop hits older than their slot TTL.
    fn expire(&mut self, now: u64, ttls: Option<&HashMap<String, u64>>) {
        self.hits.retain(|slot_id, hit| {
            let ttl = ttls.and_then(|t| t.get(slot_id)).copied().unwrap_or(600);
            now.saturating_sub(hit.ts) <= ttl
        });
    }

    /// Every required slot has a live hit and the oldest one is inside the window.
    /// The window is measured over the evidence, not from when the hypothesis started,
    /// so an early hit that aged out does not block a later chain.
    fn complete(&self, pb: &PlaybookSpec, now: u64) -> bool {
        if pb.required.is_empty() {
            return false;
        }
        let mut oldest = now;
        for slot in &pb.required {
            match self.hits.get(&slot.slot_id) {
                Some(hit) => oldest = oldest.min(hit.ts),
                None => return false,
            }
        }
        now.saturating_sub(oldest) <= pb.window_sec
    }

    /// Check the playbook's links over the live hits. A link counts only when both of
    /// its slots have a hit. None when a link between two required slots is broken,
    /// so the chain is not complete yet. Otherwise the optional slots whose hit
    /// breaks a link: those hits are left out of the evidence.
    fn broken_optional(&self, pb: &PlaybookSpec) -> Option<Vec<(String, String)>> {
        let mut out: Vec<(String, String)> = Vec::new();
        for link in &pb.links {
            let (Some(from), Some(to)) = (self.hits.get(&link.from), self.hits.get(&link.to)) else {
                continue;
            };
            if link.holds(from, to) {
                continue;
            }
            let optional: Vec<&String> = [&link.from, &link.to]
                .into_iter()
                .filter(|s| !pb.is_required(s))
                .collect();
            if optional.is_empty() {
                return None;
            }
            for s in optional {
                if !out.iter().any(|(id, _)| id == s) {
                    out.push((s.clone(), link.describe()));
                }
            }
        }
        Some(out)
    }

    fn vetoed(&self, pb: &PlaybookSpec) -> bool {
        pb.forbidden
            .iter()
            .any(|slot| self.hits.contains_key(&slot.slot_id))
    }
}

// ---- Conditions ----

/// Live hits of the playbook a fact is being matched for. in_slot reads them.
type LiveHits = HashMap<String, SlotHit>;

fn yaml_text(v: &Yaml) -> Option<String> {
    match v {
        Yaml::String(s) => Some(s.clone()),
        Yaml::Number(n) => Some(n.to_string()),
        Yaml::Bool(b) => Some(if *b { "true".into() } else { "false".into() }),
        _ => None,
    }
}

fn yaml_list(v: &Yaml) -> Option<Vec<String>> {
    match v {
        Yaml::Sequence(seq) => Some(seq.iter().filter_map(yaml_text).collect()),
        _ => None,
    }
}

/// An IPv4 or IPv6 network in CIDR form, or a bare address as a /32 or /128.
fn cidr_contains(cidr: &str, ip: &std::net::IpAddr) -> bool {
    use std::net::IpAddr;
    let (net, bits) = match cidr.split_once('/') {
        Some((n, b)) => (n, b.parse::<u32>().ok()),
        None => (cidr, None),
    };
    let Ok(net) = net.trim().parse::<IpAddr>() else {
        return false;
    };
    match (net, ip) {
        (IpAddr::V4(n), IpAddr::V4(a)) => {
            let bits = bits.unwrap_or(32).min(32);
            let mask = if bits == 0 { 0 } else { u32::MAX << (32 - bits) };
            (u32::from(n) & mask) == (u32::from(*a) & mask)
        }
        (IpAddr::V6(n), IpAddr::V6(a)) => {
            let bits = bits.unwrap_or(128).min(128);
            let mask = if bits == 0 { 0 } else { u128::MAX << (128 - bits) };
            (u128::from(n) & mask) == (u128::from(*a) & mask)
        }
        _ => false,
    }
}

fn valid_cidr(cidr: &str) -> bool {
    let (net, bits) = match cidr.split_once('/') {
        Some((n, b)) => (n, Some(b)),
        None => (cidr, None),
    };
    match net.trim().parse::<std::net::IpAddr>() {
        Ok(std::net::IpAddr::V4(_)) => bits.map(|b| b.parse::<u32>().map(|n| n <= 32).unwrap_or(false)).unwrap_or(true),
        Ok(std::net::IpAddr::V6(_)) => bits.map(|b| b.parse::<u32>().map(|n| n <= 128).unwrap_or(false)).unwrap_or(true),
        Err(_) => false,
    }
}

struct Evaluator;

impl Evaluator {
    fn new() -> Self {
        Self
    }

    /// Kind and conditions. in_slot conditions pass here and are checked later with
    /// the playbook's live hits (match_with_hits).
    fn match_slot(&self, spec: &SlotSpec, f: &Fact) -> bool {
        spec.kind == Some(f.kind) && spec.where_.iter().all(|p| self.predicate(p, f, None))
    }

    /// Every condition, in_slot included, against the live hits.
    fn match_with_hits(&self, spec: &SlotSpec, f: &Fact, hits: &LiveHits) -> bool {
        spec.kind == Some(f.kind) && spec.where_.iter().all(|p| self.predicate(p, f, Some(hits)))
    }

    /// One condition. `hits` is None in the first pass, where in_slot holds.
    fn predicate(&self, p: &PredicateSpec, f: &Fact, hits: Option<&LiveHits>) -> bool {
        if p.is_any_of() {
            return p.any_of.iter().any(|q| self.predicate(q, f, hits));
        }
        let field = p.field.as_str();
        let op = p.op.as_str();

        // tags_any: true when any of the fact's tags (comma-joined in `tags`) is in the
        // list for op in, or equals the value for op eq.
        if field == "tags_any" {
            let tags = f.get_str("tags").unwrap_or("");
            let mut each = tags.split(',').filter(|t| !t.is_empty());
            return match op {
                "in" => yaml_list(&p.value)
                    .map(|list| each.any(|t| list.iter().any(|v| v == t)))
                    .unwrap_or(false),
                "eq" => yaml_text(&p.value).map(|v| each.any(|t| t == v)).unwrap_or(false),
                _ => false,
            };
        }

        if op == "exists" {
            let want = matches!(p.value, Yaml::Bool(true));
            let present = f.get_str(field).map(|v| !v.is_empty()).unwrap_or(false);
            return present == want;
        }
        if op == "in_slot" {
            let Some(hits) = hits else {
                return true;
            };
            let target = yaml_text(&p.value).unwrap_or_default();
            let (slot, other_field) = target.split_once('.').unwrap_or((target.as_str(), field));
            let Some(hit) = hits.get(slot) else {
                return false;
            };
            return match (f.get_str(field), hit.fact.get_str(other_field)) {
                (Some(a), Some(b)) => !a.is_empty() && a == b,
                _ => false,
            };
        }

        // `__agg.*` fields are ordinary fields here. Counting slots get them from
        // agg_fields before the slot is checked. A missing field fails every op below.
        let Some(lhs) = f.get_str(field) else {
            return false;
        };
        let num = || lhs.parse::<f64>().ok().zip(yaml_f64(&p.value));
        match op {
            "eq" => yaml_text(&p.value).map(|v| lhs == v).unwrap_or(false),
            "ne" | "neq" => yaml_text(&p.value).map(|v| lhs != v).unwrap_or(false),
            "contains" => yaml_text(&p.value).map(|v| lhs.contains(&v)).unwrap_or(false),
            "not_contains" => yaml_text(&p.value).map(|v| !lhs.contains(&v)).unwrap_or(false),
            "contains_any" => yaml_list(&p.value)
                .map(|list| list.iter().any(|v| lhs.contains(v.as_str())))
                .unwrap_or(false),
            "regex" => p.re.as_ref().map(|re| re.is_match(lhs)).unwrap_or(false),
            "not_regex" => p.re.as_ref().map(|re| !re.is_match(lhs)).unwrap_or(false),
            "in" => yaml_list(&p.value)
                .map(|list| list.iter().any(|v| v == lhs))
                .unwrap_or(false),
            "not_in" => yaml_list(&p.value)
                .map(|list| !list.iter().any(|v| v == lhs))
                .unwrap_or(false),
            // The field holds a comma separated list, one item of which is in the value.
            "any_in" => yaml_list(&p.value)
                .map(|list| lhs.split(',').any(|item| list.iter().any(|v| v == item.trim())))
                .unwrap_or(false),
            "in_cidr" | "not_in_cidr" => {
                let (Ok(ip), Some(list)) = (lhs.parse::<std::net::IpAddr>(), yaml_list(&p.value)) else {
                    return false;
                };
                let inside = list.iter().any(|c| cidr_contains(c, &ip));
                if op == "in_cidr" {
                    inside
                } else {
                    !inside
                }
            }
            "gt" => num().map(|(a, b)| a > b).unwrap_or(false),
            "gte" => num().map(|(a, b)| a >= b).unwrap_or(false),
            "lt" => num().map(|(a, b)| a < b).unwrap_or(false),
            "lte" => num().map(|(a, b)| a <= b).unwrap_or(false),
            _ => false,
        }
    }
}

// ---- Aggregates: counting slots ----
//
// A slot that tests `__agg.*` fields counts events. Example, fileburst_crypto:
//   op in [write, rename], topdir in [/home, ...], __agg.files gte 500, __agg.window_sec lte 600
// Every event that passes the slot's other conditions is kept for the aggregate window
// (the `__agg.window_sec lte N` bound, else the slot TTL), per playbook, slot and host.
// When an event arrives it is added, events older than the window are dropped, and the
// counts are copied onto a clone of the event as `__agg.*` fields. Then the whole slot,
// counts included, is checked on that clone. So `__agg.files gte 500` reads as "500
// distinct files passed this slot's other conditions in the last 600 s".
//
// File bursts keep every write, rename and delete in the place the slot names, even when
// the slot only asks for writes, so the rename ratio compares renames with all changes
// there. `files` and the distinct counts use only the ops the slot asks for, or write and
// rename when it names none. A plain open never counts as a change.
//
// Kinds and what they count:
//   file_burst, partial_encrypt_burst  FileIO changes (write, rename, delete)
//   file_read_burst                    FileIO opens, distinct files read
//   errno_spike                        FileIO events, usually failed opens
//   exec_burst                         Exec events and the top dirs of their path arguments
//   proc_burst_workers                 Exec children of one parent
//   egress_volume                      NetFlow sends: bytes out and destinations
//   conn_burst                         NetFlow connects: count, rate, destinations, ports
//   sustained                          HostPerf samples of one process (or the host) in an
//                                      unbroken run: how long the condition has held
//   auth_fail_burst                    Auth events: failures, users and sources

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AggKind {
    FileBurst,
    PartialEncryptBurst,
    FileReadBurst,
    ErrnoSpike,
    ExecBurst,
    ProcBurstWorkers,
    EgressVolume,
    ConnBurst,
    Sustained,
    AuthFailBurst,
}

/// HostPerf samples come every 5 s. A gap longer than this ends a sustained run.
const SUSTAINED_MAX_GAP_SEC: u64 = 15;

impl AggKind {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "file_burst" => Some(Self::FileBurst),
            "partial_encrypt_burst" => Some(Self::PartialEncryptBurst),
            "file_read_burst" => Some(Self::FileReadBurst),
            "errno_spike" => Some(Self::ErrnoSpike),
            "exec_burst" => Some(Self::ExecBurst),
            "proc_burst_workers" => Some(Self::ProcBurstWorkers),
            "egress_volume" => Some(Self::EgressVolume),
            "conn_burst" => Some(Self::ConnBurst),
            "sustained" => Some(Self::Sustained),
            "auth_fail_burst" => Some(Self::AuthFailBurst),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::FileBurst => "file_burst",
            Self::PartialEncryptBurst => "partial_encrypt_burst",
            Self::FileReadBurst => "file_read_burst",
            Self::ErrnoSpike => "errno_spike",
            Self::ExecBurst => "exec_burst",
            Self::ProcBurstWorkers => "proc_burst_workers",
            Self::EgressVolume => "egress_volume",
            Self::ConnBurst => "conn_burst",
            Self::Sustained => "sustained",
            Self::AuthFailBurst => "auth_fail_burst",
        }
    }

    fn fact_kind(self) -> FactKind {
        match self {
            Self::FileBurst | Self::PartialEncryptBurst | Self::FileReadBurst | Self::ErrnoSpike => {
                FactKind::FileIO
            }
            Self::ExecBurst | Self::ProcBurstWorkers => FactKind::Exec,
            Self::EgressVolume | Self::ConnBurst => FactKind::NetFlow,
            Self::Sustained => FactKind::HostPerf,
            Self::AuthFailBurst => FactKind::Auth,
        }
    }

    fn is_file_burst(self) -> bool {
        matches!(self, Self::FileBurst | Self::PartialEncryptBurst)
    }

    /// `__agg.<name>` fields this kind sets.
    fn fields(self) -> &'static [&'static str] {
        match self {
            // partial_encrypt_burst is the same window. The playbook adds the
            // short-write and rename-ratio conditions that make it partial encryption.
            Self::FileBurst | Self::PartialEncryptBurst => &[
                "kind",
                "files",
                "distinct_topdirs",
                "distinct_dirs",
                "distinct_mounts",
                "rename_ratio",
                "total_bytes",
                "avg_write_kib",
                "window_sec",
            ],
            Self::FileReadBurst => &["kind", "files", "distinct_topdirs", "distinct_dirs", "window_sec"],
            Self::ErrnoSpike => &["kind", "count", "window_sec"],
            Self::ExecBurst => &["kind", "count", "distinct_topdirs", "window_sec"],
            Self::ProcBurstWorkers => &["kind", "children", "window_sec"],
            Self::EgressVolume => &["kind", "bytes_out", "bytes_out_mb", "sends", "distinct_dsts", "window_sec"],
            Self::ConnBurst => &[
                "kind",
                "count",
                "distinct_dsts",
                "distinct_ports",
                "conn_rate_eps",
                "window_sec",
            ],
            Self::Sustained => &["kind", "duration_sec", "samples", "window_sec"],
            Self::AuthFailBurst => &["kind", "failures", "distinct_users", "distinct_srcs", "window_sec"],
        }
    }

    /// The fact field that splits the window into groups, one window per value.
    fn group_field(self) -> Option<&'static str> {
        match self {
            Self::ProcBurstWorkers => Some("ppid"),
            Self::Sustained => Some("pid"),
            _ => None,
        }
    }
}

/// How one counting slot builds its window, worked out once at load.
#[derive(Debug, Clone)]
struct AggPlan {
    kind: AggKind,
    window_sec: u64,
    /// Conditions an event must pass to be counted.
    entry: Vec<PredicateSpec>,
    /// File bursts: the slot's conditions on `op`, used for `files`.
    ops: Vec<PredicateSpec>,
}

/// The counting plan for a slot, None for a plain slot, or why the slot can never match.
fn agg_plan(slot: &SlotSpec) -> std::result::Result<Option<AggPlan>, String> {
    let agg: Vec<&PredicateSpec> = slot
        .where_
        .iter()
        .filter(|p| p.field.starts_with("__agg."))
        .collect();
    if agg.is_empty() {
        return Ok(None);
    }
    let fact = parse_fact_kind(&slot.fact);

    let mut kind = None;
    for p in &agg {
        if p.field == "__agg.kind" {
            if p.op != "eq" {
                return Err("`__agg.kind` must use op eq".into());
            }
            let name = match &p.value {
                Yaml::String(s) => s.clone(),
                _ => return Err("`__agg.kind` must be a string".into()),
            };
            kind = Some(
                AggKind::parse(&name).ok_or_else(|| format!("unknown aggregate kind `{name}`"))?,
            );
        }
    }
    // A slot with counts but no kind gets the plain burst for its fact kind.
    let kind = match (kind, &fact) {
        (Some(k), _) => k,
        (None, Some(FactKind::FileIO)) => AggKind::FileBurst,
        (None, Some(FactKind::Exec)) => AggKind::ExecBurst,
        (None, _) => {
            return Err(format!(
                "`__agg` fields on a {} slot with no `__agg.kind`, only FileIO and Exec have a default",
                slot.fact
            ))
        }
    };
    if fact.as_ref() != Some(&kind.fact_kind()) {
        return Err(format!(
            "aggregate `{}` counts {:?} facts, not {}",
            kind.name(),
            kind.fact_kind(),
            slot.fact
        ));
    }
    for p in &agg {
        let name = &p.field["__agg.".len()..];
        if !kind.fields().contains(&name) {
            return Err(format!("aggregate `{}` has no field `{}`", kind.name(), p.field));
        }
    }

    let window_sec = agg
        .iter()
        .find(|p| p.field == "__agg.window_sec" && p.op == "lte")
        .and_then(|p| yaml_f64(&p.value))
        .map(|v| v as u64)
        .unwrap_or(slot.slot_ttl_sec);

    let mut entry = Vec::new();
    let mut ops = Vec::new();
    for p in &slot.where_ {
        if p.field.starts_with("__agg.") {
            continue;
        }
        if kind.is_file_burst() && p.field == "op" {
            ops.push(p.clone());
        } else {
            entry.push(p.clone());
        }
    }
    Ok(Some(AggPlan {
        kind,
        window_sec,
        entry,
        ops,
    }))
}

// Distinct-value counters kept per window.
const DIM_FILES: usize = 0; // paths changed by an op the slot asks for
const DIM_PATHS: usize = 1; // every changed path, the rename ratio denominator
const DIM_TOPDIRS: usize = 2;
const DIM_DIRS: usize = 3;
const DIM_MOUNTS: usize = 4;
const DIM_ARG_TOPDIRS: usize = 5; // exec_burst: top dirs of the path arguments
const DIM_CHILDREN: usize = 6; // proc_burst_workers: child pids
const DIM_DSTS: usize = 7; // NetFlow destination addresses
const DIM_PORTS: usize = 8; // NetFlow destination ports
const DIM_USERS: usize = 9; // Auth user names
const DIM_SRCS: usize = 10; // Auth source addresses
const DIMS: usize = 11;

/// Oldest events are dropped past this many, so one window cannot grow without bound.
const AGG_WINDOW_CAP: usize = 200_000;

#[derive(Debug)]
struct AggEntry {
    ts: u64,
    /// (dimension, 64-bit hash of the key). Distinct counts only need equality, and a
    /// window can hold 200,000 events, so a hash costs 8 bytes where the path cost a
    /// heap string twice (here and in `distinct`). A live run with 2.7 million facts held
    /// most of the agent's 2.4 GB in these windows.
    keys: Vec<(u8, u64)>,
    rename: bool,
    bytes: Option<u64>,
}

/// The key a window counts, hashed.
fn key_hash(s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

#[derive(Debug, Default)]
struct AggWindow {
    entries: VecDeque<AggEntry>,
    distinct: [HashMap<u64, u32>; DIMS],
    renames: u64,
    bytes: u64,
    sized_writes: u64,
}

impl AggWindow {
    fn push(&mut self, e: AggEntry) {
        for (dim, key) in &e.keys {
            *self.distinct[*dim as usize].entry(*key).or_insert(0) += 1;
        }
        if e.rename {
            self.renames += 1;
        }
        if let Some(b) = e.bytes {
            self.bytes += b;
            self.sized_writes += 1;
        }
        self.entries.push_back(e);
        if self.entries.len() > AGG_WINDOW_CAP {
            self.pop_front();
        }
    }

    fn pop_front(&mut self) {
        let Some(e) = self.entries.pop_front() else {
            return;
        };
        for (dim, key) in e.keys {
            let d = &mut self.distinct[dim as usize];
            if let Some(n) = d.get_mut(&key) {
                *n -= 1;
                if *n == 0 {
                    d.remove(&key);
                }
            }
        }
        if e.rename {
            self.renames -= 1;
        }
        if let Some(b) = e.bytes {
            self.bytes -= b;
            self.sized_writes -= 1;
        }
    }

    fn evict(&mut self, now: u64, window_sec: u64) {
        while let Some(front) = self.entries.front() {
            if now.saturating_sub(front.ts) > window_sec {
                self.pop_front();
            } else {
                break;
            }
        }
    }

    fn n(&self, dim: usize) -> usize {
        self.distinct[dim].len()
    }

    /// Seconds from the first sample of the newest unbroken run to the newest sample,
    /// and the samples in that run.
    fn run(&self) -> (u64, usize) {
        let mut it = self.entries.iter().rev();
        let Some(last) = it.next() else {
            return (0, 0);
        };
        let (mut start, mut n) = (last.ts, 1);
        for e in it {
            if start.saturating_sub(e.ts) > SUSTAINED_MAX_GAP_SEC {
                break;
            }
            start = e.ts;
            n += 1;
        }
        (last.ts.saturating_sub(start), n)
    }
}

/// The window entry for `f`, or None when the slot does not count this event.
fn agg_entry(plan: &AggPlan, f: &Fact, eval: &Evaluator) -> Option<AggEntry> {
    if f.kind != plan.kind.fact_kind() {
        return None;
    }
    // Most file facts are plain opens. Checking the op first skips the location regexes.
    if plan.kind.is_file_burst() && !matches!(f.get_str("op"), Some("write" | "rename" | "delete")) {
        return None;
    }
    match plan.kind {
        AggKind::FileReadBurst if f.get_str("op") != Some("open") => return None,
        AggKind::EgressVolume if f.get_str("op") != Some("send") => return None,
        AggKind::ConnBurst if f.get_str("op") != Some("connect") => return None,
        _ => {}
    }
    if !plan.entry.iter().all(|p| eval.predicate(p, f, None)) {
        return None;
    }
    let mut keys = Vec::new();
    let mut rename = false;
    let mut bytes = None;
    match plan.kind {
        AggKind::FileBurst | AggKind::PartialEncryptBurst => {
            let op = f.get_str("op")?;
            let path = f.get_str("path")?;
            let counts = if plan.ops.is_empty() {
                op == "write" || op == "rename"
            } else {
                plan.ops.iter().all(|p| eval.predicate(p, f, None))
            };
            keys.push((DIM_PATHS as u8, key_hash(path)));
            if counts {
                keys.push((DIM_FILES as u8, key_hash(path)));
                keys.push((DIM_TOPDIRS as u8, key_hash(&topdir_of(path))));
                keys.push((DIM_DIRS as u8, key_hash(&parent_dir(path))));
                if let Some(m) = f.get_str("mount_point") {
                    keys.push((DIM_MOUNTS as u8, key_hash(m)));
                }
            }
            rename = op == "rename";
            if op == "write" {
                bytes = f.get_str("bytes").and_then(|b| b.parse::<u64>().ok());
            }
        }
        AggKind::FileReadBurst => {
            let path = f.get_str("path")?;
            keys.push((DIM_FILES as u8, key_hash(path)));
            keys.push((DIM_TOPDIRS as u8, key_hash(&topdir_of(path))));
            keys.push((DIM_DIRS as u8, key_hash(&parent_dir(path))));
        }
        AggKind::ErrnoSpike | AggKind::Sustained => {}
        AggKind::ExecBurst => {
            // argv[0] is the tool itself, so only later absolute paths are targets.
            if let Some(cmd) = f.get_str("cmd") {
                for arg in cmd.split_whitespace().skip(1) {
                    if arg.starts_with('/') {
                        keys.push((DIM_ARG_TOPDIRS as u8, key_hash(&topdir_of(arg))));
                    }
                }
            }
        }
        AggKind::ProcBurstWorkers => {
            keys.push((DIM_CHILDREN as u8, key_hash(f.get_str("pid").unwrap_or("-"))));
        }
        AggKind::EgressVolume => {
            bytes = Some(f.get_str("bytes").and_then(|b| b.parse::<u64>().ok()).unwrap_or(0));
            if let Some(d) = f.get_str("dst_ip") {
                keys.push((DIM_DSTS as u8, key_hash(d)));
            }
        }
        AggKind::ConnBurst => {
            if let Some(d) = f.get_str("dst_ip") {
                keys.push((DIM_DSTS as u8, key_hash(d)));
            }
            if let Some(p) = f.get_str("dst_port") {
                keys.push((DIM_PORTS as u8, key_hash(p)));
            }
        }
        AggKind::AuthFailBurst => {
            if let Some(u) = f.get_str("user") {
                keys.push((DIM_USERS as u8, key_hash(u)));
            }
            if let Some(s) = f.get_str("src_ip") {
                keys.push((DIM_SRCS as u8, key_hash(s)));
            }
        }
    }
    Some(AggEntry {
        ts: f.ts,
        keys,
        rename,
        bytes,
    })
}

/// The `__agg.*` fields for the current state of a window.
fn agg_fields(plan: &AggPlan, w: &AggWindow) -> Vec<(String, String)> {
    let mut out = vec![
        ("__agg.kind".to_string(), plan.kind.name().to_string()),
        ("__agg.window_sec".to_string(), plan.window_sec.to_string()),
    ];
    let mut put = |k: &str, v: String| out.push((format!("__agg.{k}"), v));
    match plan.kind {
        AggKind::FileBurst | AggKind::PartialEncryptBurst => {
            put("files", w.n(DIM_FILES).to_string());
            put("distinct_topdirs", w.n(DIM_TOPDIRS).to_string());
            put("distinct_dirs", w.n(DIM_DIRS).to_string());
            put("distinct_mounts", w.n(DIM_MOUNTS).to_string());
            let changed = w.n(DIM_PATHS).max(1) as f64;
            put("rename_ratio", format!("{:.3}", (w.renames as f64 / changed).min(1.0)));
            // Only write syscalls carry a size. Without them the byte fields stay unset,
            // so a slot that needs them does not match.
            if w.sized_writes > 0 {
                put("total_bytes", w.bytes.to_string());
                put(
                    "avg_write_kib",
                    format!("{:.1}", w.bytes as f64 / w.sized_writes as f64 / 1024.0),
                );
            }
        }
        AggKind::FileReadBurst => {
            put("files", w.n(DIM_FILES).to_string());
            put("distinct_topdirs", w.n(DIM_TOPDIRS).to_string());
            put("distinct_dirs", w.n(DIM_DIRS).to_string());
        }
        AggKind::ErrnoSpike => put("count", w.entries.len().to_string()),
        AggKind::ExecBurst => {
            put("count", w.entries.len().to_string());
            put("distinct_topdirs", w.n(DIM_ARG_TOPDIRS).to_string());
        }
        AggKind::ProcBurstWorkers => put("children", w.n(DIM_CHILDREN).to_string()),
        AggKind::EgressVolume => {
            put("bytes_out", w.bytes.to_string());
            put("bytes_out_mb", format!("{:.1}", w.bytes as f64 / 1_000_000.0));
            put("sends", w.sized_writes.to_string());
            put("distinct_dsts", w.n(DIM_DSTS).to_string());
        }
        AggKind::ConnBurst => {
            let n = w.entries.len();
            put("count", n.to_string());
            put("distinct_dsts", w.n(DIM_DSTS).to_string());
            put("distinct_ports", w.n(DIM_PORTS).to_string());
            // Connects per second over the seconds the window's events span. Stamps are
            // whole seconds, so a burst inside one second divides by 1.
            let span = match (w.entries.front(), w.entries.back()) {
                (Some(a), Some(b)) => b.ts.saturating_sub(a.ts) + 1,
                _ => 1,
            };
            put("conn_rate_eps", format!("{:.1}", n as f64 / span as f64));
        }
        AggKind::Sustained => {
            let (duration, samples) = w.run();
            put("duration_sec", duration.to_string());
            put("samples", samples.to_string());
        }
        AggKind::AuthFailBurst => {
            put("failures", w.entries.len().to_string());
            put("distinct_users", w.n(DIM_USERS).to_string());
            put("distinct_srcs", w.n(DIM_SRCS).to_string());
        }
    }
    out
}

/// Top directory of a path: the first component, or the first two under /var, /run
/// and /usr, where the first level is a system area and the second names the data.
/// /home/alice/x is /home, /var/www/x is /var/www, /run/user/1000 is /run/user.
pub fn topdir_of(path: &str) -> String {
    let mut parts = path.split('/').filter(|s| !s.is_empty());
    match (path.starts_with('/'), parts.next(), parts.next()) {
        (false, _, _) | (true, None, _) => "-".to_string(),
        (true, Some(a), Some(b)) if matches!(a, "var" | "run" | "usr") => format!("/{a}/{b}"),
        (true, Some(a), _) => format!("/{a}"),
    }
}

/// Parent directory of an absolute path.
pub fn parent_dir(path: &str) -> String {
    match path.rfind('/') {
        Some(0) => "/".to_string(),
        Some(i) => path[..i].to_string(),
        None => "-".to_string(),
    }
}

// ---- Loading and validation ----

/// Playbook directory: $PB_DIR, else ./playbooks (same default as main.rs).
pub fn playbook_dir() -> String {
    std::env::var("PB_DIR").unwrap_or_else(|_| "playbooks".to_string())
}

/// What one directory load did, file by file.
#[derive(Debug, Default)]
pub struct LoadReport {
    /// Ids of playbooks that loaded.
    pub loaded: Vec<String>,
    /// (file name, reason) for YAML files that were rejected.
    pub skipped: Vec<(String, String)>,
    /// (playbook id, warning) for loaded playbooks with parts that can never match.
    pub warnings: Vec<(String, String)>,
    /// Files without a .yaml or .yml extension.
    pub ignored: Vec<String>,
}

impl LoadReport {
    pub fn log(&self, dir: &Path) {
        for (file, reason) in &self.skipped {
            eprintln!("playbooks: skipped {file}: {reason}");
        }
        for (id, w) in &self.warnings {
            eprintln!("playbooks: {id}: {w}");
        }
        for file in &self.ignored {
            eprintln!("playbooks: ignored {file} (not .yaml)");
        }
        eprintln!(
            "playbooks: loaded {} of {} YAML files from {} ({} skipped)",
            self.loaded.len(),
            self.loaded.len() + self.skipped.len(),
            dir.display(),
            self.skipped.len()
        );
    }
}

// Top-level keys PlaybookSpec reads.
const SPEC_KEYS: &[&str] = &[
    "id",
    "version",
    "name",
    "severity",
    "description",
    "window_sec",
    "cooldown_sec",
    "required",
    "optional",
    "forbidden",
    "vendor_map",
    "explain",
    "links",
    "context_keys",
    "emit_ground_truth",
    "shield_templates",
];
// Labels the engine does not read and that carry no matching logic.
const LABEL_KEYS: &[&str] = &["title", "reason_code", "rules"];
// Ops handled by Evaluator::predicate. Keep the two in sync.
const PREDICATE_OPS: &[&str] = &[
    "eq",
    "ne",
    "neq",
    "contains",
    "not_contains",
    "contains_any",
    "regex",
    "not_regex",
    "in",
    "not_in",
    "any_in",
    "in_cidr",
    "not_in_cidr",
    "gt",
    "gte",
    "lt",
    "lte",
    "exists",
    "in_slot",
];

fn load_playbook(path: &Path) -> std::result::Result<(PlaybookSpec, Vec<String>), String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read: {e}"))?;
    let mut spec: PlaybookSpec = serde_yaml::from_str(&text).map_err(|e| format!("{e}"))?;
    // Parsed again as a plain map to find keys the engine would silently ignore.
    let keys: Vec<String> = match serde_yaml::from_str::<Yaml>(&text) {
        Ok(Yaml::Mapping(m)) => m
            .keys()
            .filter_map(|k| k.as_str().map(str::to_string))
            .collect(),
        _ => Vec::new(),
    };
    let warnings = validate_playbook(&spec, &keys)?;
    prepare(&mut spec);
    Ok((spec, warnings))
}

/// Fact kinds parsed and regexes compiled once, after validation passed.
fn prepare(pb: &mut PlaybookSpec) {
    fn compile(p: &mut PredicateSpec) {
        if matches!(p.op.as_str(), "regex" | "not_regex") {
            p.re = yaml_text(&p.value).and_then(|pat| Regex::new(&pat).ok());
        }
        p.any_of.iter_mut().for_each(compile);
    }
    for slot in pb.required.iter_mut().chain(pb.optional.iter_mut()).chain(pb.forbidden.iter_mut()) {
        slot.kind = parse_fact_kind(&slot.fact);
        slot.needs_slot = slot.where_.iter().any(PredicateSpec::needs_slot);
        slot.where_.iter_mut().for_each(compile);
    }
}

/// Reject a playbook that can never fire, and warn about parts that can never match.
fn validate_playbook(pb: &PlaybookSpec, keys: &[String]) -> std::result::Result<Vec<String>, String> {
    let unknown: Vec<&str> = keys
        .iter()
        .map(String::as_str)
        .filter(|k| !SPEC_KEYS.contains(k) && !LABEL_KEYS.contains(k))
        .collect();

    // is_complete() is true for an empty list, so a playbook with no required
    // slots would fire on its first optional match.
    if pb.required.is_empty() {
        let mut why = "no required slots".to_string();
        if !unknown.is_empty() {
            why.push_str(&format!(
                " (it uses keys this engine does not read: {})",
                unknown.join(", ")
            ));
        }
        return Err(why);
    }
    // in_slot may name a required or optional slot of the same playbook.
    let slot_ids: Vec<&str> = pb
        .required
        .iter()
        .chain(&pb.optional)
        .map(|s| s.slot_id.as_str())
        .collect();
    for slot in &pb.required {
        if let Some(why) = slot_problem(slot, &slot_ids) {
            return Err(format!("required slot `{}` can never match: {why}", slot.slot_id));
        }
    }

    let mut warnings = Vec::new();
    for slot in &pb.optional {
        if let Some(why) = slot_problem(slot, &slot_ids) {
            warnings.push(format!("optional slot `{}` can never match: {why}", slot.slot_id));
        }
    }
    for slot in &pb.forbidden {
        if let Some(why) = slot_problem(slot, &slot_ids) {
            warnings.push(format!(
                "forbidden slot `{}` can never match, so it never vetoes: {why}",
                slot.slot_id
            ));
        }
    }
    for link in &pb.links {
        if let Some(why) = link_problem(link, pb) {
            return Err(format!("link {} -> {}: {why}", link.from, link.to));
        }
    }
    for k in &pb.context_keys {
        let ok = !k.is_empty()
            && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.');
        if !ok {
            return Err(format!("context key `{k}` is not a field name"));
        }
    }
    for t in &pb.shield_templates {
        if let Some(why) = template_problem(t, &slot_ids) {
            return Err(format!("shield template {} from `{}`: {why}", t.kind, t.slot));
        }
    }
    for vm in &pb.vendor_map {
        let known = pb
            .required
            .iter()
            .chain(pb.optional.iter())
            .any(|s| s.slot_id == vm.fills_slot);
        if !known {
            warnings.push(format!("vendor_map fills unknown slot `{}`", vm.fills_slot));
        }
    }
    if !unknown.is_empty() {
        warnings.push(format!(
            "ignoring keys this engine does not implement: {}",
            unknown.join(", ")
        ));
    }
    Ok(warnings)
}

/// Why a shield template cannot make a proposal, or None if it can.
fn template_problem(t: &ShieldTemplate, slot_ids: &[&str]) -> Option<String> {
    if !SHIELD_KINDS.contains(&t.kind.as_str()) {
        return Some(format!("kind is not one of {}", SHIELD_KINDS.join(", ")));
    }
    if !slot_ids.contains(&t.slot.as_str()) {
        return Some("unknown slot".into());
    }
    if let Some(scope) = &t.scope_slot {
        if !slot_ids.contains(&scope.as_str()) {
            return Some(format!("unknown scope_slot `{scope}`"));
        }
    }
    if t.ttl_sec == 0 {
        return Some("ttl_sec must be above 0".into());
    }
    if let Some(re) = &t.path_regex {
        if t.kind == "connect_deny" {
            return Some("path_regex is for exec_deny and file_deny".into());
        }
        if let Err(e) = Regex::new(re) {
            return Some(format!("bad path_regex: {e}"));
        }
    }
    None
}

/// Why a link cannot be checked, or None if it can.
fn link_problem(link: &LinkSpec, pb: &PlaybookSpec) -> Option<String> {
    for end in [&link.from, &link.to] {
        let known = pb.required.iter().chain(&pb.optional).any(|s| &s.slot_id == end);
        if !known {
            if pb.forbidden.iter().any(|s| &s.slot_id == end) {
                return Some(format!("`{end}` is a forbidden slot, which only vetoes"));
            }
            return Some(format!("unknown slot `{end}`"));
        }
    }
    if link.from == link.to {
        return Some("a link needs two different slots".into());
    }
    match (link.follows_within_sec, link.within_sec) {
        (Some(_), Some(_)) => {
            return Some("give follows_within_sec or within_sec, not both".into())
        }
        (None, None) => return Some("give follows_within_sec or within_sec".into()),
        _ => {}
    }
    if let Some(r) = link.requires.iter().find(|r| !LINK_REQUIRES.contains(&r.as_str())) {
        return Some(format!(
            "unknown requirement `{r}` (known: {})",
            LINK_REQUIRES.join(", ")
        ));
    }
    None
}

/// Why a slot can never match a fact, or None if it can.
fn slot_problem(slot: &SlotSpec, slot_ids: &[&str]) -> Option<String> {
    if parse_fact_kind(&slot.fact).is_none() {
        if slot.fact == "NetConn" {
            return Some(
                "fact kind `NetConn` is gone: use NetFlow (net_flow connects and sends) or Net (tcp_state)"
                    .into(),
            );
        }
        return Some(format!("fact kind `{}` is not one the engine produces", slot.fact));
    }
    for p in &slot.where_ {
        if let Some(why) = predicate_problem(p, slot, slot_ids) {
            return Some(why);
        }
    }
    // Counting slots: the aggregate kind must exist, fit the fact kind and set
    // every `__agg` field the slot tests.
    if let Err(why) = agg_plan(slot) {
        return Some(why);
    }
    None
}

fn predicate_problem(p: &PredicateSpec, slot: &SlotSpec, slot_ids: &[&str]) -> Option<String> {
    if p.is_any_of() {
        if !p.field.is_empty() || !p.op.is_empty() {
            return Some("`any_of` cannot also have a field or an op".into());
        }
        for q in &p.any_of {
            if q.field.starts_with("__agg.") {
                return Some("`any_of` cannot hold `__agg` conditions".into());
            }
            if let Some(why) = predicate_problem(q, slot, slot_ids) {
                return Some(why);
            }
        }
        return None;
    }
    let op = p.op.as_str();
    if p.field.is_empty() {
        return Some(format!("a condition with op `{op}` has no field"));
    }
    if !PREDICATE_OPS.contains(&op) {
        return Some(format!("op `{op}` on `{}` is not supported", p.field));
    }
    let value_ok = match op {
        "in" | "not_in" | "any_in" | "contains_any" | "in_cidr" | "not_in_cidr" => {
            matches!(p.value, Yaml::Sequence(_))
        }
        "gt" | "gte" | "lt" | "lte" => yaml_f64(&p.value).is_some(),
        "exists" => matches!(p.value, Yaml::Bool(_)),
        "in_slot" => matches!(p.value, Yaml::String(_)),
        _ => matches!(p.value, Yaml::String(_) | Yaml::Number(_) | Yaml::Bool(_)),
    };
    if !value_ok {
        return Some(format!("op `{op}` on `{}` has the wrong value type", p.field));
    }
    if matches!(op, "regex" | "not_regex") {
        if let Yaml::String(pat) = &p.value {
            if let Err(e) = Regex::new(pat) {
                let why = e.to_string();
                let last = why.lines().last().unwrap_or("").trim().to_string();
                return Some(format!("bad regex on `{}`: {last}", p.field));
            }
        }
    }
    if matches!(op, "in_cidr" | "not_in_cidr") {
        let bad: Vec<String> = yaml_list(&p.value)
            .unwrap_or_default()
            .into_iter()
            .filter(|c| !valid_cidr(c))
            .collect();
        if !bad.is_empty() {
            return Some(format!("`{}` on `{}` has bad networks: {}", op, p.field, bad.join(", ")));
        }
    }
    if op == "in_slot" {
        let target = yaml_text(&p.value).unwrap_or_default();
        let other = target.split('.').next().unwrap_or("");
        if other == slot.slot_id {
            return Some(format!("in_slot on `{}` names its own slot", p.field));
        }
        if !slot_ids.contains(&other) {
            return Some(format!("in_slot on `{}` names unknown slot `{other}`", p.field));
        }
    }
    None
}

// Same coercion as the gte/lte arm of Evaluator::predicate.
fn yaml_f64(v: &Yaml) -> Option<f64> {
    match v {
        Yaml::Number(n) => n
            .as_f64()
            .or_else(|| n.as_i64().map(|x| x as f64))
            .or_else(|| n.as_u64().map(|x| x as f64)),
        Yaml::String(s) => s.parse::<f64>().ok(),
        _ => None,
    }
}


/// Load `dir` and swap the result into `engine`. Files are parsed before the lock is
/// taken. In-flight hypotheses and counts start over, as after a restart. Cooldowns of
/// playbooks that are still loaded are kept, so a reload cannot make a playbook that
/// just fired fire again at once. If the directory cannot be read, the current set stays.
pub fn reload_into(engine: &std::sync::Mutex<PlaybookEngine>, dir: &Path) -> Result<LoadReport> {
    let (mut fresh, report) = PlaybookEngine::load_dir_report(dir)?;
    let mut guard = engine
        .lock()
        .map_err(|_| anyhow!("playbook engine lock poisoned"))?;
    fresh.cooldown = guard
        .cooldown
        .drain()
        .filter(|((id, _), _)| fresh.playbooks.contains_key(id))
        .collect();
    *guard = fresh;
    Ok(report)
}

// ---- Engine ----

/// (playbook id, slot id, host, group). The group is the parent pid for
/// proc_burst_workers, the pid for sustained, and empty for every other kind.
type WindowKey = (String, String, String, String);

pub struct PlaybookEngine {
    eval: Evaluator,
    pub playbooks: HashMap<String, PlaybookSpec>, // id -> spec
    // One hypothesis per (playbook, host).
    hypotheses: HashMap<(String, String), Hypothesis>,
    // playbook id -> slot id -> ttl
    slot_ttls: HashMap<String, HashMap<String, u64>>,
    // playbook id -> slot id -> counting plan, for slots that test __agg fields
    agg_plans: HashMap<String, HashMap<String, AggPlan>>,
    windows: HashMap<WindowKey, AggWindow>,
    // (playbook, host) -> time it last fired
    cooldown: HashMap<(String, String), u64>,
}

impl PlaybookEngine {
    pub fn empty() -> Self {
        Self {
            eval: Evaluator::new(),
            playbooks: HashMap::new(),
            hypotheses: HashMap::new(),
            slot_ttls: HashMap::new(),
            agg_plans: HashMap::new(),
            windows: HashMap::new(),
            cooldown: HashMap::new(),
        }
    }

    /// Load every playbook in `dir` and log what was loaded and what was skipped.
    /// A bad file is skipped. Only an unreadable directory is an error.
    pub fn load_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let (engine, report) = Self::load_dir_report(dir.as_ref())?;
        report.log(dir.as_ref());
        Ok(engine)
    }

    /// Same as load_dir, but returns the report instead of logging it.
    pub fn load_dir_report<P: AsRef<Path>>(dir: P) -> Result<(Self, LoadReport)> {
        let dir = dir.as_ref();
        let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
            .map_err(|e| anyhow!("cannot read playbook dir {}: {e}", dir.display()))?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<std::io::Result<_>>()?;
        // Sorted so a duplicate id is resolved the same way on every host.
        paths.sort();

        let mut report = LoadReport::default();
        let mut playbooks = HashMap::new();
        for path in paths {
            if !path.is_file() {
                continue;
            }
            let file = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            if !matches!(
                path.extension().and_then(|s| s.to_str()),
                Some("yaml") | Some("yml")
            ) {
                report.ignored.push(file);
                continue;
            }
            match load_playbook(&path) {
                Ok((spec, warnings)) => {
                    if playbooks.contains_key(&spec.id) {
                        report
                            .skipped
                            .push((file, format!("duplicate playbook id `{}`", spec.id)));
                        continue;
                    }
                    for w in warnings {
                        report.warnings.push((spec.id.clone(), w));
                    }
                    report.loaded.push(spec.id.clone());
                    playbooks.insert(spec.id.clone(), spec);
                }
                Err(reason) => report.skipped.push((file, reason)),
            }
        }

        let mut engine = Self::empty();
        for (pb_id, pb) in &playbooks {
            let ttls = engine.slot_ttls.entry(pb_id.clone()).or_default();
            let mut plans = HashMap::new();
            for s in pb.required.iter().chain(&pb.optional).chain(&pb.forbidden) {
                ttls.insert(s.slot_id.clone(), s.slot_ttl_sec);
                // A bad plan was already reported by validate_playbook (an error for a
                // required slot, a warning otherwise), so it is skipped here.
                if let Ok(Some(plan)) = agg_plan(s) {
                    plans.insert(s.slot_id.clone(), plan);
                }
            }
            if !plans.is_empty() {
                engine.agg_plans.insert(pb_id.clone(), plans);
            }
        }
        engine.playbooks = playbooks;
        Ok((engine, report))
    }

    /// (counting windows, events held in them, live hypotheses), for /pipeline/status.
    pub fn sizes(&self) -> (usize, usize, usize) {
        let held = self.windows.values().map(|w| w.entries.len()).sum();
        (self.windows.len(), held, self.hypotheses.len())
    }

    fn now_sec() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs()
    }

    /// Advance time with no new event: drop counted events older than their window,
    /// slot hits older than their TTL, empty hypotheses, and cooldowns that ended.
    pub fn tick(&mut self, now: u64) {
        let Self {
            playbooks,
            hypotheses,
            slot_ttls,
            agg_plans,
            windows,
            cooldown,
            ..
        } = self;
        windows.retain(|(pb_id, slot_id, _, _), w| {
            match agg_plans.get(pb_id).and_then(|m| m.get(slot_id)) {
                Some(plan) => {
                    w.evict(now, plan.window_sec);
                    !w.entries.is_empty()
                }
                None => false,
            }
        });
        hypotheses.retain(|(pb_id, _), h| {
            h.expire(now, slot_ttls.get(pb_id));
            !h.hits.is_empty()
        });
        cooldown.retain(|(pb_id, _), last| {
            playbooks
                .get(pb_id)
                .map(|pb| now.saturating_sub(*last) < pb.cooldown_sec)
                .unwrap_or(false)
        });
    }

    /// Match one fact against every playbook. Returns the playbooks it completed.
    pub fn ingest_fact(&mut self, mut fact: Fact) -> Vec<PlaybookHit> {
        if fact.ts == 0 {
            fact.ts = Self::now_sec();
        }
        let now = fact.ts;
        let host = fact.host.clone();
        let Self {
            eval,
            playbooks,
            hypotheses,
            slot_ttls,
            agg_plans,
            windows,
            cooldown,
        } = self;

        let mut fired = Vec::new();
        for (pb_id, pb) in playbooks.iter() {
            let plans = agg_plans.get(pb_id);
            // Counting windows, hypotheses and cooldowns are all per chain.
            let chain = pb.context_of(&fact);
            let mut matched: Vec<(&SlotSpec, Fact)> = Vec::new();
            for slot in pb.required.iter().chain(&pb.optional).chain(&pb.forbidden) {
                match plans.and_then(|m| m.get(&slot.slot_id)) {
                    None => {
                        if eval.match_slot(slot, &fact) {
                            matched.push((slot, fact.clone()));
                        }
                    }
                    Some(plan) => {
                        let Some(entry) = agg_entry(plan, &fact, eval) else {
                            continue;
                        };
                        let group = plan
                            .kind
                            .group_field()
                            .map(|k| fact.get_str(k).unwrap_or("-").to_string())
                            .unwrap_or_default();
                        let key = (pb_id.clone(), slot.slot_id.clone(), chain.clone(), group);
                        let w = windows.entry(key).or_default();
                        w.evict(now, plan.window_sec);
                        w.push(entry);
                        let mut counted = fact.clone();
                        for (k, v) in agg_fields(plan, w) {
                            counted.fields.insert(k, v);
                        }
                        if eval.match_slot(slot, &counted) {
                            matched.push((slot, counted));
                        }
                    }
                }
            }
            if matched.is_empty() {
                continue;
            }

            let key = (pb_id.clone(), chain);
            let hyp = hypotheses.entry(key.clone()).or_default();
            hyp.expire(now, slot_ttls.get(pb_id));
            // in_slot conditions are checked against the hits that were live before
            // this fact, so a step can be tied to the process an earlier step saw.
            matched.retain(|(slot, f)| !slot.needs_slot || eval.match_with_hits(slot, f, &hyp.hits));
            if matched.is_empty() {
                continue;
            }
            for (slot, f) in matched {
                hyp.hits.insert(
                    slot.slot_id.clone(),
                    SlotHit {
                        ts: now,
                        tags: slot.derive_tags.clone(),
                        fact: f,
                    },
                );
            }

            if let Some(hit) = Self::try_fire(pb, &key, hyp, cooldown, &host, now) {
                fired.push(hit);
            }
        }
        fired
    }

    /// Fire when every required slot is hit inside the window, every link between
    /// required slots holds, nothing forbidden is live and the cooldown is over.
    fn try_fire(
        pb: &PlaybookSpec,
        key: &(String, String),
        hyp: &mut Hypothesis,
        cooldown: &mut HashMap<(String, String), u64>,
        host: &str,
        now: u64,
    ) -> Option<PlaybookHit> {
        if !hyp.complete(pb, now) || hyp.vetoed(pb) {
            return None;
        }
        let left_out = hyp.broken_optional(pb)?;
        let cooled = cooldown
            .get(key)
            .map(|last| now.saturating_sub(*last) >= pb.cooldown_sec)
            .unwrap_or(true);
        if !cooled {
            return None;
        }
        cooldown.insert(key.clone(), now);
        let context = pb.context_pairs(&key.1);
        let hit = PlaybookHit::from_pb(pb, host.to_string(), hyp, now, &left_out, context);
        // Start a new chain. The counting windows keep their events.
        hyp.hits.clear();
        Some(hit)
    }

    /// Fill the slot a vendor_map entry names with a vendor alert. No caller in the agent
    /// today.
    pub fn satisfy_slot_from_vendor(
        &mut self,
        product: &str,
        alert_type: &str,
        host: &str,
        ts: u64,
    ) -> Option<PlaybookHit> {
        let now = if ts == 0 { Self::now_sec() } else { ts };
        let mut out: Option<PlaybookHit> = None;
        for (pb_id, pb) in &self.playbooks {
            for vm in &pb.vendor_map {
                if !(vm.product.eq_ignore_ascii_case(product)
                    && vm.alert_type.eq_ignore_ascii_case(alert_type))
                {
                    continue;
                }
                let Some(slot) = pb
                    .required
                    .iter()
                    .chain(pb.optional.iter())
                    .find(|s| s.slot_id == vm.fills_slot)
                else {
                    continue;
                };
                let mut fields = HashMap::new();
                fields.insert("product".to_string(), product.to_string());
                fields.insert("alert_type".to_string(), alert_type.to_string());
                let fact = Fact {
                    ts: now,
                    kind: FactKind::VendorAlert,
                    host: host.to_string(),
                    user: None,
                    exe: None,
                    cmd: None,
                    fields,
                };
                // A vendor alert names a host, not a user, so it joins every live chain
                // of the playbook on that host, or starts one when there is none.
                let mut keys: Vec<(String, String)> = self
                    .hypotheses
                    .keys()
                    .filter(|(id, chain)| {
                        id == pb_id
                            && (chain == host || chain.starts_with(&format!("{host}|")))
                    })
                    .cloned()
                    .collect();
                if keys.is_empty() {
                    keys.push((pb_id.clone(), pb.context_of(&fact)));
                }
                keys.sort();
                for key in keys {
                    let hyp = self.hypotheses.entry(key.clone()).or_default();
                    hyp.expire(now, self.slot_ttls.get(pb_id));
                    hyp.hits.insert(
                        slot.slot_id.clone(),
                        SlotHit {
                            ts: now,
                            tags: slot.derive_tags.clone(),
                            fact: fact.clone(),
                        },
                    );
                    if let Some(hit) =
                        Self::try_fire(pb, &key, hyp, &mut self.cooldown, host, now)
                    {
                        out = Some(hit);
                    }
                }
            }
        }
        out
    }
}

// ---- Hit materialization ----

#[derive(Debug, Clone)]
pub struct PlaybookHit {
    pub playbook_id: String,
    pub playbook_name: String,
    pub severity: String,
    pub host: String,
    /// Event time of the fact that completed the playbook, Unix seconds.
    pub ts: u64,
    pub tags: Vec<String>,
    pub rationale: Vec<String>,
    pub slots: HashMap<String, SlotHitSummary>,
    /// Slot ids that had a hit, sorted.
    pub filled_slots: Vec<String>,
    /// Optional slots with no hit, sorted.
    pub missing_optional: Vec<String>,
    pub primary_ttp: Option<String>,
    /// Every technique in explain.primary_ttp, primary first.
    pub techniques: Vec<String>,
    pub narrative: Option<String>,
    /// The chain's context: host first, then the playbook's other context keys.
    pub context: Vec<(String, String)>,
    /// Optional slots whose hit broke a link, with the link. Left out of the evidence.
    pub left_out: Vec<(String, String)>,
    /// The playbook asks for its hits to be written to the ground truth file.
    pub ground_truth: bool,
    /// Shields this hit proposes, from the playbook's shield_templates.
    pub shield_templates: Vec<ShieldTemplate>,
}

#[derive(Debug, Clone)]
pub struct SlotHitSummary {
    pub ts: u64,
    pub tags: Vec<String>,
    pub fields: HashMap<String, String>,
}

/// One readable line of evidence for a slot hit.
fn evidence_line(slot_id: &str, f: &Fact) -> String {
    let mut parts = Vec::new();
    for k in ["exe", "cmd", "op", "path", "path2", "errno", "mount_fstype", "tags"] {
        if let Some(v) = f.get_str(k) {
            if !v.is_empty() {
                parts.push(format!("{k}={v}"));
            }
        }
    }
    let mut agg: Vec<_> = f
        .fields
        .iter()
        .filter(|(k, _)| k.starts_with("__agg.") && k.as_str() != "__agg.kind")
        .map(|(k, v)| format!("{}={v}", &k["__agg.".len()..]))
        .collect();
    agg.sort();
    if let Some(kind) = f.fields.get("__agg.kind") {
        parts.push(format!("{kind}({})", agg.join(" ")));
    }
    format!("{slot_id}: {}", parts.join(" "))
}

/// One steps_template line filled from the slot hits.
fn render_step(t: &str, hits: &HashMap<String, SlotHit>) -> String {
    // {{#if slot}}...{{/if}} first, then {{slot.field}}.
    let mut s = t.to_string();
    while let Some(a) = s.find("{{#if ") {
        let Some(name_end) = s[a..].find("}}").map(|i| a + i) else {
            break;
        };
        let slot = s[a + 6..name_end].trim().to_string();
        let Some(end) = s[name_end..].find("{{/if}}").map(|i| name_end + i) else {
            break;
        };
        let inner = s[name_end + 2..end].to_string();
        let keep = if hits.contains_key(&slot) { inner } else { String::new() };
        s.replace_range(a..end + 7, &keep);
    }
    let mut out = String::new();
    let mut rest = s.as_str();
    while let Some(a) = rest.find("{{") {
        out.push_str(&rest[..a]);
        let Some(b) = rest[a..].find("}}").map(|i| a + i) else {
            out.push_str(&rest[a..]);
            rest = "";
            break;
        };
        let key = rest[a + 2..b].trim();
        let (slot, field) = key.split_once('.').unwrap_or((key, ""));
        let v = hits.get(slot).and_then(|h| h.fact.get_str(field)).unwrap_or("?");
        out.push_str(v);
        rest = &rest[b + 2..];
    }
    out.push_str(rest);
    out
}

impl PlaybookHit {
    fn from_pb(
        pb: &PlaybookSpec,
        host: String,
        hyp: &Hypothesis,
        now: u64,
        left_out: &[(String, String)],
        context: Vec<(String, String)>,
    ) -> Self {
        let hits: HashMap<String, SlotHit> = hyp
            .hits
            .iter()
            .filter(|(id, _)| !left_out.iter().any(|(l, _)| l == *id))
            .map(|(id, h)| (id.clone(), h.clone()))
            .collect();
        let mut tags = Vec::new();
        let mut slots: HashMap<String, SlotHitSummary> = HashMap::new();
        for (slot_id, hit) in &hits {
            tags.extend(hit.tags.iter().cloned());
            let mut fields = hit.fact.fields.clone();
            if let Some(exe) = &hit.fact.exe {
                fields.insert("exe".into(), exe.clone());
            }
            if let Some(cmd) = &hit.fact.cmd {
                fields.insert("cmd".into(), cmd.clone());
            }
            slots.insert(
                slot_id.clone(),
                SlotHitSummary {
                    ts: hit.ts,
                    tags: hit.tags.clone(),
                    fields,
                },
            );
        }
        tags.sort();
        tags.dedup();

        let mut filled_slots: Vec<String> = hits.keys().cloned().collect();
        filled_slots.sort();
        let mut missing_optional: Vec<String> = pb
            .optional
            .iter()
            .filter(|s| !hits.contains_key(&s.slot_id))
            .map(|s| s.slot_id.clone())
            .collect();
        missing_optional.sort();

        let links = if pb.links.is_empty() {
            String::new()
        } else {
            " and every link between them held".to_string()
        };
        let mut rationale = vec![format!(
            "Playbook '{}' satisfied: every required slot hit within {} s{links}.",
            pb.name, pb.window_sec
        )];
        for slot_id in &filled_slots {
            rationale.push(evidence_line(slot_id, &hits[slot_id].fact));
        }
        for (slot_id, link) in left_out {
            rationale.push(format!("left out {slot_id}: its hit breaks the link {link}"));
        }
        if let Some(e) = &pb.explain {
            for t in &e.steps_template {
                let line = render_step(t, &hits);
                if !line.trim().is_empty() {
                    rationale.push(line);
                }
            }
        }

        Self {
            playbook_id: pb.id.clone(),
            playbook_name: pb.name.clone(),
            severity: pb.severity.clone(),
            host,
            ts: now,
            tags,
            rationale,
            slots,
            filled_slots,
            missing_optional,
            primary_ttp: pb.explain.as_ref().and_then(|e| e.primary_ttp()),
            techniques: pb.explain.as_ref().map(|e| e.ttps.clone()).unwrap_or_default(),
            narrative: pb.explain.as_ref().and_then(|e| e.narrative.clone()),
            context,
            left_out: left_out.to_vec(),
            ground_truth: pb.emit_ground_truth,
            shield_templates: pb.shield_templates.clone(),
        }
    }

    /// The incident id: playbook, host, time, and the other context values when the
    /// playbook has context keys, so two users' chains at one second stay apart.
    pub fn id(&self) -> String {
        let mut id = format!("pb:{}:{}:{}", self.playbook_id, self.host, self.ts);
        for (k, v) in self.context.iter().filter(|(k, _)| k != "host") {
            id.push_str(&format!(":{k}={v}"));
        }
        id
    }
    pub fn to_signal(&self) -> Signal {
        // risk heuristic: required size + optional contribution
        let req = self.slots.len() as f32;
        let base = 0.6_f32.min(0.3 + 0.1 * req);
        let score = (base + 0.1 * (self.tags.len() as f32).min(4.0)).min(0.95);
        let mut tagset = self.tags.clone();
        for ttp in self.primary_ttp.iter().chain(&self.techniques) {
            if !tagset.contains(ttp) {
                tagset.push(ttp.clone());
            }
        }
        Signal {
            detector: DetectorKind::Playbook,
            score,
            ts: self.ts,
            exe: None,
            user: None,
            host: Some(self.host.clone()),
            cmd_prefix: None,
            parent: None,
            signer: None,
            hash: None,
            pid: None,
            tags: tagset,
        }
    }
}

// Map a fact kind string used in YAML to the engine's kind.
fn parse_fact_kind(spec: &str) -> Option<FactKind> {
    match spec {
        "Exec" => Some(FactKind::Exec),
        "FileIO" => Some(FactKind::FileIO),
        "NetFlow" => Some(FactKind::NetFlow),
        "Net" => Some(FactKind::Net),
        "Auth" => Some(FactKind::Auth),
        "RuleHit" => Some(FactKind::RuleHit),
        "HostPerf" => Some(FactKind::HostPerf),
        "VendorAlert" => Some(FactKind::VendorAlert),
        // Counts are attached to the facts they count (see agg_plan), so no
        // separate aggregate kind is produced.
        _ => None,
    }
}

// quick test helpers
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_pb() {
        let y = r#"
id: pb_demo
version: 0.1
name: Demo
severity: high
window_sec: 600
cooldown_sec: 60
required:
  - slot_id: file_burst
    slot_ttl_sec: 420
    fact: Agg
    where:
      - { field: __agg.kind, op: eq, value: file_burst }
"#;
        let pb: PlaybookSpec = serde_yaml::from_str(y).unwrap();
        assert_eq!(pb.id, "pb_demo");
        assert_eq!(pb.required.len(), 1);
        assert_eq!(pb.required[0].where_[0].field, "__agg.kind");
    }

    use std::fs;

    // Ids of every playbook in playbooks/. All of them pass validation.
    const SHIPPED_LOADABLE: &[&str] = &[
        "pb_cloud_cli_exfil_linux_aws_rclone",
        "pb_compress_then_exfil_linux",
        "pb_container_breakout_precursors_linux",
        "pb_crypto_miner_linux_pool_cpu_gpu",
        "pb_encryptor_linux_memfd_rwx_lowrename",
        "pb_exfil_linux_staging_tmpfs_ssh",
        "pb_lateral_linux_ssh_portforward_pivot",
        "pb_lotl_cred_linux",
        "pb_lotl_linux_curl_devshm_systemdrun_memfd",
        "pb_memfd_child_egress_linux",
        "pb_outage_edr_tamper_disable",
        "pb_outage_internal_ddos_egress",
        "pb_outage_ransomware_linux_attr_locker",
        "pb_outage_ransomware_linux_backup_wipe_first",
        "pb_outage_ransomware_linux_cloud_store",
        "pb_outage_ransomware_linux_container_hostpath",
        "pb_outage_ransomware_linux_db_impact",
        "pb_outage_ransomware_linux_exfil_then_encrypt",
        "pb_outage_ransomware_linux_fileburst_crypto",
        "pb_outage_ransomware_linux_fuse_overlay",
        "pb_outage_ransomware_linux_intermit_encrypt",
        "pb_outage_ransomware_linux_luks_locker",
        "pb_outage_ransomware_linux_memfd_inplace",
        "pb_outage_ransomware_linux_netshare_only",
        "pb_outage_ransomware_linux_vm_image_datastore",
        "pb_outage_wiper_linux_service_kill",
        "pb_ssh_valid_creds_pivot_linux",
        "pb_terminal_exec_core",
        "pb_webshell_reverse_shell_linux",
    ];

    const GOOD: &str = r#"
id: pb_good
name: Good
severity: high
window_sec: 600
cooldown_sec: 60
required:
  - slot_id: miner
    slot_ttl_sec: 60
    fact: Exec
    where:
      - { field: exe, op: regex, value: "xmrig$" }
"#;

    fn temp_dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("edr_pb_{tag}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn exec_fact(exe: &str) -> Fact {
        Fact {
            ts: 100,
            kind: FactKind::Exec,
            host: "h".into(),
            user: None,
            exe: Some(exe.into()),
            cmd: None,
            fields: HashMap::new(),
        }
    }

    #[test]
    fn shipped_playbooks_load_or_are_reported() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("playbooks");
        let (engine, report) =
            PlaybookEngine::load_dir_report(&dir).expect("a bad file must not abort the load");
        report.log(&dir);

        let yaml_files = fs::read_dir(&dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
            .count();
        // Every YAML file is either loaded or skipped with a reason. None vanish.
        assert_eq!(report.loaded.len() + report.skipped.len(), yaml_files);
        assert!(report.skipped.iter().all(|(_, why)| !why.is_empty()));
        // The playbook folder holds only YAML files, so the loader ignores nothing.
        assert!(report.ignored.is_empty(), "ignored: {:?}", report.ignored);

        let mut loaded = report.loaded.clone();
        loaded.sort();
        assert_eq!(loaded, SHIPPED_LOADABLE);
        assert_eq!(engine.playbooks.len(), SHIPPED_LOADABLE.len());
        // Every key and slot of the shipped set is read by the engine.
        assert!(report.warnings.is_empty(), "warnings: {:?}", report.warnings);
    }

    #[test]
    fn bad_files_are_skipped_and_the_rest_load() {
        let d = temp_dir("bad");
        fs::write(d.join("a_good.yaml"), GOOD).unwrap();
        fs::write(d.join("b_broken.yaml"), "id: b\nname: Broken: colon\n").unwrap();
        fs::write(
            d.join("c_steps.yaml"),
            "id: c\nname: c\nseverity: low\nwindow_sec: 1\ncooldown_sec: 1\nsteps: []\n",
        )
        .unwrap();
        fs::write(
            d.join("d_netflow.yaml"),
            GOOD.replace("pb_good", "pb_d").replace("fact: Exec", "fact: K8sEvent"),
        )
        .unwrap();
        fs::write(d.join("e_dup.yaml"), GOOD).unwrap();
        fs::write(d.join("notes.txt"), "not a playbook").unwrap();

        let (engine, report) =
            PlaybookEngine::load_dir_report(&d).expect("bad files must not abort the load");
        assert_eq!(report.loaded, vec!["pb_good".to_string()]);
        assert_eq!(engine.playbooks.len(), 1);
        assert_eq!(report.ignored, vec!["notes.txt".to_string()]);

        let want = [
            ("b_broken.yaml", "mapping values are not allowed"),
            ("c_steps.yaml", "no required slots (it uses keys this engine does not read: steps)"),
            ("d_netflow.yaml", "fact kind `K8sEvent`"),
            ("e_dup.yaml", "duplicate playbook id `pb_good`"),
        ];
        assert_eq!(report.skipped.len(), want.len());
        for ((file, why), (want_file, want_why)) in report.skipped.iter().zip(want) {
            assert_eq!(file, want_file);
            assert!(why.contains(want_why), "{file}: {why}");
        }
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn dead_optional_and_forbidden_slots_are_warnings() {
        let d = temp_dir("warn");
        let pb = format!(
            "{GOOD}optional:
  - slot_id: gpu
    slot_ttl_sec: 60
    fact: GpuPerf
forbidden:
  - slot_id: allow
    slot_ttl_sec: 60
    fact: Exec
    where:
      - {{ field: exe, op: nope, value: [a] }}
stages: []
"
        );
        fs::write(d.join("w.yaml"), pb).unwrap();
        let (_, report) = PlaybookEngine::load_dir_report(&d).unwrap();
        assert_eq!(report.loaded, vec!["pb_good".to_string()]);
        let warnings: Vec<&str> = report.warnings.iter().map(|(_, w)| w.as_str()).collect();
        assert!(warnings[0].starts_with("optional slot `gpu` can never match"), "{warnings:?}");
        assert!(warnings[1].starts_with("forbidden slot `allow` can never match"), "{warnings:?}");
        assert!(warnings[2].contains("stages"), "{warnings:?}");
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn loaded_playbook_fires_on_a_matching_fact() {
        let d = temp_dir("fire");
        fs::write(d.join("a.yaml"), GOOD).unwrap();
        let mut eng = PlaybookEngine::load_dir(&d).unwrap();
        assert!(eng.ingest_fact(exec_fact("/usr/bin/ls")).is_empty());
        let hits = eng.ingest_fact(exec_fact("/usr/bin/xmrig"));
        assert_eq!(hits.len(), 1, "required slot filled");
        assert_eq!(hits[0].playbook_id, "pb_good");
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn reload_into_swaps_the_set_and_keeps_it_on_error() {
        let d = temp_dir("reload");
        fs::write(d.join("a.yaml"), GOOD).unwrap();
        let engine = std::sync::Mutex::new(PlaybookEngine::load_dir(&d).unwrap());
        assert_eq!(engine.lock().unwrap().playbooks.len(), 1);

        fs::write(d.join("b.yaml"), GOOD.replace("pb_good", "pb_second")).unwrap();
        let report = reload_into(&engine, &d).unwrap();
        assert_eq!(report.loaded.len(), 2);
        assert!(engine.lock().unwrap().playbooks.contains_key("pb_second"));

        // An unreadable directory is an error and the current set stays.
        assert!(reload_into(&engine, &d.join("missing")).is_err());
        assert_eq!(engine.lock().unwrap().playbooks.len(), 2);
        let _ = fs::remove_dir_all(&d);
    }

    // Two required steps 100 s apart at most, a long TTL, and a forbidden step.
    const TWO_STEP: &str = r#"
id: pb_two
name: Two
severity: high
window_sec: 100
cooldown_sec: 1000
required:
  - slot_id: a
    slot_ttl_sec: 1000
    fact: Exec
    where:
      - { field: exe, op: eq, value: /usr/bin/a }
  - slot_id: b
    slot_ttl_sec: 30
    fact: Exec
    where:
      - { field: exe, op: eq, value: /usr/bin/b }
forbidden:
  - slot_id: quiet
    slot_ttl_sec: 50
    fact: Exec
    where:
      - { field: exe, op: eq, value: /usr/bin/maint }
"#;

    fn engine_from(tag: &str, yaml: &str) -> PlaybookEngine {
        let d = temp_dir(tag);
        fs::write(d.join("p.yaml"), yaml).unwrap();
        let (eng, report) = PlaybookEngine::load_dir_report(&d).unwrap();
        assert_eq!(report.loaded.len(), 1, "{:?}", report.skipped);
        let _ = fs::remove_dir_all(&d);
        eng
    }

    fn exec_at(exe: &str, ts: u64) -> Fact {
        let mut f = exec_fact(exe);
        f.ts = ts;
        f
    }

    #[test]
    fn window_is_measured_over_the_evidence() {
        let mut eng = engine_from("span", TWO_STEP);
        let t0 = 1_800_000_000;
        assert!(eng.ingest_fact(exec_at("/usr/bin/a", t0)).is_empty());
        // b 150 s after a: outside the 100 s window.
        assert!(eng.ingest_fact(exec_at("/usr/bin/b", t0 + 150)).is_empty());
        // a again: a and b are now 20 s apart. The old engine timed the window from the
        // first hit of the hypothesis (t0), so this chain could never complete.
        let hits = eng.ingest_fact(exec_at("/usr/bin/a", t0 + 170));
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].ts, t0 + 170);
        assert_eq!(hits[0].filled_slots, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn tick_expires_hits_without_new_events() {
        let mut eng = engine_from("tick", TWO_STEP);
        let t0 = 1_800_000_000;
        assert!(eng.ingest_fact(exec_at("/usr/bin/b", t0)).is_empty());
        assert_eq!(eng.hypotheses.len(), 1);
        // b has a 30 s TTL. With no event in between, tick drops it and the empty hypothesis.
        eng.tick(t0 + 31);
        assert!(eng.hypotheses.is_empty());
        assert!(eng.ingest_fact(exec_at("/usr/bin/a", t0 + 40)).is_empty());
        // The TTL is also checked at completion, so this holds without tick too.
        let mut eng = engine_from("tick2", TWO_STEP);
        eng.ingest_fact(exec_at("/usr/bin/b", t0));
        assert!(eng.ingest_fact(exec_at("/usr/bin/a", t0 + 40)).is_empty());
    }

    #[test]
    fn live_forbidden_hit_vetoes_until_it_expires() {
        let mut eng = engine_from("veto", TWO_STEP);
        let t0 = 1_800_000_000;
        eng.ingest_fact(exec_at("/usr/bin/maint", t0));
        eng.ingest_fact(exec_at("/usr/bin/a", t0 + 1));
        assert!(eng.ingest_fact(exec_at("/usr/bin/b", t0 + 2)).is_empty(), "vetoed");
        // The forbidden hit has a 50 s TTL. After it, the same chain fires.
        eng.ingest_fact(exec_at("/usr/bin/a", t0 + 60));
        assert_eq!(eng.ingest_fact(exec_at("/usr/bin/b", t0 + 61)).len(), 1);
    }

    #[test]
    fn cooldown_holds_and_survives_reload() {
        let d = temp_dir("cool");
        fs::write(d.join("p.yaml"), TWO_STEP).unwrap();
        let engine = std::sync::Mutex::new(PlaybookEngine::load_dir(&d).unwrap());
        let t0 = 1_800_000_000;
        let run = |t: u64| {
            let mut eng = engine.lock().unwrap();
            eng.ingest_fact(exec_at("/usr/bin/a", t));
            eng.ingest_fact(exec_at("/usr/bin/b", t + 1)).len()
        };
        assert_eq!(run(t0), 1);
        assert_eq!(run(t0 + 500), 0, "inside the 1000 s cooldown");
        reload_into(&engine, &d).unwrap();
        assert_eq!(run(t0 + 600), 0, "a reload keeps the cooldown");
        assert_eq!(run(t0 + 1001), 1, "cooldown over");
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn tags_any_matches_any_tag() {
        let y = GOOD.replace(
            r#"- { field: exe, op: regex, value: "xmrig$" }"#,
            "- { field: tags_any, op: in, value: [RWX_MAP, MEMFD_EXEC] }",
        );
        let mut eng = engine_from("tags", &y);
        let mut f = exec_fact("/memfd:x (deleted)");
        f.fields.insert("tags".into(), "memory::w_then_x".into());
        assert!(eng.ingest_fact(f.clone()).is_empty());
        f.fields.insert("tags".into(), "memory::memfd_exec,MEMFD_EXEC".into());
        assert_eq!(eng.ingest_fact(f).len(), 1);
    }

    #[test]
    fn counting_slots_are_validated_at_load() {
        let slot = |fact: &str, preds: &str| {
            format!(
                "id: pb_c\nname: c\nseverity: high\nwindow_sec: 60\ncooldown_sec: 60\nrequired:\n  - slot_id: s\n    slot_ttl_sec: 60\n    fact: {fact}\n    where:\n{preds}"
            )
        };
        let cases = [
            (slot("FileIO", "      - { field: __agg.kind, op: eq, value: mystery }\n"), "unknown aggregate kind `mystery`"),
            (slot("Exec", "      - { field: __agg.kind, op: eq, value: file_burst }\n"), "counts FileIO facts, not Exec"),
            (slot("FileIO", "      - { field: __agg.kind, op: eq, value: errno_spike }\n      - { field: __agg.files, op: gte, value: 3 }\n"), "has no field `__agg.files`"),
            (slot("FileIO", "      - { field: __agg.kind, op: ne, value: file_burst }\n"), "must use op eq"),
            (slot("Agg", "      - { field: __agg.kind, op: eq, value: file_burst }\n"), "fact kind `Agg`"),
        ];
        for (i, (yaml, want)) in cases.iter().enumerate() {
            let d = temp_dir(&format!("agg{i}"));
            fs::write(d.join("p.yaml"), yaml).unwrap();
            let (_, report) = PlaybookEngine::load_dir_report(&d).unwrap();
            assert!(report.loaded.is_empty(), "{yaml}");
            assert!(report.skipped[0].1.contains(want), "{}", report.skipped[0].1);
            let _ = fs::remove_dir_all(&d);
        }
    }

    fn file_fact(op: &str, path: &str, ts: u64) -> Fact {
        let mut fields = HashMap::new();
        fields.insert("op".to_string(), op.to_string());
        fields.insert("path".to_string(), path.to_string());
        fields.insert("topdir".to_string(), topdir_of(path));
        Fact {
            ts,
            kind: FactKind::FileIO,
            host: "h".into(),
            user: None,
            exe: None,
            cmd: None,
            fields,
        }
    }

    #[test]
    fn file_burst_counts_distinct_files_in_the_window() {
        let y = r#"
id: pb_burst
name: Burst
severity: high
window_sec: 600
cooldown_sec: 600
required:
  - slot_id: burst
    slot_ttl_sec: 600
    fact: FileIO
    where:
      - { field: op, op: eq, value: write }
      - { field: topdir, op: in, value: [/home, /srv] }
      - { field: __agg.kind, op: eq, value: file_burst }
      - { field: __agg.files, op: gte, value: 3 }
      - { field: __agg.rename_ratio, op: lte, value: 0.5 }
      - { field: __agg.window_sec, op: lte, value: 60 }
"#;
        let mut eng = engine_from("burst", y);
        let t0 = 1_800_000_000;
        // The same file three times is one file. A file outside /home and /srv does not count.
        for i in 0..3 {
            assert!(eng.ingest_fact(file_fact("write", "/home/a/1", t0 + i)).is_empty());
        }
        assert!(eng.ingest_fact(file_fact("write", "/etc/x", t0 + 3)).is_empty());
        assert!(eng.ingest_fact(file_fact("write", "/srv/2", t0 + 4)).is_empty());
        // A plain open is not a change.
        assert!(eng.ingest_fact(file_fact("open", "/home/a/9", t0 + 5)).is_empty());
        let hits = eng.ingest_fact(file_fact("write", "/home/b/3", t0 + 6));
        assert_eq!(hits.len(), 1);
        let f = &hits[0].slots["burst"].fields;
        assert_eq!(f["__agg.files"], "3");
        assert_eq!(f["__agg.distinct_topdirs"], "2");
        assert_eq!(f["__agg.window_sec"], "60");
        assert_eq!(f["__agg.rename_ratio"], "0.000");

        // Renames in the same place count toward the ratio even though the slot asks
        // for writes: 3 renames of 6 changed paths is 0.5, a fourth file pushes it over.
        let mut eng = engine_from("burst2", y);
        for (i, p) in ["/home/r1", "/home/r2", "/home/r3"].iter().enumerate() {
            eng.ingest_fact(file_fact("rename", p, t0 + i as u64));
        }
        eng.ingest_fact(file_fact("write", "/home/w1", t0 + 4));
        eng.ingest_fact(file_fact("write", "/home/w2", t0 + 5));
        assert_eq!(eng.ingest_fact(file_fact("write", "/home/w3", t0 + 6)).len(), 1);
        let mut eng = engine_from("burst3", y);
        for (i, p) in ["/home/r1", "/home/r2", "/home/r3", "/home/r4"].iter().enumerate() {
            eng.ingest_fact(file_fact("rename", p, t0 + i as u64));
        }
        eng.ingest_fact(file_fact("write", "/home/w1", t0 + 5));
        eng.ingest_fact(file_fact("write", "/home/w2", t0 + 6));
        assert!(eng.ingest_fact(file_fact("write", "/home/w3", t0 + 7)).is_empty());

        // Events older than the 60 s window drop out of the count.
        let mut eng = engine_from("burst4", y);
        eng.ingest_fact(file_fact("write", "/home/1", t0));
        eng.ingest_fact(file_fact("write", "/home/2", t0 + 1));
        assert!(eng.ingest_fact(file_fact("write", "/home/3", t0 + 100)).is_empty());
        eng.tick(t0 + 200);
        assert!(eng.windows.is_empty(), "tick drops empty windows");
    }

    #[test]
    fn top_and_parent_dirs() {
        assert_eq!(topdir_of("/home/alice/doc.txt"), "/home");
        assert_eq!(topdir_of("/var/www/html/index.php"), "/var/www");
        assert_eq!(topdir_of("/run/user/1000/x"), "/run/user");
        assert_eq!(topdir_of("/usr/local/bin/x"), "/usr/local");
        assert_eq!(topdir_of("/srv"), "/srv");
        assert_eq!(topdir_of("/var"), "/var");
        assert_eq!(topdir_of("relative/x"), "-");
        assert_eq!(parent_dir("/home/a/b.txt"), "/home/a");
        assert_eq!(parent_dir("/b.txt"), "/");
    }

    // ---- Operators, any_of and in_slot ----

    fn fact_with(kind: FactKind, fields: &[(&str, &str)], ts: u64) -> Fact {
        Fact {
            ts,
            kind,
            host: "h".into(),
            user: None,
            exe: None,
            cmd: None,
            fields: fields.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        }
    }

    fn one_slot(fact: &str, preds: &str) -> String {
        format!(
            "id: pb_o\nname: o\nseverity: high\nwindow_sec: 600\ncooldown_sec: 600\nrequired:\n  - slot_id: s\n    slot_ttl_sec: 600\n    fact: {fact}\n    where:\n{preds}"
        )
    }

    #[test]
    fn every_operator_matches_what_it_says() {
        let cases: &[(&str, &[(&str, &str)], bool)] = &[
            ("{ field: x, op: neq, value: a }", &[("x", "b")], true),
            ("{ field: x, op: not_in, value: [a, b] }", &[("x", "c")], true),
            ("{ field: x, op: not_in, value: [a, b] }", &[("x", "a")], false),
            ("{ field: x, op: not_contains, value: bot }", &[("x", "alice")], true),
            ("{ field: x, op: contains_any, value: [sync, copy] }", &[("x", "rclone copy a b")], true),
            ("{ field: x, op: contains_any, value: [sync, copy] }", &[("x", "rclone lsd")], false),
            ("{ field: x, op: any_in, value: [PUT, DELETE] }", &[("x", "GET,DELETE")], true),
            ("{ field: x, op: any_in, value: [PUT, DELETE] }", &[("x", "GET,HEAD")], false),
            ("{ field: ip, op: in_cidr, value: [10.0.0.0/8, 192.168.0.0/16] }", &[("ip", "10.9.8.7")], true),
            ("{ field: ip, op: in_cidr, value: [10.0.0.0/8] }", &[("ip", "11.0.0.1")], false),
            ("{ field: ip, op: not_in_cidr, value: [10.0.0.0/8, 'fc00::/7'] }", &[("ip", "203.0.113.4")], true),
            ("{ field: ip, op: in_cidr, value: ['fc00::/7'] }", &[("ip", "fd12::1")], true),
            ("{ field: ip, op: in_cidr, value: [10.0.0.0/8] }", &[("ip", "not-an-ip")], false),
            ("{ field: x, op: exists, value: true }", &[("x", "1")], true),
            ("{ field: x, op: exists, value: false }", &[("y", "1")], true),
            ("{ field: x, op: exists, value: true }", &[("x", "")], false),
            ("{ field: n, op: gt, value: 5 }", &[("n", "6")], true),
            ("{ field: n, op: lt, value: 5 }", &[("n", "5")], false),
            ("{ field: x, op: not_regex, value: '^/usr/' }", &[("x", "/tmp/a")], true),
            // A missing field fails every op except exists false.
            ("{ field: x, op: not_in, value: [a] }", &[("y", "1")], false),
            ("{ field: x, op: ne, value: a }", &[("y", "1")], false),
        ];
        for (i, (pred, fields, want)) in cases.iter().enumerate() {
            let mut eng = engine_from(&format!("op{i}"), &one_slot("Exec", &format!("      - {pred}\n")));
            let hits = eng.ingest_fact(fact_with(FactKind::Exec, fields, 100));
            assert_eq!(!hits.is_empty(), *want, "{pred} on {fields:?}");
        }
    }

    #[test]
    fn any_of_holds_when_one_branch_does() {
        let y = one_slot(
            "Exec",
            "      - any_of:\n          - { field: cwd, op: regex, value: '^/dev/shm' }\n          - { field: cmd, op: contains, value: /dev/shm/ }\n      - { field: exe_name, op: eq, value: tar }\n",
        );
        let mut eng = engine_from("anyof", &y);
        let mut f = fact_with(FactKind::Exec, &[("exe_name", "tar"), ("cwd", "/root")], 100);
        f.cmd = Some("tar -cf /dev/shm/a.tar /etc".into());
        assert_eq!(eng.ingest_fact(f.clone()).len(), 1, "second branch");
        let mut eng = engine_from("anyof2", &y);
        f.cmd = Some("tar -cf /root/a.tar /etc".into());
        assert!(eng.ingest_fact(f).is_empty(), "neither branch");
    }

    const JOINED: &str = r#"
id: pb_join
name: Join
severity: high
window_sec: 600
cooldown_sec: 600
required:
  - slot_id: parent
    slot_ttl_sec: 600
    fact: Exec
    where:
      - { field: exe_name, op: eq, value: nginx }
  - slot_id: child
    slot_ttl_sec: 600
    fact: Exec
    where:
      - { field: ppid, op: in_slot, value: parent.pid }
      - { field: exe_name, op: eq, value: sh }
  - slot_id: conn
    slot_ttl_sec: 600
    fact: NetFlow
    where:
      - { field: pid, op: in_slot, value: child }
"#;

    #[test]
    fn in_slot_ties_steps_to_one_process() {
        let exec = |pid: &str, ppid: &str, name: &str, ts: u64| {
            fact_with(FactKind::Exec, &[("pid", pid), ("ppid", ppid), ("exe_name", name)], ts)
        };
        let mut eng = engine_from("join", JOINED);
        eng.ingest_fact(exec("10", "1", "nginx", 100));
        // A shell whose parent is not the nginx hit does not fill the child slot.
        eng.ingest_fact(exec("21", "99", "sh", 101));
        assert!(eng.ingest_fact(fact_with(FactKind::NetFlow, &[("pid", "21")], 102)).is_empty());
        eng.ingest_fact(exec("22", "10", "sh", 103));
        // A connection from another pid does not either.
        assert!(eng.ingest_fact(fact_with(FactKind::NetFlow, &[("pid", "23")], 104)).is_empty());
        let hits = eng.ingest_fact(fact_with(FactKind::NetFlow, &[("pid", "22")], 105));
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].slots["child"].fields["pid"], "22");
    }

    #[test]
    fn new_conditions_are_validated_at_load() {
        let cases = [
            (one_slot("Exec", "      - { field: pid, op: in_slot, value: nowhere }\n"), "names unknown slot `nowhere`"),
            (one_slot("Exec", "      - { field: pid, op: in_slot, value: s.pid }\n"), "names its own slot"),
            (one_slot("NetFlow", "      - { field: ip, op: in_cidr, value: [10.0.0.0/33] }\n"), "bad networks: 10.0.0.0/33"),
            (one_slot("Exec", "      - { field: x, op: exists, value: yes }\n"), "wrong value type"),
            (one_slot("Exec", "      - { field: x, op: eq, value: a, any_of: [{ field: y, op: eq, value: b }] }\n"), "cannot also have a field"),
            (one_slot("Exec", "      - { field: x, op: eq, valeu: a }\n"), "unknown field `valeu`"),
            (one_slot("NetConn", "      - { field: x, op: eq, value: a }\n"), "use NetFlow"),
            (one_slot("NetFlow", "      - { field: __agg.bytes_out, op: gte, value: 1 }\n"), "no `__agg.kind`"),
            (one_slot("NetFlow", "      - { field: __agg.kind, op: eq, value: sustained }\n"), "counts HostPerf facts, not NetFlow"),
        ];
        for (i, (yaml, want)) in cases.iter().enumerate() {
            let d = temp_dir(&format!("val{i}"));
            fs::write(d.join("p.yaml"), yaml).unwrap();
            let (_, report) = PlaybookEngine::load_dir_report(&d).unwrap();
            assert!(report.loaded.is_empty(), "{yaml}");
            assert!(report.skipped[0].1.contains(want), "{}: {}", want, report.skipped[0].1);
            let _ = fs::remove_dir_all(&d);
        }
    }

    // ---- The new counting kinds ----

    fn counting(fact: &str, preds: &str) -> PlaybookEngine {
        // Tests run in parallel, so every engine gets its own temp folder.
        static N: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        engine_from(&format!("cnt{n}"), &one_slot(fact, preds))
    }

    #[test]
    fn egress_volume_sums_sends_and_counts_destinations() {
        let mut eng = counting(
            "NetFlow",
            "      - { field: __agg.kind, op: eq, value: egress_volume }\n      - { field: __agg.bytes_out, op: gte, value: 1000 }\n      - { field: __agg.distinct_dsts, op: gte, value: 2 }\n      - { field: __agg.window_sec, op: lte, value: 60 }\n",
        );
        let send = |dst: &str, b: &str, ts: u64| fact_with(FactKind::NetFlow, &[("op", "send"), ("dst_ip", dst), ("bytes", b)], ts);
        assert!(eng.ingest_fact(send("a", "600", 100)).is_empty());
        // A connect is not a send and adds no bytes.
        assert!(eng.ingest_fact(fact_with(FactKind::NetFlow, &[("op", "connect"), ("dst_ip", "b")], 101)).is_empty());
        assert!(eng.ingest_fact(send("a", "600", 102)).is_empty(), "one destination");
        let hits = eng.ingest_fact(send("b", "1", 103));
        assert_eq!(hits.len(), 1);
        let f = &hits[0].slots["s"].fields;
        assert_eq!(f["__agg.bytes_out"], "1201");
        assert_eq!(f["__agg.sends"], "3");
        assert_eq!(f["__agg.bytes_out_mb"], "0.0");
    }

    #[test]
    fn conn_burst_rate_is_over_the_seconds_the_burst_spans() {
        let preds = "      - { field: __agg.kind, op: eq, value: conn_burst }\n      - { field: __agg.conn_rate_eps, op: gte, value: 100 }\n      - { field: __agg.window_sec, op: lte, value: 60 }\n";
        let conn = |ts: u64| fact_with(FactKind::NetFlow, &[("op", "connect"), ("dst_ip", "x"), ("dst_port", "80")], ts);
        let mut eng = counting("NetFlow", preds);
        let fired: usize = (0..100).map(|_| eng.ingest_fact(conn(100)).len()).sum();
        assert_eq!(fired, 1, "100 connects inside one second");
        let mut eng = counting("NetFlow", preds);
        let fired: usize = (0..100).map(|i| eng.ingest_fact(conn(100 + i / 50)).len()).sum();
        assert_eq!(fired, 0, "100 connects over two seconds is 50 a second");
    }

    #[test]
    fn sustained_needs_an_unbroken_run_per_process() {
        let preds = "      - { field: cpu_pct, op: gte, value: 85 }\n      - { field: __agg.kind, op: eq, value: sustained }\n      - { field: __agg.duration_sec, op: gte, value: 60 }\n";
        let sample = |pid: &str, cpu: &str, ts: u64| fact_with(FactKind::HostPerf, &[("pid", pid), ("cpu_pct", cpu)], ts);
        let mut eng = counting("HostPerf", preds);
        let mut fired = 0;
        for i in 0..12 {
            // Two processes interleaved: each has its own run.
            fired += eng.ingest_fact(sample("1", "99", 100 + i * 5)).len();
            fired += eng.ingest_fact(sample("2", "99", 100 + i * 5)).len();
        }
        assert_eq!(fired, 0, "55 s each");
        assert_eq!(eng.ingest_fact(sample("1", "99", 160)).len(), 1, "60 s for pid 1");
        // A sample under the bar does not count, so the gap after it ends the run.
        let mut eng = counting("HostPerf", preds);
        for i in 0..6 {
            eng.ingest_fact(sample("3", "99", 100 + i * 5));
        }
        // One missed sample is tolerated (a 15 s gap), three are not.
        for ts in [130, 135, 140] {
            eng.ingest_fact(sample("3", "10", ts));
        }
        let later: usize = (0..7).map(|i| eng.ingest_fact(sample("3", "99", 145 + i * 5)).len()).sum();
        assert_eq!(later, 0, "the run restarted at 145 s");
    }

    #[test]
    fn auth_fail_burst_and_file_read_burst_count() {
        let mut eng = counting(
            "Auth",
            "      - { field: result, op: eq, value: failure }\n      - { field: __agg.kind, op: eq, value: auth_fail_burst }\n      - { field: __agg.failures, op: gte, value: 3 }\n      - { field: __agg.distinct_users, op: gte, value: 2 }\n",
        );
        let auth = |r: &str, user: &str, ts: u64| {
            let mut f = fact_with(FactKind::Auth, &[("result", r), ("src_ip", "203.0.113.9")], ts);
            f.user = Some(user.into());
            f
        };
        assert!(eng.ingest_fact(auth("failure", "a", 100)).is_empty());
        assert!(eng.ingest_fact(auth("success", "b", 101)).is_empty(), "a success is not counted");
        assert!(eng.ingest_fact(auth("failure", "a", 102)).is_empty(), "one user");
        assert_eq!(eng.ingest_fact(auth("failure", "b", 103)).len(), 1);

        let mut eng = counting(
            "FileIO",
            "      - { field: __agg.kind, op: eq, value: file_read_burst }\n      - { field: __agg.files, op: gte, value: 3 }\n",
        );
        let open = |p: &str, op: &str| fact_with(FactKind::FileIO, &[("op", op), ("path", p)], 100);
        assert!(eng.ingest_fact(open("/etc/a", "open")).is_empty());
        assert!(eng.ingest_fact(open("/etc/a", "open")).is_empty(), "same file twice");
        assert!(eng.ingest_fact(open("/etc/b", "write")).is_empty(), "a write is not a read");
        assert!(eng.ingest_fact(open("/etc/b", "open")).is_empty());
        assert_eq!(eng.ingest_fact(open("/etc/c", "open")).len(), 1);
    }

    #[test]
    fn steps_template_fills_fields_and_drops_missing_slots() {
        let mut hits = HashMap::new();
        let mut f = fact_with(FactKind::Exec, &[("__agg.files", "510")], 1);
        f.exe = Some("/usr/bin/openssl".into());
        hits.insert("crypto".to_string(), SlotHit { ts: 1, tags: vec![], fact: f });
        assert_eq!(render_step("Tool {{crypto.exe}}, {{crypto.__agg.files}} files.", &hits), "Tool /usr/bin/openssl, 510 files.");
        assert_eq!(render_step("{{#if notes}}Notes in {{notes.dir}}.{{/if}}", &hits), "");
        assert_eq!(render_step("{{#if crypto}}Seen.{{/if}} {{missing.x}}", &hits), "Seen. ?");
    }

    // Two required steps and one optional step, with links between them, split per user.
    const LINKED: &str = r#"
id: pb_linked
name: Linked
severity: high
window_sec: 1000
cooldown_sec: 1000
context_keys: [host, user]
emit_ground_truth: true
required:
  - slot_id: wipe
    slot_ttl_sec: 1000
    fact: Exec
    where:
      - { field: exe, op: eq, value: /usr/bin/wipe }
  - slot_id: burst
    slot_ttl_sec: 1000
    fact: Exec
    where:
      - { field: exe, op: eq, value: /usr/bin/burst }
optional:
  - slot_id: tool
    slot_ttl_sec: 1000
    fact: Exec
    where:
      - { field: exe, op: eq, value: /usr/bin/tool }
links:
  - { from: wipe, to: burst, requires: [same_host, same_user], follows_within_sec: 100 }
  - { from: tool, to: burst, within_sec: 50 }
"#;

    fn user_exec(exe: &str, user: &str, ts: u64) -> Fact {
        let mut f = exec_at(exe, ts);
        f.user = Some(user.into());
        f
    }

    #[test]
    fn follows_within_sec_needs_the_order_and_the_gap() {
        let t0 = 1_800_000_000;
        // Wrong order: the burst before the wipe does not complete the chain.
        let mut eng = engine_from("link_order", LINKED);
        eng.ingest_fact(user_exec("/usr/bin/burst", "u1", t0));
        assert!(eng.ingest_fact(user_exec("/usr/bin/wipe", "u1", t0 + 10)).is_empty());
        // A later burst hit is in order, so the chain completes.
        let hits = eng.ingest_fact(user_exec("/usr/bin/burst", "u1", t0 + 20));
        assert_eq!(hits.len(), 1);
        assert!(hits[0].rationale[0].contains("every link between them held"));

        // Right order but too far apart.
        let mut eng = engine_from("link_gap", LINKED);
        eng.ingest_fact(user_exec("/usr/bin/wipe", "u1", t0));
        assert!(eng.ingest_fact(user_exec("/usr/bin/burst", "u1", t0 + 101)).is_empty());
        // The same gap at 100 s holds.
        let mut eng = engine_from("link_gap2", LINKED);
        eng.ingest_fact(user_exec("/usr/bin/wipe", "u1", t0));
        assert_eq!(eng.ingest_fact(user_exec("/usr/bin/burst", "u1", t0 + 100)).len(), 1);
    }

    #[test]
    fn a_broken_link_to_an_optional_slot_leaves_that_hit_out() {
        let t0 = 1_800_000_000;
        let mut eng = engine_from("link_opt", LINKED);
        // tool is 80 s from the burst, past its 50 s link. The chain fires without it.
        eng.ingest_fact(user_exec("/usr/bin/tool", "u1", t0));
        eng.ingest_fact(user_exec("/usr/bin/wipe", "u1", t0 + 70));
        let hits = eng.ingest_fact(user_exec("/usr/bin/burst", "u1", t0 + 80));
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].filled_slots, vec!["burst".to_string(), "wipe".to_string()]);
        assert_eq!(hits[0].missing_optional, vec!["tool".to_string()]);
        assert_eq!(hits[0].left_out.len(), 1);
        assert!(hits[0].rationale.iter().any(|l| l.starts_with("left out tool")));

        // within_sec has no order: a tool hit after the burst still counts.
        let mut eng = engine_from("link_opt2", LINKED);
        eng.ingest_fact(user_exec("/usr/bin/wipe", "u1", t0));
        eng.ingest_fact(user_exec("/usr/bin/tool", "u1", t0 + 5));
        let hits = eng.ingest_fact(user_exec("/usr/bin/burst", "u1", t0 + 6));
        assert_eq!(hits[0].filled_slots.len(), 3);
        assert!(hits[0].left_out.is_empty());
    }

    #[test]
    fn context_keys_keep_each_users_chain_apart() {
        let t0 = 1_800_000_000;
        let mut eng = engine_from("ctx", LINKED);
        // One user wipes, another bursts: two chains, neither complete.
        eng.ingest_fact(user_exec("/usr/bin/wipe", "u1", t0));
        assert!(eng.ingest_fact(user_exec("/usr/bin/burst", "u2", t0 + 1)).is_empty());
        assert_eq!(eng.hypotheses.len(), 2);
        // Each user's own chain fires, and the cooldown is per chain too.
        let hits = eng.ingest_fact(user_exec("/usr/bin/burst", "u1", t0 + 2));
        assert_eq!(hits.len(), 1);
        assert_eq!(
            hits[0].context,
            vec![("host".to_string(), "h".to_string()), ("user".to_string(), "u1".to_string())]
        );
        assert!(hits[0].id().ends_with(":user=u1"));
        assert!(hits[0].ground_truth);
        eng.ingest_fact(user_exec("/usr/bin/wipe", "u2", t0 + 3));
        assert_eq!(eng.ingest_fact(user_exec("/usr/bin/burst", "u2", t0 + 4)).len(), 1);
        // A fact with no user joins the "-" chain, not either user's.
        eng.ingest_fact(exec_at("/usr/bin/wipe", t0 + 5));
        assert!(eng.ingest_fact(user_exec("/usr/bin/burst", "u3", t0 + 6)).is_empty());

        // Without context_keys the chain is per host, so two users' steps join.
        let mut eng = engine_from("ctx_host", &LINKED.replace("context_keys: [host, user]\n", ""));
        eng.ingest_fact(user_exec("/usr/bin/wipe", "u1", t0));
        let hits = eng.ingest_fact(user_exec("/usr/bin/burst", "u2", t0 + 1));
        assert_eq!(hits.len(), 0, "same_user fails on the link");
        let mut eng = engine_from(
            "ctx_host2",
            &LINKED.replace("context_keys: [host, user]\n", "").replace("same_host, same_user", "same_host"),
        );
        eng.ingest_fact(user_exec("/usr/bin/wipe", "u1", t0));
        let hits = eng.ingest_fact(user_exec("/usr/bin/burst", "u2", t0 + 1));
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].id(), format!("pb:pb_linked:h:{}", t0 + 1));
    }

    #[test]
    fn context_keys_split_counting_windows() {
        let yaml = one_slot(
            "FileIO",
            "      - { field: __agg.kind, op: eq, value: file_burst }\n      - { field: __agg.files, op: gte, value: 3 }\n",
        )
        .replace("window_sec:", "context_keys: [host, user]\nwindow_sec:");
        let mut eng = engine_from("ctx_count", &yaml);
        let write = |p: &str, user: &str| {
            let mut f = fact_with(FactKind::FileIO, &[("op", "write"), ("path", p)], 100);
            f.user = Some(user.into());
            f
        };
        eng.ingest_fact(write("/d/a", "u1"));
        eng.ingest_fact(write("/d/b", "u2"));
        assert!(eng.ingest_fact(write("/d/c", "u1")).is_empty(), "two files per user so far");
        assert_eq!(eng.ingest_fact(write("/d/d", "u1")).len(), 1);
    }

    #[test]
    fn links_and_context_keys_are_validated_at_load() {
        let d = temp_dir("link_bad");
        let cases = [
            ("unknown slot", "{ from: wipe, to: nope, within_sec: 5 }"),
            ("forbidden slot", "{ from: wipe, to: quiet, within_sec: 5 }"),
            ("not both", "{ from: wipe, to: burst, within_sec: 5, follows_within_sec: 5 }"),
            ("follows_within_sec or within_sec", "{ from: wipe, to: burst }"),
            ("unknown requirement", "{ from: wipe, to: burst, within_sec: 5, requires: [same_moon] }"),
            ("two different slots", "{ from: wipe, to: wipe, within_sec: 5 }"),
            ("unknown field", "{ from: wipe, to: burst, within_sec: 5, after: 3 }"),
        ];
        for (i, (_, link)) in cases.iter().enumerate() {
            let yaml = LINKED
                .replace("  - { from: tool, to: burst, within_sec: 50 }\n", &format!("  - {link}\n"))
                .replace("optional:", "forbidden:\n  - slot_id: quiet\n    slot_ttl_sec: 5\n    fact: Exec\noptional:");
            fs::write(d.join(format!("p{i}.yaml")), yaml).unwrap();
        }
        fs::write(d.join("z.yaml"), LINKED.replace("[host, user]", "[host, \"a b\"]")).unwrap();
        let (_, report) = PlaybookEngine::load_dir_report(&d).unwrap();
        assert!(report.loaded.is_empty(), "{:?}", report.loaded);
        assert_eq!(report.skipped.len(), cases.len() + 1);
        for (i, (want, _)) in cases.iter().enumerate() {
            let why = &report.skipped[i].1;
            assert!(why.contains(want), "{want}: {why}");
        }
        assert!(report.skipped[cases.len()].1.contains("context key"));
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn shipped_fileburst_crypto_takes_either_order() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("playbooks");
        let (engine, _) = PlaybookEngine::load_dir_report(&dir).unwrap();
        let pb = &engine.playbooks["pb_outage_ransomware_linux_fileburst_crypto"];
        let link = pb
            .links
            .iter()
            .find(|l| [&l.from, &l.to].contains(&&"crypto_exec".to_string()) && [&l.from, &l.to].contains(&&"file_burst".to_string()))
            .unwrap();
        let hit = |ts: u64| SlotHit { ts, tags: vec![], fact: exec_at("/usr/bin/x", ts) };
        assert!(link.holds(&hit(100), &hit(700)), "crypto then burst 600 s later");
        assert!(link.holds(&hit(700), &hit(100)), "burst then crypto 600 s later");
        assert!(!link.holds(&hit(100), &hit(701)));
        // Only one link joins the two slots now.
        let pair = pb
            .links
            .iter()
            .filter(|l| {
                let ends = [l.from.as_str(), l.to.as_str()];
                ends.contains(&"crypto_exec") && ends.contains(&"file_burst")
            })
            .count();
        assert_eq!(pair, 1);
    }
}
