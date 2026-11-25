// edr-agent/src/modules/net_watch.rs
use std::collections::{HashMap, HashSet};
use std::fs;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::Duration;

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::ioc::{IocItem, IocKind};
use crate::modules::replay_writer::store_replay_event;
use crate::telemetry::TelemetryRecord;
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::{write_telemetry_record, TelemetryWriter};
use crate::trust_hook::{submit_trust_event, TrustEvent};
use crate::utils::time::now_ts as utils_now_ts;

pub static SCAN_NETWORK_ANOMALIES: OnceLock<AtomicBool> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct ConnectionInfo {
    pub timestamp: u64,
    pub pid: i32,
    pub ppid: i32,
    pub uid: u32,
    pub process_name: String,
    pub local_address: String,
    pub remote_address: String,
    pub protocol: String,
    pub status: String,
}

#[derive(Clone, Debug)]
pub struct ConnectionInfoWithRisk {
    pub info: ConnectionInfo,
    pub risk: f32,
    /// What raised the risk. None when the risk is 0.
    pub reason: Option<RiskReason>,
}

/// Why a connection scored above 0.
#[derive(Clone, Debug, PartialEq)]
pub enum RiskReason {
    /// The remote port is in HIGH_RISK_REMOTE_PORTS.
    HighRiskPort(u16),
    /// The remote address matched an ip or ip_prefix entry in the IOC feed (entry id).
    IocMatch(String),
}

impl RiskReason {
    pub fn describe(&self) -> String {
        match self {
            RiskReason::HighRiskPort(port) => {
                format!("remote port {port} is on the high-risk port list")
            }
            RiskReason::IocMatch(id) => format!("remote address matches IOC {id}"),
        }
    }
}

/// Returns (ppid, uid) by reading from /proc/[pid]/status
fn get_ppid_and_uid(pid: i32) -> (i32, u32) {
    read_ppid_and_uid(Path::new("/proc"), pid)
}

/// Same as get_ppid_and_uid under any proc root, so tests can use a fake tree.
fn read_ppid_and_uid(proc_root: &Path, pid: i32) -> (i32, u32) {
    let status_path = proc_root.join(pid.to_string()).join("status");
    if let Ok(contents) = fs::read_to_string(status_path) {
        let mut ppid = -1;
        let mut uid = u32::MAX;

        for line in contents.lines() {
            if line.starts_with("PPid:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    ppid = val.parse().unwrap_or(-1);
                }
            } else if line.starts_with("Uid:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    uid = val.parse().unwrap_or(u32::MAX);
                }
            }
        }

        return (ppid, uid);
    }
    (-1, u32::MAX)
}

/// Contents of /proc/<pid>/comm without the newline.
fn read_comm(proc_root: &Path, pid: i32) -> Option<String> {
    let text = fs::read_to_string(proc_root.join(pid.to_string()).join("comm")).ok()?;
    Some(text.trim_end().to_string())
}

/// Numeric entries of a proc root, one per process.
fn list_pids(proc_root: &Path) -> Vec<i32> {
    let mut pids = Vec::new();
    let Ok(entries) = fs::read_dir(proc_root) else {
        return pids;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        if let Some(pid) = name.to_str().and_then(|n| n.parse::<i32>().ok()) {
            if pid > 0 {
                pids.push(pid);
            }
        }
    }
    pids
}

// ---- Connection snapshots on connect ----
// Each decoded connect or accept used to walk every /proc/<pid>/fd on the decoder
// thread. Now it only asks, and one worker thread runs the snapshot, at most once per
// SNAPSHOT_MIN_INTERVAL. Asks that arrive meanwhile fold into the next run, so a burst of
// connects costs one walk, and the last connect of a burst is still covered by a
// snapshot that starts at most one interval later.

const SNAPSHOT_MIN_INTERVAL: Duration = Duration::from_secs(1);

/// One pending flag plus a wakeup. Requests set the flag. The worker clears it, runs the
/// job, then rests for the interval before it looks again.
struct Coalescer {
    pending: Mutex<bool>,
    wake: Condvar,
}

impl Coalescer {
    fn new() -> Self {
        Self {
            pending: Mutex::new(false),
            wake: Condvar::new(),
        }
    }

    fn request(&self) {
        if let Ok(mut p) = self.pending.lock() {
            *p = true;
            self.wake.notify_one();
        }
    }

    /// Worker loop. Never returns.
    fn run(&self, interval: Duration, mut job: impl FnMut()) {
        loop {
            {
                let Ok(mut p) = self.pending.lock() else {
                    return;
                };
                while !*p {
                    p = match self.wake.wait(p) {
                        Ok(g) => g,
                        Err(_) => return,
                    };
                }
                *p = false;
            }
            job();
            std::thread::sleep(interval);
        }
    }
}

static SNAPSHOTS: OnceLock<Arc<Coalescer>> = OnceLock::new();

/// Asks for a snapshot of open connections. Returns at once. The first call starts the
/// worker, which writes through the writer it was given.
pub fn request_connection_snapshot(writer: &Arc<Mutex<TelemetryWriter>>) {
    let c = SNAPSHOTS.get_or_init(|| {
        let c = Arc::new(Coalescer::new());
        let worker = c.clone();
        let w = writer.clone();
        let started = std::thread::Builder::new()
            .name("net-snapshot".into())
            .spawn(move || {
                worker.run(SNAPSHOT_MIN_INTERVAL, || {
                    if let Ok(mut guard) = w.lock() {
                        log_open_connections(&mut guard);
                    }
                })
            });
        if let Err(e) = started {
            eprintln!("net_watch: snapshot worker did not start: {e}");
        }
        c
    });
    c.request();
}

// ---- One alert per risky connection ----
// Every snapshot lists every open connection again, so a connection that stays open
// used to raise one TrustEvent per snapshot. A connection is reported once per (pid,
// remote address) and again only after a snapshot no longer shows it. A socket in
// TIME_WAIT has no owner pid, so it is matched to the pid its 4-tuple had when it was
// last seen open, and a closing connection does not raise a second alert as pid -1.

type ConnKey = (i32, SocketAddr);
type Tuple = (SocketAddr, SocketAddr);

#[derive(Default)]
struct RiskyConnTracker {
    reported: HashSet<ConnKey>,
    owners: HashMap<Tuple, i32>,
}

impl RiskyConnTracker {
    /// For each risky connection of one snapshot, true when it gets a new TrustEvent.
    /// Keys and tuples not in this snapshot are forgotten.
    fn new_alerts(&mut self, risky: &[(i32, SocketAddr, SocketAddr)]) -> Vec<bool> {
        let mut seen = HashSet::new();
        let mut owners = HashMap::new();
        let mut out = Vec::with_capacity(risky.len());
        for &(pid, local, remote) in risky {
            let pid = if pid > 0 {
                pid
            } else {
                self.owners.get(&(local, remote)).copied().unwrap_or(pid)
            };
            let key = (pid, remote);
            out.push(!self.reported.contains(&key) && !seen.contains(&key));
            seen.insert(key);
            owners.insert((local, remote), pid);
        }
        self.reported = seen;
        self.owners = owners;
        out
    }
}

static RISKY_CONNS: OnceLock<Mutex<RiskyConnTracker>> = OnceLock::new();

