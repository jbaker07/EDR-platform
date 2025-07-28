use std::os::unix::net::UnixStream;
use std::io::BufRead;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::trust_hook;
use crate::replay_writer::{generate_replay_event, persist_replay_event};

const IPC_SOCKET: &str = "/tmp/edr_ipc.sock";

#[derive(Debug, Serialize, Deserialize)]
struct SyscallEvent {
    pid: u32,
    syscall_id: u32,
    syscall_name: String,
}

pub fn start_syscall_monitor(hostname: &str) {
    let hostname = hostname.to_string();

    thread::spawn(move || {
        loop {
            match UnixStream::connect(IPC_SOCKET) {
                Ok(stream) => {
                    println!("[🔍 Syscall Monitor] Connected to eBPF ring buffer...");
                    let reader = BufReader::new(stream);

                    for line in reader.lines().flatten() {
                        if let Ok(event) = serde_json::from_str::<SyscallEvent>(&line) {
                            handle_syscall_event(&hostname, &event);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[⚠️ Syscall Monitor] Failed to connect to IPC socket: {}", e);
                    thread::sleep(Duration::from_secs(5));
                }
            }
        }
    });
}

fn handle_syscall_event(hostname: &str, event: &SyscallEvent) {
    // Trust impact scoring
    let base_risk = match event.syscall_name.as_str() {
        "execve" | "clone" | "fork" => 0.6,
        "open" | "openat" => 0.4,
        "unlink" | "rename" => 0.5,
        "connect" => 0.7,
        "setuid" => 0.8,
        "mount" => 0.9,
        _ => 0.2,
    };

    let trust_payload = trust_hook::generate_trust_payload(base_risk, 40000, 12.0);

    println!(
        "[🧠 Syscall Monitor] PID {} invoked {} (ID {}) → {}",
        event.pid, event.syscall_name, event.syscall_id, trust_payload
    );

    // Replay persistence
    if let Some(replay) = generate_replay_event("syscall_monitor", hostname, &event) {
        let _ = persist_replay_event(&replay);
    }
}
