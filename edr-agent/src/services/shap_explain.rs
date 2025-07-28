use std::collections::HashMap;
use std::io::{Write, Read};
use std::net::TcpStream;
use serde_json::json;
use crate::score_reason::ScoreReason;

/// Sends telemetry to SHAP explainer and returns a float score (0.0–1.0)
/// Fallback includes basic reason tagging
pub fn get_shap_score(input: &HashMap<String, String>) -> f64 {
    let shap_host = "127.0.0.1:5050"; // change to match your setup

    match TcpStream::connect(shap_host) {
        Ok(mut stream) => {
            let request_json = json!(input).to_string();
            let _ = stream.write_all(request_json.as_bytes());

            let mut buffer = [0; 128];
            if let Ok(n) = stream.read(&mut buffer) {
                if let Ok(response) = std::str::from_utf8(&buffer[..n]) {
                    if let Ok(score) = response.trim().parse::<f64>() {
                        return score.clamp(0.0, 1.0);
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("[shap_explain.rs] SHAP server unavailable, falling back to default scoring");
        }
    }

    fallback_score(input).0
}

/// Simple local fallback using rule-based approximation
/// Returns (score, explanation tag)
fn fallback_score(input: &HashMap<String, String>) -> (f64, Option<ScoreReason>) {
    let cpu = input.get("cpu_usage").and_then(|v| v.parse::<f64>().ok()).unwrap_or(20.0);
    let mem = input.get("mem_usage").and_then(|v| v.parse::<f64>().ok()).unwrap_or(400.0);
    let risk = input.get("risk_score").and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.1);

    let score = ((cpu / 100.0) * 0.4 + (mem / 8192.0) * 0.3 + risk * 0.3).min(1.0);

    let reason = if cpu > 85.0 {
        Some(ScoreReason::HighCpuUsage)
    } else if risk > 0.7 {
        Some(ScoreReason::Custom("High raw risk score in fallback".into()))
    } else {
        None
    };

    if let Some(ref r) = reason {
        println!("[🧠 SHAP fallback] Reason inferred: {}", r.to_string());
    }

    (score, reason)
}
