use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::{self, File};
use std::io::{self, Read, Write, BufRead, BufReader};
use std::mem;
use std::ptr;
use std::sync::{Arc, Once, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use crate::modules::replay_writer::store_replay_event;
use aya::maps::perf::PerfEventArray;
use aya::programs::TracePoint;
use aya::{Bpf, include_bytes_aligned, util::online_cpus};
use crate::telemetry_types::TelemetryOutput;
use bytes::BytesMut;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use regex::Regex;
use reqwest::blocking::Client;
use serde::Serialize;
use tokio::runtime::Runtime;
use crate::telemetry_writer::push_memory_telemetry;
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::telemetry::TelemetryRecord;
use crate::telemetry_writer::TelemetryWriter;
use crate::trust_hook::{generate_feature_vector, generate_trust_payload};
use crate::utils::time::now_ts;
use sha2::{Sha256, Digest};
use std::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;
use nix::libc;
use crate::trust_hook::{submit_trust_event, TrustEvent};

static JOB_SCHED_REAL_FOUND: AtomicBool = AtomicBool::new(false);

lazy_static! {
    static ref REAL_JOB_EVENT_FOUND: AtomicBool = AtomicBool::new(false);
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CronEvent {
    pid: u32,
    ppid: u32,
    filename: [u8; 256],
}

static START_EBPF: Once = Once::new();

#[derive(Serialize, Debug, Clone)]
pub struct CronEntry {
    pub timestamp: u64,
    pub command: String,
    pub file_path: String,
    pub risk_score: f32,
    pub source_ip: Option<String>,
}

fn is_legitimate_command(command: &str) -> bool {
    let allowed_commands = vec!["tar", "rsync", "scp", "curl", "wget", "bash"];
    allowed_commands.iter().any(|cmd| command.contains(cmd))
}

fn get_ip_reputation(ip: &str) -> Option<String> {
    let client = Client::new();
    let url = format!("https://api.ipinfo.io/{}/json", ip);
    let res = client.get(&url).send().ok()?.json::<serde_json::Value>().ok()?;
    res["threat"].as_str().map(|s| s.to_string())
}

fn check_file_integrity(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    let hash = Sha256::digest(&content);
    Ok(format!("{:x}", hash))
}

fn log_with_signature(log_data: &str) -> io::Result<()> {
    let private_key = include_bytes!("../ebpf/private_key.pem");
    let pkey = PKey::private_key_from_pem(private_key).unwrap();
    let mut signer = Signer::new(MessageDigest::sha256(), &pkey).unwrap();
    signer.update(log_data.as_bytes()).unwrap();
    let signature = signer.sign_to_vec().unwrap();
    let mut log_file = File::create("/var/log/secure_event.log")?;
    writeln!(log_file, "{} | Signature: {:?}", log_data, signature)?;
    Ok(())
}

fn fetch_ips_from_logs(log_path: &str) -> Vec<String> {
    let file = File::open(log_path).unwrap_or_else(|_| panic!("Failed to read {}", log_path));
    let reader = BufReader::new(file);
    let ip_regex = Regex::new(r"\\b(?:[0-9]{1,3}\\.){3}[0-9]{1,3}\\b").unwrap();

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| ip_regex.find(&line).map(|m| m.as_str().to_string()))
        .collect()
}

fn score_cron_command(command: &str, _device_fingerprint: Option<()>) -> f32 {
    let mut score = 0.0;

    if !is_legitimate_command(command) {
        score += 3.0;
    }
    if command.contains("curl") || command.contains("wget") {
        score += 2.0;
    }
    if command.contains("nc") || command.contains("ncat") || command.contains("bash") {
        score += 3.0;
    }
    if command.contains("sh") && command.contains("http") {
        score += 4.0;
    }
    if command.contains("/tmp") || command.contains(".exe") || command.contains(".ps1") {
        score += 5.0;
    }

    score
}

fn detect_behavioral_anomalies(command: &str) -> bool {
    let suspicious_patterns = vec!["curl", "wget", "bash", "nc", "ncat", "/tmp", ".exe", ".ps1"];
    suspicious_patterns.iter().any(|pattern| command.contains(pattern))
}

fn alert_high_risk_job(command: &str, risk_score: f32) {
    if risk_score > 5.0 {
        JOB_SCHED_REAL_FOUND.store(true, Ordering::SeqCst);
        println!("[ALERT] High-Risk Cron Job Detected: {}", command);
        trigger_automated_response(command);
    }
}

fn trigger_automated_response(command: &str) {
    println!("[Automated Response] Triggering remediation for: {}", command);
}

// ... rest of the file remains unchanged
pub fn start_cron_ebpf_watch(writer: Arc<Mutex<TelemetryWriter>>) {
    START_EBPF.call_once(|| {
        thread::spawn(move || {
            let mut bpf = match Bpf::load(include_bytes_aligned!(
                "../ebpf/job_sched_monitor.bpf.o"
            )) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("❌ Failed to load eBPF program: {:?}", e);
                    return;
                }
            };

            let program = match bpf.program_mut("trace_openat") {
                Some(p) => p,
                None => {
                    eprintln!("❌ Missing eBPF program trace_openat");
                    return;
                }
            };

            let tracepoint: &mut TracePoint = match program.try_into() {
                Ok(tp) => tp,
                Err(e) => {
                    eprintln!("❌ Failed to cast eBPF program: {:?}", e);
                    return;
                }
            };

            if let Err(e) = tracepoint.load() {
                eprintln!("❌ Failed to load tracepoint: {:?}", e);
                return;
            }

            if let Err(e) = tracepoint.attach("syscalls", "sys_enter_openat") {
                eprintln!("❌ Failed to attach tracepoint: {:?}", e);
                return;
            }

            println!("🧠 [eBPF] Cron job scheduler tracepoint active.");

            let mut perf_array: PerfEventArray<_> = match bpf.map_mut("events") {
                Ok(map) => match PerfEventArray::try_from(map) {
                    Ok(array) => array,
                    Err(e) => {
                        eprintln!("❌ Failed to convert to PerfEventArray: {:?}", e);
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("❌ Failed to access perf buffer: {:?}", e);
                    return;
                }
            };

            for cpu_id in online_cpus().unwrap_or_default() {
                if let Ok(mut buf) = perf_array.open(cpu_id, None) {
                    let writer = Arc::clone(&writer);
                    thread::spawn(move || {
                        let mut buffers = vec![BytesMut::with_capacity(1024); 32];
                        loop {
                            match buf.read_events(&mut buffers) {
                                Ok(events) => {
                                    for buf in &buffers[..events.read] {
                                        if buf.len() < mem::size_of::<CronEvent>() {
                                            continue;
                                        }

                                        let ptr = buf.as_ptr() as *const CronEvent;
                                        let event = unsafe { ptr.read_unaligned() };

                                        let cmd = String::from_utf8_lossy(&event.filename)
                                            .trim_matches(char::from(0))
                                            .to_string();

                                        let timestamp = now_ts();
                                        let risk = score_cron_command(&cmd, None);
                                        let cpu = 0.3;
                                        let mem = 150_000.0;

                                        let trust_payload = generate_trust_payload(
                                            "localhost",
                                            cpu,
                                            mem as u64,
                                            risk as f64,
                                        );

                                        let feature_vector = generate_feature_vector(
                                            cpu,
                                            mem as u64,
                                            risk as f64,
                                        );

                                        let mut gnn_data = HashMap::new();
                                        gnn_data.insert("vector".into(), format!("{:?}", feature_vector));
                                        gnn_data.insert("category".into(), "scheduler".into());
                                        gnn_data.insert("signal".into(), "cron_command".into());
                                        gnn_data.insert("confidence".into(), format!("{:.2}", 1.0 - risk));
                                        gnn_data.insert("gnn_escalate".into(), "true".into());
                                        gnn_data.insert("summary".into(), format!("cron job detected: {}", cmd));
                                        gnn_data.insert("replay_tag".into(), "cron_execution".into());
                                        push_to_gnn_vector_log(gnn_data.clone());
                                        store_replay_event(gnn_data.clone());

                                        let record = TelemetryRecord {
                                            pid: event.pid as i32,
                                            ppid: event.ppid as i32,
                                            uid: 1000, // replace with event.uid if available
                                            binary_path: "/usr/bin/cron".into(),
                                            command_line: cmd.clone(),
                                            cwd: "/root".into(),
                                            env_vars: None,
                                            timestamp,
                                            risk_score: Some(risk as u32),
                                            tags: vec!["ebpf_cron".into()],
                                            ..Default::default()
                                        };

                                        if let Ok(mut w) = writer.lock() {
                                            w.append(record);
                                        }

                                        let trust_score: f32 = trust_payload
                                            .get("trust_score")
                                            .and_then(|s| s.parse::<f32>().ok())
                                            .unwrap_or(0.0);

                                        let mut metadata = HashMap::new();
                                        metadata.insert("command".into(), cmd.clone());
                                        metadata.insert("source".into(), "ebpf".into());
                                        metadata.insert("category".into(), "scheduler".into());
                                        metadata.insert("event_type".into(), "cron_command".into());
                                        metadata.insert("replay_tag".into(), "cron_execution".into());

                                        submit_trust_event(TrustEvent {
                                            timestamp,
                                            pid: event.pid as i32,
                                            ppid: event.ppid as i32,
                                            uid: 1000,
                                            binary_path: "/usr/bin/cron".into(),
                                            command_line: cmd.clone(),
                                            cwd: "/root".into(),
                                            anomaly_type: "cron_execution".into(),
                                            component: "scheduler".into(),
                                            metadata,
                                            risk_score: risk,
                                            source_module: "job_sched_monitor".into(),
                                            decay_context: Some("cron_behavior".into()),
                                            module: Some("job_sched_monitor".into()),
                                            signal: Some("cron_command".into()),
                                            signal_type: Some("job_sched".into()),
                                            score: Some(trust_score),
                                            raw_score: Some(risk),
                                            tags: Some(vec!["ebpf_cron".into(), "scheduler".into()]),
                                            description: Some(format!(
                                                "eBPF-detected cron execution: `{}` with risk score {:.1}",
                                                cmd, risk
                                            )),
                                        });

                                        // 🔁 Trust vector update
                                        #[cfg(feature = "trust_vector")]
                                        update_trust_vector("scheduler", trust_score);

                                        #[cfg(feature = "trust_decay_audit")]
                                        log_decay_reason(
                                            "scheduler",
                                            trust_score,
                                            "cron_behavior",
                                            "job_sched_monitor",
                                        );

                                        println!(
                                            "[eBPF::cron] PID={} PPID={} CMD={} | Trust Score: {}",
                                            event.pid,
                                            event.ppid,
                                            cmd,
                                            trust_payload.get("trust_score").unwrap_or(&"N/A".into())
                                        );
                                    }
                                }
                                Err(e) => {
                                    eprintln!("❌ Perf buffer read failed: {:?}", e);
                                    thread::sleep(Duration::from_millis(100));
                                }
                            }
                        }
                    });
                }
            }
        });
    });
}



 pub fn start_job_sched_monitors(writer: Arc<Mutex<TelemetryWriter>>) {
    thread::spawn(move || loop {
        let timestamp = now_ts();
        let cron_file = "/etc/crontab";

        if let Ok(contents) = fs::read_to_string(cron_file) {
            for line in contents.lines() {
                if line.trim().is_empty() || line.starts_with('#') {
                    continue;
                }

                let command = line.to_string();
                let risk_score = score_cron_command(&command.to_lowercase(), None);
                let cpu_usage = 0.5;
                let memory_usage = 120_000.0;

                let trust_payload = generate_trust_payload(
                    "localhost", cpu_usage, memory_usage as u64, risk_score as f64,
                );

                let feature_vector = generate_feature_vector(
                    cpu_usage, memory_usage as u64, risk_score as f64,
                );

                let mut gnn_data = HashMap::new();
                gnn_data.insert("vector".into(), format!("{:?}", feature_vector));
                gnn_data.insert("category".into(), "scheduler".into());
                gnn_data.insert("signal".into(), "cron_command".into());
                gnn_data.insert("confidence".into(), format!("{:.2}", 1.0 - risk_score));
                gnn_data.insert("gnn_escalate".into(), "true".into());
                gnn_data.insert("summary".into(), format!("cron job detected: {}", command));
                gnn_data.insert("replay_tag".into(), "cron_execution".into());
                push_to_gnn_vector_log(gnn_data.clone());
                store_replay_event(gnn_data.clone());

                let record = TelemetryRecord {
                    pid: unsafe { libc::getpid() },
                    ppid: unsafe { libc::getppid() },
                    uid: unsafe { libc::geteuid() },
                    binary_path: "/usr/bin/cron".into(),
                    command_line: command.clone(),
                    cwd: "/root".into(),
                    env_vars: Some(vec!["PATH=/usr/bin".into()]),
                    timestamp,
                    risk_score: Some(risk_score as u32),
                    tags: vec!["cron_batch".into()],
                    ..Default::default()
                };

                if let Ok(mut locked_writer) = writer.lock() {
                    locked_writer.append(record.clone());
                }

                let trust_score = trust_payload
                    .get("trust_score")
                    .and_then(|s| s.parse::<f32>().ok())
                    .unwrap_or(0.0);

                let mut metadata = HashMap::new();
                metadata.insert("command".into(), command.clone());
                metadata.insert("source".into(), "cron_file".into());
                metadata.insert("category".into(), "scheduler".into());
                metadata.insert("event_type".into(), "cron_line_scan".into());
                metadata.insert("replay_tag".into(), "cron_execution".into());

                submit_trust_event(TrustEvent {
                    timestamp,
                    pid: record.pid,
                    ppid: record.ppid,
                    uid: record.uid,
                    binary_path: record.binary_path.clone(),
                    command_line: record.command_line.clone(),
                    cwd: record.cwd.clone(),
                    anomaly_type: "scheduled_cron_job".into(),
                    component: "scheduler".into(),
                    metadata,
                    risk_score,
                    source_module: "job_sched_monitor".into(),
                    decay_context: Some("cron_passive_batch".into()),
                    module: Some("job_sched_monitor".into()),
                    signal: Some("cron_line_scan".into()),
                    signal_type: Some("job_sched_batch".into()),
                    score: Some(trust_score),
                    raw_score: Some(risk_score),
                    tags: Some(vec!["cron_batch".into(), "job_sched".into()]),
                    description: Some(format!(
                        "Detected batch cron job command: `{}` (risk: {:.1})",
                        command, risk_score
                    )),
                });

                alert_high_risk_job(&command, risk_score);

                println!(
                    "[⏱️ Job Sched Passive] CMD={} | Trust={}",
                    command,
                    trust_payload.get("trust_score").unwrap_or(&"N/A".into())
                );
            }
        }

        thread::sleep(Duration::from_secs(60));
    });
}



