// edr-agent/relay/transmit_or_buffer.rs

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, Debug)]
pub struct TelemetryData {
    pub hostname: String,
    pub timestamp: u64,
    pub data: Vec<String>,
}

const API_URL: &str = "http://your-backend-api-url/telemetry";
const BUFFERED_DATA_PATH: &str = "./buffered_data.json";

// Mutex for safe access in multithreaded context
lazy_static! {
    static ref BUFFER_LOCK: Mutex<()> = Mutex::new(());
}

fn send_telemetry(data: &TelemetryData) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let response = client
        .post(API_URL)
        .json(data)
        .timeout(Duration::from_secs(10))
        .send();

    match response {
        Ok(res) if res.status().is_success() => {
            println!("✅ Telemetry successfully transmitted.");
            Ok(())
        }
        Ok(res) => {
            println!("❌ Backend responded with error: {}", res.status());
            Err(reqwest::Error::new(reqwest::StatusCode::INTERNAL_SERVER_ERROR, "HTTP error"))
        }
        Err(err) => {
            println!("❌ Error transmitting telemetry: {}", err);
            Err(err)
        }
    }
}

fn buffer_data(data: &TelemetryData) {
    let _lock = BUFFER_LOCK.lock().unwrap();

    let mut file = match OpenOptions::new().append(true).create(true).open(BUFFERED_DATA_PATH) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("⚠️ Error opening buffer file: {}", e);
            return;
        }
    };

    if let Ok(serialized) = serde_json::to_string(data) {
        if let Err(e) = writeln!(file, "{}", serialized) {
            eprintln!("⚠️ Failed to write to buffer: {}", e);
        }
    } else {
        eprintln!("⚠️ Failed to serialize telemetry for buffering.");
    }
}

pub fn send_buffered_data() {
    let _lock = BUFFER_LOCK.lock().unwrap();

    if !Path::new(BUFFERED_DATA_PATH).exists() {
        return;
    }

    let file = File::open(BUFFERED_DATA_PATH);
    if file.is_err() {
        eprintln!("⚠️ Failed to open buffered file.");
        return;
    }

    let mut file = file.unwrap();
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        eprintln!("⚠️ Could not read buffered file.");
        return;
    }

    let mut success = true;
    for line in contents.lines() {
        match serde_json::from_str::<TelemetryData>(line) {
            Ok(data) => {
                if send_telemetry(&data).is_err() {
                    success = false;
                }
            }
            Err(e) => {
                eprintln!("⚠️ Malformed line in buffer: {}", e);
                success = false;
            }
        }
    }

    if success {
        std::fs::remove_file(BUFFERED_DATA_PATH).ok();
        println!("🧹 Cleared buffer after successful flush.");
    } else {
        println!("🔁 Retaining buffer due to transmission errors.");
    }
}

pub fn transmit_or_buffer(hostname: &str, timestamp: u64, data: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let telemetry = TelemetryData {
        hostname: hostname.to_string(),
        timestamp,
        data,
    };

    if send_telemetry(&telemetry).is_err() {
        buffer_data(&telemetry);
    }

    Ok(())
}
