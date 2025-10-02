// edr-agent/forensic/relay.rs

use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use reqwest::blocking::Client;
use serde::Serialize;

/// Struct used to wrap telemetry before transmission or buffering
#[derive(Serialize)]
pub struct RelayEnvelope<T> {
    pub hostname: String,
    pub timestamp: u64,
    pub payload: T,
}

// Constants
const RELAY_BUFFER_PATH: &str = "/tmp/edr_relay_queue.log";
const RELAY_ENDPOINT: &str = "http://127.0.0.1:8000/api/relay"; // Replace with prod endpoint if needed

/// Attempts to transmit payload immediately; if that fails, queues locally
pub fn transmit_or_buffer<T: Serialize>(hostname: &str, timestamp: u64, payload: T) -> Result<()> {
    let envelope = RelayEnvelope {
        hostname: hostname.to_string(),
        timestamp,
        payload,
    };

    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;

    let res = client.post(RELAY_ENDPOINT).json(&envelope).send();

    match res {
        Ok(resp) if resp.status().is_success() => {
            println!("📡 Telemetry sent to relay [{}]", resp.status());
            Ok(())
        }
        _ => {
            println!("⚠️ Relay unreachable → queuing to disk");
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

/// Tries to flush queued payloads from local buffer file
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
        fs::remove_file(RELAY_BUFFER_PATH)?; // Clear queue after successful flush
        println!("✅ Flushed {} entries from local queue.", successful);
    }

    Ok(())
}
