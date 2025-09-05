use std::{collections::HashMap, fs};
use anyhow::{anyhow, Result};
use regex::Regex;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::telemetry::Telemetry;          // expects: category/event/exe/cmdline/cwd/pid/ppid/uid/tags/attrs
use super::{Detector, Finding};           // your Detector trait + Finding type

/// JSON schema (subset) this module accepts in detection_rules.json:
/// {
///   "rules": [
///     { "id":"R-ID", "name":"...", "severity":0.8,
///       "all":[{"field":"exe","op":"endswith","value":"rundll32.exe"}],
///       "any":[{"field":"tags.threat","op":"eq","value":"persistence.cron"}],
///       "none":[{"field":"cmdline","op":"regex","value":"(?i)--allowlist"}]
///     }
///   ],
///   "... other top-level keys are ignored ..."
/// }
#[derive(Debug, Deserialize)]
pub struct RuleSet { pub rules: Vec<Rule> }

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub severity: f32,                 // [0,1]
    #[serde(default)] pub all: Vec<Clause>,   // AND
    #[serde(default)] pub any: Vec<Clause>,   // OR
    #[serde(default)] pub none: Vec<Clause>,  // NOT
}

#[derive(Debug, Deserialize, Clone)]
pub struct Clause { pub field: String, pub op: Op, pub value: Value }

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Op { Eq, Ne, Gt, Ge, Lt, Le, Contains, Startswith, Endswith, Regex }

/// Precompiled clause for fast eval (only Regex is compiled)
struct CompiledClause {
    field: String,
    op: Op,
    value: Value,
    regex: Option<Regex>,
}

struct CompiledRule {
    id: String,
    name: String,
    severity: f32,
    all: Vec<CompiledClause>,
    any: Vec<CompiledClause>,
    none: Vec<CompiledClause>,
}

pub struct RulesDetector {
    rules: Vec<CompiledRule>,
}

impl RulesDetector {
    /// Load from file; tolerates superset JSON with other top-level keys.
    pub fn from_file(path: &std::path::Path) -> Result<Self> {
        let s = fs::read_to_string(path)?;
        Self::from_str(&s)
    }

    pub fn from_str(s: &str) -> Result<Self> {
        let v: Value = serde_json::from_str(s)?;
        let raw_rules = match v.get("rules") {
            Some(Value::Array(arr)) => arr,
            Some(_) => return Err(anyhow!("top-level 'rules' must be an array")),
            None => &Vec::<Value>::new(),
        };

        let mut compiled = Vec::with_capacity(raw_rules.len());
        for r in raw_rules {
            let rule: Rule = serde_json::from_value(r.clone())?;
            compiled.push(compile_rule(rule)?);
        }
        Ok(Self { rules: compiled })
    }
}

impl Detector for RulesDetector {
    fn name(&self) -> &'static str { "rules" }

    fn score(&self, t: &Telemetry, _features: &[f64]) -> Vec<Finding> {
        let mut out = Vec::new();
        for r in &self.rules {
            if eval_rule(r, t) {
                out.push(Finding {
                    source: "rules".into(),
                    score: r.severity.clamp(0.0, 1.0) as f32,
                    label: Some(r.id.clone()),
                    details: Some(json!({ "name": r.name })),
                });
            }
        }
        out
    }
}

// ---------- compile & eval ----------

fn compile_rule(r: Rule) -> Result<CompiledRule> {
    Ok(CompiledRule {
        id: r.id,
        name: r.name,
        severity: r.severity,
        all: r.all.into_iter().map(compile_clause).collect::<Result<_>>()?,
        any: r.any.into_iter().map(compile_clause).collect::<Result<_>>()?,
        none: r.none.into_iter().map(compile_clause).collect::<Result<_>>()?,
    })
}

fn compile_clause(c: Clause) -> Result<CompiledClause> {
    let regex = match c.op {
        Op::Regex => Some(Regex::new(as_str(&c.value))?),
        _ => None,
    };
    Ok(CompiledClause { field: c.field, op: c.op, value: c.value, regex })
}

fn eval_rule(r: &CompiledRule, t: &Telemetry) -> bool {
    if !r.all.iter().all(|c| eval_clause(c, t)) { return false; }
    if !r.any.is_empty() && !r.any.iter().any(|c| eval_clause(c, t)) { return false; }
    if r.none.iter().any(|c| eval_clause(c, t)) { return false; }
    true
}

