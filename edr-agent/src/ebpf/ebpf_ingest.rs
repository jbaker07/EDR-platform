// src/ebpf/ebpf_ingest.rs
//
// Turns raw frames from the pinned BPF maps into telemetry.
// Layouts live in abi.rs (one mirror per C struct). Each map-specific decoder is split in
// two: a pure decode_* function (bytes in, fields and record out, no I/O, unit tested)
// and an on_* wrapper that writes the sinks and reads /proc for enrichment.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use forensic_hooks::gnn_hook::push_to_gnn_vector_log;
use forensic_hooks::modules::net_watch;
use forensic_hooks::pb_adapter::clock::mono_ns_to_unix_secs;
use forensic_hooks::telemetry::TelemetryRecord;
use forensic_hooks::telemetry_types::TelemetryOutput;
use forensic_hooks::telemetry_writer::{write_telemetry_record, TelemetryWriter};
use forensic_hooks::utils::strnorm::sanitize_tag_value;
use forensic_hooks::utils::time::now_ts;

use crate::ebpf::abi::{
    self, classify_shared_frame, FileAccessEvent, NetEvt, ProcLifecycleEvent, SharedFrame,
    WxEvt,
};
use crate::ebpf::ebpf_bridge::{
    edr_event_to_output, edr_event_to_record, populate_proc_exe, populate_proc_fields, EdREvent,
};
use crate::pb_adapter;

/// How often to log mismatch messages after the first few.
const MISMATCH_THROTTLE_N: usize = 1000;

static MISMATCH_WARNED: AtomicUsize = AtomicUsize::new(0);

fn warn_drift(source: &str, got: usize, detail: &str) {
    let n = MISMATCH_WARNED.fetch_add(1, Ordering::Relaxed);
    if n < 5 || n % MISMATCH_THROTTLE_N == 0 {
        eprintln!("[ebpf_ingest] {source}: dropping {got}B frame ({detail})");
    }
}

/// Frames from the agent's own process that still reached user space, by source.
/// raw_syscalls_catcher skips the agent in the kernel once raw_config has written its
/// tgid to edr_self_tgid, so the raw count stays at 0 unless that map is missing.
/// file_access_monitor has no such map, so its count is the agent's own opens.
static SELF_DROPPED_RAW: AtomicU64 = AtomicU64::new(0);
static SELF_DROPPED_FILE: AtomicU64 = AtomicU64::new(0);

pub fn self_frames_dropped() -> (u64, u64) {
    (
        SELF_DROPPED_RAW.load(Ordering::Relaxed),
        SELF_DROPPED_FILE.load(Ordering::Relaxed),
    )
}

/// True for the agent's own pid. Every record the agent writes opens and writes its log
/// files, and the sensors report those calls, so without this check the agent would
/// decode its own logging and log again. The kernel reports host pids, so this only
/// matches when the agent runs in the host pid namespace.
fn is_agent_pid(pid: u32) -> bool {
    pid != 0 && pid == std::process::id()
}

/// True (and counted) when a frame came from the agent itself and must be dropped.
fn drop_own_frame(pid: u32, counter: &AtomicU64) -> bool {
    if is_agent_pid(pid) {
        counter.fetch_add(1, Ordering::Relaxed);
        return true;
    }
    false
}

/// /proc is gone when the process exited before its exec frame was read. The kernel
/// copied the filename and argv at exec, so those fill what /proc could not.
fn fill_exec_from_kernel_copy(rec: &mut TelemetryRecord, p: &ProcLifecycleEvent) {
    if rec.binary_path.is_empty() {
        rec.binary_path = p.exec_filename().unwrap_or_default();
    }
    if rec.command_line.is_empty() {
        rec.command_line = p.exec_argv().unwrap_or_default();
    }
}

/// A decoded frame: the output for the JSON sinks and adapter, plus the record for the
/// batch loop. tag0 keeps the short legacy tag that older rules key on.
pub(crate) struct Decoded {
    pub out: TelemetryOutput,
    pub tag0: String,
    pub record: TelemetryRecord,
}

