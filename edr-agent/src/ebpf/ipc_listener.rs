use std::fs::File;
use std::io::{BufReader, Read};
use std::os::unix::net::UnixListener;
use std::thread;
use std::path::Path;

const SOCKET_PATH: &str = "/tmp/edr_ipc.sock";

pub fn start_ipc_listener() {
    // Clean up old socket if it exists
    if Path::new(SOCKET_PATH).exists() {
        std::fs::remove_file(SOCKET_PATH).expect("Failed to remove stale IPC socket");
    }

    let listener = UnixListener::bind(SOCKET_PATH).expect("Failed to bind IPC socket");
    println!("🔌 [IPC Listener] Listening on {}", SOCKET_PATH);

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let mut reader = BufReader::new(stream);
                    let mut buffer = String::new();
                    if let Ok(_) = reader.read_to_string(&mut buffer) {
                        println!("📦 [IPC] Received: {}", buffer.trim());

                        // Forward to the replay engine or forensics system
                        if let Err(e) = crate::replay_engine::handle_ipc_payload(&buffer) {
                            eprintln!("❌ Failed to handle IPC payload: {}", e);
                        }
                    } else {
                        eprintln!("⚠️ [IPC] Failed to read IPC stream.");
                    }
                }
                Err(e) => {
                    eprintln!("❌ [IPC] Connection failed: {}", e);
                }
            }
        }
    });
}
