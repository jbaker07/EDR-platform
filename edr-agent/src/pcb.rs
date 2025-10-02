// src/pcb.rs
//! PCB collector + literal /proc snapshots (Linux).
//! Pure collection only — NO scoring/tagging.
//!
//! Structured usage:
//!   let snaps = crate::pcb::collect_for_records(&records);
//!
//! Literal dump usage (evidence):
//!   let _paths = crate::pcb::dump_records_raw(&records, "state/pcb_raw", /*include_sensitive=*/false)?;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use once_cell::sync::OnceCell;
use std::sync::Mutex;

use crate::telemetry::TelemetryRecord;

// ========== Structured snapshots (analytics) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PcbSnapshot {
    pub ts_ms: u128,
    pub pid: i32,
    pub uid: Option<u32>,
    pub exe: Option<String>,
    pub state: Option<char>,    // R,S,D,T,t,Z,W,I
    pub threads: Option<u32>,   // Threads
    pub vmrss_kb: Option<u64>,  // VmRSS
    pub vmsize_kb: Option<u64>, // VmSize
    pub fds: Option<u32>,       // count of /proc/<pid>/fd
    pub cap_eff: Option<u64>,   // CapEff bitmap (hex)
    pub seccomp: Option<u8>,    // 0/1/2
    pub cpu_pct: Option<f64>,   // % across all cores (delta-based)
    pub read_bps: Option<f64>,  // bytes/sec (delta)
    pub write_bps: Option<f64>, // bytes/sec (delta)
    pub tcp_socks: u32,         // TCP + TCP6 entries
    pub udp_socks: u32,         // UDP + UDP6 entries
    pub unix_socks: u32,        // UNIX sockets
    pub in_container: bool,     // cgroup hint
}

#[derive(Default)]
struct CpuState {
    last_proc: HashMap<i32, u64>, // utime+stime per pid (jiffies)
    last_sys: u64,                // total jiffies
    ncpu: usize,
}

#[derive(Default)]
struct IoState {
    last: HashMap<i32, (u64, u64, u128)>, // read_bytes, write_bytes, ts_ms
}

pub struct PcbCollector {
    cpu: CpuState,
    io: IoState,
}

static COLLECTOR: OnceCell<Mutex<PcbCollector>> = OnceCell::new();

impl PcbCollector {
    pub fn new() -> Self {
        let ncpu = count_cpus();
        let last_sys = read_proc_stat_total().unwrap_or(0);
        Self {
            cpu: CpuState {
                last_proc: HashMap::new(),
                last_sys,
                ncpu,
            },
            io: IoState {
                last: HashMap::new(),
            },
        }
    }

    pub fn sample_records(&mut self, recs: &[TelemetryRecord]) -> Vec<PcbSnapshot> {
        let now_ms = now_ms();
        let mut out = Vec::with_capacity(recs.len());
        for r in recs {
            if r.pid <= 0 {
                continue;
            }
            if let Some(mut s) = self.sample_pid(r.pid, some_uid_hint(r), now_ms) {
                if s.exe.is_none() && !r.binary_path.is_empty() {
                    s.exe = Some(r.binary_path.clone());
                }
                out.push(s);
            }
        }
        out
    }

