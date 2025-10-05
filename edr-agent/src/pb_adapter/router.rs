use std::collections::{HashMap, HashSet};

use serde_json::{Map, Number, Value};

use crate::telemetry_types::TelemetryOutput;
use crate::utils::time::now_ts;

use super::fact::AdapterFact;

pub fn to_adapter_facts(rec: &TelemetryOutput) -> Vec<AdapterFact> {
    if rec.category.is_empty() && rec.signal.is_empty() {
        return Vec::new();
    }

    let mut meta = materialize_meta(&rec.data);
    meta.insert("category".to_string(), Value::String(rec.category.clone()));
    meta.insert("signal".to_string(), Value::String(rec.signal.clone()));
    meta.insert(
        "confidence".to_string(),
        Value::Number(number_from_f64(rec.confidence as f64)),
    );

    let kind = canonical_kind(&rec.category, &rec.signal, &meta);

    if matches!(kind.as_str(), "memory.w2x" | "memory.memfd_exec") && !meta.contains_key("map_perm")
    {
        meta.insert("map_perm".into(), Value::String("wx".into()));
    }
    meta.insert("adapter_kind".to_string(), Value::String(kind.clone()));

    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
    let detector = rec
        .data
        .get("detector")
        .cloned()
        .unwrap_or_else(|| rec.signal.clone());

    let tags = collect_tags(rec, &meta);
    let ts = extract_ts(&meta);

    let proc_key = derive_proc_key(&meta);
    let socket_key = derive_socket_key(&meta);
    let file_key = derive_file_key(&meta);
    let user_key = derive_user_key(&meta);
    let container_id = derive_container_id(&meta);

    let fact = AdapterFact {
        ts,
        host,
        detector,
        kind,
        tags,
        proc_key,
        socket_key,
        file_key,
        user_key,
        container_id,
        meta: Value::Object(meta),
    };

    vec![fact]
}

fn materialize_meta(data: &HashMap<String, String>) -> Map<String, Value> {
    let mut out = Map::new();
    for (k, v) in data {
        out.insert(k.clone(), to_json_value(v));
    }
    out
}

fn to_json_value(raw: &str) -> Value {
    let trimmed = raw.trim();
    if trimmed.eq_ignore_ascii_case("null") {
        return Value::Null;
    }
    if let Ok(b) = trimmed.parse::<bool>() {
        return Value::Bool(b);
    }
    if let Ok(i) = trimmed.parse::<i64>() {
        return Value::Number(Number::from(i));
    }
    if let Ok(u) = trimmed.parse::<u64>() {
        return Value::Number(Number::from(u));
    }
    if let Ok(f) = trimmed.parse::<f64>() {
        if let Some(n) = Number::from_f64(f) {
            return Value::Number(n);
        }
    }
    if (trimmed.starts_with('{') && trimmed.ends_with('}'))
        || (trimmed.starts_with('[') && trimmed.ends_with(']'))
    {
        if let Ok(v) = serde_json::from_str::<Value>(trimmed) {
            return v;
        }
    }
    Value::String(raw.to_string())
}

fn canonical_kind(category: &str, signal: &str, meta: &Map<String, Value>) -> String {
    let cat = category.to_ascii_lowercase();
    let sig = signal.to_ascii_lowercase();

    if cat.contains("memory") || sig.contains("memory") || sig.contains("rwx") {
        if sig.contains("memfd")
            || meta
                .get("tag0")
                .and_then(value_as_string)
                .map(|t| t.contains("memfd"))
                .unwrap_or(false)
        {
            return "memory.memfd_exec".into();
        }
        return "memory.w2x".into();
    }
    if cat.contains("network") || sig.contains("net") || sig.contains("connect") {
        if sig.contains("dns") {
            return "network.dns".into();
        }
        return "network.connect".into();
    }
    if cat.contains("process") || sig.contains("exec") {
        return "process.exec".into();
    }
    if cat.contains("file") || sig.contains("file") {
        if sig.contains("write") {
            return "file.write".into();
        }
        return "file.access".into();
    }
    if cat.contains("auth") {
        return "auth.event".into();
    }
    if cat.contains("priv") || sig.contains("priv") {
        return "privilege.change".into();
    }
    if cat.contains("persist") {
        return "persistence.change".into();
    }
    if cat.contains("ipc") {
        return "ipc.activity".into();
    }
    if cat.contains("kernel") {
        return "kernel.signal".into();
    }
    if meta.contains_key("socket_inode") || meta.contains_key("five_tuple") {
        return "network.connect".into();
    }
    "process.signal".into()
}

