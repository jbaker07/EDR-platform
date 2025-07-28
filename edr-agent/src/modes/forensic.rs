use chrono::Utc;
use serde::Serialize;
use std::collections::HashMap;

use crate::forensic::{
    auth_watch::monitor_auth_events,        // Forensic-level function
    file_watcher::monitor_file_events,      // Forensic-level function
    network_monitor::collect_network_stats, // Forensic-level function
    container_engine::analyze_container_activity, // Forensic-level function
};

use crate::modules::{
    net_watch::log_network_events,          // Modules-level function
    persistence_watch::scan_persistence_indicators, // Modules-level function
    privilege_monitor::scan_privilege_escalations, // Modules-level function
    process_monitor::monitor_processes,     // Modules-level function
    registry_watch::scan_registry_entries,  // Modules-level function
    user_tracker::track_user_sessions,      // Modules-level function
    usb_monitor::scan_usb_devices,          // Modules-level function
    logon_tracker::track_logons,            // Modules-level function
    cloud_tracker::cloud_telemetry,         // Modules-level function
    job_sched_monitor::track_job_schedulers, // Modules-level function
    script_monitor::monitor_script_execs,   // Modules-level function
    mem_scan::scan_memory_anomalies,        // Modules-level function
};

use hostname;
use sysinfo::{System, SystemExt, ProcessExt};

#[derive(Debug, Serialize)]
pub struct ForensicTelemetry {
    pub hostname: String,
    pub timestamp: String,
    pub events: Vec<String>,
}

/// Get a stable hostname identifier
fn get_hostname() -> String {
    hostname::get().unwrap_or_default().to_string_lossy().to_string()
}

/// Get a list of all active PIDs on the system
fn list_running_pids() -> Vec<i32> {
    let mut system = System::new_all();
    system.refresh_processes();
    system.processes().keys().map(|pid| pid.as_u32() as i32).collect()
}

// src/modes/forensic.rs

pub fn start_forensic_mode() {
    println!("🔍 Forensic mode activated!");
    // Actual logic goes here
}

/// Collect full forensic telemetry from all modules
pub fn collect_forensic_telemetry() -> Vec<String> {
    let hostname = get_hostname();
    let timestamp = Utc::now().to_rfc3339();
    let pids = list_running_pids();

    let mut all_events = Vec::new();

    let collectors: Vec<(&str, Box<dyn Fn() -> Vec<String>>)> = vec![
        ("auth_watch", Box::new(|| monitor_auth_events().unwrap_or_default())),        // Forensic-level function
        ("file_watcher", Box::new(|| monitor_file_events())),                           // Forensic-level function
        ("network_monitor", Box::new(|| collect_network_stats())),                     // Forensic-level function
        ("container_engine", Box::new(|| analyze_container_activity())),               // Forensic-level function
        ("net_watch", Box::new(|| log_network_events())),                               // Modules-level function
        ("persistence_watch", Box::new(|| scan_persistence_indicators())),             // Modules-level function
        ("privilege_monitor", Box::new(|| scan_privilege_escalations())),              // Modules-level function
        ("process_monitor", Box::new(|| monitor_processes(&pids))),                    // Modules-level function
        ("registry_watch", Box::new(|| scan_registry_entries())),                      // Modules-level function
        ("user_tracker", Box::new(|| track_user_sessions())),                          // Modules-level function
        ("usb_monitor", Box::new(|| scan_usb_devices())),                              // Modules-level function
        ("logon_tracker", Box::new(|| track_logons())),                                // Modules-level function
        ("cloud_tracker", Box::new(|| cloud_telemetry())),                             // Modules-level function
        ("job_sched_monitors", Box::new(|| track_job_schedulers())),                   // Modules-level function
        ("script_monitor", Box::new(|| monitor_script_execs())),                       // Modules-level function
        ("mem_scan", Box::new(|| scan_memory_anomalies(&pids))),                       // Modules-level function
    ];

    for (module, func) in collectors {
        let result = std::panic::catch_unwind(|| func());
        match result {
            Ok(events) => {
                for e in events {
                    all_events.push(format!("[{}] {}", module, e));
                }
            }
            Err(_) => {
                eprintln!("⚠️ Forensic module {} panicked — skipping.", module);
            }
        }
    }

    // Optional final payload if needed elsewhere
    let telemetry = ForensicTelemetry {
        hostname,
        timestamp,
        events: all_events.clone(),
    };

    println!(
        "✅ [forensic] Collected {} events from {} modules.",
        telemetry.events.len(),
        collectors.len()
    );

    all_events
}