    pub fn append_ndjson(path: &str, snaps: &[PcbSnapshot]) -> std::io::Result<()> {
        if let Some(dir) = Path::new(path).parent() {
            let _ = fs::create_dir_all(dir);
        }
        let mut f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        for s in snaps {
            let line = serde_json::to_string(s).unwrap_or_else(|_| "{}".into());
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }

    fn sample_pid(&mut self, pid: i32, uid_hint: Option<u32>, now_ms: u128) -> Option<PcbSnapshot> {
        if !Path::new(&format!("/proc/{pid}")).exists() {
            return None;
        }
        let mut snap = PcbSnapshot {
            ts_ms: now_ms,
            pid,
            uid: None,
            exe: None,
            state: None,
            threads: None,
            vmrss_kb: None,
            vmsize_kb: None,
            fds: None,
            cap_eff: None,
            seccomp: None,
            cpu_pct: None,
            read_bps: None,
            write_bps: None,
            tcp_socks: 0,
            udp_socks: 0,
            unix_socks: 0,
            in_container: false,
        };

        read_status(pid, &mut snap);
        if snap.uid.is_none() {
            snap.uid = uid_hint;
        }
        snap.exe = snap
            .exe
            .or_else(|| read_link_string(format!("/proc/{pid}/exe")));
        snap.fds = read_fd_count(pid);

        let (rb, wb) = read_io_totals(pid);
        if let (Some(r), Some(w)) = (rb, wb) {
            if let Some((pr, pw, pts)) = self.io.last.insert(pid, (r, w, now_ms)) {
                let dt = (now_ms.saturating_sub(pts)) as f64 / 1000.0;
                if dt > 0.0 {
                    snap.read_bps = Some((r.saturating_sub(pr)) as f64 / dt);
                    snap.write_bps = Some((w.saturating_sub(pw)) as f64 / dt);
                }
            }
        }

        let (tcp, tcp6) = (
            read_netfile_count(pid, "tcp"),
            read_netfile_count(pid, "tcp6"),
        );
        let (udp, udp6) = (
            read_netfile_count(pid, "udp"),
            read_netfile_count(pid, "udp6"),
        );
        snap.tcp_socks = tcp.saturating_add(tcp6);
        snap.udp_socks = udp.saturating_add(udp6);
        snap.unix_socks = read_netfile_count(pid, "unix");

        snap.in_container = read_cgroup_container_hint(pid);
        snap.cpu_pct = self.compute_cpu_pct(pid);

        Some(snap)
    }

    fn compute_cpu_pct(&mut self, pid: i32) -> Option<f64> {
        let proc_j = read_stat_utime_stime(pid)?;
        let sys_j = read_proc_stat_total()?;

        let prev_proc = self.cpu.last_proc.insert(pid, proc_j).unwrap_or(proc_j);
        let prev_sys = std::mem::replace(&mut self.cpu.last_sys, sys_j);

        let dproc = proc_j.saturating_sub(prev_proc);
        let dsys = sys_j.saturating_sub(prev_sys);
        if dsys == 0 {
            return None;
        }

        let ncpu = if self.cpu.ncpu == 0 { 1 } else { self.cpu.ncpu };
        Some((dproc as f64) / (dsys as f64) * (ncpu as f64) * 100.0)
    }
}

// ---------- Public structured APIs ----------

pub fn collect_for_records(records: &[TelemetryRecord]) -> Vec<PcbSnapshot> {
    let c = COLLECTOR.get_or_init(|| Mutex::new(PcbCollector::new()));
    let mut guard = c.lock().expect("pcb collector poisoned");
    guard.sample_records(records)
}

pub fn append_snapshots_ndjson(path: &str, snaps: &[PcbSnapshot]) -> std::io::Result<()> {
    PcbCollector::append_ndjson(path, snaps)
}

// ========== Literal /proc snapshots (evidence) ==========

/// Dump raw /proc state for each record's PID under `root_dir/<ts>-pid-<pid>/`.
/// If `include_sensitive` is false, we omit /proc/<pid>/{environ,cmdline}.
pub fn dump_records_raw(
    records: &[TelemetryRecord],
    root_dir: &str,
    include_sensitive: bool,
) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    fs::create_dir_all(root_dir).ok();
    let ts = now_ms();
    for r in records {
        if r.pid <= 0 {
            continue;
        }
        if let Some(p) = dump_one_pid(r.pid, ts, root_dir, include_sensitive)? {
            out.push(p);
        }
    }
    Ok(out)
}