pub fn log_open_connections(writer: &mut TelemetryWriter) {
    let timestamp = utils_now_ts();
    let (connections, _risk_score) = collect_network_connections(timestamp);

    let total_risk: f32 = connections.iter().map(|c| c.risk).sum();

    // Which risky connections are new since the last snapshot.
    let alertable = |c: &ConnectionInfoWithRisk| {
        c.risk > 0.0
            && !is_known_benign(&c.info.local_address, &c.info.remote_address, &c.info.status)
    };
    let mut new_alert = vec![false; connections.len()];
    let mut risky = Vec::new();
    let mut risky_at = Vec::new();
    for (i, c) in connections.iter().enumerate() {
        if !alertable(c) {
            continue;
        }
        match (
            c.info.local_address.parse::<SocketAddr>(),
            c.info.remote_address.parse::<SocketAddr>(),
        ) {
            (Ok(local), Ok(remote)) => {
                risky.push((c.info.pid, local, remote));
                risky_at.push(i);
            }
            // Addresses come from the formatter above, so this should not happen. If it
            // does, alert as before rather than lose the event.
            _ => new_alert[i] = true,
        }
    }
    let fresh = RISKY_CONNS
        .get_or_init(|| Mutex::new(RiskyConnTracker::default()))
        .lock()
        .map(|mut t| t.new_alerts(&risky))
        .unwrap_or_else(|_| vec![true; risky.len()]);
    for (k, i) in risky_at.into_iter().enumerate() {
        new_alert[i] = fresh[k];
    }

    for (i, conn_risk) in connections.iter().enumerate() {
        let conn = &conn_risk.info;
        let risk = conn_risk.risk;

        // pass STATUS (was process_name before)
        let is_benign = is_known_benign(&conn.local_address, &conn.remote_address, &conn.status);

        let mut record = TelemetryRecord {
            timestamp: conn.timestamp,
            pid: conn.pid,
            ppid: conn.ppid,
            uid: conn.uid,
            binary_path: conn.process_name.clone(),
            command_line: format!(
                "{} -> {} [{}]",
                conn.local_address, conn.remote_address, conn.status
            ),
            cwd: conn.protocol.clone(),
            env_vars: Some(vec![]),
            risk_score: Some(risk as u32),
            tags: vec!["network::connection".into()],
        };

        if is_benign {
            record.tags.push("benign_suppressed".into());
        }
        // Same tag ioc.rs puts on records with an IP hit.
        if let Some(RiskReason::IocMatch(_)) = &conn_risk.reason {
            record.tags.push("ioc_ip".into());
        }

        writer.write_record(record.clone());

        if new_alert[i] {
            let why = conn_risk
                .reason
                .as_ref()
                .map(|r| r.describe())
                .unwrap_or_default();
            let trust_event = TrustEvent::new_full(
                conn.timestamp,
                conn.pid,
                conn.ppid,
                conn.uid,
                conn.process_name.clone(),
                record.command_line.clone(),
                conn.protocol.clone(),
                "suspicious_connection".into(),
                "net_watch".into(),
                "net_watch".into(),
                Some(format!(
                    "Open connection from {} to {} [{}]: {}",
                    conn.local_address, conn.remote_address, conn.status, why
                )),
                Some("network::connection_scan".into()),
                Some(vec!["network".into(), "connection".into()]),
                Some(risk),
            );

            submit_trust_event(trust_event);
        }
    }

    println!(
        "[🌐 Passive Net Log] {} connection(s) recorded | Risk {:.1}",
        connections.len(),
        total_risk
    );
}

pub fn is_known_benign(local: &str, remote: &str, status: &str) -> bool {
    let benign_ports = ["53", "67", "68", "123"]; // DNS, DHCP, NTP
    let benign_statuses = ["TIME_WAIT", "CLOSE_WAIT"];
    let benign_subnets = ["127.", "::1", "192.168.", "10.", "172.16.", "172.31."];

    // IPv6 addresses are written as [addr]:port
    let remote_host = remote.trim_start_matches('[');
    let local_host = local.trim_start_matches('[');

    let is_remote_local = benign_subnets
        .iter()
        .any(|subnet| remote_host.starts_with(subnet));
    let is_local_local = benign_subnets
        .iter()
        .any(|subnet| local_host.starts_with(subnet));

    let remote_port = remote.rsplit(':').next().unwrap_or("");
    let is_common_low_risk_port = benign_ports.contains(&remote_port);
    let is_benign_state = benign_statuses.contains(&status);

    // Only suppress if all three suggest low risk
    is_remote_local && is_common_low_risk_port && is_benign_state && is_local_local
}

// ---- Connection risk ----
// Two inputs only: a fixed list of remote ports, and addresses the operator put
// in the IOC feed. There is no built-in IP list.

/// Remote ports scored as high risk. 4444 is the Metasploit handler default,
/// 31337 and 1337 are classic backdoor ports, and 135, 3389 and 5900 are
/// MS-RPC, RDP and VNC remote access.
const HIGH_RISK_REMOTE_PORTS: [u16; 6] = [4444, 31337, 1337, 135, 3389, 5900];
const HIGH_RISK_PORT_SCORE: f32 = 10.0;
/// Score for a remote address that is in the IOC feed.
const IOC_ADDRESS_SCORE: f32 = 8.0;
/// How often the IOC feed is read again, so edits and /ioc/reload show up here too.
const NET_IOC_REFRESH_SECS: u64 = 60;

/// An ip or ip_prefix entry from the IOC feed.
#[derive(Clone, Debug)]
struct NetIoc {
    id: String,
    value: String,
    /// Parsed form of an ip entry, so 2001:DB8::1 and 2001:db8::1 match.
    addr: Option<IpAddr>,
    is_prefix: bool,
    until: Option<u64>,
}

impl NetIoc {
    fn matches(&self, ip: IpAddr, now: u64) -> bool {
        if let Some(until) = self.until {
            if now > until {
                return false;
            }
        }
        if self.is_prefix {
            // Text prefix, the same rule ioc.rs uses for ip_prefix.
            ip.to_string().starts_with(&self.value)
        } else {
            self.addr == Some(ip)
        }
    }
}

/// Keeps the ip and ip_prefix entries of an IOC list.
fn net_iocs_from(items: &[IocItem]) -> Vec<NetIoc> {
    let mut iocs = Vec::new();
    for item in items {
        let is_prefix = match item.kind {
            IocKind::Ip => false,
            IocKind::IpPrefix => true,
            _ => continue,
        };
        let value = item.value.trim().to_string();
        iocs.push(NetIoc {
            // Same default id format as ioc.rs.
            id: item
                .id
                .clone()
                .unwrap_or_else(|| format!("ioc:{:?}:{}", item.kind, value)),
            addr: value.parse().ok(),
            value,
            is_prefix,
            until: item.until,
        });
    }
    iocs
}

struct NetIocCache {
    loaded_at: u64,
    iocs: Arc<Vec<NetIoc>>,
    last_error: Option<String>,
}

static NET_IOC_CACHE: OnceLock<Mutex<Option<NetIocCache>>> = OnceLock::new();

/// ip and ip_prefix entries from the agent's IOC folder: IOC_DIR, default
/// state/ioc, the same folder ioc::init reads at startup. A failed read keeps
/// the last good list and is logged once per distinct error.
fn network_iocs(now: u64) -> Arc<Vec<NetIoc>> {
    let cache = NET_IOC_CACHE.get_or_init(|| Mutex::new(None));
    let Ok(mut guard) = cache.lock() else {
        return Arc::new(Vec::new());
    };
    if let Some(cached) = guard.as_ref() {
        if now.saturating_sub(cached.loaded_at) < NET_IOC_REFRESH_SECS {
            return cached.iocs.clone();
        }
    }

    let dir = std::env::var("IOC_DIR").unwrap_or_else(|_| "state/ioc".to_string());
    let previous = guard.take();
    let (iocs, last_error) = match crate::ioc::load_all(&dir) {
        Ok(store) => (Arc::new(net_iocs_from(&store.items)), None),
        Err(e) => {
            let text = format!("{e:#}");
            let seen_before = previous
                .as_ref()
                .and_then(|p| p.last_error.as_deref())
                .is_some_and(|old| old == text);
            if !seen_before {
                eprintln!("[net_watch] IOC feed in {dir} not loaded: {text}");
            }
            let kept = previous
                .map(|p| p.iocs)
                .unwrap_or_else(|| Arc::new(Vec::new()));
            (kept, Some(text))
        }
    };
    *guard = Some(NetIocCache {
        loaded_at: now,
        iocs: iocs.clone(),
        last_error,
    });
    iocs
}

