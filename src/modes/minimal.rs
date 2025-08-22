use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use hostname::get;

use crate::modules::{
    process_monitor::start_process_monitor,
    net_watch::log_open_connections,
    auth_monitor::light_auth_monitor,
    password_spray::detect_password_spray,
    mfa_bypass::detect_mfa_bypass,
    geo_ip_anomaly::check_geo_ip_anomaly,
    cloud_tracker::minimal_cloud_tracking,
};

use crate::relay::transmit_or_buffer;
use crate::modes::forensic::start_forensic_mode;

fn current_ts() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn start_minimal_mode() {
    let hostname = get().unwrap_or_default().to_string_lossy().to_string();
    let triggered = Arc::new(AtomicBool::new(false));

    let modules: Vec<Box<dyn Fn() + Send + Sync>> = vec![
        // Process Monitor
        Box::new({
            let triggered = Arc::clone(&triggered);
            let hostname = hostname.clone();
            move || loop {
                let ts = current_ts();
                let data = start_process_monitor();
                transmit_or_buffer(&hostname, ts, data.clone()).ok();

                if data.iter().any(|s| s.contains("suspicious_process_name")) {
                    if !triggered.swap(true, Ordering::SeqCst) {
                        start_forensic_mode();
                    }
                }

                thread::sleep(Duration::from_secs(30));
            }
        }),

        // Network
        Box::new({
            let triggered = Arc::clone(&triggered);
            let hostname = hostname.clone();
            move || loop {
                let ts = current_ts();
                let data = log_open_connections();
                transmit_or_buffer(&hostname, ts, data.clone()).ok();

                if data.iter().any(|s| s.contains("suspicious_network_activity")) {
                    if !triggered.swap(true, Ordering::SeqCst) {
                        start_forensic_mode();
                    }
                }

                thread::sleep(Duration::from_secs(45));
            }
        }),

        // Auth Monitor
        Box::new({
            let triggered = Arc::clone(&triggered);
            let hostname = hostname.clone();
            move || loop {
                let ts = current_ts();
                let data = light_auth_monitor();
                transmit_or_buffer(&hostname, ts, data.clone()).ok();

                if data.iter().any(|s| s.contains("suspicious_authentication")) {
                    if !triggered.swap(true, Ordering::SeqCst) {
                        start_forensic_mode();
                    }
                }

                thread::sleep(Duration::from_secs(60));
            }
        }),

        // Password Spray
        Box::new({
            let triggered = Arc::clone(&triggered);
            let hostname = hostname.clone();
            move || loop {
                let ts = current_ts();
                if let Some(alert) = detect_password_spray() {
                    transmit_or_buffer(&hostname, ts, vec![alert.clone()]).ok();

                    if alert.contains("password_spray_alert") {
                        if !triggered.swap(true, Ordering::SeqCst) {
                            start_forensic_mode();
                        }
                    }
                }

                thread::sleep(Duration::from_secs(60));
            }
        }),

        // MFA Bypass
        Box::new({
            let triggered = Arc::clone(&triggered);
            let hostname = hostname.clone();
            move || loop {
                let ts = current_ts();
                if let Some(alert) = detect_mfa_bypass() {
                    transmit_or_buffer(&hostname, ts, vec![alert.clone()]).ok();

                    if alert.contains("mfa_bypass_alert") {
                        if !triggered.swap(true, Ordering::SeqCst) {
                            start_forensic_mode();
                        }
                    }
                }

                thread::sleep(Duration::from_secs(90));
            }
        }),

        // Geo-IP Anomaly
        Box::new({
            let triggered = Arc::clone(&triggered);
            let hostname = hostname.clone();
            move || loop {
                let ts = current_ts();
                if let Some(alert) = check_geo_ip_anomaly() {
                    transmit_or_buffer(&hostname, ts, vec![alert.clone()]).ok();

                    if alert.contains("geo_ip_alert") {
                        if !triggered.swap(true, Ordering::SeqCst) {
                            start_forensic_mode();
                        }
                    }
                }

                thread::sleep(Duration::from_secs(120));
            }
        }),

        // Cloud Tracker
        Box::new({
            let triggered = Arc::clone(&triggered);
            let hostname = hostname.clone();
            move || loop {
                let ts = current_ts();
                let data = minimal_cloud_tracking();
                transmit_or_buffer(&hostname, ts, data.clone()).ok();

                if data.iter().any(|s| s.contains("suspicious_cloud_activity")) {
                    if !triggered.swap(true, Ordering::SeqCst) {
                        start_forensic_mode();
                    }
                }

                thread::sleep(Duration::from_secs(120));
            }
        }),
    ];

    for module in modules {
        thread::spawn(module);
    }

    println!("⚡ Minimal mode engine initialized.");
}
