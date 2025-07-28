use crate::forensic::file_watcher::{start_file_watch, FileEvent};
use crate::modules::process_monitor::gather_processes;
use crate::modules::user_tracker::get_logged_in_users;
use crate::forensic::network_monitor::scan_connections;
use crate::modules::auth_monitor::{check_password_spray, check_mfa_bypass, check_geo_ip_anomaly};
use crate::forensic::relay::transmit_or_buffer;
use crate::modules::mem_scan::{scan_memory, MemorySnapshot};
use crate::modules::usb_monitor::{scan_usb_devices, USBEvent, poll_usb_devices, detect_usb_events};
use crate::modules::script_monitor::{detect_script_activity, scan_running_scripts, ScriptEvent};
use crate::modules::privilege_monitor::{scan_privilege_escalations, PrivEscalationEvent, PrivilegeEvent};
use crate::modules::persistence_watch::{scan_persistence_artifacts, scan_persistence_mechanisms, check_persistence_indicators, PersistenceEntry};
use crate::modules::cloud_tracker::{scan_cloud_activity, get_cloud_sessions, CloudActivity};
use crate::modules::container_monitor::{scan_containers, track_container_activity, ContainerActivity};
use crate::modules::logon_tracker::{collect_logon_events, LogonEvent};
use crate::modules::job_sched_monitor::{scan_schedulers, SchedulerEntry};
use crate::forensic::cloud_watch::monitor_cloud_config;
use crate::forensic::container_engine;
use crate::forensic::persistence_engine::hunt_persistence_methods;
use crate::forensic::privilege_engine::check_privilege_escalation;
use crate::forensic::memory_monitor::monitor_memory_usage;
use hostname::get as get_hostname;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::trust::trust_hook::generate_trust_payload;
use crate::gnn_hook::generate_gnn_features;
use crate::replay::replay_writer::store_replay_event;

pub fn start_forensic_engine() {
    println!("[*] Starting forensic telemetry engine...");
    thread::spawn(start_insider_data_staging);
    thread::spawn(start_trust_decay_profiler);
    thread::spawn(start_time_aware_trust_decay);
    thread::spawn(start_multi_stage_exfil_detector);
    thread::spawn(start_post_exploit_anomaly_flagger);
    thread::spawn(start_zero_day_exploit_fallout);
    thread::spawn(start_zero_day_fallout_monitor);
    thread::spawn(start_encrypted_payload_detector);
    thread::spawn(start_file_hash_watcher);

    thread::spawn(start_auth_watch);
    thread::spawn(start_net_watch);
    thread::spawn(start_process_monitor);
    thread::spawn(start_user_tracker);
    thread::spawn(start_file_watcher);

    println!("[+] All forensic modules running.");
}

