// src/ebpf/ebpf_ingest.rs

use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Once};

use forensic_hooks::gnn_hook::push_to_gnn_vector_log;
use forensic_hooks::modules::net_watch;
use forensic_hooks::telemetry::TelemetryRecord;
use forensic_hooks::telemetry_types::TelemetryOutput; // ok if unused
use forensic_hooks::telemetry_writer::{write_telemetry_record, TelemetryWriter};
use forensic_hooks::utils::strnorm::{cbytes_to_string_lossy, sanitize_tag_value};

use crate::ebpf::ebpf_bridge::{
    edr_event_to_output, edr_event_to_record, populate_proc_fields, EdREvent,
};
use crate::pb_adapter;

/// Legacy syscall event wire-size we still see in the wild (v0).
const V0_WIRE_SIZE: usize = 312;

/// How often to log mismatch messages after the first few.
const MISMATCH_THROTTLE_N: usize = 1000;

static MISMATCH_WARNED: AtomicUsize = AtomicUsize::new(0);
static DISABLED_ONCE: Once = Once::new();

/// ---------- helpers (LE reads + cstring trim) ----------
#[inline]
fn read_u16_le(b: &[u8], off: usize) -> Option<u16> {
    let s = b.get(off..off + 2)?;
    Some(u16::from_le_bytes(s.try_into().ok()?))
}
#[inline]
fn read_u32_le(b: &[u8], off: usize) -> Option<u32> {
    let s = b.get(off..off + 4)?;
    Some(u32::from_le_bytes(s.try_into().ok()?))
}
#[inline]
fn read_u64_le(b: &[u8], off: usize) -> Option<u64> {
    let s = b.get(off..off + 8)?;
    Some(u64::from_le_bytes(s.try_into().ok()?))
}
#[inline]
fn cstr_from(buf: &[u8], start: usize) -> Option<String> {
    let s = buf.get(start..)?;
    Some(cbytes_to_string_lossy(s))
}
#[inline]
fn trim_cbuf(buf: &[u8]) -> String {
    cbytes_to_string_lossy(buf)
}

/// ---------- NEW: generic counting wrapper (no direct dependency on crate statics) ----------
/// Call this from your ringbuf/perf reader when you want to bump metrics.
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