/// Side effects shared by every map-specific decoder.
fn sink(d: &Decoded) {
    pb_adapter::emit_adapter_facts(&d.out);

    let mut m = d.out.data.clone();
    m.insert("category".into(), d.out.category.clone());
    m.insert("signal".into(), d.out.signal.clone());
    m.insert("confidence".into(), format!("{:.2}", d.out.confidence));
    m.insert("tag0".into(), d.tag0.clone());

    write_telemetry_record(m.clone());
    push_to_gnn_vector_log(m);
}

fn record(
    now_secs: u64,
    pid: u32,
    ppid: u32,
    uid: u32,
    path: String,
    tags: Vec<String>,
) -> TelemetryRecord {
    TelemetryRecord {
        // Unix seconds: the kernel stamp converted by event_secs when the frame has one,
        // else the decode time.
        timestamp: now_secs,
        pid: pid as i32,
        ppid: ppid as i32,
        uid,
        binary_path: path,
        command_line: String::new(),
        cwd: String::new(),
        env_vars: None,
        tags,
        risk_score: None,
    }
}

/// Event time in Unix seconds. ts_ns is bpf_ktime_get_ns() (time since boot) and is
/// moved to the wall clock with the boot offset measured once. Frames without a kernel
/// stamp keep the decode time.
fn event_secs(ts_ns: u64, now_secs: u64) -> u64 {
    if ts_ns == 0 {
        now_secs
    } else {
        mono_ns_to_unix_secs(ts_ns)
    }
}

/// ---------- generic counting wrapper (no direct dependency on crate statics) ----------
/// Called from the ringbuf/perf reader to bump metrics.
/// `bump_in` runs for every delivered blob; `bump_acc` runs only if we accept/persist.
pub fn on_edr_event_counting<FI, FA>(
    bytes: &[u8],
    writer: &Arc<Mutex<TelemetryWriter>>,
    bump_in: FI,
    bump_acc: FA,
) -> Option<TelemetryRecord>
where
    FI: FnOnce(),
    FA: FnOnce(),
{
    // count every message delivered to userspace
    bump_in();

    let res = on_edr_event(bytes, writer);

    // count only accepted events
    if res.is_some() {
        bump_acc();
    }
    res
}

/// ---------- shared ring edr_events_rb ----------
/// raw_syscalls_catcher, tcp_state and proc_lifecycle write here (edr_attach_any.c points
/// their edr_events_rb maps at one pinned ring). abi::classify_shared_frame picks the frame
/// type by size, plus a magic check for proc_lifecycle. Anything it cannot identify,
/// short frames included, is unexpected and stays visible as ebpf_unknown_frame.
pub fn on_edr_event(bytes: &[u8], writer: &Arc<Mutex<TelemetryWriter>>) -> Option<TelemetryRecord> {
    match classify_shared_frame(bytes) {
        SharedFrame::EdrEvent(e) => on_full_edr_event(&e, bytes.len(), writer),
        SharedFrame::ProcLifecycle(p) => {
            let d = decode_proc_lifecycle(&p, now_ts())?;
            sink(&d);
            let mut rec = d.record;
            populate_proc_exe(&mut rec);
            populate_proc_fields(&mut rec);
            fill_exec_from_kernel_copy(&mut rec, &p);
            Some(rec)
        }
        SharedFrame::Unknown => Some(unknown_frame(bytes.len())),
    }
}

