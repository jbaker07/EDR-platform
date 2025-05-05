use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher, Event, Config};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct FileEvent {
    pub path: String,
    pub kind: String,
    pub timestamp: u64,
}

/// Starts a background thread that watches the given directories and pushes file events into a channel
pub fn start_file_watch(paths: Vec<PathBuf>) -> Receiver<FileEvent> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let (notify_tx, notify_rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(notify_tx, Config::default()).unwrap();

        for path in &paths {
            if let Err(e) = watcher.watch(path, RecursiveMode::Recursive) {
                eprintln!("❌ Failed to watch {:?}: {}", path, e);
            }
        }

        for event in notify_rx {
            if let Ok(Event { paths, kind, .. }) = event {
                let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
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
        }
    });

    rx
}
