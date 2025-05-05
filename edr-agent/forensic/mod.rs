pub mod file_watch;
pub mod net_watch;

use std::thread;

pub fn start_forensic_monitors() {
    thread::spawn(|| file_watch::watch_sensitive_dirs());
    thread::spawn(|| net_watch::log_open_connections());
}
