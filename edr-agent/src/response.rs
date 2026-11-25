// src/response.rs
// Response actions behind the agent API: kill a process, block egress to an address, and
// isolate the host. Every action needs a reason or an incident id, can run as a dry run,
// is allowed only when the signed policy lists it, and is written to an audit log.
//
// kill_proc
//   Refuses pid 0 and 1, negative pids, the agent itself, a thread id that is not the
//   leader of its process, and kernel threads (PF_KTHREAD in /proc/<pid>/stat). The
//   process is opened with pidfd_open and its start time is checked again after the open,
//   so a pid that exited and was reused in between is never signalled. Then SIGKILL goes
//   through pidfd_send_signal, which cannot reach a different process.
//
// block_egress, isolate_host, release
//   Rules live in one nftables table the agent owns (inet edr_agent), rebuilt as a whole
//   in one `nft -f` transaction on every change, so a half applied rule set never exists.
//   Loopback addresses (127.0.0.0/8 and ::1, matched by address, not by the lo
//   interface, because traffic to any of the host's own addresses also runs over lo),
//   the management ports (the policy's list plus the agent API port) and the backend
//   address stay reachable. iptables is the fallback when nft is missing, with
//   chains EDR_EGRESS, EDR_ISO_OUT and EDR_ISO_IN. The rules outlive the agent on
//   purpose: killing the agent does not undo an isolation. release with all=true removes
//   the table.
//
//   Each block's address sits in a named nftables set of its own, and its rule drops
//   traffic to that set. With expires_in_sec the element gets an nftables timeout, so
//   the kernel removes it on time and the rule then matches nothing, whether or not
//   the agent still runs. Isolation works the same way through the sets iso4 and iso6
//   (0.0.0.0/0 and ::/0), without a timeout unless the request gives one, because an
//   isolation should outlive the agent. A rebuild gives each element only the time it
//   has left. iptables has no element timeouts, so it refuses an expiry.
use std::sync::RwLock;

use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::utils::time::now_ts;

pub const TABLE: &str = "edr_agent";
const PF_KTHREAD: u64 = 0x0020_0000;
const MAX_REASON: usize = 500;
pub const ACTIONS: &[&str] = &["kill_proc", "block_egress", "isolate_host"];

/// An action result: the HTTP status and the JSON body.
#[derive(Debug)]
pub struct Outcome {
    pub status: u16,
    pub body: Value,
}

impl Outcome {
    fn ok(body: Value) -> Self {
        Self { status: 200, body }
    }
    fn err(status: u16, msg: impl Into<String>) -> Self {
        Self {
            status,
            body: json!({ "ok": false, "error": msg.into() }),
        }
    }
}

// ---- Requests ----

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KillRequest {
    pub pid: i64,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub incident_id: Option<String>,
    #[serde(default)]
    pub dry_run: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlockRequest {
    /// An address or a network in CIDR form.
    pub dst: String,
    #[serde(default)]
    pub port: Option<u16>,
    /// tcp or udp, needed with a port. Default tcp.
    #[serde(default)]
    pub proto: Option<String>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub incident_id: Option<String>,
    /// The kernel removes the block after this many seconds.
    #[serde(default)]
    pub expires_in_sec: Option<u64>,
    #[serde(default)]
    pub dry_run: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IsolateRequest {
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub incident_id: Option<String>,
    /// None (the default) keeps the isolation until a release, past the agent's life.
    #[serde(default)]
    pub expires_in_sec: Option<u64>,
    #[serde(default)]
    pub dry_run: bool,
}

/// The longest expiry a block or an isolation may ask for: 30 days.
pub const MAX_EXPIRY_SEC: u64 = 30 * 86_400;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseRequest {
    /// A block id from block_egress.
    #[serde(default)]
    pub id: Option<String>,
    /// End the isolation.
    #[serde(default)]
    pub isolation: bool,
    /// Remove every rule and the table.
    #[serde(default)]
    pub all: bool,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub dry_run: bool,
}

// ---- Configuration ----

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Firewall {
    Nft,
    Iptables,
}

#[derive(Clone, Debug)]
pub struct ResponseConfig {
    pub firewall: Option<Firewall>,
    pub nft: String,
    pub iptables: String,
    pub ip6tables: String,
    /// Ports that stay reachable when the host is isolated: the policy's management ports
    /// and the agent API port.
    pub keep_ports: Vec<u16>,
    /// The backend the agent reports to, kept reachable while isolated.
    pub backend: Vec<(IpAddr, u16)>,
    /// Actions the signed policy allows.
    pub allowed: Vec<String>,
    pub audit_path: PathBuf,
}

fn find_tool(names: &[&str]) -> Option<String> {
    for dir in ["/usr/sbin", "/sbin", "/usr/bin", "/bin"] {
        for n in names {
            let p = format!("{dir}/{n}");
            if Path::new(&p).exists() {
                return Some(p);
            }
        }
    }
    None
}

impl ResponseConfig {
    /// EDR_FIREWALL picks nft or iptables (default: nft when installed, else iptables).
    pub fn from_env(keep_ports: Vec<u16>, backend: Vec<(IpAddr, u16)>, allowed: Vec<String>) -> Self {
        let nft = find_tool(&["nft"]);
        let ipt = find_tool(&["iptables"]);
        let firewall = match std::env::var("EDR_FIREWALL").unwrap_or_default().as_str() {
            "nft" => nft.as_ref().map(|_| Firewall::Nft),
            "iptables" => ipt.as_ref().map(|_| Firewall::Iptables),
            _ => nft
                .as_ref()
                .map(|_| Firewall::Nft)
                .or_else(|| ipt.as_ref().map(|_| Firewall::Iptables)),
        };
        Self {
            firewall,
            nft: nft.unwrap_or_else(|| "nft".into()),
            iptables: ipt.unwrap_or_else(|| "iptables".into()),
            ip6tables: find_tool(&["ip6tables"]).unwrap_or_else(|| "ip6tables".into()),
            keep_ports,
            backend,
            allowed,
            audit_path: PathBuf::from(
                std::env::var("EDR_ACTIONS_AUDIT").unwrap_or_else(|_| "logs/actions_audit.jsonl".into()),
            ),
        }
    }
}

/// Runs a firewall command. A trait so tests can record commands instead.
pub trait Runner: Send + Sync {
    fn run(&self, prog: &str, args: &[String], stdin: Option<&str>) -> Result<String, String>;
}

pub struct SystemRunner;

impl Runner for SystemRunner {
    fn run(&self, prog: &str, args: &[String], stdin: Option<&str>) -> Result<String, String> {
        use std::process::{Command, Stdio};
        let mut child = Command::new(prog)
            .args(args)
            .stdin(if stdin.is_some() { Stdio::piped() } else { Stdio::null() })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("{prog}: {e}"))?;
        if let (Some(text), Some(mut pipe)) = (stdin, child.stdin.take()) {
            pipe.write_all(text.as_bytes()).map_err(|e| format!("{prog} stdin: {e}"))?;
        }
        let out = child.wait_with_output().map_err(|e| format!("{prog}: {e}"))?;
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).into_owned())
        } else {
            Err(format!(
                "{prog} exited {}: {}",
                out.status,
                String::from_utf8_lossy(&out.stderr).trim()
            ))
        }
    }
}

