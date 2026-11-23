// src/ebpf/abi.rs
// User-space mirrors of the C structs that BPF programs write into ring and perf buffers.
// Each mirror is #[repr(C)] with the same field order as its C struct. Each one has a
// compile-time size and offset check below, so if the C side drifts the build fails here
// instead of the decoder quietly reading the wrong bytes.
//
// Byte order: the kernel writes these structs in host order (BPF runs on the same CPU).
// Ports and IPv4/IPv6 addresses are the exception. The kernel keeps them in network
// (big-endian) order, so they are converted on read.
//
// The 376-byte struct edr_event from edr_events.h stays in ebpf_bridge.rs (EdREvent).

use std::mem::{offset_of, size_of};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use super::ebpf_bridge::EdREvent;

/// Marker for the mirrors in this file. Every field is an integer or a byte array, so any
/// bit pattern is a valid value and copying raw bytes into the struct is sound.
///
/// # Safety
/// Only implement this for #[repr(C)] structs made of integers and integer arrays.
pub unsafe trait Plain: Copy {}

unsafe impl Plain for EdREvent {}
unsafe impl Plain for NetEvt {}
unsafe impl Plain for WxEvt {}
unsafe impl Plain for FileAccessEvent {}
unsafe impl Plain for ProcLifecycleEvent {}

fn copy_struct<T: Plain>(bytes: &[u8]) -> Option<T> {
    if bytes.len() < size_of::<T>() {
        return None;
    }
    // read_unaligned because nothing promises the slice is aligned for T.
    Some(unsafe { std::ptr::read_unaligned(bytes.as_ptr() as *const T) })
}

/// Ring buffer frames carry exactly the size the program reserved, so any other length
/// means the C struct and this mirror no longer agree.
pub fn from_ring_frame<T: Plain>(bytes: &[u8]) -> Option<T> {
    if bytes.len() != size_of::<T>() {
        return None;
    }
    copy_struct(bytes)
}

/// Perf samples are padded so the 4-byte size header plus the data ends on an 8-byte
/// boundary. A 328-byte struct arrives as 332 bytes. Accept up to 7 bytes of tail padding.
pub fn from_perf_sample<T: Plain>(bytes: &[u8]) -> Option<T> {
    let want = size_of::<T>();
    if bytes.len() < want || bytes.len() - want >= 8 {
        return None;
    }
    copy_struct(bytes)
}

/// Turn a fixed C char array into a String, stopping at the first NUL.
pub fn c_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

/* ---------------- net_flow.bpf.c: struct net_evt on map net_events ---------------- */

pub const NET_KIND_CONNECT: u32 = 1;
pub const NET_KIND_TX: u32 = 2;
pub const NET_KIND_RX: u32 = 3;

/// Mirror of struct net_evt (net_flow.bpf.c:21-30). 48 bytes.
/// The C compiler adds 3 bytes of padding after is_v6 (so daddr_v4 sits on a 4-byte
/// boundary) and 4 bytes after daddr_v6 (so bytes sits on an 8-byte boundary).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NetEvt {
    pub pid: u32,           // offset 0
    pub kind: u32,          // offset 4: 1 connect, 2 tx, 3 rx
    pub family: u16,        // offset 8: AF_INET 2, AF_INET6 10
    pub dport: u16,         // offset 10: network byte order
    pub is_v6: u8,          // offset 12, then 3 bytes of padding
    pub daddr_v4: u32,      // offset 16: network byte order
    pub daddr_v6: [u8; 16], // offset 20, then 4 bytes of padding
    pub bytes: i64,         // offset 40: syscall return for tx/rx
}

impl NetEvt {
    pub fn dst_port(&self) -> u16 {
        u16::from_be(self.dport)
    }

    pub fn dst_ip(&self) -> IpAddr {
        if self.is_v6 != 0 {
            IpAddr::V6(Ipv6Addr::from(self.daddr_v6))
        } else {
            // to_ne_bytes gives the bytes in memory order, which is already network order.
            IpAddr::V4(Ipv4Addr::from(self.daddr_v4.to_ne_bytes()))
        }
    }
}

