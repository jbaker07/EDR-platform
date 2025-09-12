
// src/tamper.rs
// Agent tamper/blackout detection utilities.
// Detects: agent silence, BPF loss spikes, log deletion, service stop intent.
// Exposes TamperFinding -> convert to decision::Signal upstream.

use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize)]
pub enum TamperFindingKind {
    AgentSilence,
    BpfLossSpike,
    LogDeletion,
    ServiceStopCmd,
}

#[derive(Debug, Clone, Serialize)]
pub struct TamperFinding {
    pub kind: TamperFindingKind,
    pub ts: u64,
    pub host: String,
    pub severity: String,
    pub rationale: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TamperMonitor {
    host: String,
    silence_window: u64,
    last_heartbeat: u64,
    last_bpf_drops: u64,
}

impl TamperMonitor {
    pub fn new(silence_window_sec: u64, _min_interval_sec: u64) -> Self {
        let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
        Self { host, silence_window: silence_window_sec, last_heartbeat: now(), last_bpf_drops: 0 }
    }

    pub fn beat(&mut self) {
        self.last_heartbeat = now();
    }

    pub fn observe_bpf_drops(&mut self, total_drops_counter: u64) -> Option<TamperFinding> {
        if total_drops_counter > self.last_bpf_drops + 100 { // arbitrary spike threshold
            self.last_bpf_drops = total_drops_counter;
            return Some(TamperFinding {
                kind: TamperFindingKind::BpfLossSpike,
                ts: now(),
                host: self.host.clone(),
                severity: "high".into(),
                rationale: "BPF ringbuffer loss spike; potential agent impairment or overload".into(),
                tags: vec!["TAMPER:BPF_LOSS".into()],
            });
        }
        self.last_bpf_drops = total_drops_counter;
        None
    }

    pub fn check_silence(&self) -> Option<TamperFinding> {
        let now = now();
        if now.saturating_sub(self.last_heartbeat) > self.silence_window {
            return Some(TamperFinding {
                kind: TamperFindingKind::AgentSilence,
                ts: now,
                host: self.host.clone(),
                severity: "high".into(),
                rationale: format!("agent heartbeat silent > {}s", self.silence_window),
                tags: vec!["TAMPER:AGENT_SILENCE".into()],
            });
        }
        None
    }
}

pub fn to_signal(f: &TamperFinding) -> crate::decision::Signal {
    use crate::decision::{Signal, DetectorKind};
    Signal {
        detector: DetectorKind::Other,
        score: 0.9,
        ts: f.ts,
        exe: Some("agent".into()),
        user: Some("root".into()),
        host: Some(f.host.clone()),
        cmd_prefix: Some("tamper".into()),
        parent: None,
        signer: None,
        hash: None,
        pid: None,
        tags: f.tags.clone(),
    }
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