// ---- Process checks ----

/// What /proc says about a process before it is killed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcIdentity {
    pub tgid: i64,
    pub start_time: u64,
    pub flags: u64,
    pub comm: String,
    pub exe: Option<String>,
    pub cmdline: String,
}

/// Fields of /proc/<pid>/stat after the comm: flags is field 9, starttime field 22.
pub fn parse_stat(stat: &str) -> Option<(String, u64, u64)> {
    let open = stat.find('(')?;
    let close = stat.rfind(')')?;
    let comm = stat.get(open + 1..close)?.to_string();
    let rest: Vec<&str> = stat.get(close + 2..)?.split_whitespace().collect();
    let flags = rest.get(6)?.parse().ok()?;
    let start = rest.get(19)?.parse().ok()?;
    Some((comm, flags, start))
}

pub fn read_identity(proc_root: &Path, pid: i64) -> Option<ProcIdentity> {
    let dir = proc_root.join(pid.to_string());
    let (comm, flags, start_time) = parse_stat(&std::fs::read_to_string(dir.join("stat")).ok()?)?;
    let status = std::fs::read_to_string(dir.join("status")).unwrap_or_default();
    let tgid = status
        .lines()
        .find_map(|l| l.strip_prefix("Tgid:"))
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(pid);
    let exe = std::fs::read_link(dir.join("exe")).ok().map(|p| p.to_string_lossy().into_owned());
    let cmdline = std::fs::read(dir.join("cmdline"))
        .map(|b| crate::redact::cmdline_from_proc(&b))
        .unwrap_or_default();
    Some(ProcIdentity {
        tgid,
        start_time,
        flags,
        comm,
        exe,
        cmdline,
    })
}

/// Why `pid` must not be killed, or None. 403 for a refused target, 404 for no process.
pub fn kill_refusal(pid: i64, agent_pid: i64, id: Option<&ProcIdentity>) -> Option<(u16, String)> {
    if pid <= 1 {
        return Some((403, format!("pid {pid} is never killed (0 and 1 are init and the caller's group, negatives mean groups)")));
    }
    if pid == agent_pid {
        return Some((403, "the agent does not kill itself".into()));
    }
    let Some(id) = id else {
        return Some((404, format!("no process {pid}")));
    };
    if id.tgid == agent_pid {
        return Some((403, format!("{pid} is a thread of the agent")));
    }
    if id.tgid != pid {
        return Some((403, format!("{pid} is a thread of process {}, kill the process instead", id.tgid)));
    }
    if id.flags & PF_KTHREAD != 0 {
        return Some((403, format!("{pid} ({}) is a kernel thread", id.comm)));
    }
    None
}

#[cfg(target_os = "linux")]
fn send_sigkill_checked(proc_root: &Path, pid: i64, before: &ProcIdentity) -> Result<&'static str, String> {
    // SAFETY: plain syscalls with integer arguments. The fd is closed below.
    let fd = unsafe { libc::syscall(libc::SYS_pidfd_open, pid as libc::pid_t, 0) } as i32;
    if fd < 0 {
        let err = std::io::Error::last_os_error();
        if err.raw_os_error() != Some(libc::ENOSYS) {
            return Err(format!("pidfd_open({pid}): {err}"));
        }
        // Kernels before 5.3: check again right before kill().
        let now = read_identity(proc_root, pid).ok_or("the process exited")?;
        if now.start_time != before.start_time {
            return Err("the pid was reused by another process".into());
        }
        // SAFETY: kill with a positive pid and SIGKILL.
        if unsafe { libc::kill(pid as libc::pid_t, libc::SIGKILL) } != 0 {
            return Err(format!("kill: {}", std::io::Error::last_os_error()));
        }
        return Ok("kill");
    }
    let result = (|| {
        // The pidfd now names exactly one process. If its start time differs from the
        // one checked, the pid was reused between the check and the open.
        let now = read_identity(proc_root, pid).ok_or("the process exited")?;
        if now.start_time != before.start_time {
            return Err("the pid was reused by another process".to_string());
        }
        // SAFETY: fd is a pidfd opened above, signal SIGKILL, no siginfo, flags 0.
        let rc = unsafe {
            libc::syscall(libc::SYS_pidfd_send_signal, fd, libc::SIGKILL, std::ptr::null::<libc::siginfo_t>(), 0)
        };
        if rc != 0 {
            return Err(format!("pidfd_send_signal: {}", std::io::Error::last_os_error()));
        }
        Ok("pidfd_send_signal")
    })();
    // SAFETY: fd came from pidfd_open and is closed once.
    unsafe { libc::close(fd) };
    result
}

#[cfg(not(target_os = "linux"))]
fn send_sigkill_checked(_proc_root: &Path, _pid: i64, _before: &ProcIdentity) -> Result<&'static str, String> {
    Err("kill_proc needs Linux".into())
}

// ---- Firewall rules ----

/// A parsed destination: network address and prefix length.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Net {
    pub addr: IpAddr,
    pub prefix: u8,
}

impl Net {
    pub fn parse(s: &str) -> Result<Self, String> {
        let (a, p) = match s.trim().split_once('/') {
            Some((a, p)) => (a, Some(p)),
            None => (s.trim(), None),
        };
        let addr: IpAddr = a.parse().map_err(|_| format!("`{s}` is not an IP address or CIDR"))?;
        let max = if addr.is_ipv4() { 32 } else { 128 };
        let prefix = match p {
            None => max,
            Some(p) => p
                .parse::<u8>()
                .ok()
                .filter(|n| *n <= max)
                .ok_or_else(|| format!("bad prefix in `{s}`"))?,
        };
        Ok(Self { addr, prefix })
    }

    pub fn contains(&self, ip: &IpAddr) -> bool {
        match (self.addr, ip) {
            (IpAddr::V4(n), IpAddr::V4(a)) => {
                let m = if self.prefix == 0 { 0 } else { u32::MAX << (32 - self.prefix as u32) };
                (u32::from(n) & m) == (u32::from(*a) & m)
            }
            (IpAddr::V6(n), IpAddr::V6(a)) => {
                let m = if self.prefix == 0 { 0 } else { u128::MAX << (128 - self.prefix as u32) };
                (u128::from(n) & m) == (u128::from(*a) & m)
            }
            _ => false,
        }
    }

