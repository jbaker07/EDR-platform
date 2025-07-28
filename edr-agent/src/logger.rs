//use chrono::Local;
//use std::fs::{OpenOptions};
//use std::io::Write;
use std::sync::Mutex;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;

lazy_static::lazy_static! {
    static ref LOG_FILE: Mutex<()> = Mutex::new(());
}

pub fn log(msg: &str) {
    println!("{}", msg);
}

// src/logger.rs
pub fn init_logger() {
    use log::LevelFilter;
    use simple_logger::SimpleLogger;
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
}

pub fn log_suppression_decision(module: &str, path: &str, reason: &str) {
    let ts = Utc::now().to_rfc3339();
    let log_line = format!(
        "[🔇 Suppression] [{}] Module={} | Path={} | Reason={}\n",
        ts, module, path, reason
    );

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/edr-agent/logs/suppression_audit.log")
    {
        let _ = file.write_all(log_line.as_bytes());
    }
}