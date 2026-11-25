//! Suspicious IPC detection. Two real checks live here.
//!
//! 1. Runtime socket connects (eBPF, Linux). src/ebpf/suspicious_ipc.c reports
//!    connect() calls to container runtime control sockets such as
//!    /var/run/docker.sock. A process that can talk to one of them can start a
//!    privileged container, so it is root on the host in practice (MITRE
//!    T1611, T1610). Clients that are not known runtime tools get flagged.
//! 2. Ptrace attachments (/proc). A non-zero TracerPid in /proc/<pid>/status
//!    means another process is tracing that one. Debuggers do this, and so do
//!    injectors and credential dumpers (MITRE T1055.008). Tracers that are not
//!    known debuggers get flagged.
//!
//! scan_ipc_activity() runs every telemetry cycle. It starts the eBPF watch
//! once, returns the socket alerts queued since the last cycle, and runs the
//! ptrace scan. The ptrace scan runs whether or not eBPF loaded, because it
//! covers a different behavior and costs one small file read per process.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::{LazyLock, Mutex, PoisonError};

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::replay_writer::store_replay_event;
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::write_telemetry_record;
use crate::trust_hook::{submit_trust_event, TrustEvent};
use crate::utils::time::now_ts;

const PROC_ROOT: &str = "/proc";

// ---- Runtime socket watch: layout shared with src/ebpf/suspicious_ipc.c ----

pub const UNIX_PATH_MAX: usize = 108;
pub const TASK_COMM_LEN: usize = 16;

/// Mirrors struct ipc_connect_event in src/ebpf/suspicious_ipc.c.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IpcConnectEvent {
    pub tgid: u32,
    pub tid: u32,
    pub uid: u32,
    pub comm: [u8; TASK_COMM_LEN],
    pub path: [u8; UNIX_PATH_MAX],
    pub timestamp_ns: u64,
}

impl IpcConnectEvent {
    /// Copies one ring buffer record. None if the record is too short.
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < std::mem::size_of::<Self>() {
            return None;
        }
        // SAFETY: the length is checked above, read_unaligned has no alignment
        // requirement, and any bit pattern is valid for these integer and byte
        // array fields.
        Some(unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const Self) })
    }

    pub fn comm(&self) -> String {
        c_string(&self.comm)
    }

    pub fn path(&self) -> String {
        c_string(&self.path)
    }
}

/// Text up to the first NUL.
fn c_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

/// Container runtime control sockets loaded into the watched_sockets map.
/// /var/run is a symlink to /run on most distros, and the tracepoint sees the
/// path string the caller passed, so both spellings are listed.
pub const RUNTIME_SOCKETS: &[&str] = &[
    // Docker Engine API
    "/var/run/docker.sock",
    "/run/docker.sock",
    // containerd, standalone and the copy that dockerd starts
    "/run/containerd/containerd.sock",
    "/var/run/containerd/containerd.sock",
    "/run/docker/containerd/containerd.sock",
    "/var/run/docker/containerd/containerd.sock",
    // containerd embedded in k3s
    "/run/k3s/containerd/containerd.sock",
    // CRI-O
    "/run/crio/crio.sock",
    "/var/run/crio/crio.sock",
    // rootful Podman API service
    "/run/podman/podman.sock",
    "/var/run/podman/podman.sock",
    // cri-dockerd, the Kubernetes CRI shim for Docker Engine
    "/run/cri-dockerd.sock",
    "/var/run/cri-dockerd.sock",
];

/// Encodes a socket path as a watched_sockets key: the path bytes, then zeros
/// out to 108 bytes. None if the path is empty, holds a NUL, or is too long.
/// The BPF side copies at most 107 path bytes plus a NUL, so a longer path
/// could never match.
pub fn path_key(path: &str) -> Option<[u8; UNIX_PATH_MAX]> {
    let bytes = path.as_bytes();
    if bytes.is_empty() || bytes.len() >= UNIX_PATH_MAX || bytes.contains(&0) {
        return None;
    }
    let mut key = [0u8; UNIX_PATH_MAX];
    key[..bytes.len()].copy_from_slice(bytes);
    Some(key)
}

