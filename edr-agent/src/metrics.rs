use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

#[derive(Default)]
pub struct Metrics {
    // canonical counters maintained across the pipeline
    pub events_in: AtomicU64,
    pub events_accepted: AtomicU64,
    pub bpf_drops_total: AtomicU64,

    // adapter / playbook wiring
    pub facts_in: AtomicU64,
    pub playbooks_fired_total: AtomicU64,
    pub mapper_dropped_facts_total: AtomicU64,
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset_on_startup(&self) {
        self.events_in.store(0, Ordering::Relaxed);
        self.events_accepted.store(0, Ordering::Relaxed);
        self.bpf_drops_total.store(0, Ordering::Relaxed);
        self.facts_in.store(0, Ordering::Relaxed);
        self.playbooks_fired_total.store(0, Ordering::Relaxed);
        self.mapper_dropped_facts_total.store(0, Ordering::Relaxed);
    }
}

static GLOBAL: OnceLock<Arc<Metrics>> = OnceLock::new();

pub fn init(metrics: Arc<Metrics>) {
    let _ = GLOBAL.set(metrics);
}

fn get() -> &'static Arc<Metrics> {
    GLOBAL
        .get()
        .expect("metrics::init must be called before use")
}

pub fn events_in() -> &'static AtomicU64 {
    &get().events_in
}

pub fn events_accepted() -> &'static AtomicU64 {
    &get().events_accepted
}

pub fn bpf_drops_total() -> &'static AtomicU64 {
    &get().bpf_drops_total
}

pub fn facts_in() -> &'static AtomicU64 {
    &get().facts_in
}

pub fn playbooks_fired_total() -> &'static AtomicU64 {
    &get().playbooks_fired_total
}

pub fn mapper_dropped_facts_total() -> &'static AtomicU64 {
    &get().mapper_dropped_facts_total
}
