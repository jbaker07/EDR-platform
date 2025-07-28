use std::collections::HashMap;
use std::io::{Write, Read};
use std::net::TcpStream;
use serde_json::json;
use crate::score_reason::ScoreReason;

/// Sends telemetry map to GNN inference server and receives a score (0.0–1.0)
/// Returns both score and optional `ScoreReason` for explainability.
pub fn get_gnn_score(input: &HashMap<String, String>) -> (f64, Option<ScoreReason>) {
    let gnn_host = "127.0.0.1:6060"; // Match your local or deployment setup

    match TcpStream::connect(gnn_host) {
        Ok(mut stream) => {
            let request_json = json!(input).to_string();
            let _ = stream.write_all(request_json.as_bytes());

            let mut buffer = [0; 128];
            if let Ok(n) = stream.read(&mut buffer) {
                if let Ok(response) = std::str::from_utf8(&buffer[..n]) {
                    if let Ok(score) = response.trim().parse::<f64>() {
                        let reason = infer_score_reason(score, input);
                        return (score.clamp(0.0, 1.0), reason);
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("[gnn_score_fetcher.rs] GNN server unavailable — fallback triggered");
        }
    }

    fallback_score(input)
}

/// Simple rule-based fallback for GNN score + reason
fn fallback_score(input: &HashMap<String, String>) -> (f64, Option<ScoreReason>) {
    let cpu = input.get("cpu_usage").and_then(|v| v.parse::<f64>().ok()).unwrap_or(15.0);
    let risk = input.get("risk_score").and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.2);

    let score = ((cpu / 100.0) * 0.5 + risk * 0.5).min(1.0);
    let reason = if cpu > 80.0 {
        Some(ScoreReason::HighCpuUsage)
    } else if risk > 0.7 {
        Some(ScoreReason::Custom("High risk input to GNN fallback".into()))
    } else {
        None
    };

    if let Some(ref r) = reason {
        println!("[📉 GNN fallback] Reason inferred: {}", r.to_string());
    }

    (score, reason)
}

/// Infers a likely reason for the GNN score based on heuristics
fn infer_score_reason(score: f64, input: &HashMap<String, String>) -> Option<ScoreReason> {
    if score > 0.85 {
        if let Some(risk) = input.get("risk_score").and_then(|v| v.parse::<f64>().ok()) {
            if risk > 0.75 {
                return Some(ScoreReason::RuleTriggered("High GNN risk + high telemetry risk".into()));
            }
        }
        return Some(ScoreReason::Custom("Anomalous pattern detected by GNN".into()));
    }

    None
}
