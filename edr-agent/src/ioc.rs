// src/ioc.rs
// IOC feed loader + matcher + simple tagger + metrics.
// Public API used by main.rs:
//   - init() / reload()
//   - metrics() -> IocMetrics
//   - tag_records(&mut [TelemetryRecord]) -> Result<Vec<Hit>>
//
// File formats:
//   Directory (default: state/ioc) with .json or .jsonl/.ndjson files containing
//   objects with fields like: {kind, value, id?, family?, confidence?, until?, tags?[]}

use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::telemetry::TelemetryRecord;

// ===== Metrics surface =====

#[derive(Debug, Clone, Copy, Serialize)]
pub struct IocMetrics {
    pub matches_total: u64,
    pub yara_total: u64,
}

static MATCHES_TOTAL: AtomicU64 = AtomicU64::new(0);
static YARA_MATCHES: AtomicU64 = AtomicU64::new(0);

// ===== Public hit type expected by main.rs (for pb_introspect) =====

#[derive(Debug, Clone, Serialize)]
pub struct Hit {
    pub id: String,     // stable id (ioc:<kind>:<value> or configured id)
    pub ts: u64,        // record timestamp
    pub slot: String,   // e.g., "artifact_ioc" or "binary_ioc"
    pub fact: String,   // short class: "Net" | "FileIO" | "Exec"
    pub summary: String,
    pub tags: Vec<String>, // e.g., ["IOC"] or ["IOC","YARA"]
}

