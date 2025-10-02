// src/vendor_fusion.rs
// Helpers for turning external vendor alerts into playbook slot fulfillment.
// This acts as a small adapter layer — it doesn't parse every product schema;
// callers should map the vendor payload into (product, alert_type, host, ts).

use crate::pb_engine::PlaybookEngine;

#[derive(Debug, Clone)]
pub struct VendorAlert {
    pub product: String,    // e.g., "CrowdStrike"
    pub alert_type: String, // e.g., "Suspicious Mass File Modification"
    pub host: String,       // normalized hostname
    pub ts: u64,            // seconds
}

impl VendorAlert {
    pub fn new(
        product: impl Into<String>,
        alert_type: impl Into<String>,
        host: impl Into<String>,
        ts: u64,
    ) -> Self {
        Self {
            product: product.into(),
            alert_type: alert_type.into(),
            host: host.into(),
            ts,
        }
    }
}

pub fn fuse(
    engine: &mut PlaybookEngine,
    alert: VendorAlert,
) -> Option<crate::pb_engine::PlaybookHit> {
    engine.satisfy_slot_from_vendor(&alert.product, &alert.alert_type, &alert.host, alert.ts)
}