/// Scores one connection by its remote end. The first match wins, port first.
fn score_connection(
    remote: &SocketAddr,
    iocs: &[NetIoc],
    now: u64,
) -> (f32, Option<RiskReason>) {
    if HIGH_RISK_REMOTE_PORTS.contains(&remote.port()) {
        return (
            HIGH_RISK_PORT_SCORE,
            Some(RiskReason::HighRiskPort(remote.port())),
        );
    }
    for ioc in iocs {
        if ioc.matches(remote.ip(), now) {
            return (IOC_ADDRESS_SCORE, Some(RiskReason::IocMatch(ioc.id.clone())));
        }
    }
    (0.0, None)
}

// ---- Socket tables from /proc/net ----
// /proc/net/{tcp,tcp6,udp,udp6} list every socket in the agent's network
// namespace. Reading them directly replaces a netstat or ss child process per
// call. Sockets in other network namespaces (other containers) are not listed.

/// The tables read, with the protocol name netstat printed for each.
const PROC_NET_TABLES: [(&str, &str); 4] = [
    ("tcp", "net/tcp"),
    ("tcp6", "net/tcp6"),
    ("udp", "net/udp"),
    ("udp6", "net/udp6"),
];

const TCP_LISTEN: u8 = 0x0A;
/// UDP reuses the TCP state numbers. 7 (TCP_CLOSE) means not connected.
const UDP_UNCONNECTED: u8 = 0x07;

/// One row of a /proc/net socket table.
#[derive(Clone, Debug, PartialEq)]
struct SocketRow {
    local: SocketAddr,
    remote: SocketAddr,
    state: u8,
    uid: u32,
    inode: u64,
}

/// Parses one data line of a socket table, for example
/// `0: 0100007F:1F90 0100007F:D2A4 01 00000000:00000000 00:00000000 00000000 1000 0 31337 1 ...`.
/// Fields 1 and 2 are address:port, 3 is the state, 7 the uid and 9 the inode.
fn parse_socket_row(line: &str) -> Option<SocketRow> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 10 || !fields[0].ends_with(':') {
        return None;
    }
    Some(SocketRow {
        local: parse_hex_socket_addr(fields[1])?,
        remote: parse_hex_socket_addr(fields[2])?,
        state: u8::from_str_radix(fields[3], 16).ok()?,
        uid: fields[7].parse().ok()?,
        inode: fields[9].parse().ok()?,
    })
}

/// Decodes ADDR:PORT from a socket table. ADDR is 8 hex digits for IPv4 or 32
/// for IPv6. The kernel prints each 32-bit word of the network-order address as
/// a host-order integer, so on a little-endian machine 127.0.0.1 reads
/// 0100007F. Turning each word back into native-order bytes gives the address
/// on any host. PORT is already a plain host-order number.
fn parse_hex_socket_addr(field: &str) -> Option<SocketAddr> {
    let (addr_hex, port_hex) = field.split_once(':')?;
    let port = u16::from_str_radix(port_hex, 16).ok()?;
    let ip = match addr_hex.len() {
        8 => IpAddr::V4(Ipv4Addr::from(hex_word_bytes(addr_hex)?)),
        32 => {
            let mut bytes = [0u8; 16];
            for word in 0..4 {
                let chunk = addr_hex.get(word * 8..word * 8 + 8)?;
                bytes[word * 4..word * 4 + 4].copy_from_slice(&hex_word_bytes(chunk)?);
            }
            let v6 = Ipv6Addr::from(bytes);
            // A dual-stack socket talking IPv4 shows ::ffff:a.b.c.d. Report it as
            // IPv4 so the benign check and IPv4 IOC entries match it.
            match v6.to_ipv4_mapped() {
                Some(v4) => IpAddr::V4(v4),
                None => IpAddr::V6(v6),
            }
        }
        _ => return None,
    };
    Some(SocketAddr::new(ip, port))
}

/// Native-order bytes of one 8-digit hex word.
fn hex_word_bytes(hex: &str) -> Option<[u8; 4]> {
    if hex.len() != 8 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    Some(u32::from_str_radix(hex, 16).ok()?.to_ne_bytes())
}

/// TCP state names as netstat prints them (numbers from include/net/tcp_states.h).
fn tcp_state_name(state: u8) -> &'static str {
    match state {
        0x01 => "ESTABLISHED",
        0x02 => "SYN_SENT",
        0x03 => "SYN_RECV",
        0x04 => "FIN_WAIT1",
        0x05 => "FIN_WAIT2",
        0x06 => "TIME_WAIT",
        0x07 => "CLOSE",
        0x08 => "CLOSE_WAIT",
        0x09 => "LAST_ACK",
        0x0A => "LISTEN",
        0x0B => "CLOSING",
        0x0C => "NEW_SYN_RECV",
        _ => "UNKNOWN",
    }
}

fn udp_state_name(state: u8) -> &'static str {
    match state {
        0x01 => "ESTABLISHED",
        UDP_UNCONNECTED => "UNCONN",
        _ => "UNKNOWN",
    }
}

/// Every row of the four tables with its protocol. A missing table (tcp6 and
/// udp6 when IPv6 is off) is skipped.
fn read_socket_tables(proc_root: &Path) -> Vec<(&'static str, SocketRow)> {
    let mut rows = Vec::new();
    for (protocol, table) in PROC_NET_TABLES {
        let Ok(text) = fs::read_to_string(proc_root.join(table)) else {
            continue;
        };
        // The first line is the column header.
        for line in text.lines().skip(1) {
            if let Some(row) = parse_socket_row(line) {
                rows.push((protocol, row));
            }
        }
    }
    rows
}

/// Inode from a /proc/<pid>/fd link target such as `socket:[31337]`.
fn socket_inode(link: &str) -> Option<u64> {
    link.strip_prefix("socket:[")?.strip_suffix(']')?.parse().ok()
}

/// Maps socket inodes to the pid holding them, from the /proc/<pid>/fd links.
/// A socket shared after fork goes to the lowest pid. Processes the agent may
/// not read (when it is not root) are skipped, and their sockets show pid -1.
fn socket_owners(proc_root: &Path) -> HashMap<u64, i32> {
    let mut owners: HashMap<u64, i32> = HashMap::new();
    for pid in list_pids(proc_root) {
        let Ok(fds) = fs::read_dir(proc_root.join(pid.to_string()).join("fd")) else {
            continue;
        };
        for fd in fds.flatten() {
            let Ok(target) = fs::read_link(fd.path()) else {
                continue;
            };
            let Some(inode) = socket_inode(&target.to_string_lossy()) else {
                continue;
            };
            let owner = owners.entry(inode).or_insert(pid);
            if pid < *owner {
                *owner = pid;
            }
        }
    }
    owners
}

pub fn collect_network_connections(timestamp: u64) -> (Vec<ConnectionInfoWithRisk>, f32) {
    let iocs = network_iocs(timestamp);
    collect_connections_from(Path::new("/proc"), timestamp, &iocs)
}

