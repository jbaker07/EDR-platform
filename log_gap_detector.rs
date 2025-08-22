use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct LogEvent {
    pub timestamp: u64,     // UNIX epoch time
    pub source: String,     // e.g., "process_monitor"
    pub sequence_id: u64,   // Optional: Monotonic ID if available
}

#[derive(Debug)]
pub struct LogGapResult {
    pub source: String,
    pub gap_detected: bool,
    pub gap_duration_secs: Option<u64>,
    pub missing_sequence_ids: Vec<u64>,
}

pub fn detect_log_gaps(
    events: Vec<LogEvent>,
    expected_interval_secs: u64,
) -> Vec<LogGapResult> {
    let mut grouped_by_source: HashMap<String, Vec<LogEvent>> = HashMap::new();
    for event in events {
        grouped_by_source.entry(event.source.clone())
            .or_default()
            .push(event);
    }

    let mut results = Vec::new();

    for (source, mut logs) in grouped_by_source {
        logs.sort_by_key(|e| e.timestamp);

        let mut previous_ts = None;
        let mut previous_seq = None;
        let mut missing_seq_ids = Vec::new();
        let mut gap_detected = false;
        let mut largest_gap: Option<u64> = None;

        for event in logs {
            if let Some(prev_ts) = previous_ts {
                let delta = event.timestamp.saturating_sub(prev_ts);
                if delta > expected_interval_secs * 2 {
                    gap_detected = true;
                    largest_gap = Some(delta);
                }
            }
            if let Some(prev_seq) = previous_seq {
                if event.sequence_id > prev_seq + 1 {
                    for sid in (prev_seq + 1)..event.sequence_id {
                        missing_seq_ids.push(sid);
                    }
                    gap_detected = true;
                }
            }

            previous_ts = Some(event.timestamp);
            previous_seq = Some(event.sequence_id);
        }

        results.push(LogGapResult {
            source,
            gap_detected,
            gap_duration_secs: largest_gap,
            missing_sequence_ids,
        });
    }

    results
}

pub fn current_unix_ts() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