// ===== IOC model & store =====

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum IocKind {
    Sha256,
    Ip,
    IpPrefix,
    Domain,
    DomainGlob,
    Url,
    UrlRegex,
    PathRegex,
    CertCnRegex,
}
impl Default for IocKind {
    fn default() -> Self {
        IocKind::Sha256
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IocItem {
    pub id: Option<String>,
    pub kind: IocKind,
    pub value: String,
    #[serde(default)]
    pub family: Option<String>,
    /// 0.0..1.0, default 0.5
    #[serde(default)]
    pub confidence: Option<f32>,
    /// Expiration as unix seconds; if absent, treated as non-expiring
    #[serde(default)]
    pub until: Option<u64>,
    /// Optional tags to attach to any hits produced by this item
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct IocStore {
    pub items: Vec<IocItem>,
    // compiled extractors
    re_url: Regex,
    re_domain: Regex,
    re_ipv4: Regex,
    re_sha256: Regex,
    re_path: Regex,
}

impl Default for IocStore {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            re_url: Regex::new(r#"(?i)\bhttps?://[^\s<>"']{3,}"#).unwrap(),
            re_domain: Regex::new(r#"(?i)\b([a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z]{2,63}\b"#).unwrap(),
            re_ipv4: Regex::new(
                r#"\b(?:(?:25[0-5]|2[0-4]\d|1?\d{1,2})\.){3}(?:25[0-5]|2[0-4]\d|1?\d{1,2})\b"#,
            )
            .unwrap(),
            re_sha256: Regex::new(r#"\b[a-f0-9]{64}\b"#).unwrap(),
            // crude: absolute/tilde paths and obvious 'foo/bar' style
            re_path: Regex::new(
                r#"(?x)
                    (?:
                        (?:/|\~\/) [\w\.\-\+@%/]{2,} |
                        [A-Za-z0-9_\-\.]+ / [\w\.\-\+@%/]{1,}
                    )
                "#,
            )
            .unwrap(),
        }
    }
}

// Global store (lazy)
static STORE: OnceLock<Mutex<IocStore>> = OnceLock::new();
fn store() -> &'static Mutex<IocStore> {
    STORE.get_or_init(|| Mutex::new(IocStore::default()))
}

// ===== Public API used by main.rs =====

pub fn init() -> Result<()> {
    reload()
}

pub fn reload() -> Result<()> {
    // Default directory for IOC feeds; override via IOC_DIR env
    let dir = std::env::var("IOC_DIR").unwrap_or_else(|_| "state/ioc".to_string());
    let new_store = load_all(&dir)?;
    *store().lock().unwrap() = new_store;
    Ok(())
}

pub fn metrics() -> IocMetrics {
    IocMetrics {
        matches_total: MATCHES_TOTAL.load(Ordering::Relaxed),
        yara_total: YARA_MATCHES.load(Ordering::Relaxed),
    }
}

/// Scan & tag a batch of telemetry records; return operator-visible IOC hits.
/// - Adds tags to records (ioc_ip, ioc_domain, ioc_hash, ioc_url, ioc_path, or yara_match:<rule>)
/// - Builds compact Hit objects for pending/hits UI.
pub fn tag_records(records: &mut [TelemetryRecord]) -> Result<Vec<Hit>> {
    let st = store().lock().unwrap();
    let mut out: Vec<Hit> = Vec::new();

    // Dedup (kind,value) within the batch so we don't spam pending
    let mut seen: HashSet<(String, String)> = HashSet::new();

    for rec in records.iter_mut() {
        // 1) YARA sidecar (if someone already attached tags like `yara_match:<rule>`)
        for t in rec.tags.clone() {
            if let Some(rule) = t.strip_prefix("yara_match:") {
                let h = Hit {
                    id: format!("ioc:yara:{}", rule),
                    ts: rec.timestamp,
                    slot: "binary_ioc".into(),
                    fact: "Exec".into(),
                    summary: format!("YARA match: {}", rule),
                    tags: vec!["IOC".into(), "YARA".into()],
                };
                if seen.insert(("yara".into(), rule.to_string())) {
                    out.push(h);
                    YARA_MATCHES.fetch_add(1, Ordering::Relaxed);
                }
            }
        }

        // 2) Extract candidates from record text
        let (urls, domains, ips, shas, paths) = extract_candidates(&st, rec);

        // 3) Check against configured items
        let now = now_sec();
        'item_loop: for it in &st.items {
            if let Some(until) = it.until {
                if now > until {
                    continue;
                }
            }

            let matched = match it.kind {
                IocKind::Sha256 => shas.iter().any(|s| s == &it.value),
                IocKind::Ip => ips.iter().any(|s| s == &it.value),
                IocKind::IpPrefix => ips.iter().any(|s| s.starts_with(&it.value)),
                IocKind::Domain => {
                    let want = normalize_domain(&it.value);
                    domains.iter().any(|d| {
                        let d = normalize_domain(d);
                        d == want || d.ends_with(&format!(".{}", want))
                    })
                }
                IocKind::DomainGlob => domains.iter().any(|d| domain_matches_glob(&it.value, d)),
                IocKind::Url => urls.iter().any(|u| u.eq_ignore_ascii_case(&it.value)),
                IocKind::UrlRegex => {
                    if let Ok(re) = Regex::new(&it.value) {
                        urls.iter().any(|u| re.is_match(u))
                    } else {
                        false
                    }
                }
                IocKind::PathRegex => {
                    if let Ok(re) = Regex::new(&it.value) {
                        paths.iter().any(|p| re.is_match(p))
                    } else {
                        false
                    }
                }
                IocKind::CertCnRegex => {
                    if let Ok(re) = Regex::new(&it.value) {
                        domains.iter().any(|d| re.is_match(d))
                    } else {
                        false
                    }
                }
            };

            if !matched {
                continue;
            }

            // Tag record based on kind
            match it.kind {
                IocKind::Sha256 => rec.tags.push("ioc_hash".into()),
                IocKind::Ip | IocKind::IpPrefix => rec.tags.push("ioc_ip".into()),
                IocKind::Domain | IocKind::DomainGlob | IocKind::CertCnRegex => {
                    rec.tags.push("ioc_domain".into())
                }
                IocKind::Url | IocKind::UrlRegex => rec.tags.push("ioc_url".into()),
                IocKind::PathRegex => rec.tags.push("ioc_path".into()),
            }

            // Map to (slot,fact) for PB UI
            let (slot, fact) = match it.kind {
                IocKind::Ip | IocKind::IpPrefix | IocKind::Domain | IocKind::DomainGlob | IocKind::Url | IocKind::UrlRegex | IocKind::CertCnRegex => {
                    ("artifact_ioc", "Net")
                }
                IocKind::Sha256 | IocKind::PathRegex => ("binary_ioc", "FileIO"),
            };

            let id = it
                .id
                .clone()
                .unwrap_or_else(|| format!("ioc:{:?}:{}", it.kind, it.value));
            let key = (format!("{:?}", it.kind), it.value.clone());
            if !seen.insert(key) {
                continue;
            }

            let mut tags = vec!["IOC".into()];
            tags.extend(it.tags.iter().cloned());

            out.push(Hit {
                id,
                ts: rec.timestamp,
                slot: slot.into(),
                fact: fact.into(),
                summary: format!("{:?} match: {}", it.kind, it.value),
                tags,
            });

            MATCHES_TOTAL.fetch_add(1, Ordering::Relaxed);
        }
    }

    Ok(out)
}

// ===== Internals: loading & helpers =====

fn now_sec() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn normalize_domain(d: &str) -> String {
    d.trim_matches('.').to_ascii_lowercase()
}

fn domain_matches_glob(glob: &str, dom: &str) -> bool {
    // "*.evil.com" matches a.b.evil.com (but not evil.com); exact match passes too
    let g = normalize_domain(glob);
    let d = normalize_domain(dom);
    if g == d {
        return true;
    }
    if g.starts_with("*.") {
        let suffix = &g[1..]; // keep the dot to avoid matching "notevil.com"
        return d.ends_with(suffix);
    }
    false
}

/// Extract candidate tokens from a TelemetryRecord.
fn extract_candidates<'a>(
    store: &IocStore,
    rec: &'a TelemetryRecord,
) -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
    // urls, domains, ips, sha256s, paths
    let mut urls = Vec::new();
    let mut domains = Vec::new();
    let mut ips = Vec::new();
    let mut shas = Vec::new();
    let mut paths = Vec::new();

