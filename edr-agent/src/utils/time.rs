use std::time::{SystemTime, UNIX_EPOCH};

/// Returns a UNIX timestamp in seconds
pub fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
