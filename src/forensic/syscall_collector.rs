use std::process::Command;
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::sync::mpsc::Sender;

use crate::telemetry_writer::push_syscall_telemetry;
use crate::telemetry_types::SyscallEvent;

fn current_unix_time() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

/// Attempts to parse lines from OpenBSM or DTrace audit log
fn parse_syscall_line(line: &str) -> Option<(u32, String, Vec<String>)> {
    if line.contains("SYSCALL") {
        // Example: "pid 1234: open(\"/etc/passwd\", O_RDONLY)"
        let parts: Vec<&str> = line.splitn(2, ": ").collect();
        if parts.len() == 2 {
            if let Some(pid_str) = parts[0].strip_prefix("pid ") {
                let pid = pid_str.parse::<u32>().ok()?;
                let syscall_and_args = parts[1];
                let syscall_name = syscall_and_args.split('(').next()?.trim().to_string();
                let args_str = syscall_and_args
                    .split_once('(')
                    .map(|(_, args)| args.trim_end_matches(')'))
                    .unwrap_or("");
                let args = args_str.split(',').map(|s| s.trim().to_string()).collect();

                return Some((pid, syscall_name, args));
            }
        }
    }
    None
}

/// Spawns syscall listener thread and sends parsed events into engine
pub fn collect_syscalls(tx: Sender<SyscallEvent>) {
    let mut child = Command::new("praudit")
        .arg("/dev/auditpipe")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to launch praudit for syscall collection");

    let stdout = child.stdout.take().expect("Failed to capture praudit stdout");
    let reader = BufReader::new(stdout);

    thread::spawn(move || {
        for line in reader.lines() {
            if let Ok(raw_line) = line {
                if let Some((pid, syscall, args)) = parse_syscall_line(&raw_line) {
                    let event = SyscallEvent {
                        pid,
                        process_name: "unknown".to_string(), // can enrich later
                        syscall,
                        args,
                        timestamp: current_unix_time(),
                        trust_score: None,
                        semantic_tags: vec![],
                    };
                    if tx.send(event).is_err() {
                        eprintln!("[syscall_collector] Channel send failed");
                    }
                }
            }
        }
    });
}

/// Alternative async writer (used when not channel-based)
pub async fn process_syscall_event(pid: u32, process_name: String, syscall: String, args: Vec<String>) {
    let event = SyscallEvent {
        pid,
        process_name,
        syscall,
        args,
        timestamp: current_unix_time(),
        trust_score: None,
        semantic_tags: vec![],
    };

    if let Err(e) = push_syscall_telemetry(event).await {
        eprintln!("Failed to push syscall telemetry: {:?}", e);
    }
}
