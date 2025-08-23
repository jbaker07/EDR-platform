use anyhow::Result;
use crate::telemetry::TelemetryRecord;
use crate::trust_hook::TrustEvent;

/// Stub for Elastic Common Schema (ECS) sink.
pub fn emit<T>(events: &[TrustEvent], records: &[TelemetryRecord], _data: &T) -> Result<()> {
    log::info!("[ecs_sink] events={}, records={}", events.len(), records.len());
    Ok(())
}
