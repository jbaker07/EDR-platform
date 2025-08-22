// src/normalize.rs
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::{HashMap, VecDeque};
use crate::ingest::KernelEvent;

// Replace these with your real types.
#[derive(Debug, Clone)]
pub struct Event {
    pub ts_ns: u64,
    pub pid: u32,
    pub kind: EventKind,
}

#[derive(Debug, Clone)]
pub enum EventKind {
    Exec { path: String, comm: String },
    WxExec { path: String, comm: String, prot: u32 },
    Mmap { addr: u64, len: u64, prot: u32, flags: u32 },
    Mprotect { addr: u64, len: u64, prot: u32 },
    Connect { fd: i32, family: u16, port: u16, ip: [u8;16] },
    Write { fd: i32, count: u32, is_pipe: bool },
    Pipe { src_fd: i32, dst_fd: i32 },
    NetFlow { tx: u64, rx: u64, key: u64 },
    CredDump { kind: u32, target: u32 },
    ProcHollow { target: u32 },
    SuspiciousIpc { kind: u16, peer: u32 },
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from_pid: u32,
    pub to_pid: u32,
    pub kind: EdgeKind,
}

#[derive(Debug, Clone)]
pub enum EdgeKind {
    Spawned,
    WroteFd(i32),
    PipeLink(i32, i32),
    Connected(u16),
    MappedExec,
}

#[derive(Debug, Clone, Default)]
pub struct Episode {
    pub events: Vec<Event>,
    pub edges: Vec<Edge>,
    pub window_start_ns: u64,
    pub window_end_ns: u64,
}

pub static NORMALIZER: Lazy<Mutex<Normalizer>> = Lazy::new(|| Mutex::new(Normalizer::new()));

pub struct Normalizer {
    // rolling window
    buf: VecDeque<Event>,
    edges: Vec<Edge>,
    // lineage/cache
    ppid: HashMap<u32, u32>,
    first_ts: u64,
    last_ts: u64,
    // tuneable
    max_events: usize,
}

impl Normalizer {
    pub fn new() -> Self {
        Self {
            buf: VecDeque::with_capacity(4096),
            edges: Vec::with_capacity(4096),
            ppid: HashMap::new(),
            first_ts: 0,
            last_ts: 0,
            max_events: 8192,
        }
    }

    pub fn apply(&mut self, ev: KernelEvent) {
        use KernelEvent::*;
        match ev {
            Exec { pid, ppid, ts_ns, path, comm, .. } => {
                self.ppid.insert(pid, ppid);
                self.push(Event { ts_ns, pid, kind: EventKind::Exec { path, comm } });
                if let Some(parent) = self.ppid.get(&pid).copied() {
                    self.edges.push(Edge { from_pid: parent, to_pid: pid, kind: EdgeKind::Spawned });
                }
            }
            WxExec { pid, ts_ns, mm_prot, path, comm, .. } => {
                self.push(Event { ts_ns, pid, kind: EventKind::WxExec { path, comm, prot: mm_prot } });
                self.edges.push(Edge { from_pid: pid, to_pid: pid, kind: EdgeKind::MappedExec });
            }
            Mmap { pid, ts_ns, addr, len, prot, flags } => {
                self.push(Event { ts_ns, pid, kind: EventKind::Mmap { addr, len, prot, flags } });
            }
            Mprotect { pid, ts_ns, addr, len, prot } => {
                self.push(Event { ts_ns, pid, kind: EventKind::Mprotect { addr, len, prot } });
            }
            Connect { pid, ts_ns, fd, family: _, port, ip } => {
                self.push(Event { ts_ns, pid, kind: EventKind::Connect { fd, family: 0, port, ip } });
            }
            Write { pid, ts_ns, fd, count, is_pipe } => {
                self.push(Event { ts_ns, pid, kind: EventKind::Write { fd, count, is_pipe } });
                if is_pipe {
                    self.edges.push(Edge { from_pid: pid, to_pid: pid, kind: EdgeKind::WroteFd(fd) });
                }
            }
            Pipe { pid, ts_ns, src_fd, dst_fd } => {
                self.push(Event { ts_ns, pid, kind: EventKind::Pipe { src_fd, dst_fd } });
                self.edges.push(Edge { from_pid: pid, to_pid: pid, kind: EdgeKind::PipeLink(src_fd, dst_fd) });
            }
            NetFlow { pid, ts_ns, bytes_tx, bytes_rx, five_tuple_hash } => {
                self.push(Event { ts_ns, pid, kind: EventKind::NetFlow { tx: bytes_tx, rx: bytes_rx, key: five_tuple_hash } });
            }
            CredDump { pid, ts_ns, kind, target } => {
                self.push(Event { ts_ns, pid, kind: EventKind::CredDump { kind, target } });
            }
            ProcHollow { pid, ts_ns, target } => {
                self.push(Event { ts_ns, pid, kind: EventKind::ProcHollow { target } });
            }
            SuspiciousIpc { pid, ts_ns, kind, peer } => {
                self.push(Event { ts_ns, pid, kind: EventKind::SuspiciousIpc { kind, peer } });
            }
        }

        // backpressure: keep window bounded
        while self.buf.len() > self.max_events {
            self.buf.pop_front();
        }
    }

    fn push(&mut self, ev: Event) {
        if self.first_ts == 0 { self.first_ts = ev.ts_ns; }
        self.last_ts = ev.ts_ns;
        self.buf.push_back(ev);
    }

    /// Call this on your 30s flush to hand the Episode to scoring (TrustVector/Mahalanobis/Envelope/KRIM-lite).
    pub fn flush_batch(&mut self) -> Episode {
        let events: Vec<Event> = self.buf.drain(..).collect();
        let edges = std::mem::take(&mut self.edges);
        let ep = Episode {
            window_start_ns: self.first_ts,
            window_end_ns: self.last_ts,
            events,
            edges,
        };
        self.first_ts = 0;
        self.last_ts = 0;
        ep
    }
}
