use std::time::{SystemTime, UNIX_EPOCH};
use hostname::get;
use reqwest::blocking::Client;
use serde::Serialize;
use sysinfo::{
    System, SystemExt, ProcessExt, PidExt, Pid, Process,
};

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

fn process_to_info(pid: Pid, proc_: &Process) -> ProcessInfo {
    ProcessInfo {
        pid: pid.as_u32() as i32,
        name: proc_.name().to_string(),
        cmd: proc_.cmd().join(" "),
        memory: proc_.memory(),
        cpu_usage: proc_.cpu_usage(),
        status: format!("{:?}", proc_.status()),
    }
}

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let hostname = get()
        .unwrap()
        .into_string()
        .unwrap_or_else(|_| "unknown".to_string());

    let mut sys = System::new_all();
    sys.refresh_all();

    let processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, proc_)| process_to_info(*pid, proc_))
        .collect();

    let payload = Telemetry {
        hostname,
        timestamp,
        processes,
    };

    let client = Client::new();
    let res = client
        .post("http://127.0.0.1:8000/api/telemetry")
        .json(&payload)
        .send();

    match res {
        Ok(response) => println!("✅ Telemetry sent: {}", response.status()),
        Err(e) => eprintln!("❌ Failed to send telemetry: {}", e),
    }
}