/// Programs that use runtime sockets in normal operation, matched on comm. A
/// trailing '*' matches a prefix. comm is the task name a process gives itself
/// (15 characters at most), so anything can claim these names. This list cuts
/// noise. It is not a security boundary.
pub const RUNTIME_CLIENT_ALLOWLIST: &[(&str, &str)] = &[
    ("docker", "Docker CLI"),
    ("dockerd", "Docker daemon, talks to containerd"),
    ("docker-compose", "Compose, standalone or as a CLI plugin"),
    ("docker-buildx", "Buildx CLI plugin"),
    ("containerd", "containerd daemon"),
    (
        "containerd-shim*",
        "per container shim, such as containerd-shim-runc-v2",
    ),
    ("ctr", "containerd CLI"),
    ("nerdctl", "containerd CLI"),
    ("crictl", "CRI debug CLI"),
    ("kubelet", "Kubernetes node agent, talks to the CRI socket"),
    ("crio", "CRI-O daemon"),
    ("cri-dockerd", "CRI shim for Docker Engine"),
    ("podman", "Podman CLI and API service"),
    ("conmon", "Podman container monitor"),
];

/// Tracers that attach for debugging or profiling, matched on the tracer's
/// comm. Same caveat as above: a renamed binary can claim these names.
pub const DEBUGGER_ALLOWLIST: &[(&str, &str)] = &[
    ("gdb", "GNU debugger"),
    ("gdbserver", "gdb remote stub"),
    ("lldb", "LLVM debugger"),
    ("lldb-server", "LLDB remote stub"),
    ("strace", "syscall tracer"),
    ("ltrace", "library call tracer"),
    ("perf", "Linux profiler"),
    ("dlv", "Go debugger"),
    ("rr", "record and replay debugger"),
    ("criu", "checkpoint and restore, freezes tasks with ptrace"),
];

/// Returns the allowlist reason when comm matches an entry.
pub fn allowlist_match(comm: &str, list: &[(&'static str, &'static str)]) -> Option<&'static str> {
    list.iter()
        .find(|(name, _)| match name.strip_suffix('*') {
            Some(prefix) => comm.starts_with(prefix),
            None => comm == *name,
        })
        .map(|(_, reason)| *reason)
}

/// A flagged runtime socket connect, collapsed per process, uid and path.
#[derive(Clone, Debug)]
pub struct SocketAlert {
    pub event: IpcConnectEvent,
    pub exe: Option<String>,
    pub cmdline: Option<String>,
    pub ppid: Option<u32>,
    pub count: u32,
}

impl SocketAlert {
    /// Reads exe, command line and parent from /proc right away, while the
    /// client is most likely still running. Each is None once it has exited.
    pub fn new(event: IpcConnectEvent, proc_root: &Path) -> Self {
        let status = read_status(proc_root, event.tgid).unwrap_or_default();
        Self {
            exe: read_exe(proc_root, event.tgid),
            cmdline: read_cmdline(proc_root, event.tgid),
            ppid: parse_status_u32(&status, "PPid:"),
            event,
            count: 1,
        }
    }
}

/// Distinct alerts held between telemetry cycles.
const MAX_PENDING_SOCKET_ALERTS: usize = 256;

/// Filled by the eBPF reader thread, drained by scan_ipc_activity().
static PENDING_SOCKET_ALERTS: Mutex<Vec<SocketAlert>> = Mutex::new(Vec::new());

/// Adds a flagged connect to the queue. A repeat from the same process, uid
/// and path bumps its count, so a client polling the socket in a loop does not
/// flood the pipeline. New entries past the cap are dropped.
pub fn add_socket_alert(queue: &mut Vec<SocketAlert>, alert: SocketAlert) {
    let key = (alert.event.tgid, alert.event.uid, alert.event.path);
    if let Some(existing) = queue
        .iter_mut()
        .find(|a| (a.event.tgid, a.event.uid, a.event.path) == key)
    {
        existing.count += 1;
    } else if queue.len() < MAX_PENDING_SOCKET_ALERTS {
        queue.push(alert);
    }
}

/// Object path. Same lookup order as events_reader::resolve_bpf in the agent
/// binary.
#[cfg(target_os = "linux")]
fn bpf_object_path() -> String {
    if let Ok(path) = std::env::var("EDR_BPF_IPC_OBJ") {
        return path;
    }
    if let Ok(dir) = std::env::var("EDR_BPF_DIR") {
        return format!("{}/suspicious_ipc.bpf.o", dir.trim_end_matches('/'));
    }
    "src/ebpf/suspicious_ipc.bpf.o".to_string()
}