/// Open connections under a proc root. Listening TCP sockets and unconnected
/// UDP sockets are left out, as `netstat -tunp` did.
fn collect_connections_from(
    proc_root: &Path,
    timestamp: u64,
    iocs: &[NetIoc],
) -> (Vec<ConnectionInfoWithRisk>, f32) {
    let mut rows = read_socket_tables(proc_root);
    rows.retain(|(protocol, row)| {
        if protocol.starts_with("tcp") {
            row.state != TCP_LISTEN
        } else {
            row.state != UDP_UNCONNECTED
        }
    });

    // Walking every fd is the costly part, so skip it when no row has an owner.
    let owners = if rows.iter().any(|(_, row)| row.inode != 0) {
        socket_owners(proc_root)
    } else {
        HashMap::new()
    };
    // ppid, uid and name for each pid, read once per call.
    let mut processes: HashMap<i32, (i32, u32, String)> = HashMap::new();

    let mut results = Vec::new();
    let mut total_risk = 0.0;
    for (protocol, row) in rows {
        // TIME_WAIT sockets have inode 0 and no owner.
        let pid = if row.inode == 0 {
            -1
        } else {
            owners.get(&row.inode).copied().unwrap_or(-1)
        };
        let (ppid, uid, process_name) = if pid > 0 {
            processes
                .entry(pid)
                .or_insert_with(|| {
                    let (ppid, uid) = read_ppid_and_uid(proc_root, pid);
                    let name = read_comm(proc_root, pid).unwrap_or_else(|| "unknown".to_string());
                    (ppid, uid, name)
                })
                .clone()
        } else {
            (-1, u32::MAX, "unknown".to_string())
        };
        // Fall back to the socket owner's uid from the table.
        let uid = if uid == u32::MAX { row.uid } else { uid };

        let status = if protocol.starts_with("tcp") {
            tcp_state_name(row.state)
        } else {
            udp_state_name(row.state)
        };
        let (risk, reason) = score_connection(&row.remote, iocs, timestamp);
        total_risk += risk;

        let info = ConnectionInfo {
            timestamp,
            pid,
            ppid,
            uid,
            process_name,
            local_address: row.local.to_string(),
            remote_address: row.remote.to_string(),
            protocol: protocol.to_string(),
            status: status.to_string(),
        };
        results.push(ConnectionInfoWithRisk { info, risk, reason });
    }

    (results, total_risk)
}

// ---- eBPF network sensors ----
// Connects and TX/RX byte counts come from net_flow.bpf.c (net_events ring),
// decoded in ebpf_ingest::decode_net. The only eBPF sensor started from this
// module is the DNS tunnel monitor.

/// Starts the eBPF sensors this module owns. The sweep calls it every time,
/// but the load and attach run on the first call only.
#[cfg(target_os = "linux")]
pub fn start_ebpf_net_watch() {
    start_dns_tunnel_monitor_once();
}

// ---- DNS tunnel monitor ----
// The kprobes, the parser and the scoring live in modules/dns_tunnel.rs. This
// part starts the monitor once and turns flagged queries into agent events.

#[cfg(target_os = "linux")]
use crate::modules::dns_tunnel::{self, DnsAssessment, DnsSendEvent, DnsTunnelDetector};

/// Default object path. EDR_BPF_DNS_OBJ overrides it.
#[cfg(target_os = "linux")]
const DNS_TUNNEL_OBJ: &str = "/opt/edr-ebpf/dns_tunnel_monitor.bpf.o";

/// Alerts waiting for the next sweep. Bounded so a stalled sweep cannot grow it forever.
#[cfg(target_os = "linux")]
const MAX_PENDING_DNS_ALERTS: usize = 1024;

#[cfg(target_os = "linux")]
static DNS_MONITOR_START: std::sync::Once = std::sync::Once::new();

#[cfg(target_os = "linux")]
static DNS_ALERTS: OnceLock<Mutex<std::collections::VecDeque<TelemetryOutput>>> = OnceLock::new();

#[cfg(target_os = "linux")]
fn dns_alert_queue() -> &'static Mutex<std::collections::VecDeque<TelemetryOutput>> {
    DNS_ALERTS.get_or_init(|| Mutex::new(std::collections::VecDeque::new()))
}

/// Loads and attaches the DNS kprobes on the first call only. If that fails
/// (no object file, not root) it is logged once and not retried every sweep.
#[cfg(target_os = "linux")]
fn start_dns_tunnel_monitor_once() {
    DNS_MONITOR_START.call_once(|| {
        let obj_path =
            std::env::var("EDR_BPF_DNS_OBJ").unwrap_or_else(|_| DNS_TUNNEL_OBJ.to_string());
        let mut detector = DnsTunnelDetector::new();
        let started = dns_tunnel::start_dns_monitor(&obj_path, move |event| {
            // Datagrams that do not parse as a DNS query are dropped here.
            let Ok(assessment) = detector.process(event, utils_now_ts()) else {
                return;
            };
            if !assessment.should_alert() {
                return;
            }
            let output = report_dns_tunnel(event, &assessment);
            if let Ok(mut queue) = dns_alert_queue().lock() {
                if queue.len() >= MAX_PENDING_DNS_ALERTS {
                    queue.pop_front();
                }
                queue.push_back(output);
            }
        });
        match started {
            Ok(()) => eprintln!("[net_watch] DNS tunnel monitor attached ({obj_path})"),
            Err(e) => eprintln!("[net_watch] DNS tunnel monitor not started: {e}"),
        }
    });
}

/// Hands the alerts raised since the last sweep to the sweep.
#[cfg(target_os = "linux")]
fn take_dns_alerts() -> Vec<TelemetryOutput> {
    match dns_alert_queue().lock() {
        Ok(mut queue) => queue.drain(..).collect(),
        Err(_) => Vec::new(),
    }
}

/// Builds the agent event for one flagged query and sends it down the usual
/// trust, telemetry and GNN paths. The first tag stays "dns_tunnel" so
/// playbooks that match dns_tunnel* still fire.
#[cfg(target_os = "linux")]
fn report_dns_tunnel(event: &DnsSendEvent, a: &DnsAssessment) -> TelemetryOutput {
    let now = utils_now_ts();
    let pid = event.tgid as i32;
    let comm = event.comm();
    let (ppid, _) = get_ppid_and_uid(pid);
    // Best effort. A short-lived client may already be gone.
    let exe = fs::read_link(format!("/proc/{pid}/exe"))
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    let qtype = dns_tunnel::qtype_name(a.qtype);
    let reason = a.reason_text();
    let summary = format!(
        "Possible DNS tunneling: {qtype} query for {} from {comm} (pid {pid}): {reason}",
        a.name
    );

    let mut data = HashMap::new();
    data.insert("timestamp".to_string(), now.to_string());
    data.insert("pid".to_string(), pid.to_string());
    data.insert("tid".to_string(), event.tid.to_string());
    data.insert("uid".to_string(), event.uid.to_string());
    data.insert("comm".to_string(), comm.clone());
    data.insert("exe".to_string(), exe.clone());
    data.insert("family".to_string(), event.family_name().to_string());
    data.insert("domain".to_string(), a.name.clone());
    data.insert("base_domain".to_string(), a.base_domain.clone());
    data.insert("qtype".to_string(), qtype);
    data.insert("name_length".to_string(), a.features.name_len.to_string());
    data.insert("longest_label".to_string(), a.features.longest_label.to_string());
    data.insert("label_count".to_string(), a.features.label_count.to_string());
    data.insert(
        "subdomain_entropy".to_string(),
        format!("{:.2}", a.features.subdomain_entropy),
    );
    data.insert(
        "unique_subdomains_in_window".to_string(),
        a.unique_subdomains.to_string(),
    );
    data.insert("reason".to_string(), reason);
    data.insert("summary".to_string(), summary.clone());
    // T1071.004 Application Layer Protocol: DNS. T1048 Exfiltration Over Alternative Protocol.
    data.insert("technique".to_string(), "T1071.004".to_string());
    data.insert("mitre".to_string(), "T1071.004,T1048".to_string());
    data.insert("replay_tag".into(), "dns_tunnel_detected".into());
    data.insert("gnn_escalate".into(), "true".into());
    data.insert(
        "soc_note".into(),
        "DNS query shaped like a tunnel: long, random or high-volume subdomains".into(),
    );

    let mut trust_evt = TrustEvent::new_full(
        now,
        pid,
        ppid,
        event.uid,
        exe,
        comm,
        "n/a".into(),
        "network".into(),
        "network::dns_tunnel".into(),
        "net_watch".into(),
        Some(summary),
        Some("network::dns_tunnel".into()),
        Some(vec![
            "dns_tunnel".into(),
            "dns".into(),
            "tunnel".into(),
            "covert_channel".into(),
            "T1071.004".into(),
            "T1048".into(),
        ]),
        Some(10.0),
    );
    trust_evt.metadata = data.clone();
    submit_trust_event(trust_evt);
    write_telemetry_record(data.clone());
    push_to_gnn_vector_log(data.clone());

    TelemetryOutput {
        category: "network".to_string(),
        signal: "dns_tunnel".to_string(),
        confidence: 0.85,
        data,
    }
}

