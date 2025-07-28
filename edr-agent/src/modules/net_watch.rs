use std::process::Command;
use std::{fs, thread, sync::mpsc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::convert::TryInto;
use crate::telemetry_writer::{TelemetryWriter, write_telemetry_record};
use crate::trust_hook::{submit_trust_event, TrustEvent, build_trust_payload, generate_feature_vector};
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::telemetry::{TelemetryRecord, calculate_entropy, get_file_metadata, is_memory_only};
use crate::utils::time::now_ts as utils_now_ts;
use chrono::{Timelike, Local};
use walkdir::WalkDir;
use crate::telemetry_types::TelemetryOutput;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use crate::logger::log;
use bytes::BytesMut;
use crate::modules::replay_writer::store_replay_event;


pub static SCAN_NETWORK_ANOMALIES: OnceLock<AtomicBool> = OnceLock::new();
/// Struct to represent each network connection
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
}

/// Entry point for launching the network monitor
/// Entry point for launching the network monitor



/// Returns (ppid, uid) by reading from /proc/[pid]/status
fn get_ppid_and_uid(pid: i32) -> (i32, u32) {
    let status_path = format!("/proc/{}/status", pid);
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



pub fn start_network_monitor() {
    thread::spawn(|| loop {
        let timestamp = current_unix_timestamp();

        let (connections, risk_score) = collect_network_connections(timestamp);


        let cpu_usage = 2.0;
        let memory_usage = 15000;

        let record = TelemetryRecord {
            timestamp,
            pid: -1,
            ppid: -1,
            uid: u32::MAX,
            binary_path: "net_monitor".to_string(),
            command_line: "".to_string(),
            cwd: "".to_string(),
            env_vars: Some(vec![]),
            risk_score: Some(risk_score as u32),
            tags: vec!["network_monitor".into()],
        };

        let trust_payload = build_trust_payload(&record);
        let _features = generate_feature_vector(cpu_usage, memory_usage, risk_score.into());

        // 🔐 Metadata with required keys for from_parts()
        let metadata = HashMap::from([
            ("connection_count".into(), connections.len().to_string()),
            ("risk_score".into(), format!("{:.2}", risk_score)),
            ("command_line".into(), "".into()),
            ("cwd".into(), "".into()),
        ]);

        let trust_event = TrustEvent::from_parts(
            timestamp,
            -1,
            -1,
            u32::MAX,
            "net_monitor".into(),
            "unusual_connection_pattern",
            "network",
            Some(metadata),
            risk_score,
            "net_watch",
            Some(format!("{} suspicious connections detected", connections.len())),
            Some(vec!["network".into(), "connection".into()]),
            Some("network_connection_anomaly".into()),
            Some("network::connection_scan".into()),
            Some("network_behavior".into()),
            Some("net_watch".into()),
        );

        submit_trust_event(trust_event);
        push_to_gnn_vector_log(trust_payload.clone());

        println!(
            "[🌐 Network Trust] Risk: {:.1} | Trust: {}",
            risk_score,
            trust_payload.get("trust_score").unwrap_or(&"N/A".to_string())
        );

        for conn in &connections {
            println!("[🌐 Connection] {:?}", conn);
        }

        thread::sleep(Duration::from_secs(30));
    });
}


pub fn log_open_connections(writer: &mut TelemetryWriter) {
    let timestamp = now_ts();
    let (connections, risk_score) = collect_network_connections(timestamp);


    let total_risk: f32 = connections.iter().map(|c| c.risk).sum();

    for conn_risk in &connections {
        let conn = &conn_risk.info;
        let risk = conn_risk.risk;

        let is_benign = is_known_benign(&conn.local_address, &conn.remote_address, &conn.process_name);

        let mut record = TelemetryRecord {
            timestamp: conn.timestamp,
            pid: conn.pid,
            ppid: conn.ppid,
            uid: conn.uid,
            binary_path: conn.process_name.clone(),
            command_line: format!("{} -> {} [{}]", conn.local_address, conn.remote_address, conn.status),
            cwd: conn.protocol.clone(),
            env_vars: Some(vec![]),
            risk_score: Some(risk as u32),
            tags: vec!["network::connection".into()],
        };

        if is_benign {
            record.tags.push("benign_suppressed".into());
        }

        writer.write_record(record.clone());

        if !is_benign && risk > 0.0 {
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
                Some(format!("Open connection from {} to {} [{}]", conn.local_address, conn.remote_address, conn.status)),
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

    let is_remote_local = benign_subnets.iter().any(|subnet| remote.starts_with(subnet));
    let is_local_local = benign_subnets.iter().any(|subnet| local.starts_with(subnet));

    let remote_port = remote.rsplit(':').next().unwrap_or("");
    let is_common_low_risk_port = benign_ports.contains(&remote_port);
    let is_benign_state = benign_statuses.contains(&status);

    // Only suppress if all three suggest low risk
    is_remote_local && is_common_low_risk_port && is_benign_state
}


fn calculate_network_risk(conn: &ConnectionInfo) -> f32 {
    let ip = conn.remote_address.to_lowercase();
    let port = conn.remote_address.split(':').last().unwrap_or("0").parse::<u16>().unwrap_or(0);

    let high_risk_ports = [4444, 31337, 1337, 135, 3389, 5900];
    let known_suspicious_ips = ["34.107.243.93", "8.8.8.8"]; // extend with threat intel

    if high_risk_ports.contains(&port) {
        return 10.0;
    } else if known_suspicious_ips.iter().any(|ip_sub| ip.contains(ip_sub)) {
        return 8.0;
    } else if conn.status == "ESTABLISHED" && !ip.contains("google") && port == 443 {
        return 5.0;
    }
    0.0
}pub fn collect_network_connections(timestamp: u64) -> (Vec<ConnectionInfoWithRisk>, f32) {
    let mut results = Vec::new();
    let mut total_risk = 0.0;

    let output = Command::new("netstat")
        .arg("-tunp")
        .output();

    if let Ok(output) = output {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            for line in stdout.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    let protocol = parts[0].to_string();
                    let local = parts[3].to_string();
                    let remote = parts[4].to_string();
                    let status = parts[5].to_string();
                    let pid_program = parts.get(6).unwrap_or(&"");

                    let (pid, pname) = parse_pid_and_name(pid_program);

                    // Risk scoring
                    let mut risk = 0.0;
                    if remote.contains(":6667") || remote.contains(":31337") {
                        risk += 2.5;
                    }
                    if status.to_lowercase() == "established" && remote.contains("unknown") {
                        risk += 1.5;
                    }
                    if pname == "netcat" || pname == "ncat" {
                        risk += 2.0;
                    }

                    total_risk += risk;

                    let info = ConnectionInfo {
                        timestamp,
                        pid,
                        ppid: -1,          // placeholder unless parsed
                        uid: u32::MAX,     // placeholder unless parsed
                        process_name: pname,
                        local_address: local,
                        remote_address: remote,
                        protocol,
                        status,
                    };

                    results.push(ConnectionInfoWithRisk { info, risk });
                }
            }
        }
    }

    (results, total_risk)
}





