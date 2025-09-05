use crate::telemetry::Telemetry;

/// Extract a fixed-order numeric feature vector from a single Telemetry event.
/// Keep this stable so your anomaly models line up dimensionally.
///
/// Current layout (len = 6):
///   0: pid (or 0)
///   1: ppid (or 0)
///   2: uid (or 0)
///   3: attrs.bytes (or 0.0)
///   4: attrs.duration_ms (or 0.0)
///   5: attrs.entropy (or 0.0)
pub fn extract_features(t: &Telemetry) -> Vec<f64> {
    let mut v = Vec::with_capacity(6);
    v.push(t.pid.unwrap_or_default() as f64);
    v.push(t.ppid.unwrap_or_default() as f64);
    v.push(t.uid.unwrap_or_default() as f64);
    v.push(t.attr_f64("bytes").unwrap_or(0.0));
    v.push(t.attr_f64("duration_ms").unwrap_or(0.0));
    v.push(t.attr_f64("entropy").unwrap_or(0.0));
    v
}
