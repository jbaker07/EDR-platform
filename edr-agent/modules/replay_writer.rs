use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Write, BufWriter};
use std::path::{Path, PathBuf};
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;
use flate2::write::GzEncoder;
use flate2::Compression;

#[derive(Debug, Serialize)]
pub struct ReplayEvent {
    pub session_id: String,
    pub hostname: String,
    pub timestamp: i64,
    pub category: String,
    pub data: serde_json::Value,
    pub trust_level: Option<String>,
    pub gnn_label: Option<String>,
    pub mode: Option<String>, // minimal / forensic
}

pub fn persist_replay_event(event: &ReplayEvent, compress: bool) -> std::io::Result<()> {
    let subdir = event.category.to_lowercase();
    let base_dir = PathBuf::from("/var/log/edr/replay").join(subdir);
    create_dir_all(&base_dir)?;

    let filename = format!(
        "{}_{}_{}.json{}",
        event.hostname,
        event.category,
        Uuid::new_v4(),
        if compress { ".gz" } else { "" }
    );

    let tmp_path = base_dir.join(format!("{}.tmp", filename));
    let final_path = base_dir.join(filename);

    let file = File::create(&tmp_path)?;
    if compress {
        let mut encoder = GzEncoder::new(file, Compression::default());
        let json = serde_json::to_string_pretty(event)?;
        encoder.write_all(json.as_bytes())?;
        encoder.finish()?;
    } else {
        let mut writer = BufWriter::new(file);
        let json = serde_json::to_string_pretty(event)?;
        writeln!(writer, "{}", json)?;
    }

    std::fs::rename(tmp_path, final_path)?;
    Ok(())
}

pub fn generate_replay_event<T: Serialize>(
    category: &str,
    hostname: &str,
    payload: &T,
    trust_level: Option<String>,
    gnn_label: Option<String>,
    mode: Option<String>,
) -> Option<ReplayEvent> {
    match serde_json::to_value(payload) {
        Ok(json) => Some(ReplayEvent {
            session_id: Uuid::new_v4().to_string(),
            hostname: hostname.to_string(),
            timestamp: Utc::now().timestamp(),
            category: category.to_string(),
            data: json,
            trust_level,
            gnn_label,
            mode,
        }),
        Err(_) => None,
    }
}
