// src/detectors/rules_hot.rs
use anyhow::{Context, Result};
use std::sync::Mutex;
use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant, SystemTime},
};

use crate::detect::rules::RulesDetector;
use crate::detect::types::NormalizedAlert;
use crate::detect::{Detector, Finding};

/// mtime and size of the rules file. Size catches a rewrite that lands in the same mtime tick.
type Stamp = (SystemTime, u64);

/// Inner mutable state guarded by a Mutex for hot-reload.
struct HotRulesInner {
    path: PathBuf,
    rules: RulesDetector,
    last_check: Instant,
    check_every: Duration,
    last_stamp: Option<Stamp>,
}

/// Result of one poll of the rules file.
#[derive(Debug, PartialEq)]
pub enum Reload {
    /// Not time to check yet, or the file is unchanged or unreadable.
    Unchanged,
    /// The new file passed validation and replaced the old rules.
    Loaded(usize),
    /// The new file failed validation. The old rules stay active.
    Rejected(String),
}

/// A self-contained rules detector that hot-reloads when the JSON file changes.
pub struct HotRulesDetector {
    inner: Mutex<HotRulesInner>,
}

fn stamp(path: &Path) -> Option<Stamp> {
    let meta = fs::metadata(path).ok()?;
    Some((meta.modified().ok()?, meta.len()))
}

impl HotRulesDetector {
    pub fn new(path: &Path, check_every: Duration) -> Result<Self> {
        fs::metadata(path)
            .with_context(|| format!("rules file not found at {}", path.display()))?;
        let last_stamp = stamp(path);
        let rules = RulesDetector::from_file(path)?;
        eprintln!("rules: loaded {} rules from {}", rules.len(), path.display());
        Ok(Self {
            inner: Mutex::new(HotRulesInner {
                path: path.to_path_buf(),
                rules,
                last_check: Instant::now(),
                check_every,
                last_stamp,
            }),
        })
    }

    /// Check the file now (subject to check_every) and report what happened.
    pub fn poll(&self) -> Reload {
        let mut guard = self.inner.lock().expect("rules_hot mutex poisoned");
        Self::maybe_reload(&mut guard)
    }

    fn maybe_reload(inner: &mut HotRulesInner) -> Reload {
        if inner.last_check.elapsed() < inner.check_every {
            return Reload::Unchanged;
        }
        inner.last_check = Instant::now();

        let Some(now) = stamp(&inner.path) else {
            return Reload::Unchanged;
        };
        // Any change counts, so a file restored with an older mtime still reloads.
        if inner.last_stamp == Some(now) {
            return Reload::Unchanged;
        }
        // Record the stamp first so a bad file is reported once per change, not every poll.
        inner.last_stamp = Some(now);

        match RulesDetector::from_file(&inner.path) {
            Ok(new_rules) => {
                let n = new_rules.len();
                inner.rules = new_rules;
                eprintln!("rules: reloaded {n} rules from {}", inner.path.display());
                Reload::Loaded(n)
            }
            Err(e) => {
                let why = format!("{e:#}");
                eprintln!(
                    "rules: hot reload rejected, keeping the {} rules from the last good load: {why}",
                    inner.rules.len()
                );
                Reload::Rejected(why)
            }
        }
    }
}

impl Detector for HotRulesDetector {
    fn name(&self) -> &'static str {
        "rules_hot"
    }

    fn score(&self, e: &NormalizedAlert, features: &[f64]) -> Vec<Finding> {
        let mut guard = self.inner.lock().expect("rules_hot mutex poisoned");
        let _ = Self::maybe_reload(&mut guard);
        guard.rules.score(e, features)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::time::UNIX_EPOCH;

    const RULE_A: &str = r#"{"version":1,"rules":[{"id":"A","name":"a","severity":0.5,
        "all":[{"field":"exe","op":"eq","value":"/bin/a"}]}]}"#;
    const RULE_B: &str = r#"{"version":1,"rules":[{"id":"B","name":"b","severity":0.5,
        "all":[{"field":"exe","op":"eq","value":"/bin/b"}]}]}"#;
    // The schema src/rules.json shipped with before this fix.
    const OLD_SCHEMA: &str = r#"{"version":1,"rules":[{"id":"R001","name":"x","severity":0.9,
        "match":{"category":"file","signal":"file_access"}}]}"#;

    fn temp_path(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!("edr_rules_hot_{tag}_{}.json", std::process::id()))
    }

    // Write the file and pin its mtime, so the test does not depend on clock resolution.
    fn write(path: &Path, body: &str, mtime_secs: u64) {
        fs::write(path, body).unwrap();
        let f = fs::OpenOptions::new().write(true).open(path).unwrap();
        f.set_modified(UNIX_EPOCH + Duration::from_secs(mtime_secs)).unwrap();
    }

    fn event(exe: &str) -> NormalizedAlert {
        NormalizedAlert {
            id: "t".into(),
            ts: chrono::Utc::now(),
            source: "agent".into(),
            connector_id: "edr-agent".into(),
            severity: 0.0,
            title: "telemetry".into(),
            short_why: String::new(),
            entities: json!({}),
            attributes: json!({ "category": "process", "event": "exec", "exe": exe }),
            mitre: vec![],
        }
    }

    fn labels(det: &HotRulesDetector, exe: &str) -> Vec<String> {
        det.score(&event(exe), &[]).into_iter().filter_map(|f| f.label).collect()
    }

    #[test]
    fn reload_swaps_good_files_and_keeps_old_rules_on_bad_ones() {
        let path = temp_path("swap");
        write(&path, RULE_A, 1_000);
        let det = HotRulesDetector::new(&path, Duration::ZERO).unwrap();
        assert_eq!(labels(&det, "/bin/a"), vec!["A"]);
        assert_eq!(det.poll(), Reload::Unchanged);

        // A file in the old schema is rejected and rule A keeps working.
        write(&path, OLD_SCHEMA, 2_000);
        match det.poll() {
            Reload::Rejected(why) => assert!(why.contains("unknown field `match`"), "{why}"),
            other => panic!("expected Rejected, got {other:?}"),
        }
        assert_eq!(labels(&det, "/bin/a"), vec!["A"]);
        // Reported once per change, not on every poll.
        assert_eq!(det.poll(), Reload::Unchanged);

        // A valid file replaces the rules.
        write(&path, RULE_B, 3_000);
        assert_eq!(det.poll(), Reload::Loaded(1));
        assert_eq!(labels(&det, "/bin/b"), vec!["B"]);
        assert!(labels(&det, "/bin/a").is_empty());

        // An older mtime still counts as a change.
        write(&path, RULE_A, 500);
        assert_eq!(det.poll(), Reload::Loaded(1));
        assert_eq!(labels(&det, "/bin/a"), vec!["A"]);

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn startup_load_fails_on_invalid_file() {
        let path = temp_path("startup");
        write(&path, OLD_SCHEMA, 1_000);
        let err = HotRulesDetector::new(&path, Duration::from_secs(2)).err().expect("must fail");
        assert!(format!("{err}").contains("unknown field `match`"), "{err}");
        let _ = fs::remove_file(&path);
    }
}
