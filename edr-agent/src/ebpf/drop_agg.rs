// src/ebpf/drop_agg.rs
use flume::{Receiver, Sender};
use serde_json::json;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

static TX: OnceLock<Sender<Vec<u8>>> = OnceLock::new();
static RX: OnceLock<Receiver<Vec<u8>>> = OnceLock::new();
static STARTED: OnceLock<()> = OnceLock::new();

static SAMPLES_IN_TOTAL: AtomicU64 = AtomicU64::new(0);
static BYTES_IN_TOTAL: AtomicU64 = AtomicU64::new(0);
static BPF_DROPS_TOTAL: AtomicU64 = AtomicU64::new(0);

// We mutate the map under a mutex; drop notifications are not on the perf hot path.
static PER_CPU_DROPS: OnceLock<Mutex<HashMap<i32, u64>>> = OnceLock::new();

/// Call once during startup.
pub fn init() {
    STARTED.get_or_init(|| {
        let (tx, rx) = flume::bounded::<Vec<u8>>(65_536);
        TX.set(tx).ok();
        RX.set(rx).ok();
        PER_CPU_DROPS.set(Mutex::new(HashMap::new())).ok();

        thread::Builder::new()
            .name("drop-agg".into())
            .spawn(|| {
                let rx = RX.get().expect("drop_agg RX");
                let mut _last_tick = Instant::now();
                loop {
                    match rx.recv_timeout(Duration::from_millis(1)) {
                        Ok(buf) => {
                            SAMPLES_IN_TOTAL.fetch_add(1, Ordering::Relaxed);
                            BYTES_IN_TOTAL.fetch_add(buf.len() as u64, Ordering::Relaxed);
                            // Decoding hook can be added here later if needed.
                        }
                        Err(flume::RecvTimeoutError::Timeout) => { /* back to poll */ }
                        Err(flume::RecvTimeoutError::Disconnected) => break,
                    }
                }
            })
            .expect("spawn drop-agg thread");
    });
}

/// Cloneable sender for hot callbacks. Use try_send() only.
#[inline]
pub fn work_tx() -> Sender<Vec<u8>> {
    TX.get().expect("drop_agg::init() not called").clone()
}

/// Fast path: copy bytes once and enqueue; silently drop if the queue is full.
#[inline]
pub fn send_bytes(bytes: &[u8]) {
    if let Some(tx) = TX.get() {
        // Cap per-sample copy to 64 KiB to avoid pathological allocations.
        let cap = bytes.len().min(64 * 1024);
        let mut v = Vec::with_capacity(cap);
        v.extend_from_slice(&bytes[..cap]);
        let _ = tx.try_send(v);
    }
}

/// Record kernel-reported perf/ring buffer losses.
pub fn lost_event(cpu: i32, cnt: u64) {
    BPF_DROPS_TOTAL.fetch_add(cnt, Ordering::Relaxed);
    if let Some(map) = PER_CPU_DROPS.get() {
        let mut m = map.lock().unwrap();
        *m.entry(cpu).or_insert(0) += cnt;
    }
}

fn queue_len() -> usize {
    RX.get().map(|rx| rx.len()).unwrap_or(0)
}

/// JSON blob to merge into /metrics.
pub fn metrics() -> serde_json::Value {
    let per_cpu = PER_CPU_DROPS
        .get()
        .map(|m| m.lock().unwrap().clone())
        .unwrap_or_default();
    json!({
        "bpf_samples_in_total": SAMPLES_IN_TOTAL.load(Ordering::Relaxed),
        "bpf_bytes_in_total": BYTES_IN_TOTAL.load(Ordering::Relaxed),
        "bpf_drops_total": BPF_DROPS_TOTAL.load(Ordering::Relaxed),
        "bpf_queue_depth": queue_len(),
        "bpf_per_cpu_drops": per_cpu,
    })
}
