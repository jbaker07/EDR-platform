use aya::programs::TracePoint;
use aya::{Bpf, include_bytes_aligned};
use std::collections::{HashSet, HashMap};
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::sync::{Once, OnceLock, atomic::{AtomicBool, Ordering}, Mutex};

use crate::trust_hook::{submit_trust_event, TrustEvent, generate_feature_vector};
use crate::utils::time::now_ts;
use crate::telemetry_types::TelemetryOutput;
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::telemetry_writer::write_telemetry_record;
use crate::logger::log;
use anyhow::{Context, Result};

/// Tracks devices seen in the previous polling cycle (thread-safe; replaces `static mut`).
static PREV_DEVICES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

/// Ensures the eBPF attach happens only once.
static INIT: Once = Once::new();

/// Feature flag to enable/disable the monitor (kept for consistency with other modules).
pub static SCAN_USB_MONITOR: OnceLock<AtomicBool> = OnceLock::new();

/// Start the USB monitor:
/// 1) Polls `lsblk -o NAME,TRAN` periodically and emits a TrustEvent on new USB devices
/// 2) Attempts to attach an eBPF tracepoint for additional coverage (Linux only)
pub fn start_usb_monitor() {
    SCAN_USB_MONITOR.get_or_init(|| AtomicBool::new(true));
    PREV_DEVICES.get_or_init(|| Mutex::new(HashSet::new()));

    // Userland polling loop
    thread::spawn(|| loop {
        if !SCAN_USB_MONITOR.get().unwrap().load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(10));
            continue;
        }

        match fetch_usb_devices() {
            Ok(current) => {
                let mut guard = PREV_DEVICES.get().unwrap().lock().expect("poisoned PREV_DEVICES");
                // Compute newly seen devices
                for dev in current.difference(&*guard) {
                    emit_usb_inserted_event(dev);
                }
                // Update snapshot
                *guard = current;
            }
            Err(e) => {
                eprintln!("⚠️ usb_monitor: failed to fetch devices: {e:?}");
            }
        }

        thread::sleep(Duration::from_secs(50));
    });

    // eBPF attach (Linux only)
    INIT.call_once(|| {
        #[cfg(target_os = "linux")]
        {
            if let Err(e) = attach_ebpf_usb_monitor() {
                eprintln!("❌ Failed to attach eBPF USB monitor: {}", e);
            } else {
                println!("✅ eBPF USB monitor attached");
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            log("ℹ️ eBPF USB monitor not supported on this OS; running polling-only mode.");
        }
    });
}

/// Parse current USB devices via `lsblk -o NAME,TRAN`.
/// Returns a set of normalized "name,tran" lines that include "usb" transport.
/// Robust to headers and odd spacing.
fn fetch_usb_devices() -> Result<HashSet<String>> {
    let output = Command::new("lsblk").arg("-o").arg("NAME,TRAN").output()
        .context("failed to execute lsblk")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut current = HashSet::new();

    for line in stdout.lines() {
        // skip header if present
        let lower = line.to_lowercase();
        if lower.contains("name") && lower.contains("tran") {
            continue;
        }
        // we consider a line USB if it contains "usb" anywhere in TRAN/line
        if lower.contains("usb") {
            // normalize whitespace
            let norm = line.split_whitespace().collect::<Vec<_>>().join(" ");
            if !norm.is_empty() {
                current.insert(norm);
            }
        }
    }

    Ok(current)
}

