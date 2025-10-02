use futures::io as futures_io;
use futures_util::io as futures_util_io; // Alias this too
use std::fs;
use std::io::{self, BufRead, BufReader}; // Use this as your standard `io`
use tokio::io as tokio_io; // Alias `tokio::io` if needed // Also alias this to avoid collision

/// Pulls dmesg or journalctl output
pub fn get_kernel_logs() -> Result<String, std::io::Error> {
    use std::process::Command;
    let output = Command::new("dmesg").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Looks for memory range violations or SMEP/SMAP errors
pub fn check_protected_mem_access() -> bool {
    if let Ok(logs) = get_kernel_logs() {
        logs.contains("SMEP") || logs.contains("SMAP") || logs.contains("page fault")
    } else {
        false
    }
}

/// Scans syscall patterns for unexpected behaviors (e.g., missing audit context, ghost PIDs)
pub fn detect_syscall_anomalies() -> bool {
    // You may want to parse `/proc/sys/kernel/printk` or syscall logs here
    // Placeholder logic for now:
    false
}

/// Extracts ppid, uid, and gid from /proc/[pid]/status
pub fn extract_status_fields(pid: i32) -> (i32, i32, i32) {
    let status_path = format!("/proc/{}/status", pid);
    let file = fs::File::open(&status_path);
    if file.is_err() {
        return (0, 0, 0);
    }

    let reader = BufReader::new(file.unwrap());
    let mut ppid = 0;
    let mut uid = 0;
    let mut gid = 0;

    for line in reader.lines().flatten() {
        if line.starts_with("PPid:") {
            ppid = line
                .split_whitespace()
                .nth(1)
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
        } else if line.starts_with("Uid:") {
            uid = line
                .split_whitespace()
                .nth(1)
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
        } else if line.starts_with("Gid:") {
            gid = line
                .split_whitespace()
                .nth(1)
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
        }
    }

    (ppid, uid, gid)
}

/// Determines the container type based on cgroup paths (e.g., Docker, containerd, etc.)
pub fn read_cgroup_type(pid: i32) -> String {
    let cgroup_path = format!("/proc/{}/cgroup", pid);
    let content = fs::read_to_string(&cgroup_path);
    if let Ok(lines) = content {
        for line in lines.lines() {
            if line.contains("docker") {
                return "docker".to_string();
            } else if line.contains("kubepods") {
                return "kubernetes".to_string();
            } else if line.contains("containerd") {
                return "containerd".to_string();
            }
        }
    }
    "host".to_string()
}

pub fn read_proc_value(pid: u32, field: &str) -> Result<String, io::Error> {
    let path = format!("/proc/{}/{}", pid, field);
    let content = fs::read_to_string(&path)?;
    Ok(content.trim().to_string())
}
/// Extracts PPid and Uid from `/proc/[pid]/status`
/// Returns (ppid, uid) as (i32, u32)
pub fn get_ppid_and_uid(pid: u32) -> (i32, u32) {
    let status_path = format!("/proc/{}/status", pid);
    if let Ok(contents) = std::fs::read_to_string(status_path) {
        let mut ppid: i32 = 0;
        let mut uid: u32 = 0;

        for line in contents.lines() {
            if line.starts_with("PPid:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    ppid = val.parse::<i32>().unwrap_or(0);
                }
            } else if line.starts_with("Uid:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    uid = val.parse::<u32>().unwrap_or(0);
                }
            }
        }

        return (ppid, uid);
    }

    (0, 0) // fallback if file is missing or unreadable
}
