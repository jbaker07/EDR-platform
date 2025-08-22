use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static::lazy_static! {
    static ref DEDUP_CACHE: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
}

const DEDUP_WINDOW_SECS: u64 = 300; // 5-minute deduplication window

/// Checks whether the signal is a duplicate based on recent activity
pub fn is_duplicate_signal(hostname: &str, pid: u32, gnn_score: f64, tags: &[String]) -> bool {
    let key = format!(
        "{}-{}-{:.2}-{}",
        hostname,
        pid,
        gnn_score,
        tags.join("_")
    );
    let now = current_ts();

    let mut cache = DEDUP_CACHE.lock().unwrap();
    if let Some(last_seen) = cache.get(&key) {
        if now - *last_seen < DEDUP_WINDOW_SECS {
            return true;
        }
    }

    cache.insert(key, now);
    false
}

/// Returns the current timestamp in seconds
fn current_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
