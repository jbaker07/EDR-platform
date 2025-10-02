use crate::telemetry::TelemetryRecord;
use crate::trust_hook::TrustEvent;
use anyhow::Result;

/// Stub for Elasticsearch Bulk API output.
pub fn emit<T>(events: &[TrustEvent], records: &[TelemetryRecord], _data: &T) -> Result<()> {
    log::info!(
        "[elastic_bulk] events={}, records={}",
        events.len(),
        records.len()
    );
    Ok(())
}