    fn text(&self) -> String {
        let max = if self.addr.is_ipv4() { 32 } else { 128 };
        if self.prefix == max {
            self.addr.to_string()
        } else {
            format!("{}/{}", self.addr, self.prefix)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub id: String,
    pub dst: Net,
    pub port: Option<u16>,
    pub proto: String,
    pub reason: String,
    pub created: u64,
    /// Unix seconds when the kernel drops the element, None for no expiry.
    pub expires_at: Option<u64>,
}

impl Block {
    /// nftables set names take letters, digits and underscores.
    fn set_name(&self) -> String {
        format!("b_{}", self.id.replace('-', "_"))
    }
}

#[derive(Clone, Debug, Default)]
pub struct FwState {
    pub blocks: BTreeMap<String, Block>,
    pub isolated: Option<(u64, String)>,
    /// Unix seconds when the isolation ends by itself, None to keep it.
    pub isolated_until: Option<u64>,
}

impl FwState {
    /// The state without what has expired by `now`.
    pub fn live(&self, now: u64) -> FwState {
        let mut st = self.clone();
        st.blocks.retain(|_, b| b.expires_at.is_none_or(|e| e > now));
        if st.isolated_until.is_some_and(|e| e <= now) {
            st.isolated = None;
            st.isolated_until = None;
        }
        st
    }
}

fn timeout_text(expires_at: Option<u64>, now: u64) -> String {
    match expires_at {
        Some(e) => format!(" timeout {}s", e.saturating_sub(now).max(1)),
        None => String::new(),
    }
}

fn port_set(ports: &[u16]) -> String {
    let list: Vec<String> = ports.iter().map(|p| p.to_string()).collect();
    format!("{{ {} }}", list.join(", "))
}

/// The whole nftables script for a state at `now`: an atomic replace of the agent's
/// table, or its removal when nothing is blocked and the host is not isolated.
/// Expired blocks are left out and the others keep only the time they have left.
pub fn nft_script(st: &FwState, cfg: &ResponseConfig, now: u64) -> String {
    let st = st.live(now);
    let mut s = format!("add table inet {TABLE}\ndelete table inet {TABLE}\n");
    if st.blocks.is_empty() && st.isolated.is_none() {
        return s;
    }
    let keep = port_set(&cfg.keep_ports);
    s.push_str(&format!("table inet {TABLE} {{\n"));
    for b in st.blocks.values() {
        let ty = if b.dst.addr.is_ipv4() { "ipv4_addr" } else { "ipv6_addr" };
        s.push_str(&format!(
            "  set {} {{ type {ty}; flags interval, timeout; elements = {{ {}{} }} }}\n",
            b.set_name(),
            b.dst.text(),
            timeout_text(b.expires_at, now)
        ));
    }
    if st.isolated.is_some() {
        let t = timeout_text(st.isolated_until, now);
        s.push_str(&format!("  set iso4 {{ type ipv4_addr; flags interval, timeout; elements = {{ 0.0.0.0/0{t} }} }}\n"));
        s.push_str(&format!("  set iso6 {{ type ipv6_addr; flags interval, timeout; elements = {{ ::/0{t} }} }}\n"));
    }
    s.push_str("  chain egress {\n    type filter hook output priority 0; policy accept;\n");
    s.push_str("    ip daddr 127.0.0.0/8 accept\n    ip6 daddr ::1 accept\n");
    s.push_str(&format!("    tcp sport {keep} accept\n"));
    for b in st.blocks.values() {
        let fam = if b.dst.addr.is_ipv4() { "ip" } else { "ip6" };
        let port = b.port.map(|p| format!(" {} dport {p}", b.proto)).unwrap_or_default();
        s.push_str(&format!(
            "    {fam} daddr @{}{port} counter drop comment \"edr {}\"\n",
            b.set_name(),
            b.id
        ));
    }
    s.push_str("  }\n");
    if st.isolated.is_some() {
        s.push_str("  chain isolate_out {\n    type filter hook output priority -10; policy accept;\n");
        s.push_str("    ip daddr 127.0.0.0/8 accept\n    ip6 daddr ::1 accept\n");
        s.push_str(&format!("    tcp sport {keep} accept\n"));
        for (ip, port) in &cfg.backend {
            let fam = if ip.is_ipv4() { "ip" } else { "ip6" };
            s.push_str(&format!("    {fam} daddr {ip} tcp dport {port} accept\n"));
        }
        s.push_str("    ip daddr @iso4 counter drop comment \"edr isolation\"\n");
        s.push_str("    ip6 daddr @iso6 counter drop comment \"edr isolation\"\n  }\n");
        s.push_str("  chain isolate_in {\n    type filter hook input priority -10; policy accept;\n");
        s.push_str("    ip saddr 127.0.0.0/8 accept\n    ip6 saddr ::1 accept\n");
        s.push_str(&format!("    tcp dport {keep} accept\n"));
        for (ip, port) in &cfg.backend {
            let fam = if ip.is_ipv4() { "ip" } else { "ip6" };
            s.push_str(&format!("    {fam} saddr {ip} tcp sport {port} accept\n"));
        }
        s.push_str("    ip saddr @iso4 counter drop comment \"edr isolation\"\n");
        s.push_str("    ip6 saddr @iso6 counter drop comment \"edr isolation\"\n  }\n");
    }
    s.push_str("}\n");
    s
}

/// The same state as iptables commands: (program, args) in order. Each chain is created if
/// missing, flushed and filled, and jumped to from the first position of OUTPUT or INPUT.
pub fn iptables_commands(st: &FwState, cfg: &ResponseConfig) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::new();
    let a = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let keep: Vec<String> = cfg.keep_ports.iter().map(|p| p.to_string()).collect();
    let keep = keep.join(",");
    let iso = st.isolated.is_some();
    for (bin, v6) in [(&cfg.iptables, false), (&cfg.ip6tables, true)] {
        let lo = if v6 { "::1/128" } else { "127.0.0.0/8" };
        let blocks: Vec<&Block> = st.blocks.values().filter(|b| b.dst.addr.is_ipv6() == v6).collect();
        let backends: Vec<&(IpAddr, u16)> = cfg.backend.iter().filter(|(ip, _)| ip.is_ipv6() == v6).collect();
        // Detach and empty the chains first, so a release leaves nothing behind.
        for (parent, chain) in [("OUTPUT", "EDR_EGRESS"), ("OUTPUT", "EDR_ISO_OUT"), ("INPUT", "EDR_ISO_IN")] {
            out.push((bin.clone(), a(&["-w", "-D", parent, "-j", chain])));
            out.push((bin.clone(), a(&["-w", "-F", chain])));
            out.push((bin.clone(), a(&["-w", "-X", chain])));
        }
        if blocks.is_empty() && !iso {
            continue;
        }
        if !blocks.is_empty() {
            out.push((bin.clone(), a(&["-w", "-N", "EDR_EGRESS"])));
            out.push((bin.clone(), a(&["-w", "-A", "EDR_EGRESS", "-d", lo, "-j", "ACCEPT"])));
            out.push((bin.clone(), a(&["-w", "-A", "EDR_EGRESS", "-p", "tcp", "-m", "multiport", "--sports", &keep, "-j", "ACCEPT"])));
            for b in blocks {
                let mut r = a(&["-w", "-A", "EDR_EGRESS", "-d", &b.dst.text()]);
                if let Some(p) = b.port {
                    r.extend(a(&["-p", &b.proto, "--dport", &p.to_string()]));
                }
                r.extend(a(&["-m", "comment", "--comment", &format!("edr {}", b.id), "-j", "DROP"]));
                out.push((bin.clone(), r));
            }
            out.push((bin.clone(), a(&["-w", "-I", "OUTPUT", "1", "-j", "EDR_EGRESS"])));
        }
        if iso {
            out.push((bin.clone(), a(&["-w", "-N", "EDR_ISO_OUT"])));
            out.push((bin.clone(), a(&["-w", "-A", "EDR_ISO_OUT", "-d", lo, "-j", "ACCEPT"])));
            out.push((bin.clone(), a(&["-w", "-A", "EDR_ISO_OUT", "-p", "tcp", "-m", "multiport", "--sports", &keep, "-j", "ACCEPT"])));
            for (ip, port) in &backends {
                out.push((bin.clone(), a(&["-w", "-A", "EDR_ISO_OUT", "-d", &ip.to_string(), "-p", "tcp", "--dport", &port.to_string(), "-j", "ACCEPT"])));
            }
            out.push((bin.clone(), a(&["-w", "-A", "EDR_ISO_OUT", "-j", "DROP"])));
            out.push((bin.clone(), a(&["-w", "-I", "OUTPUT", "1", "-j", "EDR_ISO_OUT"])));
            out.push((bin.clone(), a(&["-w", "-N", "EDR_ISO_IN"])));
            out.push((bin.clone(), a(&["-w", "-A", "EDR_ISO_IN", "-s", lo, "-j", "ACCEPT"])));
            out.push((bin.clone(), a(&["-w", "-A", "EDR_ISO_IN", "-p", "tcp", "-m", "multiport", "--dports", &keep, "-j", "ACCEPT"])));
            for (ip, port) in &backends {
                out.push((bin.clone(), a(&["-w", "-A", "EDR_ISO_IN", "-s", &ip.to_string(), "-p", "tcp", "--sport", &port.to_string(), "-j", "ACCEPT"])));
            }
            out.push((bin.clone(), a(&["-w", "-A", "EDR_ISO_IN", "-j", "DROP"])));
            out.push((bin.clone(), a(&["-w", "-I", "INPUT", "1", "-j", "EDR_ISO_IN"])));
        }
    }
    out
}

/// Commands that undo nothing when they fail: deleting a chain or jump that is not there.
fn cleanup_step(args: &[String]) -> bool {
    matches!(args.get(1).map(String::as_str), Some("-D" | "-F" | "-X"))
}

// ---- The responder ----

pub struct Responder {
    cfg: RwLock<ResponseConfig>,
    runner: Box<dyn Runner>,
    proc_root: PathBuf,
    agent_pid: i64,
    state: Mutex<FwState>,
}

fn reason_of(reason: &Option<String>, incident: &Option<String>) -> Result<String, String> {
    let r = reason.as_deref().map(str::trim).unwrap_or("");
    let i = incident.as_deref().map(str::trim).unwrap_or("");
    if r.is_empty() && i.is_empty() {
        return Err("a reason or an incident_id is required".into());
    }
    if r.len() > MAX_REASON || i.len() > 200 {
        return Err(format!("reason is limited to {MAX_REASON} characters"));
    }
    Ok(match (r.is_empty(), i.is_empty()) {
        (false, false) => format!("{r} (incident {i})"),
        (false, true) => r.to_string(),
        _ => format!("incident {i}"),
    })
}

impl Responder {
    pub fn new(cfg: ResponseConfig, runner: Box<dyn Runner>, proc_root: &Path, agent_pid: i64) -> Self {
        Self {
            cfg: RwLock::new(cfg),
            runner,
            proc_root: proc_root.to_path_buf(),
            agent_pid,
            state: Mutex::new(FwState::default()),
        }
    }

