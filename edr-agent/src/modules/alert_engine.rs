use chrono::Utc;
use crate::services::ontology_mapper::SemanticTagInfo;

pub fn send_alert(
    session_id: &str,
    trust_score: f64,
    enriched_tags: Vec<SemanticTagInfo>,
    reason: String,
) {
    let now = Utc::now().to_rfc3339();
    let tag_summary: Vec<_> = enriched_tags
        .iter()
        .map(|t| format!("{} ({}): {}", t.tag, t.category, t.description))
        .collect();

    println!("[ALERT] [{}] Session {} scored {:.2}\nReason: {}\nTags: {:?}",
        now, session_id, trust_score, reason, tag_summary
    );

    // TODO: forward to API, SOC dashboard, file, or syslog
}
