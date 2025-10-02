// src/pcb_emitter.rs
//! PCB (Process Control Block) snapshot emitter.
//!
//! Gathers per-PID snapshot stats (threads, RSS, FD count, context switches) for
//! PIDs referenced by the current Episode's graph. This is a *collector*—it computes
//! robust stats but returns an empty Vec to avoid assuming your FeatureObservation ctor.
//! Wire these metrics into your unified scorer later.
//!
//! Linux-only; fails soft on permission/vanished-process errors.

use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use std::time::Duration;

use crate::baselines::BaselineStore; // use the BaselineStore expected by the trait
use crate::episode::Episode;
use crate::graph_builder::GraphNode;
use crate::traits::FeatureEmitter;

// ----- public emitter --------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PcbEmitter {
    /// Max PIDs to probe (defensive cap).
    pub max_pids: usize,
    /// Timeout-ish backoff between /proc reads to avoid bursts.
    pub tiny_sleep_ms: u64,
}

impl Default for PcbEmitter {
    fn default() -> Self {
        Self {
            max_pids: 1024,
            tiny_sleep_ms: 0,
        }
    }
}

impl PcbEmitter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FeatureEmitter for PcbEmitter {
    fn emit(
        &self,
        ep: &Episode,
        _baselines: &mut BaselineStore,
    ) -> Vec<crate::traits::FeatureObservation> {
        #[cfg(target_os = "linux")]
        {
            let (nodes, _edges) = ep.to_graph();
            let mut pids = collect_pids(&nodes);
            if pids.len() > self.max_pids {
                pids.truncate(self.max_pids);
            }

            let stats = probe_pcb_snapshot(&pids, self.tiny_sleep_ms);

            // TODO: map `stats` into your FeatureObservation type when you're ready.
            // Example (pseudo; depends on your struct):
            // vec![
            //   fo("pcb.total_pids", stats.total_pids as f64, "pcb"),
            //   fo("pcb.alive_sampled", stats.alive_sampled as f64, "pcb"),
            //   fo("pcb.max_threads", stats.max_threads as f64, "pcb"),
            //   fo("pcb.p95_threads", stats.p95_threads as f64, "pcb"),
            //   fo("pcb.max_fd", stats.max_fd as f64, "pcb"),
            //   fo("pcb.p95_fd", stats.p95_fd as f64, "pcb"),
            //   fo("pcb.max_rss_kb", stats.max_rss_kb as f64, "pcb"),
            //   fo("pcb.p95_rss_kb", stats.p95_rss_kb as f64, "pcb"),
            //   fo("pcb.max_ctx_switches", stats.max_ctx_switches as f64, "pcb"),
            //   fo("pcb.p95_ctx_switches", stats.p95_ctx_switches as f64, "pcb"),
            // ]
            let _ = stats; // keep compile-safe until you wire FeatureObservation here
            Vec::new()
        }
        #[cfg(not(target_os = "linux"))]
        {
            // Non-linux: no-op
            Vec::new()
        }
    }
}

// ----- internal: snapshot & aggregation --------------------------------------

#[derive(Debug, Clone, Default)]
struct PcbStats {
    total_pids: usize,
    alive_sampled: usize,

    max_threads: u64,
    p95_threads: u64,

    max_fd: u64,
    p95_fd: u64,

    max_rss_kb: u64,
    p95_rss_kb: u64,

    max_ctx_switches: u64, // voluntary + nonvoluntary (approx)
    p95_ctx_switches: u64,
}

