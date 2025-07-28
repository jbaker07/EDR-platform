use std::os::unix::net::UnixStream;
use std::io::Write;
use std::time::Duration;
use std::thread;
use std::fs;
use std::path::Path;

use libbpf_rs::{MapFlags, ObjectBuilder};
use nix::unistd::getpid;
use crate::syscall_map::syscall_id_to_name;

const BPF_OBJECT: &str = "/usr/local/lib/ebpf/syscall_tracer.bpf.o";
const IPC_SOCKET: &str = "/tmp/edr_ipc.sock";

pub fn start_ebpf_syscall_tracer() {
    thread::spawn(move || {
        if !Path::new(BPF_OBJECT).exists() {
            eprintln!("❌ [EBPF] Missing BPF object file: {}", BPF_OBJECT);
            return;
        }

        let builder = ObjectBuilder::default();
        let mut obj = match builder.open_file(BPF_OBJECT).and_then(|o| o.load()) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("❌ [EBPF] Failed to load BPF object: {}", e);
                return;
            }
        };

        let prog = match obj.prog_mut("trace_execve") {
            Some(p) => p,
            None => {
                eprintln!("❌ [EBPF] trace_execve program not found in object.");
                return;
            }
        };

        if let Err(e) = prog.attach_tracepoint("syscalls", "sys_enter_execve") {
            eprintln!("❌ [EBPF] Failed to attach tracepoint: {}", e);
            return;
        }

        let ringbuf = match obj.ringbuf("events") {
            Some(r) => r,
            None => {
                eprintln!("❌ [EBPF] Ring buffer 'events' not found.");
                return;
            }
        };

        let syscall_map = syscall_id_to_name();

        ringbuf.open(move |data| {
            if data.len() < 8 {
                return 0;
            }

            let pid = u32::from_ne_bytes([data[0], data[1], data[2], data[3]]);
            let syscall_id = u32::from_ne_bytes([data[4], data[5], data[6], data[7]]);
            let syscall_name = syscall_map.get(&syscall_id).unwrap_or(&"unknown_syscall");

            let payload = format!(
                "{{\"type\": \"syscall_event\", \"pid\": {}, \"syscall_id\": {}, \"syscall_name\": \"{}\", \"source\": \"ebpf\"}}\n",
                pid, syscall_id, syscall_name
            );

            if let Ok(mut stream) = UnixStream::connect(IPC_SOCKET) {
                let _ = stream.write_all(payload.as_bytes());
            } else {
                eprintln!("⚠️ [EBPF] Failed to connect to IPC socket at {}", IPC_SOCKET);
            }

            0
        }).expect("❌ [EBPF] Failed to open ring buffer.");

        println!("✅ [EBPF] Syscall tracer active on PID {}", getpid());

        loop {
            thread::sleep(Duration::from_secs(60));
        }
    });
}
