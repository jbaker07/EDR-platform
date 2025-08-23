use anyhow::Result;
use crate::telemetry::TelemetryRecord;
use crate::trust_hook::TrustEvent;

/// Stub for Splunk HEC output.
pub fn emit<T>(events: &[TrustEvent], records: &[TelemetryRecord], _data: &T) -> Result<()> {
    log::info!("[splunk_hec] events={}, records={}", events.len(), records.len());
    Ok(())
}
