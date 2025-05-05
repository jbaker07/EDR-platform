// src/forensic/network_monitor.rs

use serde::Serialize;
use std::collections::HashMap;
use std::net::{IpAddr};
use std::time::SystemTime;
use sysinfo::{System, SystemExt, ProcessExt};
use std::process::Command;

#[derive(Serialize, Debug)]
pub struct ConnectionInfo {
    pub pid: i32,
    pub process_name: String,
    pub local_address: String,
    pub remote_address: String,
    pub protocol: String,
    pub status: String,
    pub timestamp: u64,
}

pub fn collect_network_connections() -> Vec<ConnectionInfo> {
    let mut results = Vec::new();
    let timestamp = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let output = Command::new("netstat")
        .arg("-tunp") // TCP/UDP + program name (Linux only)
        .output();

    if let Ok(output) = output {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            for line in stdout.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 7 {
                    let local = parts[3].to_string();
                    let remote = parts[4].to_string();
                    let status = parts[5].to_string();
                    let pid_program = parts[6];
                    let mut pid = -1;
                    let mut pname = "unknown".to_string();

                    if let Some((p, n)) = pid_program.split_once("/") {
                        pid = p.parse().unwrap_or(-1);
                        pname = n.to_string();
                    }

                    results.push(ConnectionInfo {
                        pid,
                        process_name: pname,
                        local_address: local,
                        remote_address: remote,
                        protocol: parts[0].to_string(),
                        status,
                        timestamp,
                    });
                }
            }
        }
    }

    results
}
