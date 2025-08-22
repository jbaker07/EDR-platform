use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, Config, Result as NotifyResult};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use serde::Serialize;
use crate::trust_hook;
use crate::score_reason;

#[derive(Debug, Serialize, Clone)]
pub struct FileEvent {
    pub path: String,
    pub kind: String,
    pub timestamp: u64,
}

/// Starts a background thread to watch file events in specified directories.
/// Returns a Receiver to consume file events.
pub fn start_file_watch(paths: Vec<PathBuf>) -> Receiver<FileEvent> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let (notify_tx, notify_rx) = channel();
        let watcher_result: NotifyResult<RecommendedWatcher> =
            Watcher::new(notify_tx, Config::default());

        if let Ok(mut watcher) = watcher_result {
            for path in &paths {
                if let Err(e) = watcher.watch(path, RecursiveMode::Recursive) {
                    eprintln!("❌ Failed to watch {:?}: {}", path, e);
                } else {
                    println!("📂 Watching {:?}", path);
                }
            }

            for event in notify_rx {
                match event {
                    Ok(Event { paths, kind, .. }) => {
                        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

                        let risk_score = 0.25;
                        let trust_payload = trust_hook::generate_trust_payload(0.15, 30000.0, risk_score);
                        let reason = score_reason::build_reason("File event detected", risk_score, "N/A", "N/A");
                        println!("[📁 File Trust] {} | {}", trust_payload, reason);

                        for path in paths {
                            let fe = FileEvent {
                                path: path.to_string_lossy().to_string(),
                                kind: format!("{:?}", kind),
                                timestamp: ts,
                            };
                            let _ = tx.send(fe.clone());
                            println!("📝 File event: {:?}", fe);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Watch error: {:?}", e);
                    }
                }
            }
        } else {
            eprintln!("❌ Failed to initialize file watcher");
        }
    });

    rx
}
