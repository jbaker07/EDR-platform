use std::collections::HashMap;

/// Enum representing the type of label or node for the GNN
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GNNLabel {
    Benign,
    Malicious,
    Suspicious,
    Unknown,
    InsiderThreat,
    LateralMovement,
    PrivilegeEscalation,
    DataExfiltration,
    CredentialAccess,
    DefenseEvasion,
    InitialAccess,
    Persistence,
    Impact,
    Reconnaissance,
}

/// Maps semantic tags or pattern categories into GNN labels
pub fn map_tags_to_gnn_label(tags: &[String]) -> GNNLabel {
    let lower_tags: Vec<String> = tags.iter().map(|t| t.to_lowercase()).collect();

    if lower_tags.contains(&"exfiltration".to_string()) || lower_tags.contains(&"data_exfil".to_string()) {
        return GNNLabel::DataExfiltration;
    }
    if lower_tags.contains(&"insider".to_string()) {
        return GNNLabel::InsiderThreat;
    }
    if lower_tags.contains(&"lateral".to_string()) {
        return GNNLabel::LateralMovement;
    }
    if lower_tags.contains(&"evasion".to_string()) || lower_tags.contains(&"defense_bypass".to_string()) {
        return GNNLabel::DefenseEvasion;
    }
    if lower_tags.contains(&"creds".to_string()) || lower_tags.contains(&"dumpcreds".to_string()) {
        return GNNLabel::CredentialAccess;
    }
    if lower_tags.contains(&"initial".to_string()) || lower_tags.contains(&"entry".to_string()) {
        return GNNLabel::InitialAccess;
    }
    if lower_tags.contains(&"persist".to_string()) {
        return GNNLabel::Persistence;
    }
    if lower_tags.contains(&"impact".to_string()) {
        return GNNLabel::Impact;
    }
    if lower_tags.contains(&"recon".to_string()) {
        return GNNLabel::Reconnaissance;
    }
    if lower_tags.contains(&"escalate".to_string()) {
        return GNNLabel::PrivilegeEscalation;
    }

    if lower_tags.contains(&"malicious".to_string()) {
        return GNNLabel::Malicious;
    }
    if lower_tags.contains(&"suspicious".to_string()) {
        return GNNLabel::Suspicious;
    }
    if lower_tags.contains(&"benign".to_string()) {
        return GNNLabel::Benign;
    }

    GNNLabel::Unknown
}

/// Optional: For numerical class labels if needed by GNN model (e.g., torch_geometric)
pub fn label_to_index(label: &GNNLabel) -> usize {
    match label {
        GNNLabel::Benign => 0,
        GNNLabel::Suspicious => 1,
        GNNLabel::Malicious => 2,
        GNNLabel::InsiderThreat => 3,
        GNNLabel::LateralMovement => 4,
        GNNLabel::PrivilegeEscalation => 5,
        GNNLabel::DataExfiltration => 6,
        GNNLabel::CredentialAccess => 7,
        GNNLabel::DefenseEvasion => 8,
        GNNLabel::InitialAccess => 9,
        GNNLabel::Persistence => 10,
        GNNLabel::Impact => 11,
        GNNLabel::Reconnaissance => 12,
        GNNLabel::Unknown => 99,
    }
}
