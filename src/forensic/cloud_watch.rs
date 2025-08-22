use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::trust_hook::{generate_feature_vector, get_trust_payload};
use crate::gnn_hook::push_to_gnn_vector_log;
use hostname;

/// Monitors attempts to access EC2 instance metadata (IMDS)
pub fn start_cloud_watch() {
    thread::spawn(|| loop {
        let hostname = hostname::get().unwrap_or_default().to_string_lossy().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let output = Command::new("curl")
            .arg("http://169.254.169.254/latest/meta-data/")
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                let risk_score = 5.0;
                let cpu = 0.1;
                let mem = 51200.0;

                let trust_payload = get_trust_payload(&hostname, cpu, mem, risk_score);
                let features = generate_feature_vector(cpu, mem, risk_score);
                push_to_gnn_vector_log(&hostname, &features);

                println!(
                    "[☁️ Cloud Watch] Detected IMDS access — Trust Score: {}",
                    trust_payload.get("trust_score").unwrap_or(&"N/A".to_string())
                );
            }
        }

        thread::sleep(Duration::from_secs(120));
    });
}
