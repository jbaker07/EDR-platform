// src/services/baseline_uid.rs
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

/// Default path for persisted baselines (mirrors other json_files usage)
const DEFAULT_STORE_PATH: &str = "edr-agent/json_files/baseline_uid.json";

/// Minimum observations before we consider promoting a learned baseline.
const MIN_SAMPLES_FOR_PROMOTION: u32 = 5;
/// Modal UID fraction required to promote (e.g., 80% of samples agree).
const PROMOTION_CONF_THRESH: f32 = 0.80;

/// A single UID baseline with light telemetry for confidence & aging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineEntry {
    pub uid: i32,
    pub confidence: f32,   // 0.0..1.0
    pub last_seen: u64,    // unix secs
    /// Optional observations: uid -> count
    #[serde(default)]
    pub observations: HashMap<i32, u32>,
}

impl BaselineEntry {
    fn new(uid: i32) -> Self {
        Self {
            uid,
            confidence: 1.0,
            last_seen: now_ts(),
            observations: HashMap::new(),
        }
    }

    fn touch(&mut self) {
        self.last_seen = now_ts();
        // small decay-prevention bump capped to 1.0
        self.confidence = (self.confidence + 0.01).min(1.0);
    }

    fn observe(&mut self, uid: i32) {
        *self.observations.entry(uid).or_insert(0) += 1;
        self.last_seen = now_ts();
    }

    /// Returns (modal_uid, modal_count, total)
    fn modal_uid(&self) -> Option<(i32, u32, u32)> {
        if self.observations.is_empty() {
            return None;
        }
        let mut total = 0u32;
        let mut best_uid = 0i32;
        let mut best_ct = 0u32;
        for (u, c) in &self.observations {
            total += *c;
            if *c > best_ct {
                best_ct = *c;
                best_uid = *u;
            }
        }
        Some((best_uid, best_ct, total))
    }
}

/// Baseline store:
/// - `exact`: keyed by canonical (or raw) absolute paths
/// - `by_name`: keyed by basename ("dbus-daemon")
/// - `prefix_dirs`: paths treated as directory prefixes; if a binary resides
///                  under any prefix, the associated entry applies.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BaselineStore {
    pub exact: HashMap<String, BaselineEntry>,
    pub by_name: HashMap<String, BaselineEntry>,
    pub prefix_dirs: HashMap<String, BaselineEntry>,
    /// Optional: path used for persistence
    #[serde(skip)]
    pub backing_path: Option<String>,
}

impl BaselineStore {
    fn with_defaults() -> Self {
        let mut exact = HashMap::new();
        // Seed a few common, stable baselines (examples; adjust as needed)
        exact.insert("/usr/bin/dbus-daemon".to_string(), BaselineEntry::new(81));
        exact.insert("/usr/lib/systemd/systemd".to_string(), BaselineEntry::new(0));

        let mut by_name = HashMap::new();
        // Some daemons have consistent UIDs regardless of location/chroots
        by_name.insert("dbus-daemon".to_string(), BaselineEntry::new(81));
        by_name.insert("systemd".to_string(), BaselineEntry::new(0));

        Self {
            exact,
            by_name,
            prefix_dirs: HashMap::new(),
            backing_path: Some(DEFAULT_STORE_PATH.to_string()),
        }
    }

    /// Try multiple strategies: exact (canonical & raw), basename, then prefix dirs.
    fn resolve_uid(&self, path: &str) -> Option<i32> {
        // Exact by canonical path
        if let Some(canon) = canonicalize_lossy_opt(path) {
            if let Some(e) = self.exact.get(&canon) {
                return Some(e.uid);
            }
        }
        // Exact by raw path
        if let Some(e) = self.exact.get(path) {
            return Some(e.uid);
        }
        // Basename
        if let Some(base) = Path::new(path).file_name().and_then(|s| s.to_str()) {
            if let Some(e) = self.by_name.get(base) {
                return Some(e.uid);
            }
        }
        // Directory prefix
        if let Some(canon) = canonicalize_lossy_opt(path) {
            for (prefix, e) in &self.prefix_dirs {
                if canon.starts_with(prefix) {
                    return Some(e.uid);
                }
            }
        } else {
            for (prefix, e) in &self.prefix_dirs {
                if path.starts_with(prefix) {
                    return Some(e.uid);
                }
            }
        }
        None
    }

    fn set_exact(&mut self, path: &str, uid: i32) {
        let key = canonicalize_lossy_opt(path).unwrap_or_else(|| path.to_string());
        self.exact.insert(key, BaselineEntry::new(uid));
    }

    fn set_by_name(&mut self, name: &str, uid: i32) {
        self.by_name.insert(name.to_string(), BaselineEntry::new(uid));
    }

    fn set_prefix(&mut self, dir_prefix: &str, uid: i32) {
        let key = if dir_prefix.ends_with('/') {
            dir_prefix.to_string()
        } else {
            format!("{}/", dir_prefix)
        };
        self.prefix_dirs.insert(key, BaselineEntry::new(uid));
    }