/// Extracts PID and process name from netstat-style string
fn parse_pid_and_name(entry: &str) -> (i32, String) {
    if let Some((pid_str, pname)) = entry.split_once("/") {
        let pid = pid_str.parse::<i32>().unwrap_or(-1);
        return (pid, pname.to_string());
    }
    (-1, "unknown".to_string())
}

/// Returns the current UNIX timestamp in seconds
fn current_unix_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}


#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DnsTunnelEvent {
    pub pid: u32,
    pub query_length: u32,
    pub domain_hash: u64,
    pub timestamp: u64,
}

#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Clone, Debug)]
pub struct NetworkAnomalyEvent {
    pub pid: u32,
    pub port: u16,
    pub protocol: u8,
    pub suspicious_flags: u8,
    pub timestamp: u64,
}

#[cfg(target_os = "linux")]
use aya::{
    Bpf,
    util::online_cpus,
    maps::perf::PerfEventArray,
    programs::TracePoint
};
#[cfg(target_os = "linux")]

pub fn start_ebpf_net_watch() -> Vec<TelemetryOutput> {
    let results = Arc::new(Mutex::new(Vec::new()));

    let handle_perf_array = |path: &str,
                             prog_name: &str,
                             attach_point: (&str, &str),
                             parse_fn: fn(&[u8]) -> Option<TelemetryOutput>| {
        if let Ok(data) = fs::read(path) {
            if let Ok(mut bpf) = Bpf::load(&data) {
                if let Some(prog) = bpf.program_mut(prog_name) {
                    if let Ok(typed_prog) = TryInto::<&mut TracePoint>::try_into(prog) {
                        let _ = typed_prog.load();
                        let _ = typed_prog.attach(attach_point.0, attach_point.1);
                    }
                }

                if let Ok(mut map) = bpf.map_mut("EVENTS") {
                    if let Ok(mut perf_array) = PerfEventArray::try_from(map) {
                        for cpu_id in online_cpus().unwrap_or_default() {
                            if let Ok(mut buf) = perf_array.open(cpu_id, None) {
                                let results_clone = Arc::clone(&results);
                                thread::spawn(move || {
                                    let mut buffers: Vec<BytesMut> = (0..8)
                                        .map(|_| BytesMut::with_capacity(4096))
                                        .collect();

                                    loop {
                                        match buf.read_events(&mut buffers[..]) {
                                            Ok(_events) => {
                                                for buffer in buffers.iter_mut() {
                                                    if !buffer.is_empty() {
                                                        if let Some(output) = parse_fn(&buffer[..]) {
                                                            if let Ok(mut locked) = results_clone.lock() {
                                                                locked.push(output.clone());
                                                            }

                                                            // 🔐 Submit TrustEvent using your defined constructor
                                                            let trust_event = TrustEvent::new_full(
                                                                now_ts(),
                                                                -1,
                                                                -1,
                                                                u32::MAX,
                                                                "ebpf_net".into(),
                                                                "start_ebpf_net_watch".into(),
                                                                "".into(),
                                                                "ebpf_network_anomaly".into(),
                                                                "net::ebpf".into(),
                                                                "net_watch".into(),
                                                                Some(format!(
                                                                    "eBPF network anomaly detected: {}",
                                                                    output.signal
                                                                )),
                                                                Some("network::ebpf_pattern".into()),
                                                                Some(vec![
                                                                    "network".into(),
                                                                    output.signal.clone(),
                                                                ]),
                                                                Some(6.5),
                                                            );

                                                            submit_trust_event(trust_event);
                                                            log(&format!(
                                                                "[eBPF NET] Event: {} | Signal: {}",
                                                                output.category, output.signal
                                                            ));
                                                        }
                                                        buffer.clear();
                                                    }
                                                }
                                            }
                                            Err(_) => thread::sleep(Duration::from_millis(5)),
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }
    };

    handle_perf_array(
        "/opt/edr-ebpf/network_anomaly_monitor.bpf.o",
        "trace_net_evt",
        ("net", "net_dev_queue"),
        parse_net_event,
    );

    handle_perf_array(
        "/opt/edr-ebpf/dns_tunnel_monitor.bpf.o",
        "trace_dns_evt",
        ("syscalls", "sys_enter_sendto"),
        parse_dns_event,
    );

    Arc::try_unwrap(results).unwrap_or_default().into_inner().unwrap_or_default()
}




pub fn detect_suspicious_proxies() -> Vec<TelemetryOutput> {
    use std::collections::{HashMap, HashSet};
    use std::fs::{self, File};
    use std::path::Path;
    use std::io::{BufRead, BufReader};

    let mut outputs = Vec::new();
    let now = now_ts();
    let mut seen = HashSet::new();

    let proxy_binaries: HashSet<&str> = HashSet::from([
        "tor", "obfs4proxy", "v2ray", "xray", "shadowsocks", "ssserver", "sslocal",
        "tun2socks", "proxychains", "privoxy", "squid", "vpnkit", "openvpn",
        "stunnel", "sockd",
    ]);

    // 🔍 Scan /proc for exact binary name matches
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let pid_str = entry.file_name().to_string_lossy().to_string();
            if !pid_str.chars().all(char::is_numeric) {
                continue;
            }

            let pid: i32 = match pid_str.parse() {
                Ok(n) => n,
                Err(_) => continue,
            };

            let exe_path = format!("/proc/{}/exe", pid);
            let exe_name = fs::read_link(&exe_path)
                .ok()
                .and_then(|p| p.file_name().map(|f| f.to_string_lossy().to_string()));

            if let Some(binary) = exe_name {
                if proxy_binaries.contains(binary.as_str()) && !seen.contains(&binary) {
                    seen.insert(binary.clone());

                    // Metadata extraction
                    let status_path = format!("/proc/{}/status", pid);
                    let cmdline_path = format!("/proc/{}/cmdline", pid);
                    let cwd_path = format!("/proc/{}/cwd", pid);
                 let ppid = fs::read_to_string(&status_path)
                    .ok()
                    .and_then(|s| {
                        s.lines()
                            .find(|l| l.starts_with("PPid:"))
                            .and_then(|line| line.split_whitespace().nth(1).map(|v| v.to_string()))
                    })
                    .and_then(|val| val.parse::<i32>().ok())
                    .unwrap_or(-1);

                let uid = fs::read_to_string(&status_path)
                    .ok()
                    .and_then(|s| {
                        s.lines()
                            .find(|l| l.starts_with("Uid:"))
                            .and_then(|line| line.split_whitespace().nth(1).map(|v| v.to_string()))
                    })
                    .and_then(|val| val.parse::<u32>().ok())
                    .unwrap_or(u32::MAX);

                    let cmdline = fs::read_to_string(&cmdline_path)
                        .unwrap_or_else(|_| "unknown".into())
                        .replace('\0', " ");

                    let cwd = fs::read_link(&cwd_path)
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|_| "/proc".into());


                    let mut data = HashMap::new();
                    data.insert("binary".into(), binary.clone());
                    data.insert("summary".into(), format!("Suspicious proxy binary found: {}", binary));
                    data.insert("timestamp".into(), now.to_string());
                    data.insert("replay_tag".into(), "proxy_detect".into());
                    data.insert("gnn_escalate".into(), "true".into());
                    data.insert("soc_note".into(), "Known proxy tool running".into());

                    let trust_event = TrustEvent::new_full(
                        now,
                        pid,
                        ppid,
                        uid,
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

                    // Ensure GNN log file exists
                    let gnn_path = Path::new("/var/log/edr/gnn_vector_log.jsonl");
                    if !gnn_path.exists() {
                        if let Err(e) = File::create(gnn_path) {
                            log(&format!("[proxy_scan] Failed to create GNN log file: {}", e));

                        }
                    }
                    push_to_gnn_vector_log(data.clone());

                    outputs.push(TelemetryOutput {
                        category: "network".into(),
                        signal: "proxy_binary".into(),
                        confidence: 0.9,
                        data,
                    });
                }
            }
        }
    }

    // 🛰 Check for known proxy listener ports
    if let Ok(output) = Command::new("ss").arg("-tunlp").output() {
        if let Ok(text) = String::from_utf8(output.stdout) {
            for line in text.lines().skip(1) {
                if line.contains("LISTEN") && (line.contains(":1080") || line.contains(":9050")) {
                    let mut data = HashMap::new();
                    data.insert("summary".into(), "Potential local proxy listener".into());
                    data.insert("line".into(), line.to_string());
                    data.insert("timestamp".into(), now.to_string());
                    data.insert("replay_tag".into(), "proxy_listener".into());
                    data.insert("soc_note".into(), "Suspicious SOCKS or Tor proxy listener port".into());

                    let trust_event = TrustEvent::new_full(
                        now,
                        -1,
                        -1,
                        u32::MAX,
                        "unknown".into(),
                        line.to_string(),
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

                    outputs.push(TelemetryOutput {
                        category: "network".into(),
                        signal: "proxy_listener".into(),
                        confidence: 0.8,
                        data,
                    });
                }
            }
        }
    }

    outputs
}


#[cfg(target_os = "linux")]
fn parse_net_event(buf: &[u8]) -> Option<TelemetryOutput> {
    use std::{ptr::read_unaligned, collections::HashMap};
    use crate::trust_hook::{submit_trust_event, TrustEvent};
    use crate::utils::time::now_ts;
    use crate::gnn_hook::push_to_gnn_vector_log;
    use crate::telemetry_writer::write_telemetry_record;
    use crate::telemetry_types::TelemetryOutput;

    let ptr = buf.as_ptr() as *const NetworkAnomalyEvent;
    let evt = unsafe { read_unaligned(ptr) };

    let mut data = HashMap::new();
    data.insert("timestamp".into(), evt.timestamp.to_string());
    data.insert("pid".into(), evt.pid.to_string());
    data.insert("port".into(), evt.port.to_string());
    data.insert("flags".into(), format!("{:?}", evt.suspicious_flags));
    data.insert(
        "summary".into(),
        format!(
            "Suspicious socket activity on port {} (flags={:?})",
            evt.port, evt.suspicious_flags
        ),
    );
    data.insert("replay_tag".into(), "network_anomaly".into());
    data.insert("gnn_escalate".into(), "true".into());
    data.insert("soc_note".into(), "Unusual socket behavior from eBPF trace".into());

    let trust_evt = TrustEvent::new_full(
        evt.timestamp,
        evt.pid as i32,
        -1,
        u32::MAX,
        "unknown".into(),
        "ebpf_net_anomaly".into(),
        "/proc".into(),
        "ebpf".into(),
        "network::ebpf_net_anomaly".into(),
        "net_watch".into(),
        Some("Suspicious socket flags detected in eBPF trace".into()),
        Some("network::ebpf_flags".into()),
        Some(vec!["ebpf".into(), "net".into(), "anomaly".into()]),
        Some(8.5),
    );
    submit_trust_event(trust_evt);
    write_telemetry_record(data.clone());
    push_to_gnn_vector_log(data.clone());

    Some(TelemetryOutput {
        category: "network".into(),
        signal: "network_anomaly".into(),
        confidence: 0.8,
        data,
    })
}
#[cfg(target_os = "linux")]
fn parse_dns_event(buf: &[u8]) -> Option<TelemetryOutput> {
    use std::{ptr::read_unaligned, collections::HashMap};
    use crate::trust_hook::{submit_trust_event, TrustEvent};
    use crate::telemetry_types::TelemetryOutput;
    use crate::utils::time::now_ts;
    use crate::telemetry_writer::write_telemetry_record;
    use crate::gnn_hook::push_to_gnn_vector_log;

    let ptr = buf.as_ptr() as *const DnsTunnelEvent;
    let evt = unsafe { read_unaligned(ptr) };
    let now = now_ts();

    let mut data = HashMap::new();
    data.insert("timestamp".to_string(), evt.timestamp.to_string());
    data.insert("pid".to_string(), evt.pid.to_string());
    data.insert("query_length".to_string(), evt.query_length.to_string());
    data.insert("summary".to_string(), format!(
        "Potential DNS tunneling detected (query_len = {})",
        evt.query_length
    ));
    data.insert("replay_tag".into(), "dns_tunnel_detected".into());
    data.insert("gnn_escalate".into(), "true".into());
    data.insert("soc_note".into(), "High-entropy or oversized DNS query detected".into());

    let trust_evt = TrustEvent::new_full(
        now,
        evt.pid as i32,
        -1,
        u32::MAX,
        "unknown".into(),
        "dns_tunnel_check".into(),
        "/proc".into(),
        "network".into(),
        "network::dns_tunnel".into(),
        "net_watch".into(),
        Some("Suspicious DNS tunneling behavior detected".into()),
        Some("network::dns_tunnel".into()),
        Some(vec!["dns".into(), "tunnel".into(), "covert_channel".into()]),
        Some(10.0),
    );
    submit_trust_event(trust_evt);
    write_telemetry_record(data.clone());
    push_to_gnn_vector_log(data.clone());

    Some(TelemetryOutput {
        category: "network".to_string(),
        signal: "dns_tunnel".to_string(),
        confidence: 0.85,
        data,
    })
}


fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}/// Passive sweep function for batch-mode trust scoring

pub fn scan_network_anomalies() -> Vec<TelemetryOutput> {
    let mut results = Vec::new();

    if SCAN_NETWORK_ANOMALIES
        .get_or_init(|| AtomicBool::new(true))
        .load(Ordering::Relaxed)
    {
        let proxy_events = detect_suspicious_proxies();

        #[cfg(target_os = "linux")]
        let ebpf_events = start_ebpf_net_watch();

        results.extend(proxy_events);
        #[cfg(target_os = "linux")]
        results.extend(ebpf_events);
    }

    for output in &results {
        let score = match output.signal.as_str() {
            "proxy_binary" | "proxy_listener" => 12.0,
            "dns_tunnel" => 14.0,
            "network_anomaly" => 10.0,
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
                .unwrap_or_else(now_ts),
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