/// 376-byte struct edr_event from raw_syscalls_catcher.bpf.c and tcp_state.bpf.c.
fn on_full_edr_event(
    e: &EdREvent,
    got: usize,
    writer: &Arc<Mutex<TelemetryWriter>>,
) -> Option<TelemetryRecord> {
    // Backstop for the kernel-side check in raw_syscalls_catcher (edr_self_tgid).
    if drop_own_frame(e.tgid, &SELF_DROPPED_RAW) {
        return None;
    }

    // Build TelemetryOutput robustly.
    let out = match std::panic::catch_unwind(|| edr_event_to_output(e)) {
        Ok(o) => o,
        Err(_) => {
            let k = MISMATCH_WARNED.fetch_add(1, Ordering::Relaxed);
            if k < 5 || k % MISMATCH_THROTTLE_N == 0 {
                eprintln!(
                    "[ebpf_ingest] edr_event_to_output panicked on {}B payload (host {})",
                    got,
                    std::mem::size_of::<EdREvent>()
                );
            }
            return None;
        }
    };

    pb_adapter::emit_adapter_facts(&out);

    // Normalize a few common signal name variants
    let normalized_signal = match out.signal.as_str() {
        "net_connect" | "network_connect" => "network::connect",
        "proc_exec" | "process_exec" => "process::exec",
        s => s,
    };

    let mut m = out.data.clone();
    m.insert("category".into(), out.category.clone());
    m.insert("signal".into(), normalized_signal.to_string());
    m.insert("confidence".into(), format!("{:.2}", out.confidence));
    // keep a stable tag for rules/pipes that relied on the old key
    m.insert("tag0".into(), out.signal.clone());

    write_telemetry_record(m.clone());
    push_to_gnn_vector_log(m.clone());

    // A connect or accept asks for a snapshot of open connections. net_watch runs at
    // most one per second on its own thread, so a burst of connects costs one /proc walk.
    if normalized_signal == "network::connect" || out.signal == "net_accept" {
        net_watch::request_connection_snapshot(writer);
    }

    // Return a TelemetryRecord for batch scoring/episode.
    std::panic::catch_unwind(|| edr_event_to_record(e)).ok()
}

/// Frames no mirror claims. Kept visible with their length so new producers show up.
fn unknown_frame(len: usize) -> TelemetryRecord {
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert("category".into(), "process".into());
    m.insert("signal".into(), "ebpf_unknown_frame".into());
    m.insert("wire_len".into(), len.to_string());
    let adapter_out = TelemetryOutput {
        category: "process".into(),
        signal: "ebpf_unknown_frame".into(),
        confidence: 0.0,
        data: m.clone(),
    };
    pb_adapter::emit_adapter_facts(&adapter_out);
    write_telemetry_record(m.clone());
    push_to_gnn_vector_log(m);

    record(
        now_ts(),
        0,
        0,
        0,
        String::new(),
        vec!["process".into(), "ebpf_unknown_frame".into()],
    )
}

/// proc_lifecycle.bpf.c: sched_process_fork, sched_process_exec and sched_process_exit.
/// Fork runs in the parent, so pid is the process that forked. The C side does not
/// record the child pid.
pub(crate) fn decode_proc_lifecycle(p: &ProcLifecycleEvent, now_secs: u64) -> Option<Decoded> {
    let (tag0, signal) = match p.type_ {
        abi::PROC_LC_EXEC => ("exec", "process::exec"),
        abi::PROC_LC_FORK => ("proc_fork", "process::fork"),
        abi::PROC_LC_EXIT => ("proc_exit", "process::exit"),
        _ => return None,
    };
    let comm = abi::c_string(&p.comm);

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("pid".into(), p.tgid.to_string());
    data.insert("ppid".into(), p.ppid.to_string());
    data.insert("uid".into(), p.uid.to_string());
    data.insert("comm".into(), comm.clone());
    data.insert("ts_ns".into(), p.ts.to_string());
    data.insert("evt_type".into(), p.type_.to_string());
    data.insert("source".into(), "proc_lifecycle".into());
    // What the kernel copied at exec. pb_adapter and on_edr_event use these only when
    // /proc no longer has the process.
    if let Some(f) = p.exec_filename() {
        data.insert("exec_filename".into(), f);
    }
    if let Some(a) = p.exec_argv() {
        data.insert("exec_argv".into(), a);
        data.insert("exec_argv_total".into(), p.argv_total.to_string());
    }
    if p.type_ == abi::PROC_LC_EXIT {
        // task->exit_code packs the exit status in bits 8-15 and the signal in bits 0-6.
        data.insert("exit_code".into(), p.aux_u32.to_string());
        data.insert("exit_status".into(), ((p.aux_u32 >> 8) & 0xff).to_string());
        data.insert("exit_signal".into(), (p.aux_u32 & 0x7f).to_string());
    }

    let tags = vec![
        tag0.to_string(),
        format!("comm:{}", sanitize_tag_value(&comm)),
        signal.to_string(),
    ];

    Some(Decoded {
        out: TelemetryOutput {
            category: "process".into(),
            signal: signal.into(),
            confidence: 0.95,
            data,
        },
        tag0: tag0.into(),
        record: record(event_secs(p.ts, now_secs), p.tgid, p.ppid, p.uid, String::new(), tags),
    })
}