// ---- Proxy and tunnel tools ----

/// Proxy, tunnel and VPN tools matched by executable name.
#[cfg(target_os = "linux")]
const PROXY_BINARIES: [&str; 15] = [
    "tor",
    "obfs4proxy",
    "v2ray",
    "xray",
    "shadowsocks",
    "ssserver",
    "sslocal",
    "tun2socks",
    "proxychains",
    "privoxy",
    "squid",
    "vpnkit",
    "openvpn",
    "stunnel",
    "sockd",
];

/// Listening TCP ports that suggest a local proxy: 1080 is the SOCKS default
/// and 9050 the Tor SOCKS port. Matched exactly, so 10800 does not count.
#[cfg(target_os = "linux")]
const PROXY_LISTEN_PORTS: [u16; 2] = [1080, 9050];

/// A proxy tool process or listener seen for the first time.
#[cfg(target_os = "linux")]
#[derive(Clone, Debug, PartialEq)]
enum ProxyFinding {
    Binary {
        pid: i32,
        ppid: i32,
        uid: u32,
        binary: String,
        cmdline: String,
        cwd: String,
    },
    Listener {
        pid: i32,
        ppid: i32,
        uid: u32,
        process_name: String,
        protocol: &'static str,
        local: SocketAddr,
    },
}

/// File name of /proc/<pid>/exe. A binary deleted after it started reads
/// "name (deleted)", and the suffix is dropped so it still matches.
#[cfg(target_os = "linux")]
fn exe_name(proc_root: &Path, pid: i32) -> Option<String> {
    let target = fs::read_link(proc_root.join(pid.to_string()).join("exe")).ok()?;
    let name = target.file_name()?.to_string_lossy().to_string();
    Some(name.strip_suffix(" (deleted)").unwrap_or(&name).to_string())
}

/// Walks /proc for proxy tools. Each (pid, binary) and each listening socket is
/// reported once while it lasts, so a long-running tool raises one alert instead
/// of one per sweep.
#[cfg(target_os = "linux")]
#[derive(Default)]
struct ProxyScanner {
    reported_binaries: HashSet<(i32, String)>,
    reported_listeners: HashSet<u64>,
}

#[cfg(target_os = "linux")]
impl ProxyScanner {
    fn scan(&mut self, proc_root: &Path) -> Vec<ProxyFinding> {
        let mut found = Vec::new();

        let mut running = HashSet::new();
        for pid in list_pids(proc_root) {
            // Kernel threads have no exe link.
            let Some(binary) = exe_name(proc_root, pid) else {
                continue;
            };
            if !PROXY_BINARIES.contains(&binary.as_str()) {
                continue;
            }
            let key = (pid, binary.clone());
            running.insert(key.clone());
            if !self.reported_binaries.insert(key) {
                continue;
            }
            let dir = proc_root.join(pid.to_string());
            let (ppid, uid) = read_ppid_and_uid(proc_root, pid);
            let cmdline = fs::read(dir.join("cmdline"))
                .map(|b| crate::redact::cmdline_from_proc(&b))
                .unwrap_or_else(|_| "unknown".into());
            let cwd = fs::read_link(dir.join("cwd"))
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "/proc".into());
            found.push(ProxyFinding::Binary {
                pid,
                ppid,
                uid,
                binary,
                cmdline,
                cwd,
            });
        }
        // Forget processes that exited, so the set stays small and a restarted
        // tool is reported again.
        self.reported_binaries.retain(|key| running.contains(key));

        let mut listening = HashSet::new();
        let mut new_listeners = Vec::new();
        for (protocol, row) in read_socket_tables(proc_root) {
            if !protocol.starts_with("tcp")
                || row.state != TCP_LISTEN
                || !PROXY_LISTEN_PORTS.contains(&row.local.port())
            {
                continue;
            }
            listening.insert(row.inode);
            if self.reported_listeners.insert(row.inode) {
                new_listeners.push((protocol, row));
            }
        }
        self.reported_listeners
            .retain(|inode| listening.contains(inode));

        if !new_listeners.is_empty() {
            let owners = socket_owners(proc_root);
            for (protocol, row) in new_listeners {
                let pid = owners.get(&row.inode).copied().unwrap_or(-1);
                let (ppid, uid, process_name) = if pid > 0 {
                    let (ppid, uid) = read_ppid_and_uid(proc_root, pid);
                    let name = read_comm(proc_root, pid).unwrap_or_else(|| "unknown".into());
                    (ppid, uid, name)
                } else {
                    (-1, row.uid, "unknown".to_string())
                };
                found.push(ProxyFinding::Listener {
                    pid,
                    ppid,
                    uid,
                    process_name,
                    protocol,
                    local: row.local,
                });
            }
        }

        found
    }
}

#[cfg(target_os = "linux")]
static PROXY_SCANNER: OnceLock<Mutex<ProxyScanner>> = OnceLock::new();

#[cfg(target_os = "linux")]
pub fn detect_suspicious_proxies() -> Vec<TelemetryOutput> {
    let findings = match PROXY_SCANNER
        .get_or_init(|| Mutex::new(ProxyScanner::default()))
        .lock()
    {
        Ok(mut scanner) => scanner.scan(Path::new("/proc")),
        Err(_) => return Vec::new(),
    };

    let now = utils_now_ts();
    let mut outputs = Vec::new();
    for finding in &findings {
        outputs.push(report_proxy_finding(finding, now));
    }
    outputs
}

