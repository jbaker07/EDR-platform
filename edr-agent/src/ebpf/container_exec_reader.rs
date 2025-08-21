use std::time::Duration;
use anyhow::Result;
use libbpf_rs::{ObjectBuilder, RingBufferBuilder};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ContainerExecEvent {
    pub ts: u64,
    pub pid: u32,
    pub uid: u32,
    pub cgroup_id: u64,
    pub mnt_ns: u32,
    pub pid_ns: u32,
    pub comm: [u8; 16],
    pub filename: [u8; 256],
    pub flags: u32,
}

const BPF_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/ebpf/container_exec_monitor.bpf.o"));

fn to_string(buf: &[u8]) -> String {
    let n = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf8_lossy(&buf[..n]).to_string()
}

pub async fn spawn_container_exec_reader() -> Result<()> {
    let builder = ObjectBuilder::default().open_memory(BPF_BYTES)?;
    let mut obj = builder.load()?;

    // Attach tracepoints
    obj.prog_mut("handle_sys_enter_execve")
        .unwrap()
        .attach_tracepoint("syscalls", "sys_enter_execve")?;
    obj.prog_mut("handle_sys_enter_execveat")
        .unwrap()
        .attach_tracepoint("syscalls", "sys_enter_execveat")?;

    // Ringbuf
    let mut rb = RingBufferBuilder::new();
    let events_map = obj.map_mut("events").expect("events map not found");
    rb.add(events_map, |data: &[u8]| {
        if data.len() < std::mem::size_of::<ContainerExecEvent>() { return; }
        let mut ev = ContainerExecEvent {
            ts: 0, pid: 0, uid: 0, cgroup_id: 0, mnt_ns: 0, pid_ns: 0,
            comm: [0;16], filename: [0;256], flags: 0
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                &mut ev as *mut _ as *mut u8,
                std::mem::size_of::<ContainerExecEvent>(),
            );
        }

        // Turn into a normalized KernelEvent for your ingest pipeline
        let comm = to_string(&ev.comm);
        let file = to_string(&ev.filename);

        crate::ingest::ingest_kernel_event(crate::ingest::KernelEvent::ContainerExec {
            ts_ns: ev.ts,
            pid: ev.pid,
            uid: ev.uid,
            cgroup_id: ev.cgroup_id,
            mnt_ns: ev.mnt_ns,
            pid_ns: ev.pid_ns,
            comm,
            filename: file,
            is_execveat: (ev.flags & 1) != 0,
        });
    })?;

    let mut ring = rb.build()?;

    // Poll forever (you already spawn in telemetry::start_realtime_monitors)
    loop {
        ring.poll(Duration::from_millis(100))?;
        tokio::task::yield_now().await;
    }
}