/// ---------- map-specific decoders ----------
/// main.rs calls these by source map, so each one knows exactly which struct to expect.

/// net_events (ringbuf): struct net_evt from net_flow.bpf.c.
pub fn on_edr_event_net_only(
    bytes: &[u8],
    writer: &Arc<Mutex<TelemetryWriter>>,
) -> Option<TelemetryRecord> {
    let d = decode_net(bytes, now_ts())?;
    sink(&d);

    // Only a connect asks for a connection snapshot, and net_watch rate-limits those.
    if d.out.signal == "network::connect" {
        net_watch::request_connection_snapshot(writer);
    }

    let mut rec = d.record;
    populate_proc_fields(&mut rec);
    Some(rec)
}

pub(crate) fn decode_net(bytes: &[u8], now_secs: u64) -> Option<Decoded> {
    let Some(e) = abi::from_ring_frame::<NetEvt>(bytes) else {
        warn_drift("net_events", bytes.len(), "want 48B struct net_evt");
        return None;
    };

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("pid".into(), e.pid.to_string());
    data.insert("kind".into(), e.kind.to_string());
    data.insert("source".into(), "net_flow".into());

    let (tag0, signal, mut extra_tags) = match e.kind {
        abi::NET_KIND_CONNECT => {
            let ip = e.dst_ip().to_string();
            let port = e.dst_port();
            data.insert("family".into(), e.family.to_string());
            data.insert("is_v6".into(), e.is_v6.to_string());
            data.insert("dst_addr".into(), ip.clone());
            data.insert("dst_port".into(), port.to_string());
            (
                "net_connect",
                "network::connect",
                vec![
                    format!("dst_ip:{ip}"),
                    format!("dst_port:{port}"),
                    format!("fam:{}", e.family),
                ],
            )
        }
        abi::NET_KIND_TX | abi::NET_KIND_RX => {
            data.insert("bytes".into(), e.bytes.to_string());
            let (t, s) = if e.kind == abi::NET_KIND_TX {
                ("net_send", "network::tx_bytes")
            } else {
                ("net_recv", "network::rx_bytes")
            };
            (t, s, vec![format!("bytes:{}", e.bytes)])
        }
        other => {
            warn_drift("net_events", bytes.len(), &format!("unknown kind {other}"));
            return None;
        }
    };

    let mut tags = vec![tag0.to_string(), signal.to_string()];
    tags.append(&mut extra_tags);

    Some(Decoded {
        out: TelemetryOutput {
            category: "network".into(),
            signal: signal.into(),
            confidence: 0.90,
            data,
        },
        tag0: tag0.into(),
        record: record(now_secs, e.pid, 0, 0, String::new(), tags),
    })
}

/// wx_events (ringbuf): struct wx_evt from wx_exec.bpf.c.
pub fn on_edr_event_wx_only(
    bytes: &[u8],
    _writer: &Arc<Mutex<TelemetryWriter>>,
) -> Option<TelemetryRecord> {
    let d = decode_wx(bytes, now_ts())?;
    sink(&d);
    let mut rec = d.record;
    // The DecisionEngine drops a tag signal when the record has no exe, so look it up.
    populate_proc_exe(&mut rec);
    populate_proc_fields(&mut rec);
    Some(rec)
}

