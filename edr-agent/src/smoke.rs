use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

use forensic_hooks::pipeline::{Evaluator, Thresholds};
use forensic_hooks::telemetry::Telemetry; // same type your main.rs uses

fn mk_process(exe: &str, cmd: &str) -> Telemetry {
    let mut t = Telemetry::default();
    t.category = "process".into();
    t.event = "exec".into();
    t.exe = Some(exe.into());
    t.cmdline = Some(cmd.into());
    t.pid = Some(4242);
    t
}

fn mk_file(entropy: f64) -> Telemetry {
    let mut t = Telemetry::default();
    t.category = "file".into();
    t.event = "write".into();
    t.attrs.insert("entropy".into(), serde_json::json!(entropy));
    t
}

fn mk_dns(qname_len: u64) -> Telemetry {
    let mut t = Telemetry::default();
    t.category = "network".into();
    t.event = "connect".into();
    // both string and numeric mirror, in case your rules use either
    t.attrs.insert("qname_len".into(), serde_json::json!(qname_len.to_string()));
    t.attrs.insert("qname_len__num".into(), serde_json::json!(qname_len));
    t
}

fn mk_tagged(tag: &str) -> Telemetry {
    let mut t = Telemetry::default();
    t.category = "process".into();
    t.event = "exec".into();
    let mut tags: HashMap<String, String> = HashMap::new();
    tags.insert(tag.into(), "true".into());
    t.tags = tags;
    t
}

fn main() -> Result<()> {
    // evaluator with the same thresholds as main.rs
    let mut eval = Evaluator::new(Thresholds { low: 0.30, medium: 0.55, high: 0.80 });

    // hot-reloaded rules (every 2s) + tag→finding detector
    if let Ok(hot) = forensic_hooks::detectors::rules_hot::HotRulesDetector::new(
        Path::new("edr-agent/src/rules.json"),
        std::time::Duration::from_secs(2),
    ) {
        eval.registry.add(Box::new(hot));
    } else {
        eprintln!("⚠️ rules detector not loaded (hot): edr-agent/src/rules.json");
    }
    eval.registry.add(Box::new(forensic_hooks::detectors::tag_flags::TagFlagsDetector));

    // (optional) true anomaly detectors if you’ve got model files on disk
    let mp = Path::new("models/mahal.json");
    if mp.exists() {
        if let Ok(det) = forensic_hooks::detectors::mahalanobis::MahalDetector::from_file(mp) {
            eval.registry.add(Box::new(det));
        }
    }
    let ep = Path::new("models/elliptic.json");
    if ep.exists() {
        if let Ok(det) = forensic_hooks::detectors::elliptic_envelope::EllipticEnvelopeDetector::from_file(ep) {
            eval.registry.add(Box::new(det));
        }
    }
    let kp = Path::new("models/krim.json");
    if kp.exists() {
        if let Ok(det) = forensic_hooks::detectors::krim_lite::KrimLiteDetector::from_file(kp) {
            eval.registry.add(Box::new(det));
        }
    }

    // write to the same file your agent uses
    eval.set_output(Path::new("logs/alerts.jsonl"))?;

    // ===== synth cases =====
    // 1) LOLBin rundll32
    let t1 = mk_process("C:\\Windows\\System32\\rundll32.exe", "rundll32.exe foo.dll,Start");
    let _ = eval.evaluate(&t1)?;

    // 2) Encoded PowerShell
    let t2 = mk_process("powershell.exe", "powershell -EncodedCommand SQBFAFgAIAAkAE8AQgBqA...==");
    let _ = eval.evaluate(&t2)?;

    // 3) High-entropy file write
    let t3 = mk_file(7.9);
    let _ = eval.evaluate(&t3)?;

    // 4) DNS long qname (tunnel hint)
    let t4 = mk_dns(150);
    let _ = eval.evaluate(&t4)?;

    // 5) Anomaly tag only (should contribute via TagFlagsDetector)
    let t5 = mk_tagged("mahalanobis_outlier");
    let _ = eval.evaluate(&t5)?;

    // 6) Benign process (should not alert unless you enabled benign audit)
    let t6 = mk_process("/usr/bin/ls", "ls -la /tmp");
    let _ = eval.evaluate(&t6)?;

    eprintln!("✅ smoke: wrote to logs/alerts.jsonl");
    Ok(())
}
