use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt;

/// Canonical syscall set (extend as needed).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SyscallType {
    Execve,
    Open,
    Connect,
    Mmap,
    Mprotect,
    // Extend with more syscall types as needed
}

/// Container exec metadata (now serializable for logs/persist).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerExecEvent {
    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,
    pub gid: u32,
    pub comm: String,
    pub cmdline: String,
    pub timestamp: u64,
    pub container_type: String,
}

/// Memory anomalies — kept backward compatible (both `IpcAbuse` and `IPCAbuse`
/// are present so existing call sites compile). Serde uses snake_case so JSON is
/// stable regardless of Rust casing.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryAnomalyType {
    RWXMapping,
    PostWriteExec,
    HollowingDetected,
    ReflectiveInjection,
    AnonymousExec,
    SuspiciousIPC,       // kept for compatibility
    DllInjection,
    IpcAbuse,            // preferred canonical (snake_case -> "ipc_abuse")
    IPCAbuse,            // legacy/casing variant used elsewhere
    NullBaseExec,
    HighDirtyRSS,
    FileTampering,
    ProcHollowing,
    CodeInjection,
    KernelExploitFallout,
    HighEntropyRegion,
    HighEntropy,         // alias-y duplicate kept for compatibility
    NetworkAnomaly,
    DnsTunnel,
    SuspiciousActivity,
    ProcessInjection,    // catch-all
    CredDump,
}

impl fmt::Display for MemoryAnomalyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Compact public telemetry surface used by most modules.
/// `confidence` historically appears as either 0..1 or 0..100 in callers.
/// We keep the raw f32 for compatibility and provide helpers to set/normalize.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelemetryOutput {
    pub category: String,         // e.g., "network", "process"
    pub signal: String,           // e.g., "dns_tunnel", "suspicious_port"
    pub confidence: f32,          // tolerant: 0..1 or 0..100 from legacy callers
    pub data: HashMap<String, String>, // flexible key-value fields
}

impl TelemetryOutput {
    /// Convenience constructor with empty data and confidence=0.0.
    pub fn new(category: impl Into<String>, signal: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            signal: signal.into(),
            confidence: 0.0,
            data: HashMap::new(),
        }
    }

    /// Set confidence with tolerance for 0..1 or 0..100. Values >1.0 are
    /// interpreted as percentages and normalized to 0..1.
    pub fn with_confidence(mut self, c: f32) -> Self {
        self.confidence = if c > 1.0 { (c / 100.0).clamp(0.0, 1.0) } else { c.clamp(0.0, 1.0) };
        self
    }

    /// Set confidence as a percentage (0..100).
    pub fn with_confidence_pct(mut self, pct: f32) -> Self {
        self.confidence = (pct / 100.0).clamp(0.0, 1.0);
        self
    }

    /// Add a single data key/value.
    pub fn with_kv(mut self, k: impl Into<String>, v: impl ToString) -> Self {
        self.data.insert(k.into(), v.to_string());
        self
    }

    /// Extend data with an iterator of (k, v).
    pub fn with_data<I, K, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: ToString,
    {
        for (k, v) in iter {
            self.data.insert(k.into(), v.to_string());
        }
        self
    }
}

/// Packed cred-dump event used by some eBPF paths.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CredDumpEvent {
    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,
    pub binary_path: [u8; 256],
    pub command_line: [u8; 512],
}

/// Optional coarse anomaly enum for higher-level routing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnomalyType {
    NetworkAnomaly,
    DnsTunnel,
    ProcessInjection,
    // Add others as needed
}

/// Core enum for all telemetry variants.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TelemetryEvent {
    Syscall {
        pid: u32,
        ppid: u32,
        syscall: SyscallType,
        path: String,
        args: Vec<String>,
        timestamp: i64,
    },
    MemoryAnomaly {
        pid: u32,
        ppid: u32,
        uid: u32,
        binary_path: String,
        command_line: String,
        anomaly_type: MemoryAnomalyType,
        description: String,
        timestamp: i64,
    },
    // Extend here with: NetworkEvent, FileAccess, AuthEvent, ContainerEvent, etc.
}

/// A small collection of events captured at a moment in time.
#[derive(Debug, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    pub id: String,
    pub timestamp: u64,
    pub events: Vec<TelemetryEvent>,
}

/// Forensic batch used for offline analysis.
#[derive(Debug, Serialize, Deserialize)]
pub struct ForensicTelemetry {
    pub hostname: String,
    pub collected_at: i64,
    pub events: Vec<TelemetryEvent>,
}

/// Post-processed event for scoring, tagging, and display.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnrichedTelemetry {
    pub event_type: String, // "Syscall", "MemoryAnomaly", etc.
    pub pid: u32,
    pub ppid: Option<u32>,
    pub uid: Option<u32>,
    pub process_name: Option<String>,
    pub binary_path: Option<String>,
    pub command_line: Option<String>,
    pub syscall: Option<String>,
    pub args: Option<Vec<String>>,
    pub description: Option<String>,
    pub anomaly_type: Option<MemoryAnomalyType>,
    pub timestamp: u64,
    pub trust_score: Option<f32>, // typically 0..100
    pub semantic_tags: Vec<String>,
}