pub(crate) fn decode_wx(bytes: &[u8], now_secs: u64) -> Option<Decoded> {
    let Some(e) = abi::from_ring_frame::<WxEvt>(bytes) else {
        warn_drift("wx_events", bytes.len(), "want 48B struct wx_evt");
        return None;
    };

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("pid".into(), e.pid.to_string());
    data.insert("kind".into(), e.kind.to_string());
    data.insert("ts_ns".into(), e.ts_ns.to_string());
    data.insert("source".into(), "wx_exec".into());

    let (tag0, signal, marker, confidence) = match e.kind {
        abi::WX_KIND_W_THEN_X => {
            data.insert("addr".into(), format!("{:#x}", e.addr));
            data.insert("len".into(), e.len.to_string());
            data.insert("prot".into(), format!("{:#x}", e.prot));
            data.insert("prot_flags".into(), e.prot_string());
            // The kernel side fires when a pid maps anything writable and then anything
            // executable within 5 s. The dynamic loader does that for every shared library,
            // so only a single call that asks for W and X together gets RWX_MAP.
            if e.is_rwx() {
                ("rwx_map", "memory::rwx_map", Some("RWX_MAP"), 0.90)
            } else {
                ("w_then_x", "memory::w_then_x", None, 0.30)
            }
        }
        abi::WX_KIND_MEMFD_EXEC => {
            data.insert("memfd_bytes".into(), e.bytes.to_string());
            ("memfd_exec", "memory::memfd_exec", Some("MEMFD_EXEC"), 0.95)
        }
        other => {
            warn_drift("wx_events", bytes.len(), &format!("unknown kind {other}"));
            return None;
        }
    };

    let mut tags = vec![tag0.to_string(), signal.to_string()];
    if let Some(m) = marker {
        tags.push(m.to_string());
    }

    Some(Decoded {
        out: TelemetryOutput {
            category: "memory".into(),
            signal: signal.into(),
            confidence,
            data,
        },
        tag0: tag0.into(),
        record: record(event_secs(e.ts_ns, now_secs), e.pid, 0, 0, String::new(), tags),
    })
}

/// events (PERF_EVENT_ARRAY): struct file_access_event_t from file_access_monitor.bpf.c.
pub fn on_edr_event_file_only(
    bytes: &[u8],
    _writer: &Arc<Mutex<TelemetryWriter>>,
) -> Option<TelemetryRecord> {
    let d = decode_file(bytes, now_ts())?;
    // Every record written below opens two log files, and file_access_monitor reports
    // those opens too. Without this check the agent feeds on its own writes (59% of file
    // events in a 3-minute live run).
    if drop_own_frame(d.record.pid as u32, &SELF_DROPPED_FILE) {
        return None;
    }
    sink(&d);
    let mut rec = d.record;
    populate_proc_fields(&mut rec);
    Some(rec)
}

