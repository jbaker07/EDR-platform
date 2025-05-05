use std::process::Command;

pub fn log_open_connections() {
    println!("🌐 [net_watch] Logging open TCP connections...");

    loop {
        let output = Command::new("netstat")
            .arg("-tunap") // Linux-style netstat
            .output();

        match output {
            Ok(data) => {
                let stdout = String::from_utf8_lossy(&data.stdout);
                for line in stdout.lines().skip(2) {
                    if line.contains("ESTABLISHED") || line.contains("LISTEN") {
                        println!("🌐 Net activity: {}", line);
                    }
                }
            }
            Err(e) => {
                eprintln!("[net_watch] Failed to run netstat: {}", e);
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