fn dump_one_pid(
    pid: i32,
    ts_ms: u128,
    root_dir: &str,
    include_sensitive: bool,
) -> std::io::Result<Option<PathBuf>> {
    let proc_root = format!("/proc/{pid}");
    if !Path::new(&proc_root).exists() {
        return Ok(None);
    }
    let snap_dir = Path::new(root_dir).join(format!("{}-pid-{}", ts_ms, pid));
    fs::create_dir_all(&snap_dir).ok();

    // small helper: copy/cap text file if present
    let mut copy = |rel: &str, cap: usize| -> std::io::Result<()> {
        let src = format!("{proc_root}/{rel}");
        let dst = snap_dir.join(rel.replace('/', "_"));
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent).ok();
        }
        if let Some(s) = read_capped_to_string(&src, cap) {
            fs::write(dst, s)?;
        }
        Ok(())
    };

    // 1) text-ish files
    let _ = copy("status", 256 * 1024);
    let _ = copy("stat", 256 * 1024);
    let _ = copy("io", 256 * 1024);
    let _ = copy("limits", 256 * 1024);
    let _ = copy("sched", 512 * 1024);
    let _ = copy("wchan", 64 * 1024);
    let _ = copy("cgroup", 256 * 1024);
    let _ = copy("comm", 64 * 1024);

    // Prefer smaps_rollup; fallback to capped smaps.
    let rollup = format!("{proc_root}/smaps_rollup");
    if Path::new(&rollup).exists() {
        let _ = copy("smaps_rollup", 4 * 1024 * 1024);
    } else {
        let _ = copy("smaps", 2 * 1024 * 1024); // cap to avoid huge files
    }

    // Sensitive: cmdline/environ
    if include_sensitive {
        let _ = copy("cmdline", 128 * 1024);
        let _ = copy("environ", 256 * 1024);
    }

    // 2) symlink targets: exe, cwd, root
    let exe = read_link_string(format!("{proc_root}/exe")).unwrap_or_default();
    let cwd = read_link_string(format!("{proc_root}/cwd")).unwrap_or_default();
    let root = read_link_string(format!("{proc_root}/root")).unwrap_or_default();
    fs::write(snap_dir.join("exe.link"), exe.as_bytes()).ok();
    fs::write(snap_dir.join("cwd.link"), cwd.as_bytes()).ok();
    fs::write(snap_dir.join("root.link"), root.as_bytes()).ok();

    // 3) fd list (resolve symlinks)
    let mut fd_lines = String::new();
    if let Ok(rd) = fs::read_dir(format!("{proc_root}/fd")) {
        for entry in rd.flatten() {
            if let Ok(name) = entry.file_name().into_string() {
                let target = fs::read_link(entry.path())
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_else(|_| "<unreadable>".into());
                fd_lines.push_str(&format!("fd {} -> {}\n", name, target));
            }
        }
    }
    fs::write(snap_dir.join("fd_list.txt"), fd_lines).ok();

    // 4) per-pid network tables
    for nf in ["tcp", "tcp6", "udp", "udp6", "unix"] {
        let src = format!("{proc_root}/net/{nf}");
        let dst = snap_dir.join(format!("net_{}", nf));
        if let Some(s) = read_capped_to_string(&src, 2 * 1024 * 1024) {
            fs::write(dst, s).ok();
        }
    }

    Ok(Some(snap_dir))
}

// ========== Low-level readers shared by both ==========

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn some_uid_hint(r: &TelemetryRecord) -> Option<u32> {
    if r.uid == 0 && r.pid == 0 {
        None
    } else {
        Some(r.uid)
    }
}

fn read_link_string(p: impl AsRef<Path>) -> Option<String> {
    fs::read_link(p).ok()?.into_os_string().into_string().ok()
}

fn read_capped_to_string(path: &str, cap: usize) -> Option<String> {
    let mut f = fs::File::open(path).ok()?;
    let mut buf = String::with_capacity(cap.min(64 * 1024));
    let mut handle = f.take(cap as u64);
    handle.read_to_string(&mut buf).ok()?;
    Some(buf)
}