/// Loads the BPF object, fills watched_sockets, attaches to
/// syscalls/sys_enter_connect and calls on_event for every record until
/// keep_running returns false. The agent and harness/ipc_live both use this.
#[cfg(target_os = "linux")]
pub fn run_socket_watch(
    obj_path: &str,
    watch_list: &[&str],
    keep_running: impl Fn() -> bool,
    mut on_event: impl FnMut(&IpcConnectEvent),
) -> anyhow::Result<()> {
    use anyhow::{anyhow, Context};
    use libbpf_rs::{MapFlags, ObjectBuilder, RingBufferBuilder};
    use std::time::Duration;

    let mut obj = ObjectBuilder::default()
        .open_file(obj_path)
        .with_context(|| format!("open {obj_path}"))?
        .load()
        .with_context(|| format!("load {obj_path}"))?;

    let watched = obj
        .map("watched_sockets")
        .ok_or_else(|| anyhow!("map watched_sockets missing"))?;
    for path in watch_list {
        let key =
            path_key(path).ok_or_else(|| anyhow!("watch path {path:?} is empty or too long"))?;
        watched
            .update(&key, &[1u8], MapFlags::ANY)
            .with_context(|| format!("add {path} to watched_sockets"))?;
    }

    // Dropping the link detaches the program, so it lives as long as the loop.
    let _link = obj
        .prog_mut("trace_runtime_socket_connect")
        .ok_or_else(|| anyhow!("program trace_runtime_socket_connect missing"))?
        .attach_tracepoint("syscalls", "sys_enter_connect")
        .context("attach syscalls/sys_enter_connect")?;

    let events = obj
        .map("ipc_events")
        .ok_or_else(|| anyhow!("map ipc_events missing"))?;
    let mut builder = RingBufferBuilder::new();
    builder.add(events, |data: &[u8]| {
        if let Some(event) = IpcConnectEvent::from_bytes(data) {
            on_event(&event);
        }
        0
    })?;
    let ring = builder.build()?;

    while keep_running() {
        match ring.poll(Duration::from_millis(200)) {
            Ok(()) => {}
            // A signal interrupted the wait. Poll again.
            Err(libbpf_rs::Error::System(errno)) if errno == libc::EINTR => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}

/// Starts the runtime socket watch on a background thread, once. When the
/// object file is missing it logs that at info level and stays off. Load
/// errors (no root, no BTF) are logged as warnings.
#[cfg(target_os = "linux")]
pub fn start_ebpf_ipc_abuse_watch() {
    static START: std::sync::Once = std::sync::Once::new();
    START.call_once(|| {
        let obj_path = bpf_object_path();
        if !Path::new(&obj_path).exists() {
            log::info!("suspicious_ipc: {obj_path} not found, runtime socket watch is off");
            return;
        }
        let spawned = std::thread::Builder::new()
            .name("ipc_socket_watch".into())
            .spawn(move || {
                let result = run_socket_watch(
                    &obj_path,
                    RUNTIME_SOCKETS,
                    || true,
                    |event| {
                        if allowlist_match(&event.comm(), RUNTIME_CLIENT_ALLOWLIST).is_some() {
                            return;
                        }
                        let alert = SocketAlert::new(*event, Path::new(PROC_ROOT));
                        let mut queue = PENDING_SOCKET_ALERTS
                            .lock()
                            .unwrap_or_else(PoisonError::into_inner);
                        add_socket_alert(&mut queue, alert);
                    },
                );
                if let Err(e) = result {
                    log::warn!("suspicious_ipc: runtime socket watch stopped: {e:#}");
                }
            });
        if let Err(e) = spawned {
            log::warn!("suspicious_ipc: could not start the runtime socket watch thread: {e}");
        }
    });
}

// ---- Ptrace scan over /proc ----

/// Processes read per cycle. A cycle that hits the cap resumes after the last
/// pid it read.
pub const MAX_PIDS_PER_SCAN: usize = 4096;

/// A process seen under ptrace for the first time.
#[derive(Clone, Debug, PartialEq)]
pub struct PtraceAttach {
    pub traced_pid: u32,
    pub traced_comm: String,
    pub tracer_pid: u32,
    pub tracer_comm: String,
    pub tracer_uid: Option<u32>,
    pub tracer_ppid: Option<u32>,
    pub tracer_exe: Option<String>,
    /// Debugger allowlist reason. None when the tracer is not a known debugger.
    pub allowed_by: Option<&'static str>,
}

/// Walks /proc for TracerPid. Each (traced, tracer) pair is returned once while
/// the attachment lasts.
#[derive(Default)]
pub struct PtraceScanner {
    reported: HashSet<(u32, u32)>,
    resume_after: u32,
}

impl PtraceScanner {
    pub fn scan(&mut self, proc_root: &Path, max_pids: usize) -> Vec<PtraceAttach> {
        let mut pids = list_pids(proc_root);
        pids.sort_unstable();
        // Start after the last pid read when the previous cycle hit the cap.
        let start = pids.partition_point(|&pid| pid <= self.resume_after);
        pids.rotate_left(start);
        let capped = pids.len() > max_pids;
        pids.truncate(max_pids);
        self.resume_after = if capped {
            pids.last().copied().unwrap_or(0)
        } else {
            0
        };

        let mut found = Vec::new();
        let mut traced_now = HashSet::new();
        for &pid in &pids {
            // Gone since the directory listing.
            let Some(status) = read_status(proc_root, pid) else {
                continue;
            };
            let tracer_pid = match parse_tracer_pid(&status) {
                Some(tracer) if tracer != 0 => tracer,
                _ => continue,
            };
            traced_now.insert((pid, tracer_pid));
            if self.reported.contains(&(pid, tracer_pid)) {
                continue;
            }
            // A tracer that exits is detached by the kernel. Look again next
            // cycle instead of reporting a tracer with no name.
            let Some(tracer_comm) = read_comm(proc_root, tracer_pid) else {
                continue;
            };
            let tracer_status = read_status(proc_root, tracer_pid).unwrap_or_default();
            self.reported.insert((pid, tracer_pid));
            found.push(PtraceAttach {
                traced_pid: pid,
                traced_comm: read_comm(proc_root, pid).unwrap_or_default(),
                tracer_pid,
                allowed_by: allowlist_match(&tracer_comm, DEBUGGER_ALLOWLIST),
                tracer_comm,
                tracer_uid: parse_status_u32(&tracer_status, "Uid:"),
                tracer_ppid: parse_status_u32(&tracer_status, "PPid:"),
                tracer_exe: read_exe(proc_root, tracer_pid),
            });
        }

        // Forget pairs that ended among the pids read this cycle, so the set
        // stays small and a later attach by the same pair is reported again.
        let read_now: HashSet<u32> = pids.iter().copied().collect();
        self.reported
            .retain(|pair| !read_now.contains(&pair.0) || traced_now.contains(pair));
        found
    }
}

/// First number after `field` in a /proc/<pid>/status body, for example
/// "TracerPid:" or "Uid:" (whose first number is the real uid).
pub fn parse_status_u32(status: &str, field: &str) -> Option<u32> {
    status
        .lines()
        .find_map(|line| line.strip_prefix(field))
        .and_then(|rest| rest.split_whitespace().next())
        .and_then(|value| value.parse().ok())
}

/// TracerPid from a /proc/<pid>/status body. 0 means not traced. None if the
/// field is missing or not a number.
pub fn parse_tracer_pid(status: &str) -> Option<u32> {
    parse_status_u32(status, "TracerPid:")
}

/// Numeric entries of /proc, which are the process ids.
fn list_pids(proc_root: &Path) -> Vec<u32> {
    let Ok(entries) = fs::read_dir(proc_root) else {
        return Vec::new();
    };
    entries
        .filter_map(|entry| entry.ok()?.file_name().to_str()?.parse().ok())
        .collect()
}

fn read_status(proc_root: &Path, pid: u32) -> Option<String> {
    fs::read_to_string(proc_root.join(pid.to_string()).join("status")).ok()
}

fn read_comm(proc_root: &Path, pid: u32) -> Option<String> {
    let comm = fs::read_to_string(proc_root.join(pid.to_string()).join("comm")).ok()?;
    Some(comm.trim_end_matches('\n').to_string())
}

fn read_exe(proc_root: &Path, pid: u32) -> Option<String> {
    let exe = fs::read_link(proc_root.join(pid.to_string()).join("exe")).ok()?;
    Some(exe.display().to_string())
}

/// Command line with the NUL separators turned into spaces.
fn read_cmdline(proc_root: &Path, pid: u32) -> Option<String> {
    let raw = fs::read(proc_root.join(pid.to_string()).join("cmdline")).ok()?;
    // Masked before it reaches a finding (crate::redact).
    let text = crate::redact::cmdline_from_proc(&raw);
    (!text.is_empty()).then_some(text)
}

// ---- Findings and the shared pipeline ----

/// Fixed rule severities on the 0 to 100 TrustEvent scale. They rank these two
/// rules against each other and are not measured probabilities.
const RUNTIME_SOCKET_RISK: f32 = 60.0;
const PTRACE_ATTACH_RISK: f32 = 50.0;

/// Everything one finding sends downstream.
pub struct Finding {
    pub output: TelemetryOutput,
    pub trust: TrustEvent,
}

pub fn socket_finding(alert: &SocketAlert) -> Finding {
    let event = &alert.event;
    let comm = event.comm();
    let path = event.path();
    let reason = format!(
        "{comm} (pid {}, uid {}) called connect() on container runtime socket {path} and is not a known runtime client",
        event.tgid, event.uid
    );

    let mut data = HashMap::new();
    data.insert("pid".into(), event.tgid.to_string());
    data.insert("tid".into(), event.tid.to_string());
    data.insert("uid".into(), event.uid.to_string());
    data.insert("comm".into(), comm);
    data.insert("socket_path".into(), path);
    data.insert("connect_count".into(), alert.count.to_string());
    data.insert("binary_path".into(), alert.exe.clone().unwrap_or_default());
    data.insert(
        "command_line".into(),
        alert.cmdline.clone().unwrap_or_default(),
    );
    data.insert("mitre".into(), "T1611,T1610".into());

    build_finding(
        "suspicious_ipc_runtime_socket",
        event.tgid,
        alert.ppid,
        event.uid,
        RUNTIME_SOCKET_RISK,
        "runtime_socket",
        reason,
        data,
    )
}

/// The tracer is the actor, so the TrustEvent carries the tracer's pid and uid.
pub fn ptrace_finding(attach: &PtraceAttach) -> Finding {
    let reason = format!(
        "{} (pid {}) is being traced by {} (pid {}), which is not a known debugger",
        attach.traced_comm, attach.traced_pid, attach.tracer_comm, attach.tracer_pid
    );

    let mut data = HashMap::new();
    data.insert("pid".into(), attach.tracer_pid.to_string());
    data.insert("traced_pid".into(), attach.traced_pid.to_string());
    data.insert("traced_comm".into(), attach.traced_comm.clone());
    data.insert("tracer_pid".into(), attach.tracer_pid.to_string());
    data.insert("tracer_comm".into(), attach.tracer_comm.clone());
    if let Some(uid) = attach.tracer_uid {
        data.insert("uid".into(), uid.to_string());
    }
    data.insert(
        "binary_path".into(),
        attach.tracer_exe.clone().unwrap_or_default(),
    );
    data.insert("mitre".into(), "T1055.008".into());

    build_finding(
        "suspicious_ipc_ptrace_attach",
        attach.tracer_pid,
        attach.tracer_ppid,
        // u32::MAX marks an unknown uid, as the TrustEvent callers here did before.
        attach.tracer_uid.unwrap_or(u32::MAX),
        PTRACE_ATTACH_RISK,
        "ptrace",
        reason,
        data,
    )
}

#[allow(clippy::too_many_arguments)]
fn build_finding(
    signal: &str,
    pid: u32,
    ppid: Option<u32>,
    uid: u32,
    risk: f32,
    kind_tag: &str,
    reason: String,
    mut data: HashMap<String, String>,
) -> Finding {
    let ts = now_ts();
    data.insert("timestamp".into(), ts.to_string());
    data.insert("signal".into(), signal.into());
    data.insert("replay_tag".into(), signal.into());
    data.insert("reason".into(), reason.clone());
    data.insert("gnn_escalate".into(), "true".into());

    let trust = TrustEvent {
        timestamp: ts,
        pid: pid as i32,
        // 0 when the process exited before /proc was read.
        ppid: ppid.unwrap_or(0) as i32,
        uid,
        binary_path: data.get("binary_path").cloned().unwrap_or_default(),
        command_line: data.get("command_line").cloned().unwrap_or_default(),
        cwd: String::new(),
        anomaly_type: "ipc_abuse".into(),
        component: "suspicious_ipc_monitor".into(),
        metadata: data.clone(),
        risk_score: risk,
        source_module: "suspicious_ipc_monitor".into(),
        decay_context: Some(signal.into()),
        module: Some("suspicious_ipc_monitor".into()),
        signal: Some(signal.into()),
        signal_type: Some("process".into()),
        score: Some(risk),
        raw_score: Some(risk),
        tags: Some(vec!["ipc".into(), "abuse".into(), kind_tag.into()]),
        description: Some(reason),
    };

    // The signal becomes a record tag downstream. "suspicious_ipc" in it is what
    // the ipc_abuse rule in services/trust_vector.rs matches.
    let output = TelemetryOutput {
        category: "process".into(),
        signal: signal.into(),
        confidence: risk / 100.0,
        data,
    };
    Finding { output, trust }
}

/// Writes a finding to the same sinks the other modules use and returns its
/// TelemetryOutput for the passive pipeline.
fn emit(finding: Finding) -> TelemetryOutput {
    write_telemetry_record(finding.output.data.clone());
    push_to_gnn_vector_log(finding.output.data.clone());
    if let Err(e) = store_replay_event(finding.output.data.clone()) {
        log::warn!("suspicious_ipc: replay write failed: {e}");
    }
    submit_trust_event(finding.trust);
    finding.output
}

static PTRACE_SCANNER: LazyLock<Mutex<PtraceScanner>> = LazyLock::new(Default::default);

/// Called every telemetry cycle from telemetry.rs.
pub fn scan_ipc_activity() -> Vec<TelemetryOutput> {
    #[cfg(target_os = "linux")]
    start_ebpf_ipc_abuse_watch();

    let socket_alerts = std::mem::take(
        &mut *PENDING_SOCKET_ALERTS
            .lock()
            .unwrap_or_else(PoisonError::into_inner),
    );
    let attaches = PTRACE_SCANNER
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .scan(Path::new(PROC_ROOT), MAX_PIDS_PER_SCAN);

    let mut outputs = Vec::new();
    for alert in &socket_alerts {
        outputs.push(emit(socket_finding(alert)));
    }
    for attach in attaches.iter().filter(|a| a.allowed_by.is_none()) {
        outputs.push(emit(ptrace_finding(attach)));
    }
    outputs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn event(tgid: u32, uid: u32, comm: &str, path: &str) -> IpcConnectEvent {
        let mut e = IpcConnectEvent {
            tgid,
            tid: tgid,
            uid,
            comm: [0; TASK_COMM_LEN],
            path: [0; UNIX_PATH_MAX],
            timestamp_ns: 1,
        };
        e.comm[..comm.len()].copy_from_slice(comm.as_bytes());
        e.path[..path.len()].copy_from_slice(path.as_bytes());
        e
    }

    fn alert(e: IpcConnectEvent) -> SocketAlert {
        SocketAlert {
            event: e,
            exe: None,
            cmdline: None,
            ppid: None,
            count: 1,
        }
    }

    #[test]
    fn event_layout_matches_the_c_struct() {
        assert_eq!(std::mem::size_of::<IpcConnectEvent>(), 144);
        assert_eq!(std::mem::offset_of!(IpcConnectEvent, uid), 8);
        assert_eq!(std::mem::offset_of!(IpcConnectEvent, comm), 12);
        assert_eq!(std::mem::offset_of!(IpcConnectEvent, path), 28);
        assert_eq!(std::mem::offset_of!(IpcConnectEvent, timestamp_ns), 136);
    }

    #[test]
    fn event_from_bytes_reads_a_record_and_rejects_short_ones() {
        let mut buf = vec![0u8; 144];
        buf[0..4].copy_from_slice(&4321u32.to_le_bytes());
        buf[4..8].copy_from_slice(&4322u32.to_le_bytes());
        buf[8..12].copy_from_slice(&1000u32.to_le_bytes());
        buf[12..19].copy_from_slice(b"python3");
        buf[28..48].copy_from_slice(b"/var/run/docker.sock");
        buf[136..144].copy_from_slice(&99u64.to_le_bytes());

        let e = IpcConnectEvent::from_bytes(&buf).unwrap();
        assert_eq!(
            (e.tgid, e.tid, e.uid, e.timestamp_ns),
            (4321, 4322, 1000, 99)
        );
        assert_eq!(e.comm(), "python3");
        assert_eq!(e.path(), "/var/run/docker.sock");
        assert!(IpcConnectEvent::from_bytes(&buf[..143]).is_none());
    }

    #[test]
    fn path_key_is_zero_padded_to_108_bytes() {
        let key = path_key("/var/run/docker.sock").unwrap();
        assert_eq!(key.len(), 108);
        assert_eq!(&key[..20], b"/var/run/docker.sock");
        assert!(key[20..].iter().all(|&b| b == 0));
    }

    #[test]
    fn path_key_rejects_empty_nul_and_too_long_paths() {
        assert!(path_key("").is_none());
        assert!(path_key("/run/a\0b").is_none());
        assert!(path_key(&"a".repeat(108)).is_none());
        assert!(path_key(&"a".repeat(107)).is_some());
    }

    #[test]
    fn default_watch_list_fits_the_map() {
        assert!(RUNTIME_SOCKETS.iter().all(|p| path_key(p).is_some()));
        // max_entries of watched_sockets in suspicious_ipc.c
        assert!(RUNTIME_SOCKETS.len() <= 64);
    }

    #[test]
    fn runtime_client_allowlist_decisions() {
        let list = RUNTIME_CLIENT_ALLOWLIST;
        assert_eq!(allowlist_match("docker", list), Some("Docker CLI"));
        assert!(allowlist_match("kubelet", list).is_some());
        // containerd-shim-runc-v2 shows up as the 15 character comm "containerd-shim".
        assert!(allowlist_match("containerd-shim", list).is_some());
        assert!(allowlist_match("python3", list).is_none());
        assert!(allowlist_match("curl", list).is_none());
        // Exact match unless the entry ends in '*'.
        assert!(allowlist_match("dockerx", list).is_none());
        assert!(allowlist_match("docke", list).is_none());
        assert!(allowlist_match("", list).is_none());
    }

    #[test]
    fn debugger_allowlist_decisions() {
        let list = DEBUGGER_ALLOWLIST;
        assert_eq!(allowlist_match("gdb", list), Some("GNU debugger"));
        assert!(allowlist_match("strace", list).is_some());
        assert!(allowlist_match("python3", list).is_none());
        assert!(allowlist_match("notstrace", list).is_none());
        assert!(allowlist_match("strace2", list).is_none());
    }

    #[test]
    fn tracer_pid_parser_handles_normal_missing_and_garbage_input() {
        let traced = "Name:\tsleep\nState:\tt (tracing stop)\nTgid:\t42\nPPid:\t1\nTracerPid:\t4242\nUid:\t1000\t1000\t1000\t1000\n";
        assert_eq!(parse_tracer_pid(traced), Some(4242));
        assert_eq!(parse_status_u32(traced, "Uid:"), Some(1000));
        assert_eq!(parse_status_u32(traced, "PPid:"), Some(1));
        assert_eq!(parse_tracer_pid("Name:\tsleep\nTracerPid:\t0\n"), Some(0));
        // missing field
        assert_eq!(parse_tracer_pid("Name:\tsleep\nPid:\t42\n"), None);
        assert_eq!(parse_tracer_pid(""), None);
        // garbage
        assert_eq!(parse_tracer_pid("TracerPid:\tabc\n"), None);
        assert_eq!(parse_tracer_pid("TracerPid:\n"), None);
        assert_eq!(parse_tracer_pid("TracerPid:\t-1\n"), None);
    }

    #[test]
    fn socket_alerts_collapse_per_process_uid_and_path() {
        let mut queue = Vec::new();
        add_socket_alert(
            &mut queue,
            alert(event(10, 0, "python3", "/var/run/docker.sock")),
        );
        add_socket_alert(
            &mut queue,
            alert(event(10, 0, "python3", "/var/run/docker.sock")),
        );
        add_socket_alert(
            &mut queue,
            alert(event(10, 0, "python3", "/run/docker.sock")),
        );
        add_socket_alert(
            &mut queue,
            alert(event(11, 0, "python3", "/var/run/docker.sock")),
        );
        assert_eq!(queue.len(), 3);
        assert_eq!(queue[0].count, 2);

        for pid in 100..(100 + MAX_PENDING_SOCKET_ALERTS as u32) {
            add_socket_alert(&mut queue, alert(event(pid, 0, "x", "/run/docker.sock")));
        }
        assert_eq!(queue.len(), MAX_PENDING_SOCKET_ALERTS);
    }

    #[test]
    fn findings_keep_the_ipc_tags_trust_scoring_matches() {
        let f = socket_finding(&alert(event(10, 0, "python3", "/var/run/docker.sock")));
        assert!(f.output.signal.contains("suspicious_ipc"));
        assert_eq!(f.output.data["socket_path"], "/var/run/docker.sock");
        assert_eq!(f.trust.anomaly_type, "ipc_abuse");
        let tags = f.trust.tags.unwrap();
        assert!(tags.contains(&"ipc".to_string()) && tags.contains(&"abuse".to_string()));
    }

    fn fake_proc(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("edr_ipc_{name}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        // A non-numeric entry, like /proc/sys, must be skipped.
        fs::create_dir_all(root.join("sys")).unwrap();
        root
    }

    fn write_proc(root: &Path, pid: u32, comm: &str, tracer: u32) {
        let dir = root.join(pid.to_string());
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("comm"), format!("{comm}\n")).unwrap();
        fs::write(
            dir.join("status"),
            format!("Name:\t{comm}\nPid:\t{pid}\nPPid:\t1\nTracerPid:\t{tracer}\nUid:\t1000\t1000\t1000\t1000\n"),
        )
        .unwrap();
    }

    #[test]
    fn ptrace_scan_reports_each_pair_once_and_forgets_ended_pairs() {
        let root = fake_proc("dedupe");
        write_proc(&root, 100, "sleep", 200);
        write_proc(&root, 200, "injector", 0);
        write_proc(&root, 300, "sleep", 400);
        write_proc(&root, 400, "gdb", 0);
        let mut scanner = PtraceScanner::default();

        let first = scanner.scan(&root, 100);
        assert_eq!(first.len(), 2);
        let injector = first.iter().find(|a| a.tracer_pid == 200).unwrap();
        assert_eq!(injector.traced_pid, 100);
        assert_eq!(injector.traced_comm, "sleep");
        assert_eq!(injector.tracer_comm, "injector");
        assert_eq!(injector.tracer_uid, Some(1000));
        assert_eq!(injector.allowed_by, None);
        let gdb = first.iter().find(|a| a.tracer_pid == 400).unwrap();
        assert_eq!(gdb.allowed_by, Some("GNU debugger"));

        // Same pairs next cycle: nothing new.
        assert!(scanner.scan(&root, 100).is_empty());

        // Detach, then attach again: reported again.
        write_proc(&root, 100, "sleep", 0);
        assert!(scanner.scan(&root, 100).is_empty());
        write_proc(&root, 100, "sleep", 200);
        assert_eq!(scanner.scan(&root, 100).len(), 1);

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn ptrace_scan_waits_for_a_tracer_it_cannot_name() {
        let root = fake_proc("gone");
        // Tracer 900 has no /proc entry.
        write_proc(&root, 100, "sleep", 900);
        let mut scanner = PtraceScanner::default();
        assert!(scanner.scan(&root, 100).is_empty());
        write_proc(&root, 900, "injector", 0);
        assert_eq!(scanner.scan(&root, 100).len(), 1);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn ptrace_scan_caps_pids_per_cycle_and_resumes() {
        let root = fake_proc("cap");
        for pid in 1..=5 {
            write_proc(&root, pid, "sleep", 9);
        }
        write_proc(&root, 9, "injector", 0);
        let mut scanner = PtraceScanner::default();

        // pids sorted: 1 2 3 4 5 9, two per cycle
        let tracked =
            |found: Vec<PtraceAttach>| found.iter().map(|a| a.traced_pid).collect::<Vec<_>>();
        assert_eq!(tracked(scanner.scan(&root, 2)), vec![1, 2]);
        assert_eq!(tracked(scanner.scan(&root, 2)), vec![3, 4]);
        assert_eq!(tracked(scanner.scan(&root, 2)), vec![5]);
        // Wraps to 1 and 2, already reported.
        assert!(scanner.scan(&root, 2).is_empty());
        let _ = fs::remove_dir_all(&root);
    }
}
