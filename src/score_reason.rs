// src/score_reason.rs

use serde::{Serialize, Deserialize};

/// Trust vector domains used to map reasons into dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustDomain {
    Process,
    Auth,
    Network,
    Memory,
    File,
    Geo,
    Usb,
    Script,
    Container,
    Graph,
    Unknown,
    Behavioral,
}

/// Classification of where the signal came from
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasonSource {
    RuleEngine,       // Detection rule triggered
    SemanticTag,      // Ontology/semantic tag match
    BehaviorProfile,  // Trust decay or behavior model
    Heuristic,        // Ad-hoc logic
    ExternalSignal,   // Third-party (e.g., cloud API, IAM, MDM)
    Custom,           // Developer-defined reason
}

/// Main enum for all scoring justifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScoreReason {
    HighCpuUsage,
    SuspiciousProcess,
    UnusualLoginTime,
    MFABypassDetected,
    GeoIPAnomaly,
    TrustDecayExceeded,
    PrivilegeEscalation,
    RootLogon,
    MaliciousUSB,
    SuspiciousScriptDrop,
    LOLBinExecution,

    /// A critical semantic tag escalated the score (e.g., "crypto_miner", "mimikatz")
    CriticalTag(String),

    /// Generic semantic tag
    Tagged(String),

    /// A named rule fired (e.g., "detect_suspicious_persistence")
    RuleTriggered(String),

    /// Freeform custom text
    Custom(String),

    /// Combine multiple reasons into one
    Composite(Vec<ScoreReason>),

    /// NEW: generic outlier from a statistical method (Mahalanobis, EllipticEnvelope, etc.)
    Outlier { method: String, detail: String },

    /// NEW: anomaly arose from a suspicious causal subgraph (KRIM/graph analysis)
    CausalSubgraph,
}

impl ScoreReason {
    /// Convenience constructor used by call sites:
    /// `ScoreReason::outlier("mahalanobis", "χ² gate exceeded".into())`
    pub fn outlier(method: &str, detail: String) -> Self {
        ScoreReason::Outlier {
            method: method.to_string(),
            detail,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ScoreReason::HighCpuUsage => "High CPU usage".into(),
            ScoreReason::SuspiciousProcess => "Suspicious process behavior".into(),
            ScoreReason::UnusualLoginTime => "Unusual login time".into(),
            ScoreReason::MFABypassDetected => "MFA bypass attempt detected".into(),
            ScoreReason::GeoIPAnomaly => "Geo/IP mismatch anomaly".into(),
            ScoreReason::TrustDecayExceeded => "Trust decay threshold exceeded".into(),
            ScoreReason::PrivilegeEscalation => "Privilege escalation attempt".into(),
            ScoreReason::RootLogon => "Root login observed".into(),
            ScoreReason::MaliciousUSB => "Malicious USB activity detected".into(),
            ScoreReason::SuspiciousScriptDrop => "Suspicious script drop in temp directory".into(),
            ScoreReason::LOLBinExecution => "Living-off-the-land binary execution".into(),
            ScoreReason::CriticalTag(tag) => format!("Critical semantic tag triggered: {}", tag),
            ScoreReason::Tagged(tag) => format!("Tagged: {}", tag),
            ScoreReason::RuleTriggered(rule) => format!("Rule triggered: {}", rule),
            ScoreReason::Custom(reason) => format!("Custom reason: {}", reason),
            ScoreReason::Composite(reasons) => {
                let inner = reasons
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Composite reason: [{}]", inner)
            }
            ScoreReason::Outlier { method, detail } => {
                format!("Outlier detected by {}: {}", method, detail)
            }
            ScoreReason::CausalSubgraph => "Causal subgraph anomaly".into(),
        }
    }

    /// Risk score used to weigh trust decay or GNN input
    pub fn risk_weight(&self) -> u32 {
        match self {
            ScoreReason::HighCpuUsage => 10,
            ScoreReason::SuspiciousProcess => 40,
            ScoreReason::UnusualLoginTime => 30,
            ScoreReason::MFABypassDetected => 80,
            ScoreReason::GeoIPAnomaly => 60,
            ScoreReason::TrustDecayExceeded => 50,
            ScoreReason::PrivilegeEscalation => 90,
            ScoreReason::CriticalTag(_) => 100,
            ScoreReason::RootLogon => 70,
            ScoreReason::MaliciousUSB => 75,
            ScoreReason::SuspiciousScriptDrop => 55,
            ScoreReason::LOLBinExecution => 60,
            ScoreReason::Tagged(_) => 25,
            ScoreReason::RuleTriggered(_) => 50,
            ScoreReason::Custom(_) => 20,
            ScoreReason::Composite(inner) => inner.iter().map(|r| r.risk_weight()).max().unwrap_or(0),
            ScoreReason::Outlier { .. } => 70,      // robust statistics flagged an outlier
            ScoreReason::CausalSubgraph => 65,       // graph/KRIM anomaly
        }
    }

    /// Trust vector domain for multi-dimensional scoring
    pub fn domain(&self) -> TrustDomain {
        match self {
            ScoreReason::HighCpuUsage => TrustDomain::Process,
            ScoreReason::SuspiciousProcess => TrustDomain::Process,
            ScoreReason::UnusualLoginTime => TrustDomain::Auth,
            ScoreReason::MFABypassDetected => TrustDomain::Auth,
            ScoreReason::GeoIPAnomaly => TrustDomain::Geo,
            ScoreReason::TrustDecayExceeded => TrustDomain::Graph,
            ScoreReason::PrivilegeEscalation => TrustDomain::Process,
            ScoreReason::RootLogon => TrustDomain::Auth,
            ScoreReason::MaliciousUSB => TrustDomain::Usb,
            ScoreReason::SuspiciousScriptDrop => TrustDomain::Script,
            ScoreReason::CriticalTag(_) => TrustDomain::Behavioral,
            ScoreReason::LOLBinExecution => TrustDomain::Process,
            ScoreReason::Tagged(_) => TrustDomain::Unknown,
            ScoreReason::RuleTriggered(_) => TrustDomain::Unknown,
            ScoreReason::Custom(_) => TrustDomain::Unknown,
            ScoreReason::Composite(_) => TrustDomain::Unknown,
            ScoreReason::Outlier { .. } => TrustDomain::Behavioral,
            ScoreReason::CausalSubgraph => TrustDomain::Graph,
        }
    }

    /// Optional source classification (useful for trust digest or audit)
    pub fn source(&self) -> ReasonSource {
        match self {
            ScoreReason::Tagged(_) => ReasonSource::SemanticTag,
            ScoreReason::RuleTriggered(_) => ReasonSource::RuleEngine,
            ScoreReason::TrustDecayExceeded => ReasonSource::BehaviorProfile,
            ScoreReason::Outlier { .. } => ReasonSource::BehaviorProfile,
            ScoreReason::Custom(_) => ReasonSource::Custom,
            ScoreReason::CausalSubgraph => ReasonSource::Heuristic,
            _ => ReasonSource::Heuristic,
        }
    }
}