/// Emit telemetry + trust for a newly observed USB device string (one line from lsblk).
fn emit_usb_inserted_event(device_line: &str) {
    let ts = now_ts();
    let cpu = 0.2_f64;
    let mem = 1500_u64;
    let risk = 8.0_f64;

    let description = format!("New USB device: {}", device_line);
    let features = generate_feature_vector(cpu, mem, risk);

    let mut data = HashMap::new();
    data.insert("host".into(), "macos-host".into());
    data.insert("timestamp".into(), ts.to_string());
    data.insert("device_info".into(), device_line.to_string());
    data.insert("replay_tag".into(), "usb_inserted".into());
    data.insert("soc_note".into(), "USB device detected in real-time".into());
    data.insert("features".into(), format!("{:?}", features));
    data.insert("cpu".into(), format!("{:.2}", cpu));
    data.insert("mem_kb".into(), format!("{}", mem));
    data.insert("gnn_escalate".into(), "false".into());

    // Trust event
    submit_trust_event(TrustEvent {
        timestamp: ts,
        pid: 0,
        ppid: 0,
        uid: 0,
        binary_path: "unknown".into(),
        command_line: "usb_monitor".into(),
        cwd: "/".into(),
        anomaly_type: "usb_inserted".into(),
        component: "usb_monitor".into(),
        description: Some(description.clone()),
        metadata: data.clone(),
        score: Some(risk as f32),
        risk_score: risk as f32,
        raw_score: Some(risk as f32),
        tags: Some(vec!["usb".into(), "inserted".into()]),
        decay_context: Some("usb_activity".into()),
        source_module: "usb_monitor".into(),
        module: Some("usb_monitor".into()),
        signal: Some("usb_inserted".into()),
        signal_type: Some("usb".into()),
    });

    // GNN + Telemetry record (best-effort)
    push_to_gnn_vector_log(data.clone());
    let _ = write_telemetry_record(data.clone());

    log(&format!("[🔌 USB Monitor] New USB device inserted: {}", device_line));
}

#[cfg(target_os = "linux")]
/// Attach precompiled eBPF usb_monitor_ebpf.o probe
pub fn attach_ebpf_usb_monitor() -> Result<()> {
    // Load the precompiled eBPF object file
    let mut bpf = Bpf::load(include_bytes_aligned!("../ebpf/usb_monitor_ebpf.o"))
        .context("❌ Failed to load usb_monitor_ebpf.o")?;

    // Locate and convert the program to a TracePoint
    let program = bpf
        .program_mut("trace_usb_open")
        .context("❌ Missing trace_usb_open program in BPF object")?;

    let trace_point: &mut TracePoint = program
        .try_into()
        .context("❌ Failed to convert to TracePoint program")?;

    // Load and attach the tracepoint to the syscall
    trace_point
        .load()
        .context("❌ Failed to load trace_usb_open TracePoint")?;

    trace_point
        .attach("syscalls", "sys_enter_openat")
        .context("❌ Failed to attach trace_usb_open to sys_enter_openat")?;

    println!("✅ [eBPF] USB monitor attached via tracepoint sys_enter_openat");
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn attach_ebpf_usb_monitor() -> Result<()> {
    // No-op on non-Linux platforms
    Ok(())
}

/// Passive fallback scan for USB devices using `lsblk`.
/// This reuses the same parsing logic as the real-time loop.
pub fn scan_usb_state() -> Vec<TelemetryOutput> {
    let mut results = Vec::new();

    match fetch_usb_devices() {
        Ok(devs) => {
            for dev in devs {
                let ts = now_ts();

                let features = generate_feature_vector(0.3, 5000, 3.0);
                let mut data = HashMap::new();
                data.insert("device_info".into(), dev.clone());
                data.insert("timestamp".into(), ts.to_string());
                data.insert("replay_tag".into(), "usb_inserted".into());
                data.insert("soc_note".into(), "USB device seen during passive scan".into());
                data.insert("source".into(), "scan_usb_state".into());
                data.insert("features".into(), format!("{:?}", features));

                // Push to GNN
                push_to_gnn_vector_log(data.clone());

                // Submit TrustEvent
                submit_trust_event(TrustEvent {
                    timestamp: ts,
                    pid: 0,
                    ppid: 0,
                    uid: 0,
                    binary_path: "/dev/null".into(),
                    command_line: "scan_usb_state".into(),
                    cwd: "/".into(),
                    anomaly_type: "usb_device_seen".into(),
                    component: "usb_scanner".into(),
                    metadata: data.clone(),
                    risk_score: 3.0,
                    source_module: "usb_scanner".into(),
                    decay_context: Some("usb_scan".into()),
                    module: Some("usb_scanner".into()),
                    signal: Some("usb_device_seen".into()),
                    signal_type: Some("usb".into()),
                    score: Some(3.0),
                    raw_score: Some(3.0),
                    tags: Some(vec!["usb".into(), "passive".into()]),
                    description: Some("USB device detected during passive scan.".into()),
                });

                // Write to telemetry record
                let _ = write_telemetry_record(data.clone());

                results.push(TelemetryOutput {
                    category: "usb".into(),
                    signal: "usb_device_seen".into(),
                    confidence: 0.4,
                    data,
                });
            }
        }
        Err(e) => {
            eprintln!("❌ scan_usb_state: {}", e);
        }
    }

    results
}