/// Sends one finding down the trust, telemetry and GNN paths.
#[cfg(target_os = "linux")]
fn report_proxy_finding(finding: &ProxyFinding, now: u64) -> TelemetryOutput {
    match finding {
        ProxyFinding::Binary {
            pid,
            ppid,
            uid,
            binary,
            cmdline,
            cwd,
        } => {
            let mut data = HashMap::new();
            data.insert("binary".into(), binary.clone());
            data.insert("pid".into(), pid.to_string());
            data.insert(
                "summary".into(),
                format!("Suspicious proxy binary found: {} (pid {})", binary, pid),
            );
            data.insert("timestamp".into(), now.to_string());
            data.insert("replay_tag".into(), "proxy_detect".into());
            data.insert("gnn_escalate".into(), "true".into());
            data.insert("soc_note".into(), "Known proxy tool running".into());

            let trust_event = TrustEvent::new_full(
                now,
                *pid,
                *ppid,
                *uid,
                binary.clone(),
                cmdline.clone(),
                cwd.clone(),
                "proxy_tool".into(),
                "network::proxy_scan".into(),
                "net_watch".into(),
                Some(format!("Detected proxy binary: {}", binary)),
                Some("network::proxy_binary".into()),
                Some(vec!["proxy".into(), binary.clone()]),
                Some(12.0),
            );
            submit_trust_event(trust_event);
            write_telemetry_record(data.clone());
            push_to_gnn_vector_log(data.clone());

            TelemetryOutput {
                category: "network".into(),
                signal: "proxy_binary".into(),
                confidence: 0.9,
                data,
            }
        }
        ProxyFinding::Listener {
            pid,
            ppid,
            uid,
            process_name,
            protocol,
            local,
        } => {
            let line = format!("{} LISTEN {}", protocol, local);
            let mut data = HashMap::new();
            data.insert(
                "summary".into(),
                format!(
                    "Potential local proxy listener: {} (pid {}) on {}",
                    process_name, pid, line
                ),
            );
            data.insert("line".into(), line.clone());
            data.insert("pid".into(), pid.to_string());
            data.insert("port".into(), local.port().to_string());
            data.insert("timestamp".into(), now.to_string());
            data.insert("replay_tag".into(), "proxy_listener".into());
            data.insert(
                "soc_note".into(),
                "Suspicious SOCKS or Tor proxy listener port".into(),
            );

            let trust_event = TrustEvent::new_full(
                now,
                *pid,
                *ppid,
                *uid,
                process_name.clone(),
                line,
                "/proc".into(),
                "proxy_listener".into(),
                "network::proxy_scan".into(),
                "net_watch".into(),
                Some("Detected potential SOCKS/Tor proxy listener".into()),
                Some("network::proxy_listener".into()),
                Some(vec!["proxy".into(), "listener".into()]),
                Some(9.0),
            );
            submit_trust_event(trust_event);
            write_telemetry_record(data.clone());
            push_to_gnn_vector_log(data.clone());

            TelemetryOutput {
                category: "network".into(),
                signal: "proxy_listener".into(),
                confidence: 0.8,
                data,
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
pub fn detect_suspicious_proxies() -> Vec<TelemetryOutput> {
    Vec::new()
}

/// Top-level passive sweep
pub fn scan_network_anomalies() -> Vec<TelemetryOutput> {
    let mut results = Vec::new();

    if SCAN_NETWORK_ANOMALIES
        .get_or_init(|| AtomicBool::new(true))
        .load(Ordering::Relaxed)
    {
        let proxy_events = detect_suspicious_proxies();

        // The DNS monitor is loaded on the first sweep. Later sweeps only
        // collect the alerts it raised since the last one.
        #[cfg(target_os = "linux")]
        let ebpf_events = {
            start_ebpf_net_watch();
            take_dns_alerts()
        };

        results.extend(proxy_events);
        #[cfg(target_os = "linux")]
        results.extend(ebpf_events);
    }

    for output in &results {
        let score = match output.signal.as_str() {
            "proxy_binary" | "proxy_listener" => 12.0,
            "dns_tunnel" => 14.0,
            _ => 6.0,
        };

        let description = output
            .data
            .get("summary")
            .cloned()
            .unwrap_or_else(|| format!("Anomaly detected: {}", output.signal));

        let mut tags = vec![
            "network".into(),
            "telemetry_batch".into(),
            output.signal.clone(),
        ];
        if output.signal == "dns_tunnel" {
            tags.push("gnn_escalate".into());
            tags.push("forensic_replay".into());
        }

        let trust_event = TrustEvent {
            timestamp: output
                .data
                .get("timestamp")
                .and_then(|ts| ts.parse::<u64>().ok())
                .unwrap_or_else(utils_now_ts),
            pid: -1,
            ppid: -1,
            uid: u32::MAX,
            binary_path: "n/a".to_string(),
            command_line: "n/a".to_string(),
            cwd: "/".to_string(),
            anomaly_type: "network_anomaly".to_string(),
            component: "net_watch::batch_scan".to_string(),
            source_module: "net_watch".to_string(),
            metadata: output.data.clone(),
            risk_score: score,
            decay_context: Some("network_batch".to_string()),
            module: Some("net_watch".to_string()),
            signal: Some(output.signal.clone()),
            signal_type: Some(output.signal.clone()),
            score: Some(score),
            raw_score: None,
            tags: Some(tags),
            description: Some(description.clone()),
        };

        submit_trust_event(trust_event);

        let mut gnn_data = output.data.clone();
        gnn_data.insert("category".into(), "network".into());
        gnn_data.insert("signal".into(), output.signal.clone());
        gnn_data.insert("confidence".into(), format!("{:.2}", score / 15.0));
        gnn_data.insert("gnn_escalate".into(), "true".into());
        gnn_data.insert("summary".into(), description.clone());
        gnn_data.insert("replay_tag".into(), "network_batch".into());

        push_to_gnn_vector_log(gnn_data.clone());
        store_replay_event(gnn_data);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const TABLE_HEADER: &str = "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode\n";
    const TABLE6_HEADER: &str = "  sl  local_address                         remote_address                        st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode\n";

    fn addr(text: &str) -> SocketAddr {
        text.parse().unwrap()
    }

    // The hex samples are what a little-endian kernel (x86_64, arm64) prints.
    #[cfg(target_endian = "little")]
    #[test]
    fn ipv4_hex_is_read_in_kernel_byte_order() {
        assert_eq!(parse_hex_socket_addr("0100007F:1F90"), Some(addr("127.0.0.1:8080")));
        // Not a palindrome, so a byte-order mistake would show: 10.0.2.15 port 80.
        assert_eq!(parse_hex_socket_addr("0F02000A:0050"), Some(addr("10.0.2.15:80")));
        assert_eq!(parse_hex_socket_addr("00000000:0000"), Some(addr("0.0.0.0:0")));
    }

    #[cfg(target_endian = "little")]
    #[test]
    fn ipv6_hex_is_read_word_by_word() {
        assert_eq!(
            parse_hex_socket_addr("00000000000000000000000001000000:0035"),
            Some(addr("[::1]:53"))
        );
        // 2001:db8::1 port 443. Each 8-digit word is byte-swapped on its own.
        assert_eq!(
            parse_hex_socket_addr("B80D0120000000000000000001000000:01BB"),
            Some(addr("[2001:db8::1]:443"))
        );
        // fe80::1234:5678:9abc:def0 port 22, four distinct words.
        assert_eq!(
            parse_hex_socket_addr("000080FE0000000078563412F0DEBC9A:0016"),
            Some(addr("[fe80::1234:5678:9abc:def0]:22"))
        );
    }

    #[cfg(target_endian = "little")]
    #[test]
    fn v4_mapped_ipv6_is_reported_as_ipv4() {
        let got = parse_hex_socket_addr("0000000000000000FFFF00000100007F:1F90").unwrap();
        assert_eq!(got, addr("127.0.0.1:8080"));
        assert!(got.is_ipv4());
    }

    #[test]
    fn bad_hex_fields_are_rejected() {
        assert_eq!(parse_hex_socket_addr("0100007F"), None);
        assert_eq!(parse_hex_socket_addr("0100007:1F90"), None);
        assert_eq!(parse_hex_socket_addr("0100007G:1F90"), None);
        assert_eq!(parse_hex_socket_addr("+100007F:1F90"), None);
        assert_eq!(parse_hex_socket_addr("0100007F:1F9000"), None);
        assert_eq!(parse_hex_socket_addr("0000000000000000000000000100000:0035"), None);
        assert_eq!(parse_hex_socket_addr(""), None);
    }

    #[cfg(target_endian = "little")]
    #[test]
    fn socket_rows_parse_state_uid_and_inode() {
        let tcp = "   1: 0100007F:D2A4 0100007F:1F90 01 00000000:00000000 00:00000000 00000000  1000        0 31337 1 0000000000000000 20 4 30 10 -1";
        let row = parse_socket_row(tcp).unwrap();
        assert_eq!(row.local, addr("127.0.0.1:53924"));
        assert_eq!(row.remote, addr("127.0.0.1:8080"));
        assert_eq!(row.state, 0x01);
        assert_eq!(row.uid, 1000);
        assert_eq!(row.inode, 31337);

        let tcp6 = "   0: 00000000000000000000000001000000:0016 00000000000000000000000001000000:C350 06 00000000:00000000 03:00000F9F 00000000     0        0 0 3 0000000000000000";
        let row = parse_socket_row(tcp6).unwrap();
        assert_eq!(row.local, addr("[::1]:22"));
        assert_eq!(row.remote, addr("[::1]:50000"));
        assert_eq!(row.state, 0x06);
        assert_eq!(row.inode, 0);

        assert_eq!(parse_socket_row(TABLE_HEADER.trim_end()), None);
        assert_eq!(parse_socket_row("   2: 0100007F:D2A4 0100007F:1F90 01"), None);
        assert_eq!(tcp_state_name(0x06), "TIME_WAIT");
        assert_eq!(tcp_state_name(0x0A), "LISTEN");
        assert_eq!(udp_state_name(0x07), "UNCONN");
    }

    #[test]
    fn socket_inode_reads_only_socket_links() {
        assert_eq!(socket_inode("socket:[31337]"), Some(31337));
        assert_eq!(socket_inode("pipe:[31337]"), None);
        assert_eq!(socket_inode("/dev/null"), None);
        assert_eq!(socket_inode("socket:[]"), None);
    }

    fn fake_proc(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("edr_netwatch_{name}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("net")).unwrap();
        // A non-numeric entry, like /proc/sys, must be skipped.
        fs::create_dir_all(root.join("sys")).unwrap();
        root
    }

    #[cfg(unix)]
    fn write_process(root: &Path, pid: i32, comm: &str, exe: &str, ppid: i32, uid: u32, socket_inodes: &[u64]) {
        let dir = root.join(pid.to_string());
        fs::create_dir_all(dir.join("fd")).unwrap();
        fs::write(dir.join("comm"), format!("{comm}\n")).unwrap();
        fs::write(
            dir.join("status"),
            format!("Name:\t{comm}\nPid:\t{pid}\nPPid:\t{ppid}\nUid:\t{uid}\t{uid}\t{uid}\t{uid}\n"),
        )
        .unwrap();
        fs::write(dir.join("cmdline"), format!("{comm}\0--flag\0")).unwrap();
        std::os::unix::fs::symlink(exe, dir.join("exe")).unwrap();
        std::os::unix::fs::symlink("/", dir.join("cwd")).unwrap();
        std::os::unix::fs::symlink("/dev/null", dir.join("fd").join("0")).unwrap();
        for (n, inode) in socket_inodes.iter().enumerate() {
            let link = format!("socket:[{inode}]");
            std::os::unix::fs::symlink(link, dir.join("fd").join((n + 3).to_string())).unwrap();
        }
    }

    #[cfg(all(unix, target_endian = "little"))]
    #[test]
    fn connections_come_from_proc_tables_with_owner_pids() {
        let root = fake_proc("conns");
        // inode 555: 10.0.2.15:41000 -> 93.184.216.34:443, ESTABLISHED, held by pids 42 and 50.
        // inode 556: LISTEN on 0.0.0.0:8080, left out.
        // inode 0:   TIME_WAIT, no owner.
        let tcp = format!(
            "{TABLE_HEADER}   0: 0F02000A:A028 22D8B85D:01BB 01 00000000:00000000 00:00000000 00000000  1000        0 555 1\n   1: 00000000:1F90 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 556 1\n   2: 0100007F:D2A4 0100007F:1F91 06 00000000:00000000 03:00000F9F 00000000     0        0 0 3\n"
        );
        fs::write(root.join("net/tcp"), tcp).unwrap();
        // inode 557: [2001:db8::1]:443 -> [2001:db8::2]:50000, ESTABLISHED, held by pid 50.
        let tcp6 = format!(
            "{TABLE6_HEADER}   0: B80D0120000000000000000001000000:01BB B80D0120000000000000000002000000:C350 01 00000000:00000000 00:00000000 00000000    33        0 557 1\n"
        );
        fs::write(root.join("net/tcp6"), tcp6).unwrap();
        // inode 558: unconnected UDP, left out. inode 559: connected UDP to 10.0.2.3:53.
        let udp = format!(
            "{TABLE_HEADER}   0: 00000000:0044 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 558 2\n   1: 0F02000A:B5E2 0302000A:0035 01 00000000:00000000 00:00000000 00000000   101        0 559 2\n"
        );
        fs::write(root.join("net/udp"), udp).unwrap();
        // No udp6 file, as on a host with IPv6 off.
        write_process(&root, 50, "nginx", "/usr/sbin/nginx", 42, 33, &[555, 557]);
        write_process(&root, 42, "curl", "/usr/bin/curl", 1, 1000, &[555, 559]);

        let (conns, total) = collect_connections_from(&root, 1_700_000_000, &[]);
        assert_eq!(conns.len(), 4, "{conns:?}");
        assert_eq!(total, 0.0);

        let est = &conns[0].info;
        assert_eq!(est.protocol, "tcp");
        assert_eq!(est.local_address, "10.0.2.15:41000");
        assert_eq!(est.remote_address, "93.184.216.34:443");
        assert_eq!(est.status, "ESTABLISHED");
        // Shared after fork: the lowest pid wins.
        assert_eq!((est.pid, est.ppid, est.uid), (42, 1, 1000));
        assert_eq!(est.process_name, "curl");

        let tw = &conns[1].info;
        assert_eq!(tw.status, "TIME_WAIT");
        assert_eq!((tw.pid, tw.process_name.as_str(), tw.uid), (-1, "unknown", 0));

        let v6 = &conns[2].info;
        assert_eq!(v6.protocol, "tcp6");
        assert_eq!(v6.local_address, "[2001:db8::1]:443");
        assert_eq!(v6.remote_address, "[2001:db8::2]:50000");
        assert_eq!((v6.pid, v6.uid, v6.process_name.as_str()), (50, 33, "nginx"));

        let udp = &conns[3].info;
        assert_eq!(udp.protocol, "udp");
        assert_eq!(udp.remote_address, "10.0.2.3:53");
        assert_eq!(udp.status, "ESTABLISHED");
        assert_eq!(udp.pid, 42);

        let _ = fs::remove_dir_all(&root);
    }

    fn ioc(id: &str, value: &str, is_prefix: bool, until: Option<u64>) -> NetIoc {
        NetIoc {
            id: id.into(),
            value: value.into(),
            addr: value.parse().ok(),
            is_prefix,
            until,
        }
    }

    #[test]
    fn risk_comes_only_from_ports_and_the_ioc_feed() {
        let now = 1_700_000_000;
        // The old code scored 8.8.8.8 from a built-in list and every :443 as 5.
        assert_eq!(score_connection(&addr("8.8.8.8:443"), &[], now), (0.0, None));
        assert_eq!(score_connection(&addr("34.107.243.93:443"), &[], now), (0.0, None));
        assert_eq!(
            score_connection(&addr("203.0.113.9:4444"), &[], now),
            (10.0, Some(RiskReason::HighRiskPort(4444)))
        );

        let feed = vec![
            ioc("c2-host", "203.0.113.7", false, None),
            ioc("c2-range", "198.51.100.", true, None),
            ioc("old", "192.0.2.1", false, Some(now - 1)),
            ioc("v6", "2001:DB8::BAD", false, None),
        ];
        assert_eq!(
            score_connection(&addr("203.0.113.7:443"), &feed, now),
            (8.0, Some(RiskReason::IocMatch("c2-host".into())))
        );
        assert_eq!(
            score_connection(&addr("198.51.100.20:80"), &feed, now),
            (8.0, Some(RiskReason::IocMatch("c2-range".into())))
        );
        // Expired entries do not match.
        assert_eq!(score_connection(&addr("192.0.2.1:80"), &feed, now), (0.0, None));
        // IPv6 entries compare as addresses, not text.
        assert_eq!(
            score_connection(&addr("[2001:db8::bad]:443"), &feed, now),
            (8.0, Some(RiskReason::IocMatch("v6".into())))
        );
        // A near miss is not a match.
        assert_eq!(score_connection(&addr("203.0.113.70:443"), &feed, now), (0.0, None));
    }

    #[test]
    fn ioc_feed_files_load_through_ioc_rs() {
        let dir = std::env::temp_dir().join(format!("edr_netwatch_ioc_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("feed.jsonl"),
            "{\"kind\":\"ip\",\"value\":\"203.0.113.7\",\"id\":\"c2-host\"}\n{\"kind\":\"ip_prefix\",\"value\":\"198.51.100.\"}\n{\"kind\":\"domain\",\"value\":\"evil.example\"}\n",
        )
        .unwrap();
        let store = crate::ioc::load_all(&dir).unwrap();
        let mut iocs = net_iocs_from(&store.items);
        iocs.sort_by(|a, b| a.id.cmp(&b.id));
        assert_eq!(iocs.len(), 2, "domain entries are not used here");
        assert_eq!(iocs[0].id, "c2-host");
        assert!(!iocs[0].is_prefix);
        assert_eq!(iocs[1].id, "ioc:IpPrefix:198.51.100.");
        assert!(iocs[1].is_prefix);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn benign_check_handles_bracketed_ipv6() {
        assert!(is_known_benign("[::1]:40000", "[::1]:53", "TIME_WAIT"));
        assert!(is_known_benign("127.0.0.1:40000", "127.0.0.1:53", "CLOSE_WAIT"));
        assert!(!is_known_benign("[::1]:40000", "[::1]:53", "ESTABLISHED"));
        assert!(!is_known_benign("10.0.2.15:40000", "93.184.216.34:53", "TIME_WAIT"));
    }

    #[cfg(all(target_os = "linux", target_endian = "little"))]
    #[test]
    fn proxy_tools_are_reported_once_per_process_and_socket() {
        let root = fake_proc("proxy");
        write_process(&root, 100, "tor", "/usr/bin/tor", 1, 0, &[777]);
        write_process(&root, 200, "bash", "/usr/bin/bash", 1, 0, &[]);
        write_process(&root, 300, "xray", "/opt/xray (deleted)", 1, 1000, &[]);
        // 127.0.0.1:9050 LISTEN held by tor, and 127.0.0.1:10800 LISTEN, which must not match.
        let tcp = format!(
            "{TABLE_HEADER}   0: 0100007F:235A 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 777 1\n   1: 0100007F:2A30 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 778 1\n"
        );
        fs::write(root.join("net/tcp"), &tcp).unwrap();

        let mut scanner = ProxyScanner::default();
        let first = scanner.scan(&root);
        assert_eq!(first.len(), 3, "{first:?}");
        assert!(first.contains(&ProxyFinding::Binary {
            pid: 100,
            ppid: 1,
            uid: 0,
            binary: "tor".into(),
            cmdline: "tor --flag".into(),
            cwd: "/".into(),
        }));
        assert!(first.iter().any(|f| matches!(f, ProxyFinding::Binary { pid: 300, binary, .. } if binary == "xray")));
        assert!(first.contains(&ProxyFinding::Listener {
            pid: 100,
            ppid: 1,
            uid: 0,
            process_name: "tor".into(),
            protocol: "tcp",
            local: addr("127.0.0.1:9050"),
        }));

        // Same processes and socket on the next sweep: nothing new.
        assert!(scanner.scan(&root).is_empty());

        // tor exits and its socket closes, then tor starts again with the same pid.
        fs::remove_dir_all(root.join("100")).unwrap();
        fs::write(root.join("net/tcp"), TABLE_HEADER).unwrap();
        assert!(scanner.scan(&root).is_empty());
        assert_eq!(scanner.reported_binaries.len(), 1);
        assert!(scanner.reported_listeners.is_empty());
        write_process(&root, 100, "tor", "/usr/bin/tor", 1, 0, &[777]);
        fs::write(root.join("net/tcp"), &tcp).unwrap();
        assert_eq!(scanner.scan(&root).len(), 2);

        let _ = fs::remove_dir_all(&root);
    }

    // Reads the real /proc: a loopback connection made here must show up with
    // its addresses and this test's own pid.
    #[cfg(target_os = "linux")]
    fn sa(text: &str) -> SocketAddr {
        text.parse().unwrap()
    }

    #[test]
    fn risky_connection_alerts_once_until_it_closes() {
        let mut t = RiskyConnTracker::default();
        let open = (4242, sa("127.0.0.1:55574"), sa("127.0.0.1:4444"));
        // six snapshots while it stays open: one alert
        let alerts: Vec<bool> = (0..6).flat_map(|_| t.new_alerts(&[open])).collect();
        assert_eq!(alerts, vec![true, false, false, false, false, false]);
        // closing: TIME_WAIT has no owner pid, but the 4-tuple ties it to 4242
        let closing = (-1, sa("127.0.0.1:55574"), sa("127.0.0.1:4444"));
        assert_eq!(t.new_alerts(&[closing]), vec![false]);
        // gone from a snapshot, then a new connection to the same place alerts again
        assert!(t.new_alerts(&[]).is_empty());
        assert_eq!(t.new_alerts(&[(4242, sa("127.0.0.1:55580"), sa("127.0.0.1:4444"))]), vec![true]);
    }

    #[test]
    fn risky_connection_key_is_pid_and_remote() {
        let mut t = RiskyConnTracker::default();
        let a = (100, sa("10.0.0.5:40000"), sa("203.0.113.9:4444"));
        let b = (100, sa("10.0.0.5:40001"), sa("203.0.113.9:4444")); // same pid and remote
        let c = (200, sa("10.0.0.5:40002"), sa("203.0.113.9:4444")); // other pid
        let d = (100, sa("10.0.0.5:40003"), sa("203.0.113.9:1337")); // other port
        assert_eq!(t.new_alerts(&[a, b, c, d]), vec![true, false, true, true]);
        assert_eq!(t.new_alerts(&[a, b, c, d]), vec![false, false, false, false]);
        // a TIME_WAIT socket never seen open is still reported once, as pid -1
        let tw = (-1, sa("10.0.0.5:40009"), sa("203.0.113.9:4444"));
        assert_eq!(t.new_alerts(&[tw]), vec![true]);
        assert_eq!(t.new_alerts(&[tw]), vec![false]);
    }

    #[test]
    fn snapshot_requests_coalesce() {
        use std::sync::atomic::AtomicUsize;
        use std::time::Instant;
        let c = Arc::new(Coalescer::new());
        let runs = Arc::new(AtomicUsize::new(0));
        let (worker, counter) = (c.clone(), runs.clone());
        std::thread::spawn(move || {
            worker.run(Duration::from_millis(200), || {
                counter.fetch_add(1, Ordering::SeqCst);
            })
        });
        let wait_for = |n: usize| {
            let start = Instant::now();
            while runs.load(Ordering::SeqCst) < n && start.elapsed() < Duration::from_secs(5) {
                std::thread::sleep(Duration::from_millis(5));
            }
        };
        c.request();
        wait_for(1);
        assert_eq!(runs.load(Ordering::SeqCst), 1, "the first request runs at once");
        // 100 requests while the worker rests fold into one more run
        for _ in 0..100 {
            c.request();
        }
        wait_for(2);
        // nothing is pending now, so no third run comes after the next rest
        std::thread::sleep(Duration::from_millis(600));
        assert_eq!(runs.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn real_loopback_connection_is_found_with_its_pid() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let server_addr = listener.local_addr().unwrap();
        let client = std::net::TcpStream::connect(server_addr).unwrap();
        let (_accepted, _) = listener.accept().unwrap();
        let client_addr = client.local_addr().unwrap();

        let (conns, _) = collect_connections_from(Path::new("/proc"), 0, &[]);
        let mine: Vec<&ConnectionInfo> = conns
            .iter()
            .map(|c| &c.info)
            .filter(|c| c.local_address == client_addr.to_string() && c.remote_address == server_addr.to_string())
            .collect();
        assert_eq!(mine.len(), 1, "client side not found in {} rows", conns.len());
        assert_eq!(mine[0].protocol, "tcp");
        assert_eq!(mine[0].status, "ESTABLISHED");
        assert_eq!(mine[0].pid, std::process::id() as i32);
        // The listening socket is left out.
        assert!(!conns.iter().any(|c| c.info.local_address == server_addr.to_string() && c.info.status == "LISTEN"));
    }
}
