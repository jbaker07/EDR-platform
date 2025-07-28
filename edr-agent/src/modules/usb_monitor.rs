use aya::programs::TracePoint;
use aya::{Bpf, include_bytes_aligned};
use std::collections::{HashSet, HashMap};
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::sync::{Once, OnceLock, atomic::{AtomicBool, Ordering}};

use crate::trust_hook::{submit_trust_event, TrustEvent, generate_feature_vector};
use crate::utils::time::now_ts;
use crate::telemetry_types::TelemetryOutput;
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::telemetry_writer::write_telemetry_record;
use crate::logger::log;
use anyhow::{Context, Result};

static mut PREVIOUS_DEVICES: Option<HashSet<String>> = None;
static INIT: std::sync::Once = std::sync::Once::new();

pub static SCAN_USB_MONITOR: OnceLock<AtomicBool> = OnceLock::new();
pub fn start_usb_monitor() {
    SCAN_USB_MONITOR.get_or_init(|| AtomicBool::new(true));

    thread::spawn(|| loop {
        let output = Command::new("lsblk")
            .arg("-o")
            .arg("NAME,TRAN")
            .output();

        if let Ok(out) = output {
            if let Ok(stdout) = String::from_utf8(out.stdout) {
                let mut current_devices = HashSet::new();

                for line in stdout.lines() {
                    if line.contains("usb") {
                        current_devices.insert(line.to_string());

                        let is_new = unsafe {
                            match &PREVIOUS_DEVICES {
                                Some(prev) => !prev.contains(line),
                                None => true,
                            }
                        };

                        if is_new {
                            let ts = now_ts();
                            let cpu = 0.2;
                            let mem = 1500;
                            let risk = 8.0;

                            let description = format!("New USB device: {}", line.trim());
                            let features = generate_feature_vector(cpu, mem, risk);

                            let mut data = HashMap::new();
                            data.insert("host".into(), "macos-host".into());
                            data.insert("timestamp".into(), ts.to_string());
                            data.insert("device_info".into(), line.trim().into());
                            data.insert("replay_tag".into(), "usb_inserted".into());
                            data.insert("soc_note".into(), "USB device detected in real-time".into());
                            data.insert("features".into(), format!("{:?}", features));
                            data.insert("cpu".into(), format!("{:.2}", cpu));
                            data.insert("mem_kb".into(), format!("{}", mem));
                            data.insert("gnn_escalate".into(), "false".into());

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

                            push_to_gnn_vector_log(data.clone());

                            let _ = write_telemetry_record(data.clone());

                            log(&format!(
                                "[🔌 USB Monitor] New USB device inserted: {}",
                                line.trim()
                            ));
                        }
                    }
                }

                unsafe {
                    PREVIOUS_DEVICES = Some(current_devices);
                }
            }
        }

        thread::sleep(Duration::from_secs(50));
    });

    INIT.call_once(|| {
        if let Err(e) = attach_ebpf_usb_monitor() {
            eprintln!("❌ Failed to attach eBPF USB monitor: {}", e);
        } else {
            println!("✅ eBPF USB monitor attached");
        }
    });
}


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

/// Passive fallback scan for USB devices using `lsblk`
pub fn scan_usb_state() -> Vec<TelemetryOutput> {
    let mut results = Vec::new();

    // Run lsblk to get device transport types
    let output = Command::new("lsblk")
        .arg("-o")
        .arg("NAME,TRAN")
        .output();

    if let Ok(out) = output {
        if let Ok(stdout) = String::from_utf8(out.stdout) {
            for line in stdout.lines() {
                if line.to_lowercase().contains("usb") {
                    let ts = now_ts();
                    let mut data = HashMap::new();
                    data.insert("device_info".into(), line.trim().to_string());
                    data.insert("timestamp".into(), ts.to_string());
                    data.insert("replay_tag".into(), "usb_inserted".into());
                    data.insert("soc_note".into(), "USB device seen during passive scan".into());
                    data.insert("source".into(), "scan_usb_state".into());

                    let features = generate_feature_vector(0.3, 5000, 3.0);
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
        } else {
            eprintln!("⚠️ Failed to decode lsblk output");
        }
    } else {
        eprintln!("❌ Failed to run lsblk for passive USB scan");
    }

    results
}

