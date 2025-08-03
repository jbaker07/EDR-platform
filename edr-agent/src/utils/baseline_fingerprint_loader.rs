use std::collections::HashMap;
use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::services::trust_vector::TrustVector;
use crate::logger::log;

pub fn load_baseline(role: &str) -> Option<TrustVector> {
    let path = format!("json_files/baselines/{}.vector.json", role);
    let path_buf = PathBuf::from(&path);

    if !path_buf.exists() {
        log("baseline_loader", &format!("No baseline file found for role '{}'", role));
        return None;
    }

    match File::open(&path_buf) {
        Ok(mut file) => {
            let mut json_str = String::new();
            if let Err(err) = file.read_to_string(&mut json_str) {
                log("baseline_loader", &format!("Failed to read baseline file: {}", err));
                return None;
            }

            match serde_json::from_str::<TrustVector>(&json_str) {
                Ok(vector) => Some(vector),
                Err(err) => {
                    log("baseline_loader", &format!("Failed to parse baseline JSON: {}", err));
                    None
                }
            }
        }
        Err(err) => {
            log("baseline_loader", &format!("Failed to open baseline file: {}", err));
            None
        }
    }
}

pub fn save_baseline(role: &str, vector: &TrustVector) {
    let dir_path = PathBuf::from("json_files/baselines/");
    if let Err(e) = create_dir_all(&dir_path) {
        log("baseline_loader", &format!("Failed to create baseline directory: {}", e));
        return;
    }

    let path = dir_path.join(format!("{}.vector.json", role));
    match File::create(&path) {
        Ok(mut file) => {
            let serialized = serde_json::to_string_pretty(vector).unwrap_or_default();
            if let Err(e) = file.write_all(serialized.as_bytes()) {
                log("baseline_loader", &format!("Failed to write baseline file: {}", e));
            } else {
                log("baseline_loader", &format!("Wrote baseline for role '{}' to {:?}", role, path));
            }
        }
        Err(e) => {
            log("baseline_loader", &format!("Failed to create baseline file: {}", e));
        }
    }
}
