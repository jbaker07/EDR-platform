use anyhow::Result;
use std::cmp::Ordering;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;

use crate::telemetry::TelemetryRecord;
use crate::trust_hook::TrustEvent;

pub fn explain<T>(events: &[TrustEvent], _records: &[TelemetryRecord], _data: &T) -> Result<()> {
    // Sort a copy of references by decreasing risk score (fallback to 0.0 if needed)
    let mut ordered: Vec<&TrustEvent> = events.iter().collect();
    ordered.sort_by(|a, b| {
        let sa = a.risk_score;
        let sb = b.risk_score;
        sb.partial_cmp(&sa).unwrap_or(Ordering::Equal)
    });

    // Prepare a concise human-readable summary for the top N
    let top_n = ordered.iter().take(5);

    // Ensure state dir exists
    create_dir_all("state")?;
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open("state/explanations.log")?;

    for (i, ev) in top_n.enumerate() {
        // Pull interesting context defensively
        let tags = ev
            .tags
            .as_ref()
            .map(|v| {
                if v.is_empty() {
                    String::from("[]")
                } else {
                    // cap to a few to avoid huge lines
                    let mut s = String::new();
                    for (j, t) in v.iter().take(8).enumerate() {
                        if j > 0 {
                            s.push_str(", ");
                        }
                        s.push_str(t);
                    }
                    format!("[{}{}]", s, if v.len() > 8 { ", …" } else { "" })
                }
            })
            .unwrap_or_else(|| "[]".into());

        let meta_path = ev
            .metadata
            .get("path")
            .cloned()
            .or_else(|| ev.metadata.get("sequence").cloned())
            .unwrap_or_default();

        // Prefer provided description; otherwise synthesize a short one
        let desc = ev.description.clone().unwrap_or_else(|| {
            format!(
                "{}:{} (uid {}) → rs={:.2} tags={} path={}",
                ev.binary_path,
                ev.pid,
                ev.uid,
                ev.risk_score,
                tags,
                meta_path
            )
        });

        // Log line (stdout) and persist to file
        println!(
            "[explain] #{:02} rs={:.2} {} [{}] {}",
            i + 1,
            ev.risk_score,
            ev.anomaly_type,
            ev.component,
            desc
        );

        writeln!(
            f,
            "[explain] idx={} ts={} rs={:.2} anomaly={} component={} pid={} uid={} bin={} tags={} path={} desc={}",
            i + 1,
            ev.timestamp,
            ev.risk_score,
            ev.anomaly_type,
            ev.component,
            ev.pid,
            ev.uid,
            ev.binary_path,
            tags,
            meta_path,
            desc.replace('\n', " ")
        )?;
    }

    Ok(())
}