    /// A copy of the settings in force.
    pub fn cfg(&self) -> ResponseConfig {
        self.cfg.read().unwrap_or_else(|e| e.into_inner()).clone()
    }

    /// A new signed policy: the allowed actions and the ports isolation keeps
    /// open. Rules already in place keep their ports until the next change.
    pub fn set_policy(&self, allowed: Vec<String>, keep_ports: Vec<u16>) {
        let mut c = self.cfg.write().unwrap_or_else(|e| e.into_inner());
        c.allowed = allowed;
        c.keep_ports = keep_ports;
    }

    fn allowed(&self, action: &str) -> Option<Outcome> {
        (!self.cfg().allowed.iter().any(|a| a == action))
            .then(|| Outcome::err(403, format!("{action} is not allowed by the signed policy")))
    }

    fn check_expiry(&self, expires_in_sec: Option<u64>) -> Result<Option<u64>, Outcome> {
        let Some(secs) = expires_in_sec else {
            return Ok(None);
        };
        if secs == 0 || secs > MAX_EXPIRY_SEC {
            return Err(Outcome::err(400, format!("expires_in_sec must be 1 to {MAX_EXPIRY_SEC}")));
        }
        if self.cfg().firewall != Some(Firewall::Nft) {
            return Err(Outcome::err(501, "an expiry needs nftables element timeouts, iptables has none"));
        }
        Ok(Some(now_ts() + secs))
    }

    /// Drops expired blocks and an expired isolation from the agent's state. The
    /// kernel already removed their elements, and the rebuild tidies the table.
    pub fn sweep(&self) -> Vec<String> {
        let now = now_ts();
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let live = st.live(now);
        let mut gone: Vec<String> = st.blocks.keys().filter(|k| !live.blocks.contains_key(*k)).cloned().collect();
        if st.isolated.is_some() && live.isolated.is_none() {
            gone.push("isolation".into());
        }
        if gone.is_empty() {
            return gone;
        }
        if self.apply(&live, false).is_ok() {
            *st = live;
            drop(st);
            self.audit("expire", json!({ "released": gone }), &Outcome::ok(json!({ "ok": true, "expired": gone })));
        }
        gone
    }