/* ---------------- wx_exec.bpf.c: struct wx_evt on map wx_events ---------------- */

pub const WX_KIND_W_THEN_X: u32 = 1;
pub const WX_KIND_MEMFD_EXEC: u32 = 2;

pub const PROT_READ: u64 = 0x1;
pub const PROT_WRITE: u64 = 0x2;
pub const PROT_EXEC: u64 = 0x4;

/// Mirror of struct wx_evt (wx_exec.bpf.c:15-23). 48 bytes, no padding.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WxEvt {
    pub pid: u32,   // offset 0
    pub kind: u32,  // offset 4: 1 writable map then exec map within 5 s, 2 memfd then exec
    pub ts_ns: u64, // offset 8: bpf_ktime_get_ns, time since boot
    pub addr: u64,  // offset 16: mmap/mprotect address (kind 1)
    pub len: u64,   // offset 24: mmap/mprotect length (kind 1)
    pub prot: u64,  // offset 32: requested PROT_* bits (kind 1)
    pub bytes: u64, // offset 40: bytes written to memfds before the exec (kind 2)
}

impl WxEvt {
    /// True when the one call itself asked for a writable and executable mapping.
    pub fn is_rwx(&self) -> bool {
        self.prot & (PROT_WRITE | PROT_EXEC) == (PROT_WRITE | PROT_EXEC)
    }

    pub fn prot_string(&self) -> String {
        let mut s = String::with_capacity(3);
        s.push(if self.prot & PROT_READ != 0 { 'r' } else { '-' });
        s.push(if self.prot & PROT_WRITE != 0 { 'w' } else { '-' });
        s.push(if self.prot & PROT_EXEC != 0 { 'x' } else { '-' });
        s
    }
}

/* ------------- file_access_monitor.bpf.c: struct file_access_event_t on perf map events ------------- */

/// Mirror of struct file_access_event_t (file_access_monitor.bpf.c:6). 328 bytes, no padding.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FileAccessEvent {
    pub pid: u32,            // offset 0
    pub uid: u32,            // offset 4
    pub comm: [u8; 64],      // offset 8
    pub filename: [u8; 256], // offset 72: openat path argument
}

/* ------------- proc_lifecycle.bpf.c: struct proc_lc_event on the shared ring ------------- */

pub const PROC_LC_EXEC: u32 = 1001;
pub const PROC_LC_FORK: u32 = 1002;
pub const PROC_LC_EXIT: u32 = 1003;

/// Length of the older proc_lifecycle frame, before argv was added. Sensors loaded from
/// an object built before that change keep writing it until they are reloaded, because
/// pinned programs outlive an agent restart. It is the first 312 bytes of the new layout.
pub const PROC_LC_LEGACY_LEN: usize = 312;

/// Mirror of struct proc_lc_event (proc_lifecycle.bpf.c). 576 bytes, no padding.
/// The first 312 bytes are the older layout. exec frames also carry the filename the
/// tracepoint reports and the start of argv, copied in the kernel so they survive a
/// process that exits before user space reads /proc.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ProcLifecycleEvent {
    pub ts: u64,          // offset 0: bpf_ktime_get_ns
    pub type_: u32,       // offset 8: 1001 exec, 1002 fork, 1003 exit
    pub syscall_id: i32,  // offset 12: always 0
    pub tgid: u32,        // offset 16
    pub ppid: u32,        // offset 20: real_parent->tgid
    pub uid: u32,         // offset 24
    pub fd: i32,          // offset 28: always -1
    pub ret: i32,         // offset 32: always 0
    pub aux_u32: u32,     // offset 36: task->exit_code for exit
    pub comm: [u8; 16],   // offset 40
    pub path: [u8; 256],  // offset 56: exec filename, empty otherwise
    pub argv_len: u32,    // offset 312: bytes copied into argv
    pub argv_total: u32,  // offset 316: full argv length in the process
    pub argv: [u8; 256],  // offset 320: NUL-separated arguments
}