fn collect_tags(rec: &TelemetryOutput, meta: &Map<String, Value>) -> Vec<String> {
    let mut tags: HashSet<String> = HashSet::new();

    if let Some(Value::Array(arr)) = meta.get("tags") {
        for v in arr {
            if let Some(s) = value_as_string(v) {
                if !s.is_empty() {
                    tags.insert(s);
                }
            }
        }
    }

    if let Some(Value::String(s)) = meta.get("tags") {
        for tag in s.split(|c| c == ',' || c == ' ' || c == '|') {
            let tag = tag.trim();
            if !tag.is_empty() {
                tags.insert(tag.to_string());
            }
        }
    }

    for (k, v) in meta.iter() {
        if k.starts_with("tag") {
            if let Some(s) = value_as_string(v) {
                if !s.is_empty() {
                    tags.insert(s);
                }
            }
        }
    }

    if !rec.category.is_empty() {
        tags.insert(rec.category.clone());
    }
    if !rec.signal.is_empty() {
        tags.insert(rec.signal.clone());
    }

    let mut out: Vec<String> = tags.into_iter().collect();
    out.sort();
    out
}

fn extract_ts(meta: &Map<String, Value>) -> i64 {
    let candidates: [(&str, i64); 6] = [
        ("ts", 1),
        ("timestamp", 1),
        ("time", 1),
        ("ts_ms", 1_000),
        ("ts_us", 1_000_000),
        ("ts_ns", 1_000_000_000),
    ];

    for (key, scale) in candidates {
        if let Some(v) = meta.get(key) {
            if let Some(raw) = value_as_i64(v) {
                return match scale {
                    1 => raw,
                    1_000 => raw / 1_000,
                    1_000_000 => raw / 1_000_000,
                    1_000_000_000 => raw / 1_000_000_000,
                    _ => raw,
                };
            }
        }
    }

    if let Some(v) = meta.get("ts_sec") {
        if let Some(raw) = value_as_i64(v) {
            return raw;
        }
    }

    now_ts() as i64
}

fn derive_proc_key(meta: &Map<String, Value>) -> Option<String> {
    let pid = meta
        .get("pid")
        .and_then(value_as_i64)
        .or_else(|| meta.get("tgid").and_then(value_as_i64));
    pid.map(|p| {
        let exe = meta
            .get("binary_path")
            .and_then(value_as_string)
            .or_else(|| meta.get("exe").and_then(value_as_string))
            .or_else(|| meta.get("comm").and_then(value_as_string));
        if let Some(exe) = exe {
            format!("pid:{p}|exe:{exe}")
        } else {
            format!("pid:{p}")
        }
    })
}

fn derive_socket_key(meta: &Map<String, Value>) -> Option<String> {
    if let Some(inode) = meta.get("socket_inode").and_then(value_as_string) {
        if !inode.is_empty() {
            return Some(format!("inode:{inode}"));
        }
    }
    if let Some(key) = meta.get("five_tuple").and_then(value_as_string) {
        if !key.is_empty() {
            return Some(key);
        }
    }

    let laddr = meta
        .get("laddr6")
        .or_else(|| meta.get("laddr4"))
        .or_else(|| meta.get("saddr"))
        .or_else(|| meta.get("src"))
        .and_then(value_as_string);
    let lport = meta
        .get("lport")
        .or_else(|| meta.get("sport"))
        .and_then(value_as_string);
    let raddr = meta
        .get("raddr6")
        .or_else(|| meta.get("raddr4"))
        .or_else(|| meta.get("daddr"))
        .or_else(|| meta.get("dst"))
        .and_then(value_as_string);
    let rport = meta
        .get("rport")
        .or_else(|| meta.get("dport"))
        .and_then(value_as_string);

    match (laddr, lport) {
        (Some(addr), Some(port)) if !addr.is_empty() && !port.is_empty() => {
            let mut key = format!("{addr}:{port}");
            if let (Some(raddr), Some(rport)) = (raddr, rport) {
                if !raddr.is_empty() && !rport.is_empty() {
                    key.push_str("->");
                    key.push_str(&format!("{raddr}:{rport}"));
                }
            }
            Some(key)
        }
        _ => None,
    }
}

fn derive_file_key(meta: &Map<String, Value>) -> Option<String> {
    let path = meta
        .get("path")
        .or_else(|| meta.get("file"))
        .or_else(|| meta.get("binary_path"))
        .or_else(|| meta.get("filename"))
        .and_then(value_as_string)?;
    let trimmed = path.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn derive_user_key(meta: &Map<String, Value>) -> Option<String> {
    if let Some(user) = meta.get("user").and_then(value_as_string) {
        if !user.is_empty() {
            return Some(user);
        }
    }
    meta.get("uid")
        .and_then(value_as_i64)
        .map(|uid| format!("uid:{uid}"))
}

fn derive_container_id(meta: &Map<String, Value>) -> Option<String> {
    meta.get("container_id")
        .or_else(|| meta.get("container"))
        .or_else(|| meta.get("containerid"))
        .and_then(value_as_string)
        .map(|s| s.to_string())
}

fn value_as_i64(v: &Value) -> Option<i64> {
    match v {
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Some(i)
            } else if let Some(u) = n.as_u64() {
                i64::try_from(u).ok()
            } else {
                None
            }
        }
        Value::String(s) => s.trim().parse::<i64>().ok(),
        _ => None,
    }
}

fn value_as_string(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn number_from_f64(f: f64) -> Number {
    Number::from_f64(f).unwrap_or_else(|| Number::from(0))
}
