use crate::trust_hook::{build_trust_payload, submit_trust_signal};
use crate::telemetry::{get_loaded_modules, get_kernel_anomalies, check_unlinked_binaries};
use std::thread;
use std::time::Duration;

/// Arbitrary kernel PID to check module load status — in production, consider fetching this dynamically.
const KERNEL_PID: i32 = 0;

/// Launches the zero-day fallout detection loop every 5 minutes.
pub fn start_zero_day_fallout_monitor() {
    thread::spawn(|| loop {
        detect_kernel_module_anomalies();
        detect_unlinked_binary_exec();
        detect_syscall_surface_shift();

        thread::sleep(Duration::from_secs(300)); // ⏱️ Every 5 minutes
    });
}

/// Detects unsigned or recently loaded kernel modules (possible malicious load).
fn detect_kernel_module_anomalies() {
    if let Some(modules) = get_loaded_modules(KERNEL_PID) {
        for module in modules {
            let mut risk = 0;

            if module.is_unsigned {
                risk += 40;
            }
            if module.loaded_duration_secs < 60 {
                risk += 35;
            }
            if module.name.contains("kpatch") || module.name.contains("mod_") {
                risk += 10;
            }

            if risk >= 60 {
                let payload = build_trust_payload(
                    "zero_day_fallout:unsigned_or_rush_loaded_kernel_module",
                    risk,
                );
                submit_trust_signal(payload);
            }
        }
    }
}

/// Detects execution of binaries in stealth paths (/tmp, /dev/shm), suggesting fileless or evasive behavior.
fn detect_unlinked_binary_exec() {
    if let Ok(binaries) = check_unlinked_binaries() {
        for bin in binaries {
            let path = bin.path.to_lowercase();
            if path.contains("/tmp/") || path.contains("/dev/shm/") || path.contains("/run/") {
                let payload = build_trust_payload(
                    "zero_day_fallout:unlinked_exec_in_stealth_path",
                    75,
                );
                submit_trust_signal(payload);
            }
        }
    }
}

/// Detects signs of hooked or inline-patched syscall tables (indicative of rootkits or exploit kits).
fn detect_syscall_surface_shift() {
    if let Ok(anomalies) = get_kernel_anomalies() {
        for anomaly in anomalies {
            let risk_score = match anomaly.kind.as_str() {
                "hooked_syscall" => 85,
                "inline_patch" => 90,
                "shadow_table" => 80,
                _ => continue,
            };

            let payload = build_trust_payload(
                &format!("zero_day_fallout:{}", anomaly.kind),
                risk_score,
            );
            submit_trust_signal(payload);
        }
    }
}