fn read_status(pid: i32, out: &mut PcbSnapshot) {
    if let Ok(s) = fs::read_to_string(format!("/proc/{pid}/status")) {
        for line in s.lines() {
            if let Some(rest) = line.strip_prefix("State:") {
                out.state = rest.chars().find(|c| !c.is_whitespace());
            } else if let Some(rest) = line.strip_prefix("Threads:") {
                out.threads = rest
                    .split_whitespace()
                    .last()
                    .and_then(|t| t.parse::<u32>().ok());
            } else if let Some(rest) = line.strip_prefix("VmRSS:") {
                out.vmrss_kb = parse_kb(rest);
            } else if let Some(rest) = line.strip_prefix("VmSize:") {
                out.vmsize_kb = parse_kb(rest);
            } else if let Some(rest) = line.strip_prefix("CapEff:") {
                out.cap_eff = u64::from_str_radix(rest.trim(), 16).ok();
            } else if let Some(rest) = line.strip_prefix("Seccomp:") {
                out.seccomp = rest
                    .split_whitespace()
                    .last()
                    .and_then(|t| t.parse::<u8>().ok());
            } else if let Some(rest) = line.strip_prefix("Uid:") {
                out.uid = rest
                    .split_whitespace()
                    .next()
                    .and_then(|t| t.parse::<u32>().ok());
            }
        }
    }
}

fn parse_kb(line: &str) -> Option<u64> {
    line.split_whitespace()
        .filter_map(|t| t.parse::<u64>().ok())
        .next()
}

fn read_fd_count(pid: i32) -> Option<u32> {
    let rd = fs::read_dir(format!("/proc/{pid}/fd")).ok()?;
    let mut c: u32 = 0;
    for _ in rd.flatten() {
        c = c.saturating_add(1);
        if c >= 100_000 {
            break;
        }
    }
    Some(c)
}

fn read_io_totals(pid: i32) -> (Option<u64>, Option<u64>) {
    if let Ok(s) = fs::read_to_string(format!("/proc/{pid}/io")) {
        let mut r = None;
        let mut w = None;
        for line in s.lines() {
            if let Some(rest) = line.strip_prefix("read_bytes:") {
                r = rest
                    .split_whitespace()
                    .last()
                    .and_then(|t| t.parse::<u64>().ok());
            } else if let Some(rest) = line.strip_prefix("write_bytes:") {
                w = rest
                    .split_whitespace()
                    .last()
                    .and_then(|t| t.parse::<u64>().ok());
            }
        }
        return (r, w);
    }
    (None, None)
}

fn read_netfile_count(pid: i32, name: &str) -> u32 {
    if let Ok(s) = fs::read_to_string(format!("/proc/{pid}/net/{name}")) {
        return s.lines().skip(1).filter(|l| !l.trim().is_empty()).count() as u32;
    }
    0
}

fn read_cgroup_container_hint(pid: i32) -> bool {
    if let Ok(s) = fs::read_to_string(format!("/proc/{pid}/cgroup")) {
        let lower = s.to_ascii_lowercase();
        return lower.contains("docker")
            || lower.contains("containerd")
            || lower.contains("kubepods")
            || lower.contains("libpod");
    }
    false
}

fn count_cpus() -> usize {
    if let Ok(s) = fs::read_to_string("/proc/stat") {
        let mut n = 0usize;
        for line in s.lines() {
            if line.starts_with("cpu") && line.len() > 3 && line.as_bytes()[3].is_ascii_digit() {
                n += 1;
            }
        }
        if n > 0 {
            return n;
        }
    }
    1
}

fn read_proc_stat_total() -> Option<u64> {
    let s = fs::read_to_string("/proc/stat").ok()?;
    for line in s.lines() {
        if line.starts_with("cpu ") {
            let mut total = 0u64;
            for t in line.split_whitespace().skip(1) {
                if let Ok(v) = t.parse::<u64>() {
                    total = total.saturating_add(v);
                }
            }
            return Some(total);
        }
    }
    None
}

fn read_stat_utime_stime(pid: i32) -> Option<u64> {
    // /proc/<pid>/stat: last ')' then fields; 14:utime, 15:stime
    let mut s = String::new();
    fs::File::open(format!("/proc/{pid}/stat"))
        .ok()?
        .read_to_string(&mut s)
        .ok()?;
    let rparen = s.rfind(')')?;
    let after = &s[rparen + 2..]; // skip ") "
    let mut it = after.split_whitespace();
    let utime = it.nth(11)?.parse::<u64>().ok()?; // field 14
    let stime = it.next()?.parse::<u64>().ok()?; // field 15
    Some(utime.saturating_add(stime))
}
