// src/yara_sidecar.rs
// Lightweight tailer for YARA / IOC sidecar hits written as NDJSON to state/yara_hits.ndjson
// Each line: {"ts": 1710000000, "host": "...", "path": "...", "rule": "SuspiciousPacked", "meta": {"family":"xyz"}}

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YaraHit {
    pub ts: u64,
    pub host: String,
    pub path: String,
    pub rule: String,
    #[serde(default)]
    pub meta: serde_json::Value,
}

pub struct YaraTail {
    path: String,
    offset: u64,
}

impl YaraTail {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            offset: 0,
        }
    }

    pub fn poll(&mut self) -> Vec<YaraHit> {
        let mut out = Vec::new();
        if let Ok(mut f) = File::open(&self.path) {
            let _ = f.seek(SeekFrom::Start(self.offset));
            let mut reader = BufReader::new(f);
            let mut pos = self.offset;
            loop {
                let mut line = String::new();
                let bytes = reader.read_line(&mut line).unwrap_or(0);
                if bytes == 0 {
                    break;
                }
                pos += bytes as u64;
                if line.trim().is_empty() {
                    continue;
                }
                if let Ok(hit) = serde_json::from_str::<YaraHit>(&line) {
                    out.push(hit);
                }
            }
            self.offset = pos;
        }
        out
    }
}

pub fn to_signal(hit: &YaraHit) -> crate::decision::Signal {
    use crate::decision::{DetectorKind, Signal};
    Signal {
        detector: DetectorKind::Other,
        score: 0.85,
        ts: hit.ts,
        exe: Some(hit.path.clone()),
        user: Some("unknown".into()),
        host: Some(hit.host.clone()),
        cmd_prefix: Some("yara".into()),
        parent: None,
        signer: None,
        hash: None,
        pid: None,
        tags: vec!["IOC".into(), format!("YARA:{}", hit.rule)],
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