#[cfg(target_os = "linux")]
fn collect_pids(nodes: &[GraphNode]) -> Vec<u32> {
    let mut out = Vec::with_capacity(nodes.len());
    for n in nodes {
        if let Some(pid) = n.pid {
            if pid > 0 {
                out.push(pid);
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

#[cfg(target_os = "linux")]
fn probe_pcb_snapshot(pids: &[u32], tiny_sleep_ms: u64) -> PcbStats {
    let mut threads: Vec<u64> = Vec::new();
    let mut fds: Vec<u64> = Vec::new();
    let mut rss_kb: Vec<u64> = Vec::new();
    let mut ctx: Vec<u64> = Vec::new();

    let mut alive = 0usize;

    for &pid in pids {
        if tiny_sleep_ms > 0 {
            std::thread::sleep(Duration::from_millis(tiny_sleep_ms));
        }

        match read_one_pid(pid) {
            Ok(one) => {
                alive += 1;
                if let Some(v) = one.threads {
                    threads.push(v)
                }
                if let Some(v) = one.fd_count {
                    fds.push(v)
                }
                if let Some(v) = one.rss_kb {
                    rss_kb.push(v)
                }
                if let Some(v) = one.ctx_switch {
                    ctx.push(v)
                }
            }
            Err(_e) => {
                // Process might have exited / permission denied; skip quietly.
            }
        }
    }

    PcbStats {
        total_pids: pids.len(),
        alive_sampled: alive,
        max_threads: max_or_zero(&threads),
        p95_threads: percentile_or_zero(&threads, 95.0),
        max_fd: max_or_zero(&fds),
        p95_fd: percentile_or_zero(&fds, 95.0),
        max_rss_kb: max_or_zero(&rss_kb),
        p95_rss_kb: percentile_or_zero(&rss_kb, 95.0),
        max_ctx_switches: max_or_zero(&ctx),
        p95_ctx_switches: percentile_or_zero(&ctx, 95.0),
    }
}

#[cfg(target_os = "linux")]
#[derive(Debug, Default)]
struct OnePid {
    threads: Option<u64>,
    fd_count: Option<u64>,
    rss_kb: Option<u64>,
    ctx_switch: Option<u64>, // voluntary + nonvoluntary
}

#[cfg(target_os = "linux")]
fn read_one_pid(pid: u32) -> anyhow::Result<OnePid> {
    let mut out = OnePid::default();
    let root = PathBuf::from("/proc").join(pid.to_string());

    // /proc/<pid>/status: Threads, VmRSS, voluntary_ctxt_switches, nonvoluntary_ctxt_switches
    let status = read_to_string(root.join("status"))?;
    let mut vol: u64 = 0;
    let mut nvol: u64 = 0;

    for line in status.lines() {
        if let Some(val) = line.strip_prefix("Threads:") {
            out.threads = parse_last_u64(val);
        } else if let Some(val) = line.strip_prefix("VmRSS:") {
            out.rss_kb = parse_last_u64(val); // already in kB
        } else if let Some(val) = line.strip_prefix("voluntary_ctxt_switches:") {
            vol = parse_last_u64(val).unwrap_or(0);
        } else if let Some(val) = line.strip_prefix("nonvoluntary_ctxt_switches:") {
            nvol = parse_last_u64(val).unwrap_or(0);
        }
    }
    let sum = vol.saturating_add(nvol);
    if sum > 0 {
        out.ctx_switch = Some(sum)
    }

    // /proc/<pid>/fd: count
    if let Ok(rd) = read_dir(root.join("fd")) {
        let mut count = 0u64;
        for _ in rd {
            count = count.saturating_add(1);
        }
        out.fd_count = Some(count);
    }

    Ok(out)
}

#[cfg(target_os = "linux")]
fn parse_last_u64(s: &str) -> Option<u64> {
    // Extract the last numeric token in a line like "VmRSS:\t  1234 kB"
    s.split_whitespace()
        .rev()
        .find_map(|tok| tok.parse::<u64>().ok())
}

#[cfg(target_os = "linux")]
fn max_or_zero(v: &[u64]) -> u64 {
    v.iter().copied().max().unwrap_or(0)
}

#[cfg(target_os = "linux")]
fn percentile_or_zero(v: &[u64], p: f64) -> u64 {
    if v.is_empty() {
        return 0;
    }
    let mut vv = v.to_vec();
    vv.sort_unstable();
    let rank = ((p / 100.0) * (vv.len() as f64 - 1.0)).round() as usize;
    vv.get(rank).copied().unwrap_or(*vv.last().unwrap())
}