pub(crate) fn decode_file(bytes: &[u8], now_secs: u64) -> Option<Decoded> {
    let Some(e) = abi::from_perf_sample::<FileAccessEvent>(bytes) else {
        warn_drift(
            "events",
            bytes.len(),
            "want 328B struct file_access_event_t plus perf padding",
        );
        return None;
    };
    let comm = abi::c_string(&e.comm);
    let path = abi::c_string(&e.filename);

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("pid".into(), e.pid.to_string());
    data.insert("uid".into(), e.uid.to_string());
    data.insert("comm".into(), comm.clone());
    data.insert("path".into(), path.clone());
    data.insert("source".into(), "file_access_monitor".into());

    let tags = vec![
        "file_access".to_string(),
        "file_open".to_string(),
        format!("comm:{}", sanitize_tag_value(&comm)),
    ];

    Some(Decoded {
        out: TelemetryOutput {
            category: "file".into(),
            signal: "file_access".into(),
            confidence: 0.85,
            data,
        },
        tag0: "file_access".into(),
        // Same convention as the 376-byte open path: binary_path holds the opened file.
        record: record(now_secs, e.pid, 0, e.uid, path, tags),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ebpf::abi::test_bytes::*;

    const NOW: u64 = 1_700_000_000;

    fn has(d: &Decoded, tag: &str) -> bool {
        d.record.tags.iter().any(|t| t == tag)
    }

    #[test]
    fn net_connect_decodes_ip_port_and_tags() {
        let b = net_connect_v4(4242, [93, 184, 216, 34], 8080);
        let d = decode_net(&b, NOW).unwrap();
        assert_eq!(d.out.signal, "network::connect");
        assert_eq!(d.out.data["pid"], "4242");
        assert_eq!(d.out.data["dst_addr"], "93.184.216.34");
        assert_eq!(d.out.data["dst_port"], "8080");
        assert_eq!(d.out.data["family"], "2");
        assert_eq!(d.record.pid, 4242);
        assert_eq!(d.record.timestamp, NOW);
        assert!(has(&d, "net_connect"));
        assert!(has(&d, "dst_ip:93.184.216.34"));
        assert!(has(&d, "dst_port:8080"));
    }

    #[test]
    fn net_tx_rx_decode_byte_counts() {
        let d = decode_net(&net_bytes(77, 2, 5000), NOW).unwrap();
        assert_eq!(d.out.signal, "network::tx_bytes");
        assert_eq!(d.out.data["bytes"], "5000");
        assert!(has(&d, "net_send"));

        let d = decode_net(&net_bytes(77, 3, 123), NOW).unwrap();
        assert_eq!(d.out.signal, "network::rx_bytes");
        assert_eq!(d.out.data["bytes"], "123");

        // unknown kind and wrong size are dropped, not guessed
        assert!(decode_net(&net_bytes(77, 9, 1), NOW).is_none());
        assert!(decode_net(&vec![0u8; 376], NOW).is_none());
    }

    #[test]
    fn wx_memfd_exec_emits_memfd_exec_tag() {
        let b = wx(555, 2, 42, 0, 0, 0, 8192);
        let d = decode_wx(&b, NOW).unwrap();
        assert_eq!(d.out.signal, "memory::memfd_exec");
        assert_eq!(d.out.data["pid"], "555");
        assert_eq!(d.out.data["memfd_bytes"], "8192");
        assert_eq!(d.out.data["ts_ns"], "42");
        assert_eq!(d.record.pid, 555);
        assert!(has(&d, "MEMFD_EXEC"));
        assert!(!has(&d, "RWX_MAP"));
    }

    #[test]
    fn memfd_record_reaches_decision_engine_as_rules_signal() {
        use crate::decision::{AllowSuppressList, DecisionEngine, DecisionPolicy, DetectorKind};

        let mut rec = decode_wx(&wx(555, 2, 42, 0, 0, 0, 8192), NOW).unwrap().record;
        // populate_proc_exe fills this from /proc/<pid>/exe in the live path
        rec.binary_path = "/memfd:edr_probe (deleted)".into();

        let sig = crate::signal_from_tag(&rec, "MEMFD_EXEC").expect("MEMFD_EXEC maps to a signal");
        assert_eq!(sig.detector, DetectorKind::Rules);
        assert_eq!(sig.score, 1.0);
        assert_eq!(sig.pid, Some(555));

        // Default policy: one rules signal gives support 1.0, under min_support 1.2,
        // so MEMFD_EXEC alone opens no incident. A second detector on the same exe does.
        let mut eng = DecisionEngine::new(DecisionPolicy::default(), AllowSuppressList::default());
        assert!(eng.ingest(sig.clone()).is_empty());

        let mut outlier = sig.clone();
        outlier.detector = DetectorKind::Mahala;
        outlier.score = 0.7;
        outlier.tags = vec!["mahalanobis_outlier".into()];
        let alerts = eng.ingest(outlier);
        assert_eq!(alerts.len(), 1);
        assert!(alerts[0].tags.iter().any(|t| t == "MEMFD_EXEC"));
    }

    #[test]
    fn wx_w_then_x_tags_rwx_only_for_rwx_prot() {
        // mmap(PROT_READ|PROT_WRITE|PROT_EXEC)
        let d = decode_wx(&wx(600, 1, 1, 0x4000_0000, 4096, 0x7, 0), NOW).unwrap();
        assert_eq!(d.out.signal, "memory::rwx_map");
        assert_eq!(d.out.data["prot"], "0x7");
        assert_eq!(d.out.data["prot_flags"], "rwx");
        assert_eq!(d.out.data["addr"], "0x40000000");
        assert!(has(&d, "RWX_MAP"));

        // PROT_READ|PROT_EXEC after an earlier writable map, the loader pattern
        let d = decode_wx(&wx(600, 1, 1, 0, 4096, 0x5, 0), NOW).unwrap();
        assert_eq!(d.out.signal, "memory::w_then_x");
        assert!(!has(&d, "RWX_MAP"));
        assert!(!has(&d, "MEMFD_EXEC"));

        assert!(decode_wx(&wx(600, 3, 1, 0, 0, 0, 0), NOW).is_none());
    }

    #[test]
    fn file_access_decodes_perf_sample() {
        let b = file_access(31, 1000, "cat", "/etc/passwd");
        let d = decode_file(&b, NOW).unwrap();
        assert_eq!(d.out.signal, "file_access");
        assert_eq!(d.out.data["pid"], "31");
        assert_eq!(d.out.data["uid"], "1000");
        assert_eq!(d.out.data["comm"], "cat");
        assert_eq!(d.out.data["path"], "/etc/passwd");
        assert_eq!(d.record.binary_path, "/etc/passwd");
        assert_eq!(d.record.uid, 1000);
        assert!(has(&d, "comm:cat"));
        assert!(has(&d, "file_open"));
    }

    #[test]
    fn exec_decode_keeps_the_kernel_filename_and_argv() {
        let b = proc_lc_exec(4321, "true", "/usr/bin/mysql", &["mysql", "-e", "DROP TABLE t1"]);
        let SharedFrame::ProcLifecycle(p) = abi::classify_shared_frame(&b) else {
            panic!("not classified as proc_lifecycle");
        };
        let d = decode_proc_lifecycle(&p, NOW).unwrap();
        assert_eq!(d.out.signal, "process::exec");
        assert_eq!(d.out.data["exec_filename"], "/usr/bin/mysql");
        assert_eq!(d.out.data["exec_argv"], "mysql -e DROP TABLE t1");
        assert_eq!(d.out.data["exec_argv_total"], "23");
        // fork frames carry neither
        let d = proc_decoded(1002, 0);
        assert!(!d.out.data.contains_key("exec_filename"));
        assert!(!d.out.data.contains_key("exec_argv"));
    }

    #[test]
    fn renameat_frame_becomes_a_rename_fact() {
        use crate::ebpf::ebpf_bridge::edr_event_to_output;
        use forensic_hooks::pb_adapter::{adapter_facts_with, ProcInfo};
        use forensic_hooks::pb_wiring::{FixedMounts, Mapper};
        // 376-byte struct edr_event as raw_syscalls_catcher fills it for renameat (38),
        // the syscall glibc rename() uses. Offsets are the C ones.
        let mut b = vec![0u8; 376];
        put_u32(&mut b, 8, 11); // EVT_RENAME
        put_u32(&mut b, 12, 38); // renameat
        put_u32(&mut b, 16, 4242); // tgid
        put_i32(&mut b, 28, -1); // fd
        put_str(&mut b, 104, "/srv/share/a.txt"); // path
        put_str(&mut b, 232, "/srv/share/a.txt.enc"); // path2
        put_str(&mut b, 360, "python3"); // comm
        let SharedFrame::EdrEvent(e) = abi::classify_shared_frame(&b) else {
            panic!("376-byte frame should be the raw struct");
        };
        let out = edr_event_to_output(&e);
        assert_eq!(out.signal, "file_rename");
        assert_eq!(out.data["syscall_id"], "38");
        assert_eq!(out.data["path2"], "/srv/share/a.txt.enc");

        let read = |_pid: i64| ProcInfo::default();
        let mut m = Mapper::with_mounts(Box::new(FixedMounts(vec![("/".into(), "ext4".into())])));
        let facts: Vec<_> = adapter_facts_with(&out, NOW, &read)
            .iter()
            .flat_map(|af| m.facts(af))
            .collect();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].fields["op"], "rename");
        assert_eq!(facts[0].fields["path"], "/srv/share/a.txt");
        assert_eq!(facts[0].fields["ext_new"], ".enc");
    }

    #[test]
    fn exec_record_falls_back_to_the_kernel_copy() {
        let b = proc_lc_exec(4321, "chattr", "/usr/bin/chattr", &["chattr", "+i", "/home/x"]);
        let SharedFrame::ProcLifecycle(p) = abi::classify_shared_frame(&b) else { panic!() };
        // /proc had nothing: the record takes the kernel copy
        let mut rec = decode_proc_lifecycle(&p, NOW).unwrap().record;
        fill_exec_from_kernel_copy(&mut rec, &p);
        assert_eq!(rec.binary_path, "/usr/bin/chattr");
        assert_eq!(rec.command_line, "chattr +i /home/x");
        // /proc answered first: its values stay
        let mut rec = decode_proc_lifecycle(&p, NOW).unwrap().record;
        rec.binary_path = "/usr/bin/chattr.real".into();
        rec.command_line = "chattr +i /home/x /srv/y".into();
        fill_exec_from_kernel_copy(&mut rec, &p);
        assert_eq!(rec.binary_path, "/usr/bin/chattr.real");
        assert_eq!(rec.command_line, "chattr +i /home/x /srv/y");
    }

    #[test]
    fn agent_frames_are_dropped_and_counted() {
        let me = std::process::id();
        let n = AtomicU64::new(0);
        assert!(drop_own_frame(me, &n));
        assert!(!drop_own_frame(me + 1, &n));
        assert!(!drop_own_frame(0, &n), "pid 0 is the idle task, never the agent");
        assert_eq!(n.load(Ordering::Relaxed), 1);
    }

    fn proc_decoded(ty: u32, aux: u32) -> Decoded {
        let b = proc_lc(ty, 1234, 1000, 0, "sh", aux);
        match abi::classify_shared_frame(&b) {
            SharedFrame::ProcLifecycle(p) => decode_proc_lifecycle(&p, NOW).unwrap(),
            _ => panic!("not classified as proc_lifecycle"),
        }
    }

    #[test]
    fn proc_fork_exec_exit_decode_with_right_signal() {
        let d = proc_decoded(1002, 0);
        assert_eq!(d.out.signal, "process::fork");
        assert_eq!((d.record.pid, d.record.ppid), (1234, 1000));
        assert!(has(&d, "proc_fork"));
        assert!(has(&d, "comm:sh"));

        let d = proc_decoded(1001, 0);
        assert_eq!(d.out.signal, "process::exec");
        assert_eq!(d.out.data["comm"], "sh");
        assert!(has(&d, "exec"));
        assert!(has(&d, "process::exec"));

        // exit(3) shows up in task->exit_code as 3 << 8
        let d = proc_decoded(1003, 3 << 8);
        assert_eq!(d.out.signal, "process::exit");
        assert_eq!(d.out.data["exit_code"], "768");
        assert_eq!(d.out.data["exit_status"], "3");
        assert_eq!(d.out.data["exit_signal"], "0");
        assert!(has(&d, "proc_exit"));

        // killed by SIGKILL
        let d = proc_decoded(1003, 9);
        assert_eq!(d.out.data["exit_signal"], "9");
    }
}