impl ProcLifecycleEvent {
    /// The sensor pins three fields to fixed values. Together with the frame length
    /// they act as a magic number for this frame type.
    pub fn has_lifecycle_magic(&self) -> bool {
        matches!(self.type_, PROC_LC_EXEC | PROC_LC_FORK | PROC_LC_EXIT)
            && self.syscall_id == 0
            && self.fd == -1
    }

    /// The path passed to execve, as the kernel saw it. For a script this is the script,
    /// not the interpreter that /proc/<pid>/exe shows. None for fork and exit.
    pub fn exec_filename(&self) -> Option<String> {
        if self.type_ != PROC_LC_EXEC {
            return None;
        }
        let f = c_string(&self.path);
        (!f.is_empty()).then_some(f)
    }

    /// argv joined with spaces, the same form read_proc gives for /proc/<pid>/cmdline.
    /// Holds at most the first 256 bytes, argv_total says if more was cut.
    pub fn exec_argv(&self) -> Option<String> {
        if self.type_ != PROC_LC_EXEC {
            return None;
        }
        let n = (self.argv_len as usize).min(self.argv.len());
        let s = self.argv[..n]
            .split(|b| *b == 0)
            .filter(|part| !part.is_empty())
            .map(|part| String::from_utf8_lossy(part).into_owned())
            .collect::<Vec<_>>()
            .join(" ");
        (!s.is_empty()).then_some(s)
    }

    pub fn argv_truncated(&self) -> bool {
        self.argv_total > self.argv_len
    }
}

/// A 312-byte frame from an older sensor, read into the new layout. The new fields stay
/// zero. That layout put a copy of comm in path, so path is cleared: in the new layout
/// path only ever holds a filename.
fn from_legacy_proc_frame(bytes: &[u8]) -> Option<ProcLifecycleEvent> {
    if bytes.len() != PROC_LC_LEGACY_LEN {
        return None;
    }
    let mut buf = [0u8; size_of::<ProcLifecycleEvent>()];
    buf[..PROC_LC_LEGACY_LEN].copy_from_slice(bytes);
    let mut p: ProcLifecycleEvent = copy_struct(&buf)?;
    p.path = [0; 256];
    Some(p)
}

/* ---------------- demux for the shared ring edr_events_rb ---------------- */

/// What a frame on the shared ring turned out to be.
pub enum SharedFrame {
    /// 376-byte struct edr_event from raw_syscalls_catcher.bpf.c and tcp_state.bpf.c.
    EdrEvent(EdREvent),
    /// 576-byte (or older 312-byte) frame from proc_lifecycle.bpf.c that passed the
    /// magic check.
    ProcLifecycle(ProcLifecycleEvent),
    /// Anything else. Only the three producers above write to this ring, so an unknown
    /// frame means a C struct and its mirror no longer agree.
    Unknown,
}

/// Size alone picks the 376-byte struct: no other producer on the shared ring writes
/// 376 bytes. A 576-byte or 312-byte frame also has to pass the proc_lifecycle magic
/// check, so a future module that happens to write either size is reported as unknown,
/// not guessed.
pub fn classify_shared_frame(bytes: &[u8]) -> SharedFrame {
    if let Some(e) = from_ring_frame::<EdREvent>(bytes) {
        return SharedFrame::EdrEvent(e);
    }
    let proc = from_ring_frame::<ProcLifecycleEvent>(bytes).or_else(|| from_legacy_proc_frame(bytes));
    if let Some(p) = proc {
        if p.has_lifecycle_magic() {
            return SharedFrame::ProcLifecycle(p);
        }
    }
    SharedFrame::Unknown
}

/* ---------------- compile-time layout guards (sizes and offsets from the C structs) ---------------- */

