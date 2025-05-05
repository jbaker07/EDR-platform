use std::thread;
use std::time::Duration;
use sysinfo::{System, SystemExt, ProcessExt};

pub fn start_process_monitor() {
    thread::spawn(|| loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        for (pid, process) in sys.processes() {
            println!(
                "👁️ [process_monitor] PID: {}, Name: {}, CPU: {:.2}%, Mem: {} KB",
                pid,
                process.name(),
                process.cpu_usage(),
                process.memory()
            );
        }

        thread::sleep(Duration::from_secs(30));
    });
}
