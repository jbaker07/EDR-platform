use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};

use sysinfo::{PidExt, ProcessExt, System, SystemExt};

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::user_tracker::{get_logged_in_users, UserSession};
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::write_telemetry_record;
use crate::trust_hook::{submit_trust_event, TrustEvent};
use crate::utils::time::now_ts;
use lazy_static::lazy_static;
use parking_lot::Mutex;

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: i32,
    pub ppid: i32,
    pub name: String,
    pub exe: String,
    pub cpu_usage: f32,
    pub memory_usage_kb: u64,
    pub status: String,
    pub user: Option<String>,
    pub user_sessions: Option<Vec<UserSession>>,
    pub command_line: String,
}

#[derive(Default, Debug, Clone)]
struct RiskFlags {
    has_exe: bool,
    has_cmd: bool,
    cwd_unusual: bool,
    exe_deleted: bool,
    is_kernel_name: bool,
    ppid_is_2: bool,
    uid_is_root: bool,
    cpu_hot: bool,
    mem_hot: bool,
    from_tmp_or_devshm: bool,
    suspicious_name: bool,
}

fn read_proc_exe(pid: i32) -> String {
    fs::read_link(format!("/proc/{pid}/exe"))
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn read_proc_cmdline(pid: i32) -> String {
    fs::read_to_string(format!("/proc/{pid}/cmdline"))
        .map(|s| s.replace('\0', " ").trim().to_string())
        .unwrap_or_default()
}

fn read_proc_cwd(pid: i32) -> String {
    fs::read_link(format!("/proc/{pid}/cwd"))
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn looks_like_kernel_name(name: &str) -> bool {
    name.starts_with("kworker/")
        || name.starts_with("ksoftirqd")
        || name.starts_with("rcu_")
        || name.starts_with("migration/")
        || name.starts_with("kblockd")
        || name.starts_with("cpuhp/")
        || name.starts_with("watchdog/")
        || name.contains("mm_percpu_wq")
        || name == "kthreadd"
}

fn path_in_tmp_like(path: &str) -> bool {
    path.starts_with("/tmp/")
        || path == "/tmp"
        || path.starts_with("/dev/shm/")
        || path.starts_with("/var/tmp/")
        || path.starts_with("/run/user/")
}

fn suspicious_basename(name: &str) -> bool {
    let n = name.trim();
    n.starts_with('.') && n.len() > 1 && !n.starts_with("..")
}

fn compute_risk_flags(
    pid: i32,
    ppid: i32,
    uid: u32,
    name: &str,
    cpu_pct: f32,
    mem_kb: u64,
) -> (RiskFlags, String, String, String) {
    let exe = read_proc_exe(pid);
    let cmd = read_proc_cmdline(pid);
    let cwd = read_proc_cwd(pid);

    let has_exe = !exe.is_empty();
    let has_cmd = !cmd.is_empty();
    let exe_deleted = exe.ends_with(" (deleted)");
    let cwd_unusual = cwd.is_empty() || cwd == "/proc" || cwd == "/";

    let is_kernel_name = looks_like_kernel_name(name);
    let ppid_is_2 = ppid == 2;
    let uid_is_root = uid == 0;

    let cpu_hot = cpu_pct > 15.0;
    let mem_hot = mem_kb > 64 * 1024;

    let from_tmp_or_devshm = path_in_tmp_like(&exe) || path_in_tmp_like(&cwd);
    let suspicious_name = suspicious_basename(name);

    let rf = RiskFlags {
        has_exe,
        has_cmd,
        cwd_unusual,
        exe_deleted,
        is_kernel_name,
        ppid_is_2,
        uid_is_root,
        cpu_hot,
        mem_hot,
        from_tmp_or_devshm,
        suspicious_name,
    };

    (rf, exe, cmd, cwd)
}

lazy_static! {
    static ref LAST_SNAPSHOT: Mutex<HashMap<i32, Instant>> = Mutex::new(HashMap::new());
    static ref GNN_LOG_DISABLED: Mutex<bool> = Mutex::new(false);
}

const SNAPSHOT_DEBOUNCE: Duration = Duration::from_secs(600);

fn should_emit_snapshot(pid: i32) -> bool {
    let mut guard = LAST_SNAPSHOT.lock();
    match guard.get(&pid) {
        Some(ts) if ts.elapsed() < SNAPSHOT_DEBOUNCE => false,
        _ => {
            guard.insert(pid, Instant::now());
            true
        }
    }
}

fn is_risky(flags: &RiskFlags) -> bool {
    flags.exe_deleted
        || flags.from_tmp_or_devshm
        || flags.suspicious_name
        || flags.cpu_hot
        || flags.mem_hot
        || (!flags.is_kernel_name && (!flags.has_exe || !flags.has_cmd || flags.cwd_unusual))
}

fn allowed_to_skip(flags: &RiskFlags) -> bool {
    (flags.is_kernel_name || flags.ppid_is_2) && !is_risky(flags)
}

fn open_gnn_log() -> Option<std::fs::File> {
    if *GNN_LOG_DISABLED.lock() {
        return None;
    }

    let base = std::env::var("EDR_OUT_DIR").unwrap_or_else(|_| "/var/tmp/edr-agent".into());
    let dir = PathBuf::from(base).join("json_files");
    if let Err(e) = fs::create_dir_all(&dir) {
        log::warn!("gnn log dir failed: {e}");
        *GNN_LOG_DISABLED.lock() = true;
        return None;
    }

    let path = dir.join("gnn_vector_log.jsonl");
    match fs::OpenOptions::new().create(true).append(true).open(&path) {
        Ok(file) => Some(file),
        Err(e) => {
            log::warn!("gnn log open failed: {e}");
            *GNN_LOG_DISABLED.lock() = true;
            None
        }
    }
}

fn log_snapshot_to_gnn(data: &HashMap<String, String>) {
    if let Some(mut file) = open_gnn_log() {
        if let Ok(line) = serde_json::to_string(data) {
            if let Err(e) = writeln!(file, "{}", line) {
                log::warn!("gnn log write failed: {e}");
            }
        }
    } else {
        push_to_gnn_vector_log(data.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_noise_is_skipped_unless_risky() {
        let mut flags = RiskFlags::default();
        flags.is_kernel_name = true;
        flags.ppid_is_2 = true;
        flags.has_exe = true;
        flags.has_cmd = true;
        assert!(!is_risky(&flags));
        assert!(allowed_to_skip(&flags));

        let mut hot = flags.clone();
        hot.cpu_hot = true;
        assert!(is_risky(&hot));
        assert!(!allowed_to_skip(&hot));
    }

    #[test]
    fn tmp_exec_is_risky() {
        let mut flags = RiskFlags::default();
        flags.has_exe = true;
        flags.has_cmd = true;
        flags.from_tmp_or_devshm = true;
        assert!(is_risky(&flags));
    }
}

/// Gathers all running processes and returns their metadata.
pub fn gather_processes() -> Vec<ProcessInfo> {
    let mut system = System::new_all();
    system.refresh_processes();

    let sessions = get_logged_in_users();
    let mut process_list = Vec::new();

    for (pid, process) in system.processes() {
        let pid_i32 = pid.as_u32() as i32;
        let ppid_i32 = process.parent().map_or(0, |ppid| ppid.as_u32() as i32);
        let user_id = process.user_id().map(|uid| uid.to_string());
        let command_line = process.cmd().join(" ");

        let info = ProcessInfo {
            pid: pid_i32,
            ppid: ppid_i32,
            name: process.name().to_string(),
            exe: process.exe().to_string_lossy().to_string(),
            cpu_usage: process.cpu_usage(),
            // Keep naming consistent (KB). If sysinfo already returns KB, the /1024 is harmlessly small.
            memory_usage_kb: process.memory() / 1024,
            status: format!("{:?}", process.status()),
            user: user_id,
            user_sessions: Some(sessions.clone()),
            command_line,
        };

        process_list.push(info);
    }

    process_list
}

pub fn scan_processes() -> Vec<TelemetryOutput> {
    let processes = gather_processes();
    let mut outputs = Vec::new();

    for proc in processes {
        let uid_num = proc
            .user
            .as_ref()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);

        let (risk_flags, exe, cmdline, cwd) = compute_risk_flags(
            proc.pid,
            proc.ppid,
            uid_num,
            &proc.name,
            proc.cpu_usage,
            proc.memory_usage_kb,
        );

        if allowed_to_skip(&risk_flags) {
            continue;
        }

        let is_risky_entry = is_risky(&risk_flags);
        if !is_risky_entry && !should_emit_snapshot(proc.pid) {
            continue;
        }

        let timestamp = now_ts();
        let mut data = HashMap::new();
        data.insert("pid".into(), proc.pid.to_string());
        data.insert("ppid".into(), proc.ppid.to_string());
        data.insert("name".into(), proc.name.clone());
        data.insert("exe".into(), exe.clone());
        data.insert("command_line".into(), cmdline.clone());
        data.insert("cwd".into(), cwd.clone());
        data.insert("cpu_usage".into(), format!("{:.2}", proc.cpu_usage));
        data.insert("memory_usage_kb".into(), proc.memory_usage_kb.to_string());
        data.insert("status".into(), proc.status.clone());
        data.insert("timestamp".into(), timestamp.to_string());
        data.insert("replay_tag".into(), "process_snapshot".into());
        data.insert("risk_flags".into(), format!("{:?}", risk_flags));

        if let Some(user) = &proc.user {
            data.insert("user".into(), user.clone());
        }
        if let Some(sessions) = &proc.user_sessions {
            data.insert("user_sessions_count".into(), sessions.len().to_string());
        }

        let mut tags = vec!["snapshot".into(), format!("binary::{}", proc.name)];
        let telemetry_signal = "process::snapshot";

        data.insert("category".into(), "process".into());
        data.insert("signal".into(), telemetry_signal.into());
        data.insert("kind".into(), "process.snapshot".into());

        if is_risky_entry {
            data.insert("gnn_escalate".into(), "true".into());
            log_snapshot_to_gnn(&data);
            write_telemetry_record(data.clone());
            crate::modules::replay_writer::store_replay_event(data.clone());

            tags.push("risky".into());
            let event = TrustEvent {
                timestamp,
                pid: proc.pid,
                ppid: proc.ppid,
                uid: uid_num,
                binary_path: exe,
                command_line: cmdline,
                cwd,
                anomaly_type: "process_snapshot".into(),
                component: "process".into(),
                metadata: data.clone(),
                risk_score: 25.0,
                source_module: "process_monitor".into(),
                decay_context: Some("process_snapshot".into()),
                module: Some("process_monitor".into()),
                signal: Some("process_snapshot".into()),
                signal_type: Some("process_snapshot".into()),
                description: Some("Process snapshot flagged as risky".into()),
                score: Some(25.0),
                raw_score: Some(25.0),
                tags: Some(tags.clone()),
            };

            submit_trust_event(event);
        } else {
            data.insert("gnn_escalate".into(), "false".into());
            log_snapshot_to_gnn(&data);
            write_telemetry_record(data.clone());
            crate::modules::replay_writer::store_replay_event(data.clone());
        }

        outputs.push(TelemetryOutput {
            category: "process".into(),
            signal: telemetry_signal.into(),
            confidence: if is_risky_entry { 0.9 } else { 0.6 },
            data,
        });
    }

    outputs
}
