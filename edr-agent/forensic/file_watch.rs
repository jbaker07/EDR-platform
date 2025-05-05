use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;

pub fn watch_sensitive_dirs() {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(2)).expect("Failed to create watcher");

    let paths = ["/etc", "/usr/bin", "/var/log"];

    for path in &paths {
        if Path::new(path).exists() {
            watcher
                .watch(path, RecursiveMode::Recursive)
                .expect(&format!("Failed to watch path: {}", path));
        }
    }

    println!("📁 [file_watch] Watching sensitive directories...");

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(p) |
                DebouncedEvent::Remove(p) |
                DebouncedEvent::Write(p) |
                DebouncedEvent::Rename(_, p) => {
                    println!("⚠️ File event: {:?}", p);
                }
                _ => {}
            },
            Err(e) => println!("[file_watch] Watch error: {:?}", e),
        }
    }
}
