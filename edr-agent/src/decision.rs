// src/decision.rs
// Phase 3 Decision Engine — consensus, cooldown/dedupe, allow/suppress, correlation
// Deps: anyhow = "1", serde = { version = "1", features = ["derive"] }, serde_json = "1", regex = "1"

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DetectorKind {
    #[serde(rename = "rules")]
    Rules,
    #[serde(rename = "krim")]
    Krim,
    #[serde(rename = "mahala")]
    Mahala,
    #[serde(rename = "envelope")]
    Envelope,
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub detector: DetectorKind,
    /// 0.0..=1.0
    pub score: f32,
    /// Unix seconds; if you ingest ms, normalize before calling
    pub ts: u64,
    /// Helpful context for correlation/dedupe/allow-suppress
    #[serde(default)]
    pub exe: Option<String>,
    #[serde(default)]
    pub user: Option<String>,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub cmd_prefix: Option<String>,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub signer: Option<String>,
    #[serde(default)]
    pub hash: Option<String>,
    #[serde(default)]
    pub pid: Option<i32>,
    /// Technique tags, e.g. ["T1059.001", "RWX_MAP"]
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityCutoffs {
    pub low: f32,
    pub medium: f32,
    pub high: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRequirements {
    /// Minimum number of *distinct* detectors by severity
    pub min_detectors_low: usize,
    pub min_detectors_medium: usize,
    pub min_detectors_high: usize,
    /// Require a structural signal (graph/rules with TTP) by severity
    pub require_structural_medium: bool,
    pub require_structural_high: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionPolicy {
    pub weights: HashMap<DetectorKind, f32>,
    pub min_support: f32,
    /// Sliding correlation window in seconds
    pub window_secs: u64,
    /// Cooldowns by severity (seconds)
    pub cooldown_low: u64,
    pub cooldown_medium: u64,
    pub cooldown_high: u64,
    pub severity_cutoffs: SeverityCutoffs,
    pub evidence: EvidenceRequirements,
    /// Additional jump needed to break cooldown early
    pub min_delta_to_break_cooldown: f32,
}

impl Default for DecisionPolicy {
    fn default() -> Self {
        let mut weights = HashMap::new();
        weights.insert(DetectorKind::Rules, 1.0);
        weights.insert(DetectorKind::Krim, 0.8);
        weights.insert(DetectorKind::Mahala, 0.6);
        weights.insert(DetectorKind::Envelope, 0.6);
        Self {
            weights,
            min_support: 1.2,
            window_secs: 60,
            cooldown_low: 600,
            cooldown_medium: 1200,
            cooldown_high: 1800,
            severity_cutoffs: SeverityCutoffs {
                low: 0.35,
                medium: 0.55,
                high: 0.80,
            },
            evidence: EvidenceRequirements {
                min_detectors_low: 1,
                min_detectors_medium: 2,
                min_detectors_high: 3,
                require_structural_medium: true,
                require_structural_high: true,
            },
            min_delta_to_break_cooldown: 0.25,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompiledAllowRule {
    exe: Option<Regex>,
    user: Option<Regex>,
    host: Option<Regex>,
    path: Option<Regex>,
    signer: Option<Regex>,
    parent: Option<Regex>,
    cmd_prefix: Option<Regex>,
    hash: Option<Regex>,
    until: Option<u64>,
    reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowRuleConfig {
    pub exe_regex: Option<String>,
    pub user_regex: Option<String>,
    pub host_regex: Option<String>,
    pub path_regex: Option<String>,
    pub signer_regex: Option<String>,
    pub parent_regex: Option<String>,
    pub cmd_prefix_regex: Option<String>,
    pub hash_regex: Option<String>,
    pub until: Option<u64>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowSuppressFile {
    pub rules: Vec<AllowRuleConfig>,
}

#[derive(Debug, Default, Clone)]
pub struct AllowSuppressList {
    pub rules: Vec<CompiledAllowRule>,
}

impl AllowSuppressList {
    pub fn from_json_str(s: &str) -> anyhow::Result<Self> {
        let parsed: AllowSuppressFile = serde_json::from_str(s)?;
        Ok(Self::from_configs(&parsed.rules))
    }
    pub fn from_configs(cfgs: &[AllowRuleConfig]) -> Self {
        let rules = cfgs
            .iter()
            .map(|c| CompiledAllowRule {
                exe: c.exe_regex.as_ref().and_then(|r| Regex::new(r).ok()),
                user: c.user_regex.as_ref().and_then(|r| Regex::new(r).ok()),
                host: c.host_regex.as_ref().and_then(|r| Regex::new(r).ok()),
                path: c.path_regex.as_ref().and_then(|r| Regex::new(r).ok()),
                signer: c.signer_regex.as_ref().and_then(|r| Regex::new(r).ok()),
                parent: c.parent_regex.as_ref().and_then(|r| Regex::new(r).ok()),
                cmd_prefix: c.cmd_prefix_regex.as_ref().and_then(|r| Regex::new(r).ok()),
                hash: c.hash_regex.as_ref().and_then(|r| Regex::new(r).ok()),
                until: c.until,
                reason: c.reason.clone(),
            })
            .collect();
        Self { rules }
    }
    pub fn is_suppressed(&self, sig: &Signal, now: u64) -> Option<String> {
        for r in &self.rules {
            if let Some(until) = r.until {
                if now > until {
                    continue;
                }
            }
            let exe_ok = r
                .exe
                .as_ref()
                .map(|re| sig.exe.as_deref().map(|s| re.is_match(s)).unwrap_or(false))
                .unwrap_or(true);
            let user_ok = r
                .user
                .as_ref()
                .map(|re| sig.user.as_deref().map(|s| re.is_match(s)).unwrap_or(false))
                .unwrap_or(true);
            let host_ok = r
                .host
                .as_ref()
                .map(|re| sig.host.as_deref().map(|s| re.is_match(s)).unwrap_or(false))
                .unwrap_or(true);
            let path_ok = r
                .path
                .as_ref()
                .map(|re| sig.exe.as_deref().map(|s| re.is_match(s)).unwrap_or(false))
                .unwrap_or(true);
            let signer_ok = r
                .signer
                .as_ref()
                .map(|re| {
                    sig.signer
                        .as_deref()
                        .map(|s| re.is_match(s))
                        .unwrap_or(false)
                })
                .unwrap_or(true);
            let parent_ok = r
                .parent
                .as_ref()
                .map(|re| {
                    sig.parent
                        .as_deref()
                        .map(|s| re.is_match(s))
                        .unwrap_or(false)
                })
                .unwrap_or(true);
            let cmd_ok = r
                .cmd_prefix
                .as_ref()
                .map(|re| {
                    sig.cmd_prefix
                        .as_deref()
                        .map(|s| re.is_match(s))
                        .unwrap_or(false)
                })
                .unwrap_or(true);
            let hash_ok = r
                .hash
                .as_ref()
                .map(|re| sig.hash.as_deref().map(|s| re.is_match(s)).unwrap_or(false))
                .unwrap_or(true);
            if exe_ok
                && user_ok
                && host_ok
                && path_ok
                && signer_ok
                && parent_ok
                && cmd_ok
                && hash_ok
            {
                return Some(
                    r.reason
                        .clone()
                        .unwrap_or_else(|| "allow/suppress match".to_string()),
                );
            }
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IncidentKey {
    exe: String,
    user: String,
    host: String,
}
impl PartialEq for IncidentKey {
    fn eq(&self, o: &Self) -> bool {
        self.exe == o.exe && self.user == o.user && self.host == o.host
    }
}
impl Eq for IncidentKey {}
impl Hash for IncidentKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.exe.hash(state);
        self.user.hash(state);
        self.host.hash(state);
    }
}

#[derive(Debug, Clone)]
struct DedupeKey {
    exe: String,
    user: String,
    cmd_prefix: String,
}
impl PartialEq for DedupeKey {
    fn eq(&self, o: &Self) -> bool {
        self.exe == o.exe && self.user == o.user && self.cmd_prefix == o.cmd_prefix
    }
}
impl Eq for DedupeKey {}
impl Hash for DedupeKey {
    fn hash<H: Hasher>(&self, s: &mut H) {
        self.exe.hash(s);
        self.user.hash(s);
        self.cmd_prefix.hash(s);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DetectorSummary {
    pub kind: DetectorKind,
    pub score: f32,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Low
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FinalAlert {
    pub ts: u64,
    pub incident: IncidentKey,
    pub severity: Severity,
    pub risk: f32,
    pub support: f32,
    pub detectors: Vec<DetectorSummary>,
    pub tags: Vec<String>,
    pub rationale: Vec<String>,
    pub exe: String,
    pub user: String,
    pub host: String,
    pub cmd_prefix: String,
}

#[derive(Default, Debug, Clone)]
struct DedupeState {
    last_ts: u64,
    last_risk: f32,
    last_sev: Severity,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct EngineCounters {
    pub total_signals: u64,
    pub suppressed_allow: u64,
    pub suppressed_cooldown: u64,
    pub incidents_emitted: u64,
}

pub struct DecisionEngine {
    policy: DecisionPolicy,
    allow: AllowSuppressList,
    // Active buckets within the correlation window
    buckets: HashMap<IncidentKey, BTreeMap<u64, Vec<Signal>>>,
    dedupe: HashMap<DedupeKey, DedupeState>,
    pub counters: EngineCounters,
}

impl DecisionEngine {
    pub fn new(policy: DecisionPolicy, allow: AllowSuppressList) -> Self {
        Self {
            policy,
            allow,
            buckets: HashMap::new(),
            dedupe: HashMap::new(),
            counters: EngineCounters::default(),
        }
    }

    pub fn ingest(&mut self, mut sig: Signal) -> Vec<FinalAlert> {
        self.counters.total_signals += 1;
        let now = sig.ts; // stream ts; acceptable for replay

        // Allow/suppress check
        if let Some(_why) = self.allow.is_suppressed(&sig, now) {
            self.counters.suppressed_allow += 1;
            return vec![];
        }

        // Normalize milliseconds to seconds if needed
        if sig.ts > 10_u64.pow(12) {
            sig.ts /= 1000;
        }

        let ik = IncidentKey {
            exe: sig.exe.clone().unwrap_or_else(|| "unknown".into()),
            user: sig.user.clone().unwrap_or_else(|| "unknown".into()),
            host: sig.host.clone().unwrap_or_else(|| "unknown".into()),
        };
        let cmd_pref = sig.cmd_prefix.clone().unwrap_or_else(|| "".into());
        let dk = DedupeKey {
            exe: ik.exe.clone(),
            user: ik.user.clone(),
            cmd_prefix: cmd_pref.clone(),
        };

        let ts_bucket = (sig.ts / self.policy.window_secs) * self.policy.window_secs;

        // Insert into the bucket in its own scope to avoid borrow conflicts.
        {
            let bucket = self.buckets.entry(ik.clone()).or_default();
            bucket.entry(ts_bucket).or_default().push(sig.clone());
        }

        // Evaluate this bucket now using an immutable borrow.
        let mut out = Vec::new();
        if let Some(map) = self.buckets.get(&ik) {
            if let Some(signals) = map.get(&ts_bucket) {
                if let Some(alert) = self.evaluate_bucket(&ik, &cmd_pref, ts_bucket, signals) {
                    // Cooldown/dedupe check
                    let sev_cooldown = match alert.severity {
                        Severity::Low => self.policy.cooldown_low,
                        Severity::Medium => self.policy.cooldown_medium,
                        Severity::High => self.policy.cooldown_high,
                    };
                    let can_break = |prev: &DedupeState, current_risk: f32| {
                        current_risk >= prev.last_risk + self.policy.min_delta_to_break_cooldown
                    };
                    let entry = self.dedupe.entry(dk.clone()).or_default();
                    let on_cooldown = alert.ts <= entry.last_ts + sev_cooldown;
                    if on_cooldown && !can_break(entry, alert.risk) {
                        self.counters.suppressed_cooldown += 1;
                        // Keep the higher watermark
                        if alert.risk > entry.last_risk {
                            entry.last_risk = alert.risk;
                            entry.last_sev = alert.severity;
                        }
                    } else {
                        *entry = DedupeState {
                            last_ts: alert.ts,
                            last_risk: alert.risk,
                            last_sev: alert.severity,
                        };
                        self.counters.incidents_emitted += 1;
                        out.push(alert);
                    }
                }
            }
        }

        // Garbage collect old buckets
        self.gc(now);
        out
    }

    pub fn flush(&mut self) -> Vec<FinalAlert> {
        let now = now_sec();
        let mut out = Vec::new();
        let keys: Vec<_> = self.buckets.keys().cloned().collect();
        for ik in keys {
            if let Some(map) = self.buckets.get(&ik) {
                for (ts_bucket, signals) in map.iter() {
                    if now >= ts_bucket + self.policy.window_secs {
                        // finalize old
                        if let Some(alert) = self.evaluate_bucket(&ik, "", *ts_bucket, signals) {
                            out.push(alert);
                        }
                    }
                }
            }
        }
        self.buckets.clear();
        out
    }

    fn evaluate_bucket(
        &self,
        ik: &IncidentKey,
        cmd_prefix: &str,
        ts_bucket: u64,
        signals: &Vec<Signal>,
    ) -> Option<FinalAlert> {
        // Consensus support
        let mut by_kind: HashMap<DetectorKind, f32> = HashMap::new();
        let mut tags: HashSet<String> = HashSet::new();
        for s in signals {
            *by_kind.entry(s.detector).or_insert(0.0) = by_kind
                .get(&s.detector)
                .cloned()
                .unwrap_or(0.0)
                .max(s.score);
            for t in &s.tags {
                tags.insert(t.clone());
            }
        }
        let mut support = 0.0f32;
        for (k, sc) in &by_kind {
            if let Some(w) = self.policy.weights.get(k) {
                support += w * sc;
            }
        }
        if support < self.policy.min_support {
            return None;
        }

        // Evidence checks
        let distinct = by_kind.len();
        let has_structural =
            by_kind.contains_key(&DetectorKind::Krim) || has_rules_with_ttp(signals);
        let has_critical_ttp = tags.iter().any(|t| is_critical_ttp(t));

        // Risk ~ support clamped
        let risk = support.min(1.5);
        let sev = if distinct >= self.policy.evidence.min_detectors_high
            && has_structural
            && has_critical_ttp
            && risk >= self.policy.severity_cutoffs.high
        {
            Severity::High
        } else if distinct >= self.policy.evidence.min_detectors_medium
            && (!self.policy.evidence.require_structural_medium || has_structural)
            && risk >= self.policy.severity_cutoffs.medium
        {
            Severity::Medium
        } else if distinct >= self.policy.evidence.min_detectors_low
            && risk >= self.policy.severity_cutoffs.low
        {
            Severity::Low
        } else {
            return None;
        };

        let detectors = by_kind
            .into_iter()
            .map(|(k, sc)| DetectorSummary { kind: k, score: sc })
            .collect::<Vec<_>>();
        let mut tag_vec = tags.into_iter().collect::<Vec<_>>();
        tag_vec.sort();
        let rationale = build_rationale(&detectors, &tag_vec, sev, risk);

        Some(FinalAlert {
            ts: ts_bucket + self.policy.window_secs - 1,
            incident: ik.clone(),
            severity: sev,
            risk,
            support,
            detectors,
            tags: tag_vec,
            rationale,
            exe: ik.exe.clone(),
            user: ik.user.clone(),
            host: ik.host.clone(),
            cmd_prefix: cmd_prefix.to_string(),
        })
    }

    fn gc(&mut self, now: u64) {
        let window = self.policy.window_secs;
        self.buckets.retain(|_, map| {
            let mut keep_any = false;
            let keys: Vec<_> = map.keys().cloned().collect();
            for k in keys {
                if now < k + window {
                    keep_any = true;
                } else {
                    map.remove(&k);
                }
            }
            keep_any || !map.is_empty()
        });
    }
}

fn has_rules_with_ttp(signals: &Vec<Signal>) -> bool {
    signals
        .iter()
        .any(|s| s.detector == DetectorKind::Rules && s.tags.iter().any(|t| t.starts_with('T')))
}

fn is_critical_ttp(tag: &str) -> bool {
    tag.starts_with("T1055")
        || tag.starts_with("T1003")
        || tag.starts_with("T1547")
        || tag == "RWX_MAP"
        || tag == "MEMFD_EXEC"
}

fn build_rationale(
    detectors: &Vec<DetectorSummary>,
    tags: &Vec<String>,
    sev: Severity,
    risk: f32,
) -> Vec<String> {
    let kinds: Vec<String> = detectors
        .iter()
        .map(|d| format!("{:?}:{:.2}", d.kind, d.score))
        .collect();
    let mut r = vec![format!(
        "consensus {:?} (risk {:.2}) via {}",
        sev,
        risk,
        kinds.join(", ")
    )];
    if !tags.is_empty() {
        r.push(format!("tags: {}", tags.join(", ")));
    }
    r
}

fn now_sec() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}
