// src/services/fingerprint.rs (improved, backward-compatible)

use serde::Deserialize;
use serde_json::Value;
use std::{collections::{HashMap, HashSet}, fs};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct KnownFingerprint {
    /// Optional path (may be present in some input formats)
    pub path: Option<String>,
    /// File/content hash (required in most cases; optional for key-triple inputs)
    pub hash: Option<String>,
    /// UID owner baseline; defaults to 0 if omitted
    pub uid: Option<u32>,
    /// Allow unknown fields without failing deserialization
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Primary loader used across the codebase.
///
/// Returns a map keyed by a **canonical triple**: `"path:hash:uid"`.
/// Values include whatever was provided (path/hash/uid if available).
///
/// Accepted input formats:
/// - JSON object mapping either:
///     - `"path:hash:uid" -> { "hash": "...", "uid": 0, ... }`
///     - `"<path>" -> { "hash": "...", "uid": 1000, ... }`
/// - JSON array: `[ { "path":"...", "hash":"...", "uid":1000 }, ... ]`
/// - Plain text fallback:
///     - `path:hash:uid`
///     - `path,hash,uid`
///   Lines starting with `#` or empty lines are ignored.
pub fn load_known_safe_fingerprints(path: &str) -> HashMap<String, KnownFingerprint> {
    let txt = fs::read_to_string(path).unwrap_or_default();
    if txt.trim().is_empty() {
        return HashMap::new();
    }

    // Try JSON first
    if let Ok(val) = serde_json::from_str::<Value>(&txt) {
        return match val {
            Value::Object(map) => from_json_object(map),
            Value::Array(arr) => from_json_array(arr),
            _ => HashMap::new(),
        };
    }

    // Fallback to plaintext (lines)
    from_plaintext_lines(&txt)
}

/// Convenience: get a `HashSet` of the canonical triples (the map keys).
pub fn fingerprint_key_set(map: &HashMap<String, KnownFingerprint>) -> HashSet<String> {
    map.keys().cloned().collect()
}

// --------------------------- internal helpers ---------------------------

fn from_json_object(obj: serde_json::Map<String, Value>) -> HashMap<String, KnownFingerprint> {
    let mut out = HashMap::with_capacity(obj.len());

    for (key, v) in obj {
        // Try to parse the value into a flexible record
        let mut rec: KnownFingerprint = match serde_json::from_value(v.clone()) {
            Ok(r) => r,
            Err(_) => KnownFingerprint::default(),
        };

        // If the key itself is already a triple, prefer it; otherwise treat key as a path
        if let Some((p, h, u)) = parse_triple(&key) {
            let hash = rec.hash.take().unwrap_or_else(|| h.to_string());
            let uid = rec.uid.take().unwrap_or(u);
            let path = rec.path.take().unwrap_or_else(|| p.to_string());
            let k = make_key(&path, &hash, uid);
            // Ensure we store normalized fields back
            rec.path = Some(path);
            rec.hash = Some(normalize_hash(&hash));
            rec.uid = Some(uid);
            out.insert(k, rec);
        } else {
            // Key is likely a path; value must contain hash (and maybe uid)
            let path = key.trim().to_string();
            if let Some(hash) = rec.hash.clone() {
                let uid = rec.uid.unwrap_or(0);
                let k = make_key(&path, &hash, uid);
                rec.path = Some(path);
                rec.hash = Some(normalize_hash(&hash));
                rec.uid = Some(uid);
                out.insert(k, rec);
            }
        }
    }

    out
}

fn from_json_array(arr: Vec<Value>) -> HashMap<String, KnownFingerprint> {
    let mut out = HashMap::with_capacity(arr.len());
    for v in arr {
        if let Ok(mut rec) = serde_json::from_value::<KnownFingerprint>(v) {
            if let (Some(path), Some(hash)) = (rec.path.clone(), rec.hash.clone()) {
                let uid = rec.uid.unwrap_or(0);
                let k = make_key(&path, &hash, uid);
                rec.hash = Some(normalize_hash(&hash));
                rec.uid = Some(uid);
                out.insert(k, rec);
            }
        }
    }
    out
}

fn from_plaintext_lines(s: &str) -> HashMap<String, KnownFingerprint> {
    let mut out = HashMap::new();

    for raw in s.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Support "a,b,c" or "a:b:c". Also accept "a:b" (uid defaults to 0).
        // Normalize so the third element is always a String (uid as text),
        // then parse once below.
        let (path, hash, uid_txt): (&str, &str, String) = if line.contains(',') && !line.contains(':') {
            let mut it = line.split(',').map(|x| x.trim());
            let p = it.next().unwrap_or_default();
            let h = it.next().unwrap_or_default();
            let u = it.next().unwrap_or("0").to_string();
            (p, h, u)
        } else if let Some((p, h, u)) = parse_triple(line) {
            (p, h, u.to_string())
        } else if let Some((p, h)) = parse_pair(line) {
            (p, h, "0".to_string())
        } else {
            continue;
        };

        let uid_num = uid_txt.parse::<u32>().unwrap_or(0);
        let key = make_key(path, hash, uid_num);

        out.insert(
            key.clone(),
            KnownFingerprint {
                path: Some(path.trim().to_string()),
                hash: Some(normalize_hash(hash)),
                uid: Some(uid_num),
                extra: HashMap::new(),
            },
        );
    }

    out
}

