use std::collections::HashMap;
use crate::services::trust_config::TrustConfig;
use crate::replay_writer;
use crate::modules::alert_engine;
use crate::services::ontology_mapper;
use crate::logger; 

#[derive(Debug)]
pub enum EscalationAction {
    None,
    TriggerReplay,
    RaiseAlert,
    IsolateEndpoint,
}

pub fn evaluate_escalation(
    trust_score: f64,
    session_id: &str,
    process_id: u32,
    tags: &[String],
    config: &TrustConfig,
    reason: &str,
) -> EscalationAction {
    if trust_score <= config.trust_floor {
        logger::log(format!(
            "[ESCALATION] Trust score critically low ({:.2}) – Isolation triggered",
            trust_score
        ));
        isolate_endpoint(session_id);
        return EscalationAction::IsolateEndpoint;
    } else if trust_score <= config.escalation_threshold {
        logger::log(format!(
            "[ESCALATION] Trust score {:.2} below escalation threshold – Replay + Alert triggered",
            trust_score
        ));
        trigger_replay(session_id, process_id, tags, trust_score, reason);
        raise_alert(session_id, trust_score, tags, reason);
        return EscalationAction::RaiseAlert;
    } else if trust_score <= config.forensic_trigger_score {
        logger::log(format!(
            "[ESCALATION] Trust score {:.2} – Forensic mode suggested",
            trust_score
        ));
        // Optional: Add forensic trigger signal
        return EscalationAction::TriggerReplay;
    }

    EscalationAction::None
}

fn trigger_replay(
    session_id: &str,
    process_id: u32,
    tags: &[String],
    trust_score: f64,
    reason: &str,
) {
    let ontology_context = ontology_mapper::map_tags(tags.to_vec());
    replay_writer::queue_replay(session_id, process_id, trust_score, reason, ontology_context);
}

fn raise_alert(
    session_id: &str,
    trust_score: f64,
    tags: &[String],
    reason: &str,
) {
    let enriched_tags = ontology_mapper::map_tags(tags.to_vec());
    alert_engine::send_alert(session_id, trust_score, enriched_tags, reason.to_string());
}

fn isolate_endpoint(session_id: &str) {
    // Optional hard block or isolation logic
    logger::log(format!(
        "[ISOLATION] Endpoint session {} flagged for network/process quarantine",
        session_id
    ));
    // TODO: Call endpoint isolation controller if implemented
}