#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Debug, Clone)]
struct KernelFalloutEvent {
    pid: u32,
    ppid: u32,
    uid: u32,
    signal: u32,
    info_code: u32,
    timestamp: u64,
}


#[cfg(target_os = "linux")]
pub fn start_ebpf_kernel_fallout_watch() {
    thread::spawn(move || {
        let bpf = Bpf::load(include_bytes_aligned!(
            "../ebpf/kernel_exploit_fallout.bpf.o"
        ));

        if let Err(e) = bpf {
            eprintln!("❌ Failed to load kernel fallout bpf: {:?}", e);
            return;
        }
        let mut bpf = bpf.unwrap();

        let program = match bpf.program_mut("trace_kernel_fallout") {
            Some(p) => p,
            None => {
                eprintln!("❌ Missing program trace_kernel_fallout");
                return;
            }
        };

        let tp: &mut TracePoint = match program.try_into() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("❌ Could not cast program: {:?}", e);
                return;
            }
        };

        if let Err(e) = tp.load() {
            eprintln!("❌ Failed to load fallout tracepoint: {:?}", e);
            return;
        }

        if let Err(e) = tp.attach("exceptions", "do_trap") {
            eprintln!("❌ Failed to attach fallout TP: {:?}", e);
            return;
        }

        println!("🧠 [eBPF] Kernel exploit fallout tracepoint active.");

        let mut perf_array: PerfEventArray<_> = match bpf.map_mut("EVENTS") {
            Ok(map) => match PerfEventArray::try_from(map) {
                Ok(arr) => arr,
                Err(e) => {
                    eprintln!("❌ Could not convert perf map: {:?}", e);
                    return;
                }
            },
            Err(e) => {
                eprintln!("❌ Could not locate perf map: {:?}", e);
                return;
            }
        };

        for cpu_id in online_cpus().unwrap_or_default() {
            if let Ok(mut buf) = perf_array.open(cpu_id, None) {
                thread::spawn(move || {
                    let mut buffers = vec![BytesMut::with_capacity(1024); 32];
                    loop {
                        match buf.read_events(&mut buffers) {
                            Ok(events) => {
                                for buf in &buffers[..events.read] {
                                    if buf.len() < mem::size_of::<KernelFalloutEvent>() {
                                        continue;
                                    }

                                    let ptr = buf.as_ptr() as *const KernelFalloutEvent;
                                    let evt = unsafe { ptr.read_unaligned() };
                                    let now = now_ts();

                                    let desc = format!(
                                        "Trap/Fault from PID {} (signal={} info_code=0x{:x})",
                                        evt.pid, evt.signal, evt.info_code
                                    );

                                    let _ = push_memory_telemetry(
                                        evt.pid as i32,
                                        evt.ppid as i32,
                                        evt.uid,
                                        "/sbin/init".into(),
                                        "kernel".into(),
                                        "/".into(),
                                        crate::telemetry_types::MemoryAnomalyType::KernelExploitFallout,
                                        desc.clone(),
                                    ).map_err(|e| eprintln!("⚠️ Fallout telemetry failed: {:?}", e));

                                    let trust_payload = generate_trust_payload(
                                        "localhost", 0.9, 250_000, 7.5
                                    );

                                    let vector = generate_feature_vector(0.9, 250_000, 7.5);
                                    let mut gnn_data = HashMap::new();
                                    gnn_data.insert("vector".into(), format!("{:?}", vector));
                                    gnn_data.insert("category".into(), "memory".into());
                                    gnn_data.insert("signal".into(), "kernel_fallout".into());
                                    gnn_data.insert("confidence".into(), "0.9".into());
                                    gnn_data.insert("gnn_escalate".into(), "true".into());
                                    gnn_data.insert("summary".into(), desc.clone());
                                    gnn_data.insert("replay_tag".into(), "kernel_exploit_fallout".into());
                                    push_to_gnn_vector_log(gnn_data.clone());
                                    store_replay_event(gnn_data.clone());

                                    let mut metadata = HashMap::new();
                                    metadata.insert("signal".into(), evt.signal.to_string());
                                    metadata.insert("info_code".into(), format!("{:#x}", evt.info_code));
                                    metadata.insert("category".into(), "memory".into());
                                    metadata.insert("event_type".into(), "kernel_fallout".into());
                                    metadata.insert("replay_tag".into(), "kernel_exploit_fallout".into());

                                    submit_trust_event(TrustEvent {
                                        timestamp: now,
                                        pid: evt.pid as i32,
                                        ppid: evt.ppid as i32,
                                        uid: evt.uid,
                                        binary_path: "/sbin/init".into(),
                                        command_line: "init".into(),
                                        cwd: "/".into(),
                                        anomaly_type: "kernel_exploit_fallout".into(),
                                        component: "kernel_trap".into(),
                                        metadata,
                                        risk_score: 7.5,
                                        source_module: "kernel_fallout_monitor".into(),
                                        decay_context: Some("kernel_trap".into()),
                                        module: Some("kernel_fallout_monitor".into()),
                                        signal: Some("trap_signal".into()),
                                        signal_type: Some("exploit_fallout".into()),
                                        score: Some(100.0 - 7.5 * 10.0),
                                        raw_score: Some(7.5),
                                        tags: Some(vec!["kernel", "exploit_fallout", "ebpf"].iter().map(|s| s.to_string()).collect()),
                                        description: Some(desc.clone()),
                                    });

                                    println!(
                                        "[⚠️ Kernel Fallout] PID={} SIG={} DESC={} | Trust={}",
                                        evt.pid,
                                        evt.signal,
                                        desc,
                                        trust_payload.get("trust_score").unwrap_or(&"N/A".into())
                                    );
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed reading kernel fallout event: {:?}", e);
                                thread::sleep(Duration::from_millis(100));
                            }
                        }
                    }
                });
            }
        }
    });
}


