use anyhow::Result;
use crate::telemetry::TelemetryRecord;
use crate::trust_hook::TrustEvent;

/// Stub for Slack webhook notifications.
pub fn emit<T>(events: &[TrustEvent], records: &[TelemetryRecord], _data: &T) -> Result<()> {
    log::info!("[slack_webhook] events={}, records={}", events.len(), records.len());
    Ok(())
}