/// ---------- LEGACY: unified syscall/lifecycle decoder (for edr_events_rb only) ----------
/// Core parser/ingest used by both the legacy path and the counting wrapper.
/// Returns a TelemetryRecord you can batch (optional).
pub fn on_edr_event(bytes: &[u8], writer: &Arc<Mutex<TelemetryWriter>>) -> Option<TelemetryRecord> {
    // Allow hard-disabling this legacy ingest path to avoid double-consumption of the ringbuf.
    if std::env::var("EDR_DISABLE_LEGACY_SYS_INGEST")
        .ok()
        .as_deref()
        == Some("1")
    {
        // Print this once to avoid log spam.
        DISABLED_ONCE.call_once(|| {
            eprintln!("[ebpf/legacy] disabled via EDR_DISABLE_LEGACY_SYS_INGEST=1");
        });
        return None;
    }

    let want = std::mem::size_of::<EdREvent>();
    let got = bytes.len();

    // Guard against obviously malformed records (too small even for a header).
    if got < 16 {
        let n = MISMATCH_WARNED.fetch_add(1, Ordering::Relaxed);
        if n < 5 {
            eprintln!("[ebpf_ingest] dropping tiny record: got {got} (<16)");
        }
        return None;
    }

    match got {
        n if n == want => {
            // Fast path: exact match with host struct
            let mut e: EdREvent = unsafe { std::mem::zeroed() };
            unsafe {
                std::ptr::copy_nonoverlapping(
                    bytes.as_ptr(),
                    &mut e as *mut EdREvent as *mut u8,
                    want,
                );
            }

            // Build TelemetryOutput robustly.
            let out = match std::panic::catch_unwind(|| edr_event_to_output(&e)) {
                Ok(o) => o,
                Err(_) => {
                    let k = MISMATCH_WARNED.fetch_add(1, Ordering::Relaxed);
                    if k < 5 || k % MISMATCH_THROTTLE_N == 0 {
                        eprintln!(
                            "[ebpf_ingest] edr_event_to_output panicked on {}B payload (host {})",
                            got, want
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

            // Optional network watcher tap
            if let Ok(mut w) = writer.lock() {
                if normalized_signal == "network::connect" || out.signal == "net_accept" {
                    let _ = net_watch::log_open_connections(&mut *w);
                }
            }

            // Return a TelemetryRecord for batch scoring/episode.
            std::panic::catch_unwind(|| edr_event_to_record(&e)).ok()
        }

        // Legacy path: accept 312-byte events and emit a minimal process::exec record.
        // IMPORTANT: parse by offsets; don't depend on a local struct size.
        n if n == V0_WIRE_SIZE => {
            // Layout: pid(0..4) ppid(4..8) uid(8..12) pad(12..16) ts(16..24) comm(24..40) filename(40..296) trailing(296..312)
            let pid = read_u32_le(bytes, 0).unwrap_or(0);
            let ppid = read_u32_le(bytes, 4).unwrap_or(0);
            let uid = read_u32_le(bytes, 8).unwrap_or(0);
            let ts = read_u64_le(bytes, 16).unwrap_or(0);
            let comm = cbytes_to_string_lossy(&bytes[24..40]);
            let filename = cbytes_to_string_lossy(&bytes[40..296]);
            let comm_tag = format!("comm:{}", sanitize_tag_value(&comm));

            let mut m: HashMap<String, String> = HashMap::new();
            m.insert("pid".into(), pid.to_string());
            m.insert("ppid".into(), ppid.to_string());
            m.insert("uid".into(), uid.to_string());
            m.insert("ts_ns".into(), ts.to_string());
            m.insert("comm".into(), comm.clone());
            m.insert("filename".into(), filename.clone());
            m.insert("category".into(), "process".into());
            m.insert("signal".into(), "process::exec".into());
            m.insert("confidence".into(), format!("{:.2}", 0.50));
            m.insert("tag0".into(), "process_exec_v0".into());

            let adapter_out = TelemetryOutput {
                category: "process".into(),
                signal: "process::exec".into(),
                confidence: 0.50,
                data: m.clone(),
            };
            pb_adapter::emit_adapter_facts(&adapter_out);

            write_telemetry_record(m.clone());
            push_to_gnn_vector_log(m);

            let mut record = TelemetryRecord {
                timestamp: ts,
                pid: pid as i32,
                ppid: ppid as i32,
                uid,
                binary_path: filename,
                command_line: String::new(),
                cwd: String::new(),
                env_vars: None,
                tags: vec!["process".into(), "process::exec".into(), comm_tag],
                risk_score: None,
            };

            populate_proc_fields(&mut record);

            Some(record)
        }

        other => {
            let n = MISMATCH_WARNED.fetch_add(1, Ordering::Relaxed);
            if n < 5 || n % MISMATCH_THROTTLE_N == 0 {
                eprintln!(
                    "[ebpf_ingest] unknown wire size {} (host {}) — dropping",
                    other, want
                );
            }
            use std::time::{SystemTime, UNIX_EPOCH};
            let ts: u64 = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let mut m: HashMap<String, String> = HashMap::new();
            m.insert("category".into(), "process".into());
            m.insert("signal".into(), "ebpf_unknown_frame".into());
            m.insert("wire_len".into(), other.to_string());
            let adapter_out = TelemetryOutput {
                category: "process".into(),
                signal: "ebpf_unknown_frame".into(),
                confidence: 0.0,
                data: m.clone(),
            };
            pb_adapter::emit_adapter_facts(&adapter_out);
            write_telemetry_record(m.clone());
            push_to_gnn_vector_log(m);

            let mut record = TelemetryRecord {
                timestamp: ts,
                pid: 0,
                ppid: 0,
                uid: 0,
                binary_path: String::new(),
                command_line: String::new(),
                cwd: String::new(),
                env_vars: None,
                tags: vec!["process".into(), "ebpf_unknown_frame".into()],
                risk_score: None,
            };

            populate_proc_fields(&mut record);

            Some(record)
        }
    }
}

/// ---------- NEW: map-specific decoders ----------
/// These are invoked from main.rs based on the source stream so we *expect* different sizes.
/// They emit JSON telemetry for parity with your sinks, *and* return TelemetryRecord
/// instances so `writer.append(rec)` will bump EVENTS_ACCEPTED.

/// net_events (ringbuf): commonly ~48B tcp_state frames.
pub fn on_edr_event_net_only(
    bytes: &[u8],
    writer: &Arc<Mutex<TelemetryWriter>>,
) -> Option<TelemetryRecord> {
    if bytes.len() < 32 {
        return None;
    }

    let ts_ns = read_u64_le(bytes, 0).unwrap_or(0);
    let pid = read_u32_le(bytes, 8).unwrap_or(0);
    let tgid = read_u32_le(bytes, 12).unwrap_or(0);
    let laddr4 = read_u32_le(bytes, 16).unwrap_or(0);
    let lport = read_u16_le(bytes, 20).unwrap_or(0);
    let proto = *bytes.get(22).unwrap_or(&0) as u32; // 6=tcp, 17=udp
    let fam = *bytes.get(23).unwrap_or(&0) as u32; // 2=AF_INET
    let evt_ty = read_u32_le(bytes, 24).unwrap_or(200); // tcp_state
    let flags = read_u32_le(bytes, 28).unwrap_or(0);

    // emit JSON for existing sinks
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert("ts_ns".into(), ts_ns.to_string());
    m.insert("pid".into(), pid.to_string());
    m.insert("tgid".into(), tgid.to_string());
    m.insert("laddr4".into(), laddr4.to_string());
    m.insert("lport".into(), lport.to_string());
    m.insert("proto".into(), proto.to_string());
    m.insert("fam".into(), fam.to_string());
    m.insert("evt_type".into(), evt_ty.to_string());
    m.insert("flags".into(), flags.to_string());
    m.insert("category".into(), "network".into());
    m.insert("signal".into(), "tcp_state".into());
    m.insert("confidence".into(), format!("{:.2}", 0.95));
    m.insert("tag0".into(), "tcp_state".into());

    let adapter_out = TelemetryOutput {
        category: "network".into(),
        signal: "tcp_state".into(),
        confidence: 0.95,
        data: m.clone(),
    };
    pb_adapter::emit_adapter_facts(&adapter_out);

    write_telemetry_record(m.clone());
    push_to_gnn_vector_log(m);

    // optional watcher
    if let Ok(mut w) = writer.lock() {
        let _ = net_watch::log_open_connections(&mut *w);
    }

    // Build TelemetryRecord (best-effort mapping)
    let mut record = TelemetryRecord {
        timestamp: ts_ns,
        pid: pid as i32,
        ppid: 0,
        uid: 0,
        binary_path: String::new(),
        command_line: String::new(),
        cwd: String::new(),
        env_vars: None,
        tags: vec![
            "network".into(),
            "tcp_state".into(),
            format!("proto:{proto}"),
            format!("fam:{fam}"),
        ],
        risk_score: None,
    };

    populate_proc_fields(&mut record);

    Some(record)
}

/// wx_events (ringbuf): write/exec stream; sizes vary, path as trailing C string.
pub fn on_edr_event_wx_only(
    bytes: &[u8],
    _writer: &Arc<Mutex<TelemetryWriter>>,
) -> Option<TelemetryRecord> {
    if bytes.len() < 24 {
        return None;
    }

    let ts_ns = read_u64_le(bytes, 0).unwrap_or(0);
    let pid = read_u32_le(bytes, 8).unwrap_or(0);
    let tgid = read_u32_le(bytes, 12).unwrap_or(0);
    let kind = read_u32_le(bytes, 16).unwrap_or(0); // 1=write, 2=exec (adjust to your enum)
    let flags = read_u32_le(bytes, 20).unwrap_or(0);
    let path = cstr_from(bytes, 24).unwrap_or_default();

    let (category, signal) = if kind == 2 {
        ("exec", "execve")
    } else {
        ("file", "write")
    };

    // emit JSON for existing sinks
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert("ts_ns".into(), ts_ns.to_string());
    m.insert("pid".into(), pid.to_string());
    m.insert("tgid".into(), tgid.to_string());
    m.insert("flags".into(), flags.to_string());
    m.insert("path".into(), path.clone());
    m.insert("category".into(), category.into());
    m.insert("signal".into(), signal.into());
    m.insert("confidence".into(), format!("{:.2}", 0.90));
    m.insert("tag0".into(), signal.into());

    let adapter_out = TelemetryOutput {
        category: category.into(),
        signal: signal.into(),
        confidence: 0.90,
        data: m.clone(),
    };
    pb_adapter::emit_adapter_facts(&adapter_out);

    write_telemetry_record(m.clone());
    push_to_gnn_vector_log(m);

    // Build TelemetryRecord
    let mut record = TelemetryRecord {
        timestamp: ts_ns,
        pid: pid as i32,
        ppid: 0, // unknown without extra fields
        uid: 0,  // unknown here
        binary_path: path,
        command_line: String::new(),
        cwd: String::new(),
        env_vars: None,
        tags: vec![category.into(), signal.into()],
        risk_score: None,
    };

    populate_proc_fields(&mut record);

    Some(record)
}

/// events (PERF_EVENT_ARRAY): file access samples, often ~332/376B with fixed path buffer.
pub fn on_edr_event_file_only(
    bytes: &[u8],
    _writer: &Arc<Mutex<TelemetryWriter>>,
) -> Option<TelemetryRecord> {
    if bytes.len() < 40 {
        return None;
    }

    let ts_ns = read_u64_le(bytes, 0).unwrap_or(0);
    let pid = read_u32_le(bytes, 8).unwrap_or(0);
    let tgid = read_u32_le(bytes, 12).unwrap_or(0);
    let uid = read_u32_le(bytes, 16).unwrap_or(0);
    let evt_ty = read_u32_le(bytes, 20).unwrap_or(300); // file_access
    let ret = read_u32_le(bytes, 24).unwrap_or(0);
    let flags = read_u32_le(bytes, 28).unwrap_or(0);
    let path = cstr_from(bytes, 32).unwrap_or_default();

    // emit JSON for existing sinks
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert("ts_ns".into(), ts_ns.to_string());
    m.insert("pid".into(), pid.to_string());
    m.insert("tgid".into(), tgid.to_string());
    m.insert("uid".into(), uid.to_string());
    m.insert("evt_type".into(), evt_ty.to_string());
    m.insert("ret".into(), (ret as i32).to_string());
    m.insert("flags".into(), flags.to_string());
    m.insert("path".into(), path.clone());
    m.insert("category".into(), "file".into());
    m.insert("signal".into(), "file_access".into());
    m.insert("confidence".into(), format!("{:.2}", 0.85));
    m.insert("tag0".into(), "file_access".into());

    let adapter_out = TelemetryOutput {
        category: "file".into(),
        signal: "file_access".into(),
        confidence: 0.85,
        data: m.clone(),
    };
    pb_adapter::emit_adapter_facts(&adapter_out);

    write_telemetry_record(m.clone());
    push_to_gnn_vector_log(m);

    // Build TelemetryRecord
    let mut record = TelemetryRecord {
        timestamp: ts_ns,
        pid: pid as i32,
        ppid: 0, // not present in this minimal header
        uid,
        binary_path: path,
        command_line: String::new(),
        cwd: String::new(),
        env_vars: None,
        tags: vec![
            "file".into(),
            "file_access".into(),
            format!("flags:{flags}"),
            format!("ret:{ret}"),
        ],
        risk_score: None,
    };

    populate_proc_fields(&mut record);

    Some(record)
}
