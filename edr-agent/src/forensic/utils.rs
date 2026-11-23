use std::fs;
use std::io;

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