    /// One JSON line per request, done or refused, in a 0600 file.
    fn audit(&self, action: &str, request: Value, out: &Outcome) {
        let line = json!({
            "ts": now_ts(),
            "action": action,
            "request": request,
            "status": out.status,
            "result": out.body,
        });
        let audit_path = self.cfg().audit_path;
        if let Some(dir) = audit_path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        use std::os::unix::fs::OpenOptionsExt;
        let opened = OpenOptions::new()
            .create(true)
            .append(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(&audit_path);
        match opened {
            Ok(mut f) => {
                let _ = writeln!(f, "{line}");
            }
            Err(e) => eprintln!("⚠️ action audit log {}: {e}", audit_path.display()),
        }
        eprintln!("action {action}: status {} {}", out.status, out.body);
    }

    /// `incident_exists` says whether an incident id is known to the agent.
    pub fn kill_proc(&self, req: KillRequest, incident_exists: &dyn Fn(&str) -> bool) -> Outcome {
        let request = json!({ "pid": req.pid, "reason": req.reason, "incident_id": req.incident_id, "dry_run": req.dry_run });
        let out = self.kill_inner(&req, incident_exists);
        self.audit("kill_proc", request, &out);
        out
    }

    fn kill_inner(&self, req: &KillRequest, incident_exists: &dyn Fn(&str) -> bool) -> Outcome {
        if let Some(o) = self.allowed("kill_proc") {
            return o;
        }
        let reason = match reason_of(&req.reason, &req.incident_id) {
            Ok(r) => r,
            Err(e) => return Outcome::err(400, e),
        };
        if let Some(i) = req.incident_id.as_deref().filter(|i| !i.trim().is_empty()) {
            if !incident_exists(i.trim()) {
                return Outcome::err(404, format!("no incident {i}"));
            }
        }
        let id = if req.pid > 1 { read_identity(&self.proc_root, req.pid) } else { None };
        if let Some((status, why)) = kill_refusal(req.pid, self.agent_pid, id.as_ref()) {
            return Outcome::err(status, why);
        }
        let id = id.expect("checked by kill_refusal");
        let target = json!({ "pid": req.pid, "comm": id.comm, "exe": id.exe, "cmdline": id.cmdline, "start_time": id.start_time });
        if req.dry_run {
            return Outcome::ok(json!({ "ok": true, "action": "kill_proc", "dry_run": true, "would_send": "SIGKILL", "target": target, "reason": reason }));
        }
        match send_sigkill_checked(&self.proc_root, req.pid, &id) {
            Ok(how) => Outcome::ok(json!({ "ok": true, "action": "kill_proc", "dry_run": false, "sent": "SIGKILL", "via": how, "target": target, "reason": reason })),
            Err(e) => Outcome::err(409, e),
        }
    }

    pub fn block_egress(&self, req: BlockRequest) -> Outcome {
        let request = json!({ "dst": req.dst, "port": req.port, "proto": req.proto, "reason": req.reason, "incident_id": req.incident_id, "expires_in_sec": req.expires_in_sec, "dry_run": req.dry_run });
        let out = self.block_inner(&req);
        self.audit("block_egress", request, &out);
        out
    }

    fn block_inner(&self, req: &BlockRequest) -> Outcome {
        if let Some(o) = self.allowed("block_egress") {
            return o;
        }
        let reason = match reason_of(&req.reason, &req.incident_id) {
            Ok(r) => r,
            Err(e) => return Outcome::err(400, e),
        };
        let dst = match Net::parse(&req.dst) {
            Ok(n) => n,
            Err(e) => return Outcome::err(400, e),
        };
        if dst.prefix == 0 {
            return Outcome::err(400, "blocking every address is isolate_host, not block_egress");
        }
        let loopback: [IpAddr; 2] = ["127.0.0.1".parse().unwrap(), "::1".parse().unwrap()];
        if loopback.iter().any(|l| dst.contains(l)) || dst.addr.is_loopback() {
            return Outcome::err(403, "loopback stays reachable");
        }
        if let Some((ip, _)) = self.cfg().backend.iter().find(|(ip, _)| dst.contains(ip)) {
            return Outcome::err(403, format!("{} covers the backend {ip}, which stays reachable", dst.text()));
        }
        let proto = req.proto.clone().unwrap_or_else(|| "tcp".into());
        if !matches!(proto.as_str(), "tcp" | "udp") {
            return Outcome::err(400, "proto is tcp or udp");
        }
        if req.proto.is_some() && req.port.is_none() {
            return Outcome::err(400, "proto needs a port");
        }
        let expires_at = match self.check_expiry(req.expires_in_sec) {
            Ok(e) => e,
            Err(o) => return o,
        };
        let created = now_ts();
        let digest = Sha256::digest(format!("{}|{:?}|{proto}|{created}", dst.text(), req.port).as_bytes());
        let id = format!("blk-{}", &hex::encode(digest)[..8]);
        let block = Block {
            id: id.clone(),
            dst,
            port: req.port,
            proto,
            reason: reason.clone(),
            created,
            expires_at,
        };
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut next = st.clone();
        next.blocks.insert(id.clone(), block.clone());
        match self.apply(&next, req.dry_run) {
            Ok(rules) => {
                if !req.dry_run {
                    *st = next;
                }
                Outcome::ok(json!({ "ok": true, "action": "block_egress", "dry_run": req.dry_run, "id": id, "dst": block.dst.text(), "port": block.port, "proto": block.proto, "reason": reason, "expires_at": block.expires_at, "kernel_expiry": block.expires_at.is_some(), "firewall": self.fw_name(), "rules": rules }))
            }
            Err(o) => o,
        }
    }

    pub fn isolate_host(&self, req: IsolateRequest) -> Outcome {
        let request = json!({ "reason": req.reason, "incident_id": req.incident_id, "expires_in_sec": req.expires_in_sec, "dry_run": req.dry_run });
        let out = self.isolate_inner(&req);
        self.audit("isolate_host", request, &out);
        out
    }

    fn isolate_inner(&self, req: &IsolateRequest) -> Outcome {
        if let Some(o) = self.allowed("isolate_host") {
            return o;
        }
        let reason = match reason_of(&req.reason, &req.incident_id) {
            Ok(r) => r,
            Err(e) => return Outcome::err(400, e),
        };
        let until = match self.check_expiry(req.expires_in_sec) {
            Ok(e) => e,
            Err(o) => return o,
        };
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut next = st.clone();
        next.isolated = Some((now_ts(), reason.clone()));
        next.isolated_until = until;
        match self.apply(&next, req.dry_run) {
            Ok(rules) => {
                if !req.dry_run {
                    *st = next;
                }
                let cfg = self.cfg();
                let backend: Vec<String> = cfg.backend.iter().map(|(i, p)| format!("{i}:{p}")).collect();
                Outcome::ok(json!({ "ok": true, "action": "isolate_host", "dry_run": req.dry_run, "reason": reason, "expires_at": until, "kept_open": { "loopback": true, "ports": cfg.keep_ports, "backend": backend }, "firewall": self.fw_name(), "rules": rules }))
            }
            Err(o) => o,
        }
    }

    pub fn release(&self, req: ReleaseRequest) -> Outcome {
        let request = json!({ "id": req.id, "isolation": req.isolation, "all": req.all, "reason": req.reason, "dry_run": req.dry_run });
        let out = self.release_inner(&req);
        self.audit("release", request, &out);
        out
    }

    fn release_inner(&self, req: &ReleaseRequest) -> Outcome {
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut next = st.clone();
        let mut released = Vec::new();
        if req.all {
            released.extend(next.blocks.keys().cloned());
            next.blocks.clear();
            if next.isolated.take().is_some() {
                released.push("isolation".into());
            }
            next.isolated_until = None;
        } else {
            if let Some(id) = &req.id {
                if next.blocks.remove(id).is_none() {
                    return Outcome::err(404, format!("no block {id}"));
                }
                released.push(id.clone());
            }
            if req.isolation {
                if next.isolated.take().is_none() {
                    return Outcome::err(404, "the host is not isolated");
                }
                next.isolated_until = None;
                released.push("isolation".into());
            }
            if released.is_empty() {
                return Outcome::err(400, "say what to release: id, isolation or all");
            }
        }
        match self.apply(&next, req.dry_run) {
            Ok(rules) => {
                if !req.dry_run {
                    *st = next;
                }
                Outcome::ok(json!({ "ok": true, "action": "release", "dry_run": req.dry_run, "released": released, "firewall": self.fw_name(), "rules": rules }))
            }
            Err(o) => o,
        }
    }

    pub fn state_json(&self) -> Value {
        let st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let now = now_ts();
        let blocks: Vec<Value> = st
            .blocks
            .values()
            .map(|b| json!({ "id": b.id, "dst": b.dst.text(), "port": b.port, "proto": b.proto, "reason": b.reason, "created": b.created, "expires_at": b.expires_at, "expires_in_sec": b.expires_at.map(|e| e.saturating_sub(now)) }))
            .collect();
        let cfg = self.cfg();
        json!({
            "firewall": self.fw_name(),
            "allowed": cfg.allowed,
            "keep_ports": cfg.keep_ports,
            "blocks": blocks,
            "isolated": st.isolated.as_ref().map(|(t, r)| json!({ "since": t, "reason": r, "until": st.isolated_until })),
        })
    }

    fn fw_name(&self) -> &'static str {
        match self.cfg().firewall {
            Some(Firewall::Nft) => "nftables",
            Some(Firewall::Iptables) => "iptables",
            None => "none",
        }
    }