const _: () = {
    assert!(size_of::<EdREvent>() == 376);

    assert!(size_of::<NetEvt>() == 48);
    assert!(offset_of!(NetEvt, kind) == 4);
    assert!(offset_of!(NetEvt, family) == 8);
    assert!(offset_of!(NetEvt, dport) == 10);
    assert!(offset_of!(NetEvt, is_v6) == 12);
    assert!(offset_of!(NetEvt, daddr_v4) == 16);
    assert!(offset_of!(NetEvt, daddr_v6) == 20);
    assert!(offset_of!(NetEvt, bytes) == 40);

    assert!(size_of::<WxEvt>() == 48);
    assert!(offset_of!(WxEvt, kind) == 4);
    assert!(offset_of!(WxEvt, ts_ns) == 8);
    assert!(offset_of!(WxEvt, prot) == 32);
    assert!(offset_of!(WxEvt, bytes) == 40);

    assert!(size_of::<FileAccessEvent>() == 328);
    assert!(offset_of!(FileAccessEvent, comm) == 8);
    assert!(offset_of!(FileAccessEvent, filename) == 72);

    assert!(size_of::<ProcLifecycleEvent>() == 576);
    assert!(offset_of!(ProcLifecycleEvent, type_) == 8);
    assert!(offset_of!(ProcLifecycleEvent, tgid) == 16);
    assert!(offset_of!(ProcLifecycleEvent, fd) == 28);
    assert!(offset_of!(ProcLifecycleEvent, aux_u32) == 36);
    assert!(offset_of!(ProcLifecycleEvent, comm) == 40);
    assert!(offset_of!(ProcLifecycleEvent, path) == 56);
    assert!(offset_of!(ProcLifecycleEvent, argv_len) == PROC_LC_LEGACY_LEN);
    assert!(offset_of!(ProcLifecycleEvent, argv_total) == 316);
    assert!(offset_of!(ProcLifecycleEvent, argv) == 320);
};

/* ---------------- tests: frames are built byte by byte at the C offsets ---------------- */

#[cfg(test)]
pub(crate) mod test_bytes {
    // Writers used by the tests here and in ebpf_ingest.rs. They place each value at the
    // offset the C compiler uses, so the tests never go through the Rust mirror to build
    // a frame.
    pub fn put_u16(buf: &mut [u8], off: usize, v: u16) {
        buf[off..off + 2].copy_from_slice(&v.to_ne_bytes());
    }
    pub fn put_u32(buf: &mut [u8], off: usize, v: u32) {
        buf[off..off + 4].copy_from_slice(&v.to_ne_bytes());
    }
    pub fn put_i32(buf: &mut [u8], off: usize, v: i32) {
        buf[off..off + 4].copy_from_slice(&v.to_ne_bytes());
    }
    pub fn put_u64(buf: &mut [u8], off: usize, v: u64) {
        buf[off..off + 8].copy_from_slice(&v.to_ne_bytes());
    }
    pub fn put_i64(buf: &mut [u8], off: usize, v: i64) {
        buf[off..off + 8].copy_from_slice(&v.to_ne_bytes());
    }
    pub fn put_str(buf: &mut [u8], off: usize, s: &str) {
        buf[off..off + s.len()].copy_from_slice(s.as_bytes());
    }

    // struct net_evt, IPv4 connect. Port and address go in as big-endian bytes.
    pub fn net_connect_v4(pid: u32, ip: [u8; 4], port: u16) -> Vec<u8> {
        let mut b = vec![0u8; 48];
        put_u32(&mut b, 0, pid);
        put_u32(&mut b, 4, 1);
        put_u16(&mut b, 8, 2);
        b[10..12].copy_from_slice(&port.to_be_bytes());
        b[12] = 0;
        b[16..20].copy_from_slice(&ip);
        b
    }

