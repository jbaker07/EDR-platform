// src/forensic/forensic_engine.rs

use crate::forensic::file_watcher::{start_file_watch, FileEvent};
use crate::forensic::process_monitor::{gather_processes, ProcessSnapshot};
use crate::forensic::user_tracker::{get_logged_in_users, UserSession};
use crate::forensic::network_monitor::{scan_connections, NetworkConnection};
use crate::forensic::relay::transmit_or_buffer;

use hostname::get;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn start_forensic_engine() {
    let hostname = get().unwrap_or_default().to_string_lossy().to_string();
    let ts_now = || SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    // --- Spawn: Process Monitoring ---
    thread::spawn({
        let hostname = hostname.clone();
        move || loop {
            let timestamp = ts_now();
            let processes = gather_processes();
            if let Err(e) = transmit_or_buffer(&hostname, timestamp, processes) {
                eprintln!("⚠️ Failed to transmit process snapshot: {}", e);
            }
            thread::sleep(Duration::from_secs(15));
        }
    });

    // --- Spawn: User Session Tracker ---
    thread::spawn({
        let hostname = hostname.clone();
        move || loop {
            let timestamp = ts_now();
            let users = get_logged_in_users();
            if let Err(e) = transmit_or_buffer(&hostname, timestamp, users) {
                eprintln!("⚠️ Failed to transmit user sessions: {}", e);
            }
            thread::sleep(Duration::from_secs(30));
        }
    });

    // --- Spawn: Network Connection Scanner ---
    thread::spawn({
        let hostname = hostname.clone();
        move || loop {
            let timestamp = ts_now();
            let conns = scan_connections();
            if let Err(e) = transmit_or_buffer(&hostname, timestamp, conns) {
                eprintln!("⚠️ Failed to transmit net connections: {}", e);
            }
            thread::sleep(Duration::from_secs(20));
        }
    });

    // --- Spawn: File Watcher ---
    thread::spawn({
        let hostname = hostname.clone();
        let rx = start_file_watch(vec![PathBuf::from("/tmp"), PathBuf::from("/etc")]);

        move || {
            for event in rx {
                if let Err(e) = transmit_or_buffer(&hostname, event.timestamp, vec![event]) {
                    eprintln!("⚠️ Failed to transmit file event: {}", e);
                }
            }
        }
    });

    println!("🧠 Forensic engine is now running...");
}
