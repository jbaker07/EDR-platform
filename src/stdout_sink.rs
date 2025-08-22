use anyhow::Result;
use crate::telemetry::TelemetryRecord;
use crate::trust_hook::TrustEvent;

/// Emit events to stdout in a minimal, safe way.
pub fn emit<T>(events: &[TrustEvent], records: &[TelemetryRecord], _data: &T) -> Result<()> {
    println!("[stdout_sink] events={}, records={}", events.len(), records.len());
    Ok(())
}
