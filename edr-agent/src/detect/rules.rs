use std::fs;
use anyhow::Result;
use regex::Regex as Re;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::detect::{Detector, Finding};
use crate::detect::types::NormalizedAlert;
// keep the rest of the file using NormalizedEvent without refactors
type NormalizedEvent = NormalizedAlert;

#[derive(Debug, Deserialize)]
pub struct RuleSet {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub severity: f32,                    // [0,1]
    #[serde(default)] pub all: Vec<Clause>,   // AND
    #[serde(default)] pub any: Vec<Clause>,   // OR
    #[serde(default)] pub none: Vec<Clause>,  // NOT
}

#[derive(Debug, Deserialize, Clone)]
pub struct Clause {
    pub field: String,
    pub op: Op,
    pub value: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Op {
    Eq, Ne, Gt, Ge, Lt, Le,
    Contains, Startswith, Endswith, Regex
}

pub struct RulesDetector {
    rules: Vec<Rule>,
}

impl RulesDetector {
    pub fn from_file(path: &std::path::Path) -> Result<Self> {
        let s = fs::read_to_string(path)?;
        let rs: RuleSet = serde_json::from_str(&s)?;
        Ok(Self { rules: rs.rules })
    }
}

impl Detector for RulesDetector {
    fn name(&self) -> &'static str { "rules" }

    fn score(&self, e: &NormalizedEvent, _features: &[f64]) -> Vec<Finding> {
        let mut out = Vec::new();
        for r in &self.rules {
            if eval_rule(r, e) {
                out.push(Finding {
                    source: "rules".to_string(),
                    score: r.severity.clamp(0.0, 1.0) as f32,
                    label: Some(r.id.clone()),
                    details: Some(json!({ "name": r.name })),
                });
            }
        }
        out
    }
}

fn eval_rule(r: &Rule, e: &NormalizedEvent) -> bool {
    if !r.all.iter().all(|c| eval_clause(c, e)) { return false; }
    if !r.any.is_empty() && !r.any.iter().any(|c| eval_clause(c, e)) { return false; }
    if r.none.iter().any(|c| eval_clause(c, e)) { return false; }
    true
}

fn eval_clause(c: &Clause, e: &NormalizedEvent) -> bool {
    let mut lookup = |f: &str| -> Option<String> {
        // Preferred fields in the new NormalizedAlert
        // attributes.* / entities.* support dotted paths (and numeric indexes)
        if let Some(rest) = f.strip_prefix("attributes.") {
            return get_path_as_string(&e.attributes, rest);
        }
        if let Some(rest) = f.strip_prefix("entities.") {
            return get_path_as_string(&e.entities, rest);
        }
        // Back-compat alias: attrs.* -> attributes.*
        if let Some(rest) = f.strip_prefix("attrs.") {
            return get_path_as_string(&e.attributes, rest);
        }
        // Very loose back-compat for legacy "tags.foo" → attributes.tags.foo
        if let Some(rest) = f.strip_prefix("tags.") {
            let key = format!("tags.{}", rest);
            return get_path_as_string(&e.attributes, &key);
        }

        // Legacy top-level aliases that used to exist on the old event type:
        // Try to read from attributes first.
        match f {
            "category" => get_path_as_string(&e.attributes, "category"),
            "event"    => get_path_as_string(&e.attributes, "event"),
            "exe"      => get_path_as_string(&e.attributes, "exe"),
            "cmdline"  => get_path_as_string(&e.attributes, "cmdline"),
            "cwd"      => get_path_as_string(&e.attributes, "cwd"),
            // Actual top-level fields on NormalizedAlert
            "id"            => Some(e.id.clone()),
            "source"        => Some(e.source.clone()),
            "connector_id"  => Some(e.connector_id.clone()),
            "title"         => Some(e.title.clone()),
            "short" | "short_why" => Some(e.short_why.clone()),
            "severity"      => Some(format!("{}", e.severity)),
            "ts"            => Some(e.ts.to_rfc3339()),
            "mitre"         => Some(e.mitre.join(",")),
            "attributes"    => Some(e.attributes.to_string()),
            "entities"      => Some(e.entities.to_string()),
            _               => None,
        }
    };

    match c.op {
        Op::Eq         => eq_cmp(&lookup(&c.field), &c.value),
        Op::Ne         => !eq_cmp(&lookup(&c.field), &c.value),
        Op::Gt         => num_cmp(&lookup(&c.field), &c.value, |a,b| a >  b),
        Op::Ge         => num_cmp(&lookup(&c.field), &c.value, |a,b| a >= b),
        Op::Lt         => num_cmp(&lookup(&c.field), &c.value, |a,b| a <  b),
        Op::Le         => num_cmp(&lookup(&c.field), &c.value, |a,b| a <= b),
        Op::Contains   => lookup(&c.field).map(|v| v.contains(as_str(&c.value))).unwrap_or(false),
        Op::Startswith => lookup(&c.field).map(|v| v.starts_with(as_str(&c.value))).unwrap_or(false),
        Op::Endswith   => lookup(&c.field).map(|v| v.ends_with(as_str(&c.value))).unwrap_or(false),
        Op::Regex      => {
            Re::new(as_str(&c.value))
                .ok()
                .and_then(|re| lookup(&c.field).map(|v| re.is_match(&v)))
                .unwrap_or(false)
        }
    }
}

fn as_str(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}

fn eq_cmp(lhs: &Option<String>, rhs: &Value) -> bool {
    match (lhs, rhs) {
        (Some(l), Value::Number(n)) => l.parse::<f64>().ok() == n.as_f64(),
        (Some(l), _)                => l == as_str(rhs),
        _                           => false,
    }
}

fn num_cmp<F: Fn(f64,f64)->bool>(lhs: &Option<String>, rhs: &Value, f: F) -> bool {
    if let (Some(l), Some(r)) = (lhs.as_ref().and_then(|s| s.parse::<f64>().ok()), rhs.as_f64()) {
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
        Value::Bool(b)   => Some(b.to_string()),
        other            => Some(other.to_string()),
    }
}