    /// Writes the rules for `st`, or with dry_run only returns them.
    fn apply(&self, st: &FwState, dry_run: bool) -> Result<Value, Outcome> {
        let cfg = self.cfg();
        match cfg.firewall {
            Some(Firewall::Nft) => {
                let script = nft_script(st, &cfg, now_ts());
                if !dry_run {
                    self.runner
                        .run(&cfg.nft, &["-f".to_string(), "-".to_string()], Some(&script))
                        .map_err(|e| Outcome::err(500, e))?;
                }
                Ok(json!(script))
            }
            Some(Firewall::Iptables) => {
                let cmds = iptables_commands(st, &cfg);
                if !dry_run {
                    for (prog, args) in &cmds {
                        if let Err(e) = self.runner.run(prog, args, None) {
                            if !cleanup_step(args) {
                                return Err(Outcome::err(500, e));
                            }
                        }
                    }
                }
                Ok(json!(cmds.iter().map(|(p, a)| format!("{p} {}", a.join(" "))).collect::<Vec<_>>()))
            }
            None => Err(Outcome::err(503, "neither nft nor iptables is installed")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[derive(Default)]
    struct Recorder {
        calls: Mutex<Vec<(String, Vec<String>, Option<String>)>>,
    }

    impl Runner for Arc<Recorder> {
        fn run(&self, prog: &str, args: &[String], stdin: Option<&str>) -> Result<String, String> {
            self.calls.lock().unwrap().push((prog.into(), args.to_vec(), stdin.map(str::to_string)));
            Ok(String::new())
        }
    }

    fn cfg(fw: Firewall, dir: &Path) -> ResponseConfig {
        ResponseConfig {
            firewall: Some(fw),
            nft: "/usr/sbin/nft".into(),
            iptables: "/usr/sbin/iptables".into(),
            ip6tables: "/usr/sbin/ip6tables".into(),
            keep_ports: vec![22, 8080],
            backend: vec![("198.51.100.5".parse().unwrap(), 8000)],
            allowed: ACTIONS.iter().map(|s| s.to_string()).collect(),
            audit_path: dir.join("audit.jsonl"),
        }
    }

    fn temp(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("edr_resp_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    /// A fake /proc entry: stat with flags and start time, status with the Tgid.
    fn fake_proc(root: &Path, pid: i64, tgid: i64, flags: u64, start: u64, comm: &str) {
        let d = root.join(pid.to_string());
        std::fs::create_dir_all(&d).unwrap();
        std::fs::write(
            d.join("stat"),
            format!("{pid} ({comm}) S 1 {pid} {pid} 0 -1 {flags} 0 0 0 0 1 1 0 0 20 0 1 0 {start} 1000 100"),
        )
        .unwrap();
        std::fs::write(d.join("status"), format!("Name:\t{comm}\nTgid:\t{tgid}\nPid:\t{pid}\n")).unwrap();
        std::fs::write(d.join("cmdline"), b"sleep\x00--token\x00abc123\x00").unwrap();
    }

    #[test]
    fn stat_fields_parse() {
        let (comm, flags, start) = parse_stat("42 (a b) S 1 42 42 0 -1 2097216 0 0 0 0 1 1 0 0 20 0 1 0 9876 1 1").unwrap();
        assert_eq!((comm.as_str(), flags, start), ("a b", 2_097_216, 9876));
        assert!(flags & PF_KTHREAD != 0);
    }

    #[test]
    fn kill_refuses_init_the_agent_its_threads_kernel_threads_and_missing_reasons() {
        let root = temp("kill");
        fake_proc(&root, 500, 500, 0x400100, 77, "sleep");
        fake_proc(&root, 2, 2, 0x208040, 1, "kthreadd");
        fake_proc(&root, 601, 600, 0x400100, 80, "agent-worker");
        fake_proc(&root, 701, 700, 0x400100, 90, "python3");
        let rec = Arc::new(Recorder::default());
        let r = Responder::new(cfg(Firewall::Nft, &root), Box::new(rec.clone()), &root, 600);
        let none = |_: &str| false;
        let kill = |pid: i64, reason: Option<&str>, dry: bool| {
            r.kill_proc(KillRequest { pid, reason: reason.map(str::to_string), incident_id: None, dry_run: dry }, &none)
        };
        assert_eq!(kill(1, Some("x"), false).status, 403);
        assert_eq!(kill(0, Some("x"), false).status, 403);
        assert_eq!(kill(-1, Some("x"), false).status, 403, "kill(-1) would signal everything");
        assert_eq!(kill(600, Some("x"), false).status, 403, "the agent");
        assert_eq!(kill(601, Some("x"), false).status, 403, "a thread of the agent");
        let t = kill(701, Some("x"), false);
        assert_eq!(t.status, 403, "a thread of another process: {:?}", t.body);
        let k = kill(2, Some("x"), false);
        assert_eq!(k.status, 403);
        assert!(k.body["error"].as_str().unwrap().contains("kernel thread"));
        assert_eq!(kill(500, None, false).status, 400, "no reason");
        assert_eq!(kill(99999, Some("x"), false).status, 404);
        // An incident id the agent does not know is refused.
        let o = r.kill_proc(KillRequest { pid: 500, reason: None, incident_id: Some("nope".into()), dry_run: true }, &none);
        assert_eq!(o.status, 404);
        // A dry run checks everything and sends nothing. The command line is masked.
        let d = kill(500, Some("incident 42"), true);
        assert_eq!(d.status, 200, "{:?}", d.body);
        assert_eq!(d.body["would_send"], "SIGKILL");
        assert_eq!(d.body["target"]["cmdline"], "sleep --token [REDACTED]");
        // Every request is in the audit log, refused ones too, and the file is 0600.
        let audit = std::fs::read_to_string(root.join("audit.jsonl")).unwrap();
        assert_eq!(audit.lines().count(), 11);
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(root.join("audit.jsonl")).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        assert!(rec.calls.lock().unwrap().is_empty());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn kill_really_kills_a_child_and_a_reused_pid_is_refused() {
        let mut child = std::process::Command::new("sleep").arg("30").spawn().unwrap();
        let pid = child.id() as i64;
        let root = temp("realkill");
        let r = Responder::new(cfg(Firewall::Nft, &root), Box::new(SystemRunner), Path::new("/proc"), std::process::id() as i64);
        let before = read_identity(Path::new("/proc"), pid).unwrap();
        // A start time that does not match is what a reused pid looks like.
        let mut stale = before.clone();
        stale.start_time += 1;
        assert!(send_sigkill_checked(Path::new("/proc"), pid, &stale).is_err());
        assert!(child.try_wait().unwrap().is_none(), "still running");
        let o = r.kill_proc(KillRequest { pid, reason: Some("test".into()), incident_id: None, dry_run: false }, &|_| false);
        assert_eq!(o.status, 200, "{:?}", o.body);
        let status = child.wait().unwrap();
        use std::os::unix::process::ExitStatusExt;
        assert_eq!(status.signal(), Some(libc::SIGKILL));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn nft_rules_keep_loopback_management_and_backend_and_replace_atomically() {
        let root = temp("nft");
        let rec = Arc::new(Recorder::default());
        let r = Responder::new(cfg(Firewall::Nft, &root), Box::new(rec.clone()), &root, 1);
        let b = r.block_egress(BlockRequest { dst: "203.0.113.10".into(), port: Some(443), proto: None, reason: Some("c2".into()), incident_id: None, expires_in_sec: None, dry_run: false });
        assert_eq!(b.status, 200, "{:?}", b.body);
        let id = b.body["id"].as_str().unwrap().to_string();
        let set = format!("b_{}", id.replace('-', "_"));
        let script = rec.calls.lock().unwrap()[0].2.clone().unwrap();
        assert!(script.starts_with("add table inet edr_agent\ndelete table inet edr_agent\ntable inet edr_agent {"));
        assert!(script.contains("ip daddr 127.0.0.0/8 accept"));
        assert!(script.contains("ip6 daddr ::1 accept"));
        assert!(script.contains("tcp sport { 22, 8080 } accept"));
        // No expiry: the element has no timeout.
        assert!(script.contains(&format!("set {set} {{ type ipv4_addr; flags interval, timeout; elements = {{ 203.0.113.10 }} }}")), "{script}");
        assert!(script.contains(&format!("ip daddr @{set} tcp dport 443 counter drop comment \"edr {id}\"")));
        assert!(!script.contains("isolate"));

        let i = r.isolate_host(IsolateRequest { reason: None, incident_id: Some("inc-1".into()), expires_in_sec: None, dry_run: false });
        assert_eq!(i.status, 200);
        let script = rec.calls.lock().unwrap()[1].2.clone().unwrap();
        assert!(script.contains("chain isolate_out"));
        assert!(script.contains("ip daddr 198.51.100.5 tcp dport 8000 accept"));
        assert!(script.contains("ip saddr 198.51.100.5 tcp sport 8000 accept"));
        assert!(script.contains("tcp dport { 22, 8080 } accept"));
        assert!(script.contains("ip saddr 127.0.0.0/8 accept"));
        assert!(script.contains("elements = { 0.0.0.0/0 } }"), "an isolation keeps no timeout by default");
        assert!(script.contains("ip daddr @iso4 counter drop") && script.contains("ip6 saddr @iso6 counter drop"));

        // Release the isolation: the block stays. Release the block: the table goes.
        assert_eq!(r.release(ReleaseRequest { id: None, isolation: true, all: false, reason: None, dry_run: false }).status, 200);
        let script = rec.calls.lock().unwrap()[2].2.clone().unwrap();
        assert!(!script.contains("isolate") && script.contains(&id));
        assert_eq!(r.release(ReleaseRequest { id: Some(id.clone()), isolation: false, all: false, reason: None, dry_run: false }).status, 200);
        assert_eq!(rec.calls.lock().unwrap()[3].2.as_deref(), Some("add table inet edr_agent\ndelete table inet edr_agent\n"));
        assert_eq!(r.release(ReleaseRequest { id: Some(id), isolation: false, all: false, reason: None, dry_run: false }).status, 404);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn block_refuses_loopback_the_backend_everything_and_bad_input() {
        let root = temp("blockbad");
        let rec = Arc::new(Recorder::default());
        let r = Responder::new(cfg(Firewall::Nft, &root), Box::new(rec.clone()), &root, 1);
        let block = |dst: &str, reason: Option<&str>| {
            r.block_egress(BlockRequest { dst: dst.into(), port: None, proto: None, reason: reason.map(str::to_string), incident_id: None, expires_in_sec: None, dry_run: false }).status
        };
        assert_eq!(block("127.0.0.1", Some("x")), 403);
        assert_eq!(block("127.0.0.0/8", Some("x")), 403);
        assert_eq!(block("::1", Some("x")), 403);
        assert_eq!(block("198.51.100.0/24", Some("x")), 403, "covers the backend");
        assert_eq!(block("0.0.0.0/0", Some("x")), 400);
        assert_eq!(block("not-an-ip", Some("x")), 400);
        assert_eq!(block("10.0.0.0/33", Some("x")), 400);
        assert_eq!(block("203.0.113.0/24", None), 400, "no reason");
        assert!(rec.calls.lock().unwrap().is_empty(), "nothing ran");
        // A dry run returns the rules and runs nothing, and the state does not change.
        let d = r.block_egress(BlockRequest { dst: "203.0.113.0/24".into(), port: None, proto: None, reason: Some("x".into()), incident_id: None, expires_in_sec: None, dry_run: true });
        assert_eq!(d.status, 200);
        assert!(d.body["rules"].as_str().unwrap().contains("elements = { 203.0.113.0/24 }"));
        assert!(rec.calls.lock().unwrap().is_empty());
        assert_eq!(r.state_json()["blocks"].as_array().unwrap().len(), 0);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn expiring_blocks_and_isolation_use_element_timeouts() {
        let root = temp("expiry");
        let rec = Arc::new(Recorder::default());
        let r = Responder::new(cfg(Firewall::Nft, &root), Box::new(rec.clone()), &root, 1);
        let b = r.block_egress(BlockRequest { dst: "203.0.113.0/24".into(), port: Some(8000), proto: None, reason: Some("c2".into()), incident_id: None, expires_in_sec: Some(120), dry_run: false });
        assert_eq!(b.status, 200, "{:?}", b.body);
        assert_eq!(b.body["kernel_expiry"], true);
        let script = rec.calls.lock().unwrap()[0].2.clone().unwrap();
        assert!(script.contains("elements = { 203.0.113.0/24 timeout 120s }") || script.contains("elements = { 203.0.113.0/24 timeout 119s }"), "{script}");
        let i = r.isolate_host(IsolateRequest { reason: Some("x".into()), incident_id: None, expires_in_sec: Some(600), dry_run: false });
        assert_eq!(i.status, 200);
        let script = rec.calls.lock().unwrap()[1].2.clone().unwrap();
        assert!(script.contains("0.0.0.0/0 timeout 600s") || script.contains("0.0.0.0/0 timeout 599s"), "{script}");
        assert!(script.contains("::/0 timeout"));
        // Bad expiries, and an expiry with iptables, are refused.
        let bad = |secs| r.block_egress(BlockRequest { dst: "203.0.113.9".into(), port: None, proto: None, reason: Some("x".into()), incident_id: None, expires_in_sec: Some(secs), dry_run: true }).status;
        assert_eq!(bad(0), 400);
        assert_eq!(bad(MAX_EXPIRY_SEC + 1), 400);
        let ri = Responder::new(cfg(Firewall::Iptables, &root), Box::new(rec.clone()), &root, 1);
        let o = ri.block_egress(BlockRequest { dst: "203.0.113.9".into(), port: None, proto: None, reason: Some("x".into()), incident_id: None, expires_in_sec: Some(60), dry_run: true });
        assert_eq!(o.status, 501);

        // A rebuild leaves expired entries out and gives the rest their time left.
        let mut st = FwState::default();
        let mk = |id: &str, exp: Option<u64>| Block { id: id.into(), dst: Net::parse("198.51.100.7").unwrap(), port: None, proto: "tcp".into(), reason: "r".into(), created: 0, expires_at: exp };
        st.blocks.insert("blk-a".into(), mk("blk-a", Some(1_000)));
        st.blocks.insert("blk-b".into(), mk("blk-b", Some(1_100)));
        st.blocks.insert("blk-c".into(), mk("blk-c", None));
        let c = cfg(Firewall::Nft, &root);
        let s = nft_script(&st, &c, 1_000);
        assert!(!s.contains("b_blk_a"), "expired at 1000");
        assert!(s.contains("set b_blk_b { type ipv4_addr; flags interval, timeout; elements = { 198.51.100.7 timeout 100s } }"));
        assert!(s.contains("set b_blk_c { type ipv4_addr; flags interval, timeout; elements = { 198.51.100.7 } }"));
        st.blocks.clear();
        st.isolated = Some((0, "x".into()));
        st.isolated_until = Some(900);
        assert_eq!(nft_script(&st, &c, 1_000), "add table inet edr_agent\ndelete table inet edr_agent\n", "an expired isolation leaves no table");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn a_new_policy_changes_the_allowed_actions() {
        let root = temp("policy");
        let rec = Arc::new(Recorder::default());
        let r = Responder::new(cfg(Firewall::Nft, &root), Box::new(rec.clone()), &root, 1);
        r.set_policy(vec!["kill_proc".into()], vec![22]);
        let o = r.isolate_host(IsolateRequest { reason: Some("x".into()), incident_id: None, expires_in_sec: None, dry_run: true });
        assert_eq!(o.status, 403);
        assert_eq!(r.state_json()["keep_ports"], json!([22]));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn iptables_fallback_and_policy_gate() {
        let root = temp("ipt");
        let rec = Arc::new(Recorder::default());
        let r = Responder::new(cfg(Firewall::Iptables, &root), Box::new(rec.clone()), &root, 1);
        let b = r.block_egress(BlockRequest { dst: "203.0.113.10".into(), port: Some(53), proto: Some("udp".into()), reason: Some("dns c2".into()), incident_id: None, expires_in_sec: None, dry_run: false });
        assert_eq!(b.status, 200, "{:?}", b.body);
        let calls: Vec<String> = rec.calls.lock().unwrap().iter().map(|(p, a, _)| format!("{p} {}", a.join(" "))).collect();
        assert!(calls.iter().any(|c| c == "/usr/sbin/iptables -w -A EDR_EGRESS -d 127.0.0.0/8 -j ACCEPT"));
        assert!(calls.iter().any(|c| c.starts_with("/usr/sbin/iptables -w -A EDR_EGRESS -d 203.0.113.10 -p udp --dport 53 -m comment")));
        assert!(calls.iter().any(|c| c == "/usr/sbin/iptables -w -I OUTPUT 1 -j EDR_EGRESS"));
        // No IPv6 block, so ip6tables only cleans up.
        assert!(calls.iter().filter(|c| c.starts_with("/usr/sbin/ip6tables")).all(|c| c.contains(" -D ") || c.contains(" -F ") || c.contains(" -X ")));

        let mut c = cfg(Firewall::Nft, &root);
        c.allowed = vec!["kill_proc".into()];
        let r = Responder::new(c, Box::new(rec.clone()), &root, 1);
        let o = r.isolate_host(IsolateRequest { reason: Some("x".into()), incident_id: None, expires_in_sec: None, dry_run: true });
        assert_eq!(o.status, 403);
        assert!(o.body["error"].as_str().unwrap().contains("signed policy"));
        let _ = std::fs::remove_dir_all(&root);
    }
}