    pub fn net_bytes(pid: u32, kind: u32, n: i64) -> Vec<u8> {
        let mut b = vec![0u8; 48];
        put_u32(&mut b, 0, pid);
        put_u32(&mut b, 4, kind);
        put_i64(&mut b, 40, n);
        b
    }

    // struct wx_evt
    pub fn wx(pid: u32, kind: u32, ts: u64, addr: u64, len: u64, prot: u64, bytes: u64) -> Vec<u8> {
        let mut b = vec![0u8; 48];
        put_u32(&mut b, 0, pid);
        put_u32(&mut b, 4, kind);
        put_u64(&mut b, 8, ts);
        put_u64(&mut b, 16, addr);
        put_u64(&mut b, 24, len);
        put_u64(&mut b, 32, prot);
        put_u64(&mut b, 40, bytes);
        b
    }

    // struct file_access_event_t as a perf sample: 328 bytes of struct plus 4 of padding.
    pub fn file_access(pid: u32, uid: u32, comm: &str, path: &str) -> Vec<u8> {
        let mut b = vec![0u8; 332];
        put_u32(&mut b, 0, pid);
        put_u32(&mut b, 4, uid);
        put_str(&mut b, 8, comm);
        put_str(&mut b, 72, path);
        b
    }

    // The common 312-byte head of a proc_lifecycle frame, filled the way lc_reserve()
    // fills it.
    fn proc_lc_head(b: &mut [u8], ty: u32, tgid: u32, ppid: u32, uid: u32, comm: &str, aux: u32) {
        put_u64(b, 0, 123_456_789);
        put_u32(b, 8, ty);
        put_i32(b, 12, 0);
        put_u32(b, 16, tgid);
        put_u32(b, 20, ppid);
        put_u32(b, 24, uid);
        put_i32(b, 28, -1);
        put_i32(b, 32, 0);
        put_u32(b, 36, aux);
        put_str(b, 40, comm);
    }

    // struct proc_lc_event, 576 bytes. exec frames get a filename and no argv here.
    pub fn proc_lc(ty: u32, tgid: u32, ppid: u32, uid: u32, comm: &str, aux: u32) -> Vec<u8> {
        let mut b = vec![0u8; 576];
        proc_lc_head(&mut b, ty, tgid, ppid, uid, comm, aux);
        if ty == 1001 {
            put_str(&mut b, 56, &format!("/usr/bin/{comm}"));
        }
        b
    }

    // An exec frame with argv as the kernel copies it: NUL after each argument, cut at
    // 256 bytes, argv_total holding the full length.
    pub fn proc_lc_exec(tgid: u32, comm: &str, filename: &str, args: &[&str]) -> Vec<u8> {
        let mut b = proc_lc(1001, tgid, 1, 0, comm, 0);
        b[56..312].fill(0);
        put_str(&mut b, 56, filename);
        let mut raw = Vec::new();
        for a in args {
            raw.extend_from_slice(a.as_bytes());
            raw.push(0);
        }
        let n = raw.len().min(256);
        put_u32(&mut b, 312, n as u32);
        put_u32(&mut b, 316, raw.len() as u32);
        b[320..320 + n].copy_from_slice(&raw[..n]);
        b
    }

