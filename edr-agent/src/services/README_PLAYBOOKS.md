# Playbook Engine (YAML-driven) — quick integration

This engine lets you define cross-domain *step chains* (process → file → network → privilege, etc.) in YAML.
When all required steps are satisfied **within per-step TTLs and an overall playbook window**, the engine emits a synthetic `Signal`
so your existing `DecisionEngine` can score and escalate with no policy surgery.

## Files
- `src/services/playbook_engine.rs` — the engine
- `playbooks/*.yaml` — starter playbooks (memfd+egress, compress+exfil, LoTL cred)

## YAML schema (subset)

```yaml
id: pb_id
name: Optional name
description: Optional long text
window_secs: 900              # overall sliding window
key_by: [host, user, exe]     # correlation key (defaults to host+user)
steps:
  - id: step1
    domain: process|network|file|auth|privilege|persistence|memory|other
    any_tags: ["T1055*", "MEMFD_EXEC"]     # at least one must match (glob or regex if starts with ^)
    all_tags: ["krim_alert"]               # all must match
    detector: rules|krim|mahala|envelope   # optional detector constraint
    exe_regex: "^/(usr|opt)/.*"
    cmd_regex: ".*(curl|wget).*"
    max_gap_secs: 300                      # TTL between this and previous step
  - id: step2
    ...
emit:
  add_tags: ["ATTACK:XYZ", "Txxxx"]
  risk: 0.95
```

## Wiring (summary)

1. Add the module and singleton:

```rust
mod services { pub mod playbook_engine; }
use services::playbook_engine::PlaybookEngine;
static PLAYBOOK_ENGINE: OnceCell<Mutex<PlaybookEngine>> = OnceCell::new();
fn playbooks() -> &'static Mutex<PlaybookEngine> {
    PLAYBOOK_ENGINE.get_or_init(|| {
        let eng = PlaybookEngine::new_from_dir("playbooks").unwrap_or_else(|_| PlaybookEngine::empty());
        Mutex::new(eng)
    })
}
```

2. Ensure `playbooks/` exists at boot:

```rust
let _ = std::fs::create_dir_all("playbooks");
```

3. Feed *both* your per-record tags **and** your Rules alerts-derived `Signal` into the playbooks before passing into `DecisionEngine`:
```rust
{
    let mut pb = playbooks().lock().unwrap();
    for rec in &records {
        for tag in &rec.tags {
            if let Some(sig) = signal_from_tag(rec, tag) {
                for s in pb.ingest(&sig) {
                    for a in decision_engine().lock().unwrap().ingest(s) {
                        emit_final_incident(a, &alert_tx, &inc_tx);
                    }
                }
            }
        }
    }
}
```

…and in the Rules alert section (where you already build `sig` for the decider):
```rust
let mut pb = playbooks().lock().unwrap();
for s in pb.ingest(&sig) {
    for a in decision_engine().lock().unwrap().ingest(s) {
        emit_final_incident(a, &alert_tx, &inc_tx);
    }
}
```

That’s it — completed playbooks show up as incidents with tags like `PLAYBOOK:<id>` and `SLOTS:<a+b+c>`.
```

## Dependencies
Add to `Cargo.toml` if missing:
```toml
anyhow = "1"
regex = "1"
serde_yaml = "0.9"
regex-syntax = "0.8"
```

## Notes
- The engine uses simple glob-to-regex for `any_tags` / `all_tags`. If your pattern starts with `^` or ends with `$`, it’s treated as a raw regex.
- Keys default to `(host,user)` so different users on the same host won’t co-mingle chains. Add `exe` to tighten if needed.
- Use per-step `max_gap_secs` to enforce tempo differences per attack type; the overall `window_secs` bounds state growth.
- Synthetic Signals reuse `DetectorKind::Rules` to benefit from existing weights. You can later add a `Playbook` detector kind if desired.