    let mut buf = String::new();
    if !rec.binary_path.is_empty() {
        buf.push_str(&rec.binary_path);
        buf.push(' ');
    }
    if !rec.cwd.is_empty() {
        buf.push_str(&rec.cwd);
        buf.push(' ');
    }
    if !rec.command_line.is_empty() {
        buf.push_str(&rec.command_line);
        buf.push(' ');
    }
    for t in &rec.tags {
        buf.push_str(t);
        buf.push(' ');
    }

    for m in store.re_url.find_iter(&buf) {
        urls.push(m.as_str().to_string());
    }
    for m in store.re_domain.find_iter(&buf) {
        domains.push(m.as_str().to_ascii_lowercase());
    }
    for m in store.re_ipv4.find_iter(&buf) {
        ips.push(m.as_str().to_string());
    }
    for m in store.re_sha256.find_iter(&buf) {
        shas.push(m.as_str().to_ascii_lowercase());
    }
    for m in store.re_path.find_iter(&buf) {
        paths.push(m.as_str().to_string());
    }

    // also parse simple "key=value" tags
    for t in &rec.tags {
        let low = t.to_ascii_lowercase();
        if let Some(rest) = low.strip_prefix("sha256=") {
            shas.push(rest.to_string());
        }
        if let Some(rest) = low.strip_prefix("domain=") {
            domains.push(rest.to_string());
        }
        if let Some(rest) = low.strip_prefix("url=") {
            urls.push(rest.to_string());
        }
        if let Some(rest) = low.strip_prefix("ip=") {
            ips.push(rest.to_string());
        }
        if let Some(rest) = low.strip_prefix("cert_cn=") {
            domains.push(rest.to_string());
        }
        if let Some(rest) = low.strip_prefix("path=") {
            paths.push(rest.to_string());
        }
    }

    // unique
    let uniq = |mut v: Vec<String>| {
        v.sort();
        v.dedup();
        v
    };
    (uniq(urls), uniq(domains), uniq(ips), uniq(shas), uniq(paths))
}

