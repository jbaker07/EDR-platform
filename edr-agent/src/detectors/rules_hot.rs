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

/// Inner mutable state guarded by a Mutex for hot-reload.
struct HotRulesInner {
    path: PathBuf,
    rules: RulesDetector,
    last_check: Instant,
    check_every: Duration,
    last_mtime: Option<SystemTime>,
}

/// A self-contained rules detector that hot-reloads when the JSON file changes.
pub struct HotRulesDetector {
    inner: Mutex<HotRulesInner>,
}

impl HotRulesDetector {
    pub fn new(path: &Path, check_every: Duration) -> Result<Self> {
        let meta = fs::metadata(path)
            .with_context(|| format!("rules file not found at {}", path.display()))?;
        let last_mtime = meta.modified().ok();
        let rules = RulesDetector::from_file(path)
            .with_context(|| format!("failed to parse rules at {}", path.display()))?;
        Ok(Self {
            inner: Mutex::new(HotRulesInner {
                path: path.to_path_buf(),
                rules,
                last_check: Instant::now(),
                check_every,
                last_mtime,
            }),
        })
    }

    fn maybe_reload(inner: &mut HotRulesInner) {
        if inner.last_check.elapsed() < inner.check_every {
            return;
        }
        inner.last_check = Instant::now();

        let Ok(meta) = fs::metadata(&inner.path) else {
            return;
        };
        let Ok(mtime) = meta.modified() else { return };

        let should_reload = inner.last_mtime.map(|m| mtime > m).unwrap_or(true);
        if should_reload {
            match RulesDetector::from_file(&inner.path) {
                Ok(new_rules) => {
                    inner.rules = new_rules;
                    inner.last_mtime = Some(mtime);
                    eprintln!("♻️  Reloaded rules from {}", inner.path.display());
                }
                Err(e) => {
                    eprintln!("⚠️ rules hot-reload failed (keeping old copy): {e}");
                }
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
        Self::maybe_reload(&mut guard);
        guard.rules.score(e, features)
    }
}
