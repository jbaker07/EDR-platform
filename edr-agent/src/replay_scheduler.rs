use std::collections::{HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    static ref REPLAY_QUEUE: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplayJob {
    pub hostname: String,
    pub pid: u32,
    pub reason: String,
    pub triggered_tags: Vec<String>,
    pub trust_score: f64,
    pub timestamp: u64,
    pub ttl_secs: u64,
}

pub fn schedule_replay_job(job: ReplayJob) {
    let key = format!("{}-{}", job.hostname, job.pid);

    {
        let mut queue = REPLAY_QUEUE.lock().unwrap();
        if queue.contains(&key) {
            return; // job already scheduled
        }
        queue.insert(key);
    }

    let path = "replay_jobs/queue.jsonl";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open replay job queue");

    let line = serde_json::to_string(&job).expect("Failed to serialize ReplayJob");
    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Error writing replay job: {}", e);
    }
}

/// Generate timestamp for scheduling
pub fn current_unix_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Trigger manually or from trust/ontology module
pub fn trigger_from_event(
    hostname: &str,
    pid: u32,
    triggered_tags: Vec<String>,
    trust_score: f64,
    reason: &str,
) {
    let job = ReplayJob {
        hostname: hostname.into(),
        pid,
        reason: reason.into(),
        triggered_tags,
        trust_score,
        timestamp: current_unix_ts(),
        ttl_secs: 600, // capture 10-minute window
    };

    schedule_replay_job(job);
}