fn parse_item_from_obj(obj: &serde_json::Map<String, Value>) -> Option<IocItem> {
    // tolerant field extraction
    let get_str = |keys: &[&str]| -> Option<String> {
        for k in keys {
            if let Some(Value::String(s)) = obj.get(*k) {
                if !s.is_empty() {
                    return Some(s.clone());
                }
            }
        }
        None
    };
    let kind_str = get_str(&["kind", "type"])?;
    let value = get_str(&["value", "indicator", "match", "pattern"])?;
    let id = get_str(&["id", "rule_id"]);
    let family = get_str(&["family", "malware", "actor"]);
    let confidence = obj
        .get("confidence")
        .and_then(|v| v.as_f64())
        .map(|f| f as f32);
    let until = obj.get("until").and_then(|v| v.as_u64()); // <-- fixed () call
    let tags: Vec<String> = obj
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();

    let kind = match kind_str.to_ascii_lowercase().as_str() {
        "sha256" | "sha2" => IocKind::Sha256,
        "ip" | "ipv4" => IocKind::Ip,
        "ip_prefix" | "ipv4_prefix" | "cidr_text" => IocKind::IpPrefix,
        "domain" => IocKind::Domain,
        "domain_glob" | "domain_wildcard" | "fqdn_glob" => IocKind::DomainGlob,
        "url" => IocKind::Url,
        "url_regex" => IocKind::UrlRegex,
        "path_regex" | "file_regex" => IocKind::PathRegex,
        "cert_cn_regex" | "certificate_cn_regex" => IocKind::CertCnRegex,
        _ => return None,
    };

    Some(IocItem {
        id,
        kind,
        value,
        family,
        confidence,
        until,
        tags,
    })
}

fn read_items_from_file(path: &Path) -> Result<Vec<IocItem>> {
    let s = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let v: Value =
        serde_json::from_str(&s).with_context(|| format!("parsing JSON in {}", path.display()))?;

    let mut out = Vec::new();
    match v {
        Value::Array(arr) => {
            for it in arr {
                if let Value::Object(obj) = it {
                    if let Some(item) = parse_item_from_obj(&obj) {
                        out.push(item);
                    }
                }
            }
        }
        Value::Object(obj) => {
            // could be one item or {"items":[...]}
            if let Some(Value::Array(arr)) = obj.get("items") {
                for it in arr {
                    if let Value::Object(o) = it {
                        if let Some(item) = parse_item_from_obj(&o) {
                            out.push(item);
                        }
                    }
                }
            } else if let Some(item) = parse_item_from_obj(&obj) {
                out.push(item);
            }
        }
        _ => {}
    }
    Ok(out)
}

fn read_items_from_jsonl(path: &Path) -> Result<Vec<IocItem>> {
    let mut out = Vec::new();
    let s = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    for (i, line) in s.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let v: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("IOC: {} line {}: parse error: {}", path.display(), i + 1, e);
                continue;
            }
        };
        if let Value::Object(obj) = v {
            if let Some(item) = parse_item_from_obj(&obj) {
                out.push(item);
            }
        }
    }
    Ok(out)
}

pub fn load_all(dir: impl AsRef<Path>) -> Result<IocStore> {
    let dir = dir.as_ref();
    let mut store = IocStore::default();
    if !dir.exists() {
        return Ok(store);
    }

    let mut items = Vec::new();
    for entry in fs::read_dir(dir)? {
        if let Ok(ent) = entry {
            let p = ent.path();
            if !p.is_file() {
                continue;
            }
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let is_jsonl = name.ends_with(".jsonl") || name.ends_with(".ndjson");
            let is_json = name.ends_with(".json");
            if !(is_json || is_jsonl) {
                continue;
            }

            let mut add = if is_jsonl {
                read_items_from_jsonl(&p).with_context(|| format!("reading jsonl {}", p.display()))?
            } else {
                read_items_from_file(&p).with_context(|| format!("reading json {}", p.display()))?
            };
            items.append(&mut add);
        }
    }

    // de-duplicate identical (kind,value) — keep highest confidence and union tags
    let mut uniq = Vec::<IocItem>::new();
    let mut seen = HashMap::<(IocKind, String), usize>::new();
    for mut it in items {
        let key = (it.kind.clone(), it.value.clone());
        if let Some(idx) = seen.get(&key).cloned() {
            let exist = &mut uniq[idx];
            if it.confidence.unwrap_or(0.0) > exist.confidence.unwrap_or(0.0) {
                exist.confidence = it.confidence;
            }
            if exist.until.is_none() {
                exist.until = it.until;
            }
            // union tags
            let mut set: HashSet<String> = exist.tags.iter().cloned().collect();
            set.extend(it.tags.into_iter());
            exist.tags = set.into_iter().collect();
        } else {
            seen.insert(key, uniq.len());
            uniq.push(it);
        }
    }
    store.items = uniq;
    Ok(store)
}