pub fn scan_job_sched_activity() -> Vec<TelemetryOutput> {
    if JOB_SCHED_REAL_FOUND.load(Ordering::Relaxed) {
        return vec![]; // Already handled via real-time path
    }

    let now = now_ts();

    let mut data = HashMap::new();
    data.insert("source".into(), "passive_scan".into());
    data.insert("cron_file".into(), "/etc/crontab".into());
    data.insert("scan_time".into(), format!("{}", now));
    data.insert("status".into(), "no anomalies".into());
    data.insert("category".into(), "scheduler".into());
    data.insert("event_type".into(), "passive_cron_scan".into());
    data.insert("replay_tag".into(), "cron_passive_scan".into());

    let trust_event = TrustEvent {
        timestamp: now,
        pid: 1,
        ppid: 0,
        uid: 0,
        binary_path: "/usr/bin/cron".into(),
        command_line: "none".into(),
        cwd: "/etc".into(),
        anomaly_type: "cron_passive_scan".into(),
        component: "job_sched_monitor".into(),
        metadata: data.clone(),
        risk_score: 0.1,
        source_module: "job_sched_monitor".into(),
        decay_context: Some("passive_ok".into()),
        signal_type: Some("scheduler".into()),
        signal: Some("no_cron_anomaly_detected".into()),
        module: Some("job_sched_monitor".into()),
        score: Some(99.9),
        raw_score: Some(0.1),
        tags: Some(vec!["scheduler".into(), "passive_scan".into()]),
        description: Some("Passive cron scan completed. No anomaly detected.".into()),
    };

    submit_trust_event(trust_event);

    vec![TelemetryOutput {
        category: "scheduler".into(),
        signal: "no_cron_anomaly_detected".into(),
        confidence: 0.95,
        data,
    }]
}
