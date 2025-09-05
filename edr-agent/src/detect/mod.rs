// src/detect/mod.rs
use serde::Serialize;

pub mod types;
pub mod rules;
pub mod registry;
pub mod okta;
pub mod controller;

// 👇 Unify: detectors work on NormalizedAlert via this alias
pub type NormalizedEvent = crate::detect::types::NormalizedAlert;

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub source: String,
    pub score: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

pub trait Detector: Send + Sync {
    fn name(&self) -> &'static str;
    fn supports(&self, _e: &NormalizedEvent) -> bool { true }
    fn score(&self, e: &NormalizedEvent, features: &[f64]) -> Vec<Finding>;
}