    // The older 312-byte layout: no argv, and exec put a copy of comm in path.
    pub fn proc_lc_legacy(ty: u32, tgid: u32, ppid: u32, uid: u32, comm: &str, aux: u32) -> Vec<u8> {
        let mut b = vec![0u8; 312];
        proc_lc_head(&mut b, ty, tgid, ppid, uid, comm, aux);
        if ty == 1001 {
            put_str(&mut b, 56, comm);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::test_bytes::*;
    use super::*;

    #[test]
    fn mirror_sizes_match_c_structs() {
        // Same numbers as the const guards, kept as a test so they show up in cargo test.
        assert_eq!(size_of::<EdREvent>(), 376);
        assert_eq!(size_of::<NetEvt>(), 48);
        assert_eq!(size_of::<WxEvt>(), 48);
        assert_eq!(size_of::<FileAccessEvent>(), 328);
        assert_eq!(size_of::<ProcLifecycleEvent>(), 576);
    }

    #[test]
    fn net_evt_reads_network_order_fields() {
        let b = net_connect_v4(4242, [93, 184, 216, 34], 8080);
        let e: NetEvt = from_ring_frame(&b).expect("48-byte frame");
        assert_eq!(e.pid, 4242);
        assert_eq!(e.kind, NET_KIND_CONNECT);
        assert_eq!(e.family, 2);
        assert_eq!(e.dst_port(), 8080);
        assert_eq!(e.dst_ip().to_string(), "93.184.216.34");
    }

    #[test]
    fn net_evt_ipv6_connect() {
        let mut b = vec![0u8; 48];
        put_u32(&mut b, 0, 7);
        put_u32(&mut b, 4, 1);
        put_u16(&mut b, 8, 10);
        b[10..12].copy_from_slice(&443u16.to_be_bytes());
        b[12] = 1;
        let v6: [u8; 16] = [0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        b[20..36].copy_from_slice(&v6);
        let e: NetEvt = from_ring_frame(&b).unwrap();
        assert_eq!(e.dst_ip().to_string(), "2001:db8::1");
        assert_eq!(e.dst_port(), 443);
    }

    #[test]
    fn wx_evt_fields_by_offset() {
        let b = wx(99, WX_KIND_W_THEN_X, 5, 0x7f00_0000, 4096, 0x7, 0);
        let e: WxEvt = from_ring_frame(&b).unwrap();
        assert_eq!((e.pid, e.kind, e.ts_ns), (99, 1, 5));
        assert_eq!((e.addr, e.len, e.prot), (0x7f00_0000, 4096, 7));
        assert!(e.is_rwx());
        assert_eq!(e.prot_string(), "rwx");

        let b = wx(99, WX_KIND_W_THEN_X, 5, 0, 0, PROT_READ | PROT_EXEC, 0);
        let e: WxEvt = from_ring_frame(&b).unwrap();
        assert!(!e.is_rwx());
        assert_eq!(e.prot_string(), "r-x");
    }

    #[test]
    fn file_access_accepts_perf_padding_only() {
        let b = file_access(31, 1000, "cat", "/etc/hostname");
        let e: FileAccessEvent = from_perf_sample(&b).unwrap();
        assert_eq!((e.pid, e.uid), (31, 1000));
        assert_eq!(c_string(&e.comm), "cat");
        assert_eq!(c_string(&e.filename), "/etc/hostname");
        // the bare struct with no padding is also fine
        assert!(from_perf_sample::<FileAccessEvent>(&b[..328]).is_some());
        // short frames and frames 8 or more bytes too long are drift
        assert!(from_perf_sample::<FileAccessEvent>(&b[..327]).is_none());
        assert!(from_perf_sample::<FileAccessEvent>(&vec![0u8; 336]).is_none());
    }

    #[test]
    fn shared_ring_demux_uses_size_and_magic() {
        let fork = proc_lc(PROC_LC_FORK, 10, 1, 0, "bash", 0);
        match classify_shared_frame(&fork) {
            SharedFrame::ProcLifecycle(p) => {
                assert_eq!((p.type_, p.tgid, p.ppid), (PROC_LC_FORK, 10, 1));
                assert_eq!(c_string(&p.comm), "bash");
            }
            _ => panic!("312-byte frame with the magic should be proc_lifecycle"),
        }

        // 576 bytes but fd is not -1: not ours, so it stays unknown
        let mut other = fork.clone();
        put_i32(&mut other, 28, 3);
        assert!(matches!(classify_shared_frame(&other), SharedFrame::Unknown));

        // 576 bytes with an id outside 1001-1003
        let mut other = fork.clone();
        put_u32(&mut other, 8, 101);
        assert!(matches!(classify_shared_frame(&other), SharedFrame::Unknown));

        // the older 312-byte layout needs the same magic
        let old = proc_lc_legacy(PROC_LC_FORK, 10, 1, 0, "bash", 0);
        assert!(matches!(classify_shared_frame(&old), SharedFrame::ProcLifecycle(_)));
        let mut other = old.clone();
        put_i32(&mut other, 28, 3);
        assert!(matches!(classify_shared_frame(&other), SharedFrame::Unknown));

        // sizes in between belong to nobody
        assert!(matches!(classify_shared_frame(&fork[..400]), SharedFrame::Unknown));

        let mut full = vec![0u8; 376];
        put_u32(&mut full, 8, 21);
        assert!(matches!(classify_shared_frame(&full), SharedFrame::EdrEvent(e) if e.type_ == 21));

        // any other size, short frames included, is unknown
        assert!(matches!(classify_shared_frame(&vec![0u8; 200]), SharedFrame::Unknown));
        assert!(matches!(classify_shared_frame(&vec![0u8; 8]), SharedFrame::Unknown));
    }

    #[test]
    fn exec_frame_carries_filename_and_argv() {
        let b = proc_lc_exec(42, "chattr", "/usr/bin/chattr", &["chattr", "+i", "/home/a", "/srv/b"]);
        let SharedFrame::ProcLifecycle(p) = classify_shared_frame(&b) else {
            panic!("576-byte exec frame should be proc_lifecycle");
        };
        assert_eq!((p.type_, p.tgid), (PROC_LC_EXEC, 42));
        assert_eq!(p.exec_filename().as_deref(), Some("/usr/bin/chattr"));
        assert_eq!(p.exec_argv().as_deref(), Some("chattr +i /home/a /srv/b"));
        assert!(!p.argv_truncated());

        // argv longer than the 256-byte copy: the start is kept and the cut is visible
        let long = "x".repeat(300);
        let b = proc_lc_exec(43, "sh", "/bin/sh", &["sh", "-c", &long]);
        let SharedFrame::ProcLifecycle(p) = classify_shared_frame(&b) else { panic!() };
        assert!(p.argv_truncated());
        assert_eq!(p.argv_total, 307); // "sh\0" + "-c\0" + 300 x + "\0"
        let argv = p.exec_argv().unwrap();
        // 256 bytes kept: "sh\0-c\0" and 250 x. Each NUL becomes one space.
        assert_eq!(argv, format!("sh -c {}", "x".repeat(250)));

        // fork and exit frames have no filename or argv even if bytes are there
        let mut b = proc_lc_exec(44, "sh", "/bin/sh", &["sh"]);
        put_u32(&mut b, 8, PROC_LC_EXIT);
        let SharedFrame::ProcLifecycle(p) = classify_shared_frame(&b) else { panic!() };
        assert!(p.exec_filename().is_none() && p.exec_argv().is_none());
    }

    #[test]
    fn legacy_312_byte_exec_frame_has_no_filename() {
        // The older layout copied comm into path. That is not a filename, so it is dropped.
        let b = proc_lc_legacy(PROC_LC_EXEC, 9, 1, 0, "sleep", 0);
        let SharedFrame::ProcLifecycle(p) = classify_shared_frame(&b) else {
            panic!("312-byte frame with the magic should still be proc_lifecycle");
        };
        assert_eq!((p.type_, p.tgid), (PROC_LC_EXEC, 9));
        assert_eq!(c_string(&p.comm), "sleep");
        assert!(p.exec_filename().is_none());
        assert!(p.exec_argv().is_none());
        assert_eq!((p.argv_len, p.argv_total), (0, 0));
    }

    #[test]
    fn ring_frames_must_match_exactly() {
        assert!(from_ring_frame::<NetEvt>(&vec![0u8; 47]).is_none());
        assert!(from_ring_frame::<NetEvt>(&vec![0u8; 56]).is_none());
        assert!(from_ring_frame::<WxEvt>(&vec![0u8; 48]).is_some());
    }
}
