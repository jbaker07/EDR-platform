use std::collections::HashMap;
use crate::semantic::tagging::generate_tags;
use crate::semantic::ontology_loader::get_ontology_entry;

#[derive(Debug, Clone)]
pub struct TelemetryEvent {
    pub pid: i32,
    pub ppid: i32,
    pub cmdline: String,
    pub syscall: String,
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
    pub semantics: HashMap<String, String>
}

pub fn tag_telemetry_event(mut event: TelemetryEvent) -> TelemetryEvent {
    let tags = generate_tags(&event.syscall, &event.cmdline, &event.metadata);

    let mut semantics = HashMap::new();
    for tag in &tags {
        if let Some(entry) = get_ontology_entry(tag) {
            semantics.insert(format!("{}_desc", tag), entry.description);
            semantics.insert(format!("{}_cat", tag), entry.category);
        }
    }

    event.tags = tags;
    event.semantics = semantics;
    event
}