    fn remove(&mut self, path_or_name: &str) -> bool {
        let canon = canonicalize_lossy_opt(path_or_name);
        let mut removed = false;
        if let Some(c) = canon {
            removed |= self.exact.remove(&c).is_some();
        }
        removed |= self.exact.remove(path_or_name).is_some();
        removed |= self.by_name.remove(path_or_name).is_some();
        removed |= self.prefix_dirs.remove(path_or_name).is_some();
        removed
    }

    fn learn(&mut self, path: &str, observed_uid: i32) -> Option<i32> {
        let key = canonicalize_lossy_opt(path).unwrap_or_else(|| path.to_string());
        let entry = self.exact.entry(key.clone()).or_insert_with(|| BaselineEntry::new(observed_uid));
        entry.observe(observed_uid);

        if let Some((modal_uid, modal_ct, total)) = entry.modal_uid() {
            if total >= MIN_SAMPLES_FOR_PROMOTION {
                let frac = (modal_ct as f32) / (total as f32);
                if frac >= PROMOTION_CONF_THRESH {
                    entry.uid = modal_uid;
                    entry.confidence = frac;
                    entry.touch();
                    return Some(entry.uid);
                }
            }
        }
        None
    }
}

// --------------------- global, concurrent store ---------------------

static STORE: Lazy<RwLock<BaselineStore>> = Lazy::new(|| {
    let mut store = BaselineStore::with_defaults();
    // Attempt to load persisted baselines and merge.
    if let Ok(loaded) = load_from_disk(DEFAULT_STORE_PATH) {
        // Merge with defaults; loaded takes precedence.
        for (k, v) in loaded.exact {
            store.exact.insert(k, v);
        }
        for (k, v) in loaded.by_name {
            store.by_name.insert(k, v);
        }
        for (k, v) in loaded.prefix_dirs {
            store.prefix_dirs.insert(k, v);
        }
    }
    RwLock::new(store)
});

/// Public API: returns a baseline UID for `binary` if known.
pub fn get_baseline_uid(binary: &str) -> Option<i32> {
    STORE.read().ok()?.resolve_uid(binary)
}

/// Manually set baseline for an exact path.
pub fn set_baseline_uid(binary: &str, uid: i32) {
    if let Ok(mut s) = STORE.write() {
        s.set_exact(binary, uid);
        let _ = save_to_disk_locked(&s);
    }
}

/// Manually set baseline for a basename (e.g., "dbus-daemon").
pub fn set_baseline_uid_by_name(name: &str, uid: i32) {
    if let Ok(mut s) = STORE.write() {
        s.set_by_name(name, uid);
        let _ = save_to_disk_locked(&s);
    }
}

/// Register a directory prefix rule (applies to all binaries under prefix).
pub fn register_prefix_rule(dir_prefix: &str, uid: i32) {
    if let Ok(mut s) = STORE.write() {
        s.set_prefix(dir_prefix, uid);
        let _ = save_to_disk_locked(&s);
    }
}

/// Remove a baseline (by exact path, basename, or prefix key).
pub fn remove_baseline(key: &str) -> bool {
    if let Ok(mut s) = STORE.write() {
        let removed = s.remove(key);
        if removed {
            let _ = save_to_disk_locked(&s);
        }
        return removed;
    }
    false
}

/// Observe a (binary, uid) pair; once the modal UID has enough support,
/// the baseline is updated and returned.
pub fn learn_uid_observation(binary: &str, observed_uid: i32) -> Option<i32> {
    if let Ok(mut s) = STORE.write() {
        let res = s.learn(binary, observed_uid);
        if res.is_some() {
            let _ = save_to_disk_locked(&s);
        }
        return res;
    }
    None
}

/// Load baselines from disk (JSON). Returns a store (not applied).
pub fn load_from_disk(path: &str) -> io::Result<BaselineStore> {
    let data = fs::read(path)?;
    let mut store: BaselineStore = serde_json::from_slice(&data)?;
    store.backing_path = Some(path.to_string());
    Ok(store)
}

/// Save current baselines to disk (JSON).
pub fn save_to_disk(path: Option<&str>) -> io::Result<()> {
    let s = STORE.read().map_err(|_| io::Error::new(io::ErrorKind::Other, "lock poisoned"))?;
    let path = path
        .map(|p| p.to_string())
        .or_else(|| s.backing_path.clone())
        .unwrap_or_else(|| DEFAULT_STORE_PATH.to_string());
    save_json_atomic(&path, &*s)
}

// --------------------- internals & utils ---------------------

fn save_to_disk_locked(store: &BaselineStore) -> io::Result<()> {
    let path = store
        .backing_path
        .clone()
        .unwrap_or_else(|| DEFAULT_STORE_PATH.to_string());
    save_json_atomic(&path, store)
}

fn save_json_atomic<T: ?Sized + Serialize>(path: &str, val: &T) -> io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = format!("{}.tmp", path);
    {
        let mut f = fs::File::create(&tmp)?;
        let bytes = serde_json::to_vec_pretty(val)?;
        f.write_all(&bytes)?;
        f.sync_all()?;
    }
    fs::rename(tmp, path)
}

fn canonicalize_lossy_opt(p: &str) -> Option<String> {
    fs::canonicalize(p)
        .ok()
        .map(|pb| pb.to_string_lossy().to_string())
}

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
