// src/yara_layer.rs
// YARA wrapper (optional). Requires libyara + `yara` crate when built with `with_yara` feature.
// Falls back to a no-op if the feature is disabled.

use crate::decision::{Signal, DetectorKind};

#[cfg(feature = "with_yara")]
mod inner {
    use super::*;
    use serde::Deserialize;
    use std::fs;
    use std::path::Path;
    use yara::{Compiler, Rules, Scanner};

    #[derive(Debug, Deserialize)]
    struct YaraConfig {
        /// Directory containing .yar files
        dir: String,
    }

    pub struct YaraLayer {
        rules: Rules,
        #[allow(dead_code)]
        source_dir: String,
    }

    impl YaraLayer {
        pub fn from_dir(dir: &str) -> anyhow::Result<Self> {
            let mut comp = Compiler::new()?;
            if !Path::new(dir).exists() {
                std::fs::create_dir_all(dir).ok();
            }
            if let Ok(rd) = fs::read_dir(dir) {
                for entry in rd.flatten() {
                    let p = entry.path();
                    if p.is_file() && p.extension().and_then(|e| e.to_str()).map(|s| s.eq_ignore_ascii_case("yar") || s.eq_ignore_ascii_case("yara")).unwrap_or(false) {
                        let s = fs::read_to_string(&p)?;
                        comp.add_rules_str(&s)?;
                    }
                }
            }
            let rules = comp.compile_rules()?;
            Ok(Self { rules, source_dir: dir.to_string() })
        }

        pub fn try_from_env() -> Option<Self> {
            let dir = std::env::var("EDR_YARA_DIR").unwrap_or_else(|_| "state/yara".into());
            Self::from_dir(&dir).ok()
        }

        pub fn scan_record(&self, rec: &crate::telemetry::TelemetryRecord) -> Vec<Signal> {
            let mut out = Vec::new();
            let mut scanner = Scanner::new(&self.rules);
            // We scan the command line + path bytes as a blob (works for simple string/hexx patterns)
            let mut blob = Vec::new();
            blob.extend_from_slice(rec.binary_path.as_bytes());
            blob.push(b'\n');
            blob.extend_from_slice(rec.command_line.as_bytes());
            blob.push(b'\n');
            blob.extend_from_slice(rec.cwd.as_bytes());
            if let Ok(ms) = scanner.scan_mem(&blob) {
                for m in ms {
                    let rule = m.identifier.to_string();
                    // score from rule meta: "score" or "confidence" else 0.7
                    let mut score = 0.7f32;
                    if let Some(mv) = m.meta.iter().find(|mm| mm.identifier == "score") {
                        if let Ok(v) = mv.as_integer().map(|x| x as f32 / 100.0) { score = v; }
                    } else if let Some(mv) = m.meta.iter().find(|mm| mm.identifier == "confidence") {
                        if let Ok(v) = mv.as_integer().map(|x| x as f32 / 100.0) { score = v; }
                    }
                    let exe = if !rec.binary_path.is_empty() { rec.binary_path.clone() } else {
                        rec.command_line.split_whitespace().next().unwrap_or("-").to_string()
                    };
                    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
                    let user = if rec.uid == 0 { "root".into() } else { format!("uid:{}", rec.uid) };

                    out.push(Signal {
                        detector: DetectorKind::Other,
                        score: score.clamp(0.1, 1.0),
                        ts: rec.timestamp,
                        exe: Some(exe),
                        user: Some(user),
                        host: Some(host),
                        cmd_prefix: Some(rec.command_line.split_whitespace().next().unwrap_or("").to_string()),
                        parent: None,
                        signer: None,
                        hash: None,
                        pid: Some(rec.pid),
                        tags: vec![format!("YARA:{}", rule)],
                    });
                }
            }
            out
        }
    }
}

#[cfg(not(feature = "with_yara"))]
mod inner {
    use super::*;

    pub struct YaraLayer {}

    impl YaraLayer {
        pub fn from_dir(_dir: &str) -> anyhow::Result<Self> {
            Ok(Self {})
        }
        pub fn try_from_env() -> Option<Self> {
            None
        }
        pub fn scan_record(&self, _rec: &crate::telemetry::TelemetryRecord) -> Vec<Signal> {
            Vec::new()
        }
    }
}

pub use inner::YaraLayer;
