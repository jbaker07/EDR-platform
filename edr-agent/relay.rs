// src/forensic/relay.rs

use std::fs::{self, OpenOptions};
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use std::time::Duration;

use reqwest::blocking::Client;
use serde::Serialize;
use anyhow::Result;

/// Struct used to serialize telemetry bundles from main loop
#[derive(Serialize)]
pub struct RelayEnvelope<T> {
    pub hostname: String,
    pub timestamp: u64,
    pub payload: T,
}

const RELAY_BUFFER_PATH: &str = "/tmp/edr_relay_queue.log";
const RELAY_ENDPOINT: &str = "http://127.0.0.1:8000/api/relay"; // Change this to production relay

/// Attempts to transmit payload immediately, else saves to local buffer
pub fn transmit_or_buffer<T: Serialize>(hostname: &str, timestamp: u64, payload: T) -> Result<()> {
    let envelope = RelayEnvelope {
        hostname: hostname.to_string(),
        timestamp,
        payload,
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let res = client.post(RELAY_ENDPOINT).json(&envelope).send();

    match res {
        Ok(resp) if resp.status().is_success() => {
            println!("📡 Sent payload to relay → {}", resp.status());
            Ok(())
        }
        _ => {
            println!("⚠️ Relay unreachable, queuing locally.");
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(RELAY_BUFFER_PATH)?;
            let line = serde_json::to_string(&envelope)?;
            writeln!(file, "{}", line)?;
            Ok(())
        }
    }
}

/// Attempts to flush the local relay buffer
pub fn flush_buffer() -> Result<()> {
    if !Path::new(RELAY_BUFFER_PATH).exists() {
        return Ok(());
    }

    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;
    let file = fs::File::open(RELAY_BUFFER_PATH)?;
    let reader = BufReader::new(file);
    let mut successful = 0;

    for line in reader.lines() {
        if let Ok(json) = line {
            let parsed: serde_json::Value = serde_json::from_str(&json)?;
            let res = client.post(RELAY_ENDPOINT).json(&parsed).send();

            if let Ok(resp) = res {
                if resp.status().is_success() {
                    successful += 1;
                }
            }
        }
    }

    if successful > 0 {
        fs::remove_file(RELAY_BUFFER_PATH)?;
        println!("✅ Flushed {} entries from relay queue.", successful);
    }

    Ok(())
}
