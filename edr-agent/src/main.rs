mod config;
mod forensic;

use anyhow::Context;
use config::load_and_verify_policy;
use hostname::get;
use serde::Serialize;
use std::{thread, time::Duration};
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use sysinfo::{System, SystemExt, ProcessExt, PidExt};

// Forensic modules
use forensic::auth_watch::start_auth_watch;
use forensic::registry_watch::start_registry_watch;
use forensic::process_monitor::start_process_monitor;
use forensic::user_tracker::start_user_tracker;
use forensic::file_watcher::{start_file_watch, FileEvent};
use forensic::relay::{transmit_or_buffer, flush_buffer};

#[derive(Serialize)]
struct ProcessInfo {
    pid: i32,
    name: String,
    cmd: String,
    memory: u64,
    cpu_usage: f32,
    status: String,
}

#[derive(Serialize)]
struct Telemetry {
    hostname: String,
    timestamp: u64,
    processes: Vec<ProcessInfo>,
}

fn process_to_info(pid: sysinfo::Pid, proc_: &sysinfo::Process) -> ProcessInfo {
    ProcessInfo {
        pid: pid.as_u32() as i32,
        name: proc_.name().to_string(),
        cmd: proc_.cmd().join(" "),
        memory: proc_.memory(),
        cpu_usage: proc_.cpu_usage(),
        status: format!("{:?}", proc_.status()),
    }
}

fn main() -> anyhow::Result<()> {
    // --- Step 1: Load policy ---
    let policy = load_and_verify_policy("signed_policy.json")
        .context("Failed to load or verify agent policy")?;
    println!("✅ Loaded policy: {:?}", policy);

    // --- Step 2: Resolve hostname ---
    let hostname = get()?
        .into_string()
        .unwrap_or_else(|_| "unknown".into());

    // --- Step 3: Flush buffer if any ---
    flush_buffer().ok(); // Optional buffer flush

    // --- Step 4: Forensic mode startup ---
    if policy.mode == "forensic" {
        println!("🔬 Forensic mode enabled");
        start_auth_watch();
        start_registry_watch();
        start_process_monitor();
        start_user_tracker();

        let rx = start_file_watch(vec![PathBuf::from("/tmp"), PathBuf::from("/etc")]);
        thread::spawn(move || {
            for event in rx {
                println!("📁 FILE EVENT: {:?}", event);
                transmit_or_buffer(&hostname, current_timestamp(), event).ok();
            }
        });
    } else {
        println!("⚡ Minimal mode enabled");
    }

    // --- Step 5: Main loop ---
    loop {
        let timestamp = current_timestamp();

        let mut sys = System::new_all();
        sys.refresh_all();
        let processes = sys
            .processes()
            .iter()
            .map(|(pid, pr)| process_to_info(*pid, pr))
            .collect::<Vec<_>>();

        let payload = Telemetry {
            hostname: hostname.clone(),
            timestamp,
            processes,
        };

        transmit_or_buffer(&hostname, timestamp, payload).ok();

        thread::sleep(Duration::from_secs(policy.collection_interval));
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
