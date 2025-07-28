use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TrustDigestInput {
    pub timestamp: u64,
    pub hostname: String,
    pub process_name: String,
    pub command_line: String,
    pub user_id: String,
    pub parent_process: Option<String>,
    pub cpu_usage: Option<f32>,
    pub memory_usage: Option<f32>,
    pub risk_score: Option<f32>,
    pub tags: Option<Vec<String>>,
    pub category: Option<String>,
    pub semantic_label: Option<String>,
    pub mode: String,
}

#[derive(Debug, Serialize)]
pub struct TrustDigestResult {
    pub timestamp: u64,
    pub hostname: String,
    pub user_id: String,
    pub trust_score: f32,
    pub outcome: String,
}

/// Compute a synthetic trust score based on input fields
pub fn compute_trust_score(input: &TrustDigestInput) -> f32 {
    let mut score = 1.0;

    if let Some(risk) = input.risk_score {
        score -= risk.min(1.0);
    }

    if let Some(tags) = &input.tags {
        for tag in tags {
            if tag.contains("suspicious") {
                score -= 0.2;
            } else if tag.contains("exec") {
                score -= 0.1;
            }
        }
    }

    if input.cpu_usage.unwrap_or(0.0) > 80.0 {
        score -= 0.15;
    }

    if input.memory_usage.unwrap_or(0.0) > 512.0 {
        score -= 0.15;
    }

    score.clamp(0.0, 1.0)
}

/// Process a single input and return result
pub fn process_digest(input: TrustDigestInput) -> TrustDigestResult {
    let trust_score = compute_trust_score(&input);
    let outcome = if trust_score < 0.4 {
        "isolate"
    } else if trust_score < 0.7 {
        "monitor"
    } else {
        "allow"
    };

    TrustDigestResult {
        timestamp: now_ts(),
        hostname: input.hostname,
        user_id: input.user_id,
        trust_score,
        outcome: outcome.to_string(),
    }
}

/// Load and process all lines from a JSONL file
pub fn process_digest_file(path: &str) {
    let file = File::open(Path::new(path)).expect("Failed to open trust digest input file.");
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        if let Ok(input) = serde_json::from_str::<TrustDigestInput>(&line) {
            let result = process_digest(input);
            println!("[digest_engine] {:?}", result);
        }
    }
}

/// Utility to get UNIX timestamp
fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
