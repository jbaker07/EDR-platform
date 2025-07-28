use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SyscallType {
    Execve,
    Open,
    Connect,
    Mmap,
    Mprotect,
    // Extend with more syscall types as needed
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MemoryAnomalyType {
    RWXMapping,
    PostWriteExec,
    HollowingDetected,
    ReflectiveInjection,
    AnonymousExec,
    SuspiciousIPC,
    DllInjection,
    IpcAbuse,
    IPCAbuse,
    NullBaseExec,
    HighDirtyRSS,
    FileTampering,
    ProcHollowing,
    CodeInjection,
    KernelExploitFallout,
    HighEntropyRegion,
    HighEntropy,
    NetworkAnomaly,
    DnsTunnel,
    SuspiciousActivity,
    ProcessInjection,    // Optional catch-all
    CredDump,               // Optional: included here to keep the original intent
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelemetryOutput {
    pub category: String,         // e.g. "network", "process", etc.
    pub signal: String,           // e.g. "dns_tunnel", "suspicious_port"
    pub confidence: f32,          // 0.0 to 1.0 trust signal
    pub data: std::collections::HashMap<String, String>, // flexible key-value fields
}

use std::fmt;

impl fmt::Display for MemoryAnomalyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[repr(C)]
#[derive(Debug, Clone)]
pub struct CredDumpEvent {
    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,
    pub binary_path: [u8; 256],
    pub command_line: [u8; 512],
}


// Optional enum if you still want to use it
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyType {
    NetworkAnomaly,
    DnsTunnel,
    ProcessInjection,
    // Add others as needed
}
/// Core enum for all telemetry variants.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    // Extend here with:
    // NetworkEvent, FileAccess, AuthEvent, ContainerEvent, etc.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    pub id: String,
    pub timestamp: u64,
    pub events: Vec<TelemetryEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForensicTelemetry {
    pub hostname: String,
    pub collected_at: i64,
    pub events: Vec<TelemetryEvent>,
}

/// Post-processed event for scoring, tagging, and display
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
    pub trust_score: Option<f32>,
    pub semantic_tags: Vec<String>,
}
