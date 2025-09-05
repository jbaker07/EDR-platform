// src/detect/registry.rs
use crate::detect::types::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use std::{collections::HashMap, fs, path::Path, sync::Arc};
use tokio::{sync::oneshot, task::JoinHandle};

const STATE_DIR: &str = "state";
const CFG_FILE: &str = "state/integrations.json";

#[derive(Clone)]
pub struct IntegrationsRegistry {
    inner: Arc<RwLock<Inner>>,
}

struct Inner {
    cfgs: HashMap<String, IntegrationConfig>,
    rt: HashMap<String, IntegrationRuntime>,
    tasks: HashMap<String, TaskCtl>,
}

struct TaskCtl {
    join: JoinHandle<()>,
    cancel: oneshot::Sender<()>,
}

impl IntegrationsRegistry {
    pub fn new() -> Self {
        fs::create_dir_all(STATE_DIR).ok();

        let mut cfgs: HashMap<String, IntegrationConfig> = HashMap::new();
        if let Ok(bytes) = fs::read(CFG_FILE) {
            if let Ok(v) = serde_json::from_slice::<Vec<IntegrationConfig>>(&bytes) {
                for c in v {
                    cfgs.insert(c.id.clone(), c);
                }
            }
        }

        Self {
            inner: Arc::new(RwLock::new(Inner {
                cfgs,
                rt: HashMap::new(),
                tasks: HashMap::new(),
            })),
        }
    }

    pub fn list(&self) -> Vec<(IntegrationConfig, IntegrationRuntime)> {
        let g = self.inner.read();
        g.cfgs
            .values()
            .cloned()
            .map(|c| {
                let rt = g.rt.get(&c.id).cloned().unwrap_or_default();
                (c, rt)
            })
            .collect()
    }

    pub fn persist(&self) -> Result<()> {
        let g = self.inner.read();
        let mut vec: Vec<IntegrationConfig> = g.cfgs.values().cloned().collect();
        vec.sort_by_key(|c| c.created_at);
        fs::write(CFG_FILE, serde_json::to_vec_pretty(&vec)?)?;
        Ok(())
    }

    pub fn add(&self, cfg: IntegrationConfig) -> Result<IntegrationConfig> {
        self.inner.write().cfgs.insert(cfg.id.clone(), cfg.clone());
        self.persist()?;
        Ok(cfg)
    }

    pub fn remove(&self, id: &str) -> Result<()> {
        let mut w = self.inner.write();
        if let Some(TaskCtl { cancel, .. }) = w.tasks.remove(id) {
            let _ = cancel.send(());
        }
        w.cfgs.remove(id);
        w.rt.remove(id);
        drop(w);
        self.persist()?;
        Ok(())
    }

    pub fn update_status(
        &self,
        id: &str,
        status: IntegrationStatus,
        err: Option<String>,
        last_ok: Option<DateTime<Utc>>,
        poll_delta: Option<(u64, u64)>,
    ) {
        let mut w = self.inner.write();
        let entry = w.rt.entry(id.to_string()).or_default();
        entry.status = status;
        entry.last_error = err;

        if let Some((seen, em)) = poll_delta {
            entry.poll.last_poll = Some(Utc::now());
            entry.poll.events_seen += seen;
            entry.poll.events_emitted += em;
        }

        if let Some(c) = w.cfgs.get_mut(id) {
            if last_ok.is_some() {
                c.last_ok = last_ok;
            }
        }
    }

    pub fn cfg(&self, id: &str) -> Option<IntegrationConfig> {
        self.inner.read().cfgs.get(id).cloned()
    }

    pub fn spawn(&self, id: &str, base_url: String) -> Result<()> {
        let cfg = match self.cfg(id) {
            Some(c) => c,
            None => return Ok(()),
        };

        let (tx_cancel, rx_cancel) = oneshot::channel::<()>();
        let reg = self.clone();

        let handle = tokio::spawn(async move {
            reg.update_status(&cfg.id, IntegrationStatus::Starting, None, None, None);

            match cfg.kind.as_str() {
                "okta_syslog" => {
                    crate::detect::okta::run_okta_connector(
                        cfg,
                        base_url,
                        rx_cancel,
                        reg.clone(),
                    )
                    .await
                }
                other => reg.update_status(
                    &cfg.id,
                    IntegrationStatus::Error,
                    Some(format!("Unsupported kind: {}", other)),
                    None,
                    None,
                ),
            }
        });

        self.inner
            .write()
            .tasks
            .insert(id.to_string(), TaskCtl { join: handle, cancel: tx_cancel });

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// DetectorRegistry (rules / anomaly / ML detectors registry) used by pipeline.rs
// -----------------------------------------------------------------------------
use crate::detect::rules::RulesDetector;
use crate::detect::{Detector, Finding, NormalizedEvent}; // 👈 use the alias exported by detect::mod
pub struct DetectorRegistry {
    detectors: Vec<Box<dyn Detector>>,
}

impl Default for DetectorRegistry {
    fn default() -> Self { Self { detectors: Vec::new() } }
}

impl DetectorRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, detector: Box<dyn Detector>) {
        self.detectors.push(detector);
    }

    pub fn add_rules_from(&mut self, path: &Path) -> Result<()> {
        let det = RulesDetector::from_file(path)?;
        self.add(Box::new(det));
        Ok(())
    }

    /// Back-compat alias for older pipeline code.
    pub fn run_all(&self, event: &NormalizedEvent, features: &[f64]) -> Vec<Finding> {
        self.score_all(event, features)
    }

    /// Score an event with all registered detectors and flatten results.
    pub fn score_all(&self, event: &NormalizedEvent, features: &[f64]) -> Vec<Finding> {
        let mut out = Vec::new();
        for det in &self.detectors {
            if det.supports(event) {
                out.extend(det.score(event, features));
            }
        }
        out
    }
}