// "path:hash:uid"
fn parse_triple(s: &str) -> Option<(&str, &str, u32)> {
    let mut parts = s.split(':');
    let p = parts.next()?;
    let h = parts.next()?;
    let u = parts.next()?;
    if parts.next().is_some() {
        return None; // too many fields
    }
    let uid = u.parse::<u32>().ok()?;
    Some((p, h, uid))
}

// "path:hash" (uid defaults to 0)
fn parse_pair(s: &str) -> Option<(&str, &str)> {
    let mut parts = s.split(':');
    let p = parts.next()?;
    let h = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    Some((p, h))
}

fn make_key(path: &str, hash: &str, uid: u32) -> String {
    format!("{}:{}:{}", path.trim(), normalize_hash(hash), uid)
}

fn normalize_hash(h: &str) -> String {
    h.trim().to_ascii_lowercase()
}

// --------------------------- tests (optional) ---------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_json_object_triple_keys() {
        let j = r#"{"/bin/ls:deadbeef:0":{"hash":"DEADBEEF"}}"#;
        let v = serde_json::from_str::<Value>(j).unwrap();
        let map = match v { Value::Object(m) => from_json_object(m), _ => unreachable!() };
        assert!(map.contains_key("/bin/ls:deadbeef:0"));
    }

    #[test]
    fn parses_json_object_path_keys() {
        let j = r#"{"/bin/ls":{"hash":"ABCDEF", "uid": 1000}}"#;
        let v = serde_json::from_str::<Value>(j).unwrap();
        let map = match v { Value::Object(m) => from_json_object(m), _ => unreachable!() };
        assert!(map.contains_key("/bin/ls:abcdef:1000"));
    }

    #[test]
    fn parses_json_array() {
        let j = r#"[{"path":"/bin/ls","hash":"AA11","uid":0}]"#;
        let v = serde_json::from_str::<Value>(j).unwrap();
        let map = match v { Value::Array(a) => from_json_array(a), _ => unreachable!() };
        assert!(map.contains_key("/bin/ls:aa11:0"));
    }

    #[test]
    fn parses_plaintext() {
        let t = "/bin/ls:aa11:0\n#comment\n/bin/cat,bb22,1000\n";
        let map = from_plaintext_lines(t);
        assert!(map.contains_key("/bin/ls:aa11:0"));
        assert!(map.contains_key("/bin/cat:bb22:1000"));
    }
}