fn eval_clause(c: &CompiledClause, t: &Telemetry) -> bool {
    use Op::*;
    // Look up field value as either numeric or string
    let lv = lookup_field(t, &c.field);

    match c.op {
        Eq => compare_eq(&lv, &c.value),
        Ne => !compare_eq(&lv, &c.value),
        Gt => compare_num(&lv, &c.value, |a,b| a >  b),
        Ge => compare_num(&lv, &c.value, |a,b| a >= b),
        Lt => compare_num(&lv, &c.value, |a,b| a <  b),
        Le => compare_num(&lv, &c.value, |a,b| a <= b),
        Contains   => lv.as_str().map(|s| s.contains(as_str(&c.value))).unwrap_or(false),
        Startswith => lv.as_str().map(|s| s.starts_with(as_str(&c.value))).unwrap_or(false),
        Endswith   => lv.as_str().map(|s| s.ends_with(as_str(&c.value))).unwrap_or(false),
        Regex      => {
            if let (Some(re), Some(s)) = (c.regex.as_ref(), lv.as_str()) { re.is_match(s) } else { false }
        }
    }
}

// ---------- field access & comparisons ----------

/// Value extracted from Telemetry in a type-aware way.
enum LVal {
    Str(String),
    Num(f64),
    None,
}
impl LVal {
    fn as_str(&self) -> Option<&str> { if let LVal::Str(s) = self { Some(s) } else { None } }
    fn as_num(&self) -> Option<f64> { if let LVal::Num(n) = self { Some(*n) } else { None } }
}

fn lookup_field(t: &Telemetry, field: &str) -> LVal {
    // tags.* and attrs.* first
    if let Some(rest) = field.strip_prefix("tags.") {
        if let Some(v) = t.tags.get(rest) { return LVal::Str(v.clone()); }
        return LVal::None;
    }
    if let Some(rest) = field.strip_prefix("attrs.") {
        if let Some(v) = t.attrs.get(rest) {
            // try numeric then string
            if let Some(n) = v.as_f64() { return LVal::Num(n); }
            if let Some(s) = v.as_str() { return LVal::Str(s.to_string()); }
            return LVal::Str(v.to_string());
        }
        return LVal::None;
    }

    // top-level fields commonly present in Telemetry
    match field {
        "category" => LVal::Str(t.category.clone()),
        "event"    => LVal::Str(t.event.clone()),
        "exe"      => t.exe.as_ref().map(|s| LVal::Str(s.clone())).unwrap_or(LVal::None),
        "cmdline"  => t.cmdline.as_ref().map(|s| LVal::Str(s.clone())).unwrap_or(LVal::None),
        "cwd"      => t.cwd.as_ref().map(|s| LVal::Str(s.clone())).unwrap_or(LVal::None),
        "pid"      => t.pid.map(|n| LVal::Num(n as f64)).unwrap_or(LVal::None),
        "ppid"     => t.ppid.map(|n| LVal::Num(n as f64)).unwrap_or(LVal::None),
        "uid"      => t.uid.map(|n| LVal::Num(n as f64)).unwrap_or(LVal::None),
        _          => LVal::None,
    }
}

fn as_str(v: &Value) -> &str { v.as_str().unwrap_or("") }

fn rhs_to_num(v: &Value) -> Option<f64> {
    if let Some(n) = v.as_f64() { return Some(n); }
    if let Some(s) = v.as_str() { return s.parse::<f64>().ok(); }
    None
}

fn compare_eq(lhs: &LVal, rhs: &Value) -> bool {
    // string equality if rhs is string; numeric equality if rhs is number
    if let Some(rs) = rhs.as_str() {
        return lhs.as_str().map(|ls| ls == rs).unwrap_or(false);
    }
    if let Some(rn) = rhs.as_f64() {
        return lhs.as_num().map(|ln| (ln - rn).abs() < std::f64::EPSILON).unwrap_or(false);
    }
    // fallback: stringify rhs
    lhs.as_str().map(|ls| ls == rhs.to_string()).unwrap_or(false)
}

fn compare_num<F: Fn(f64, f64) -> bool>(lhs: &LVal, rhs: &Value, f: F) -> bool {
    match (lhs.as_num(), rhs_to_num(rhs)) {
        (Some(l), Some(r)) => f(l, r),
        _ => false,
    }
}
