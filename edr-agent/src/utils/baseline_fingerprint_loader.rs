use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use serde_json::json;

use crate::logger::log;

pub fn generate_baseline_fingerprint(output_path: &str, patterns: &[&str]) {
    let mut fingerprint_map = HashMap::new();

    for pattern in patterns {
        for entry in WalkDir::new(pattern).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            match fs::read(path) {
                Ok(contents) => {
                    let mut hasher = Sha256::new();
                    hasher.update(&contents);
                    let hash = format!("{:x}", hasher.finalize());
                    fingerprint_map.insert(hash, path.to_string_lossy().to_string());
                }
                Err(err) => {
                    log("baseline_creator", &format!("Failed to read {:?}: {}", path, err));
                }
            }
        }
    }

    let serialized = json!(fingerprint_map);
    match File::create(output_path) {
        Ok(mut file) => {
            if let Err(e) = write!(file, "{}", serialized.to_string()) {
                log("baseline_creator", &format!("Failed to write JSON: {}", e));
            } else {
                log("baseline_creator", &format!("Wrote {} safe fingerprints", fingerprint_map.len()));
            }
        }
        Err(e) => {
            log("baseline_creator", &format!("Failed to create baseline file: {}", e));
        }
    }
}