pub fn run_forensics_cycle(pid: u32) {
    let memory_events = monitor_memory_usage(pid);
    for evt in memory_events {
        println!("[FOR] {:?}", evt);
        let trust_payload = generate_trust_payload(&evt);
        let gnn_features = generate_gnn_features(&evt);
        let _ = store_replay_event(evt.clone());
        let _ = transmit_or_buffer("local", evt.timestamp as u64, vec![evt]);
        let _ = transmit_or_buffer("local", trust_payload.timestamp, vec![trust_payload]);
        let _ = transmit_or_buffer("local", gnn_features.timestamp, vec![gnn_features]);
    }
}

       spawn_loop!("Process Monitor", {
           let processes = gather_processes();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, processes) {
               eprintln!("⚠️ Process snapshot: {}", e);
           }
       }, 15);
  
       spawn_loop!("User Tracker", {
           let users = get_logged_in_users();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, users) {
               eprintln!("⚠️ User sessions: {}", e);
           }
       }, 30);
  
       spawn_loop!("Network Connections", {
           let conns = scan_connections();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, conns) {
               eprintln!("⚠️ Net connections: {}", e);
           }
       }, 20);
  
       spawn_loop!("Memory Scanner", {
           let mem_data: Vec<MemorySnapshot> = scan_memory();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, mem_data) {
               eprintln!("⚠️ Memory snapshot: {}", e);
           }
       }, 30);
  
       spawn_loop!("Persistence Artifact Scanner", {
           let entries: Vec<PersistenceEntry> = scan_persistence_artifacts();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, entries) {
               eprintln!("⚠️ Persistence artifacts: {}", e);
           }
       }, 60);
  
       // File Watcher — event-driven
       thread::spawn({
           let hostname = hostname.clone();
           let rx = start_file_watch(vec![PathBuf::from("/tmp"), PathBuf::from("/etc")]);
           move || {
               for event in rx {
                   if let Err(e) = transmit_or_buffer(&hostname, event.timestamp, vec![event]) {
                       eprintln!("⚠️ File event: {}", e);
                   }
               }
           }
       });
  
       spawn_loop!("Logon Tracker", {
           let logons: Vec<LogonEvent> = collect_logon_events();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, logons) {
               eprintln!("⚠️ Logon events: {}", e);
           }
       }, 45);
  
       spawn_loop!("Scheduler Monitor", {
           let jobs: Vec<SchedulerEntry> = scan_schedulers();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, jobs) {
               eprintln!("⚠️ Scheduler jobs: {}", e);
           }
       }, 60);
  
       spawn_loop!("Script Runner", {
           let script_events: Vec<ScriptEvent> = scan_running_scripts();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, script_events) {
               eprintln!("⚠️ Script run: {}", e);
           }
       }, 30);
  
       spawn_loop!("Privilege Escalation (PrivilegeEvent)", {
           let events: Vec<PrivilegeEvent> = scan_privilege_escalations();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, events) {
               eprintln!("⚠️ Privilege escalation (1): {}", e);
           }
       }, 60);
  
       spawn_loop!("Persistence Mechanism Watch", {
           let entries: Vec<PersistenceEntry> = scan_persistence_mechanisms();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, entries) {
               eprintln!("⚠️ Persistence mechanisms: {}", e);
           }
       }, 120);
  
       spawn_loop!("Cloud Activity", {
           let cloud_events: Vec<CloudActivity> = scan_cloud_activity();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, cloud_events) {
               eprintln!("⚠️ Cloud activity: {}", e);
           }
       }, 45);
  
       spawn_loop!("Container Monitor", {
           let container_events: Vec<ContainerActivity> = scan_containers();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, container_events) {
               eprintln!("⚠️ Container activity: {}", e);
           }
       }, 45);
  
       spawn_loop!("USB Monitor 1", {
           let usb_events: Vec<USBEvent> = scan_usb_devices();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, usb_events) {
               eprintln!("⚠️ USB events (1): {}", e);
           }
       }, 30);
  
       spawn_loop!("USB Monitor 2", {
           let usb_events: Vec<UsbEvent> = poll_usb_devices();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, usb_events) {
               eprintln!("⚠️ USB events (2): {}", e);
           }
       }, 30);
  
       spawn_loop!("Script Activity Detector", {
           let script_events: Vec<ScriptEvent> = detect_script_activity();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, script_events) {
               eprintln!("⚠️ Script detection: {}", e);
           }
       }, 30);
  
       spawn_loop!("Password Spray", {
           if let Some(alert) = check_password_spray() {
               if let Err(e) = transmit_or_buffer(&hostname, timestamp, vec![alert]) {
                   eprintln!("⚠️ Password spray alert: {}", e);
               }
           }
       }, 60);
  
       spawn_loop!("Privilege Escalation (PrivEscalationEvent)", {
           let events: Vec<PrivEscalationEvent> = scan_privilege_escalations();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, events) {
               eprintln!("⚠️ Priv escalation (2): {}", e);
           }
       }, 30);
  
       spawn_loop!("MFA Bypass", {
           if let Some(alert) = check_mfa_bypass() {
               if let Err(e) = transmit_or_buffer(&hostname, timestamp, vec![alert]) {
                   eprintln!("⚠️ MFA bypass alert: {}", e);
               }
           }
       }, 60);
  
       spawn_loop!("USB Detector", {
           let usb_events = detect_usb_events();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, usb_events) {
               eprintln!("⚠️ USB detect events: {}", e);
           }
       }, 30);
  
       spawn_loop!("Persistence Watch", {
           let items = check_persistence_indicators();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, items) {
               eprintln!("⚠️ Persistence indicators: {}", e);
           }
       }, 60);
  
       spawn_loop!("Cloud Config", {
           let events = monitor_cloud_config();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, events) {
               eprintln!("⚠️ Cloud config: {}", e);
           }
       }, 60);
  
       spawn_loop!("Cloud Sessions", {
           let sessions = get_cloud_sessions();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, sessions) {
               eprintln!("⚠️ Cloud sessions: {}", e);
           }
       }, 45);
  
       spawn_loop!("Container Engine", {
           let containers = container_engine::scan_containers();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, containers) {
               eprintln!("⚠️ Container engine: {}", e);
           }
       }, 60);
  
       spawn_loop!("Container Tracker", {
           let activity = track_container_activity();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, activity) {
               eprintln!("⚠️ Container tracking: {}", e);
           }
       }, 60);
  
       spawn_loop!("Persistence Engine", {
           let results = hunt_persistence_methods();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, results) {
               eprintln!("⚠️ Persistence hunting: {}", e);
           }
       }, 90);
  
       spawn_loop!("Privilege Engine", {
           let elevation_attempts = check_privilege_escalation();
           if let Err(e) = transmit_or_buffer(&hostname, timestamp, elevation_attempts) {
               eprintln!("⚠️ Privilege escalation attempts: {}", e);
           }
       }, 45);
  
       spawn_loop!("Geo/IP Anomaly", {
           if let Some(alert) = check_geo_ip_anomaly() {
               if let Err(e) = transmit_or_buffer(&hostname, timestamp, vec![alert]) {
                   eprintln!("⚠️ Geo/IP anomaly: {}", e);
               }
           }
       }, 90);
  
       println!("🧠 Forensic engine is now running...");
    

