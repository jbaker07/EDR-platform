edr-agent/
  src/userspace/
    ingest/
      ringbuf_reader.rs          # unified reader for shared ringbufs
      perf_reader.rs             # legacy perf buffer reader(s)
      mapper.rs                  # probe-name → schema converter (raw → common)
    schema/
      common_event.rs            # Common Host Event (Rust struct + serde)
      external_event.rs          # IDS/EPP adapter structs
    feature_bus/
      feature_defs.rs            # feature definitions + units
      feature_engine.rs          # online updates, windowing
      sketches.rs                # CMS/HLL/tdigest wrappers
    baselines/
      store.rs                   # LMDB/SQLite impl
      subjects.rs                # keying (proc/user/container/path/dst)
      snapshot.rs                # export for explainers
    scoring/
      quantiles.rs               # p95/p99.5 lookups
      robust_z.rs                # MAD/robust z-score
      change_detect.rs           # Page-Hinkley/ADWIN
      fusion.rs                  # monotonic logistic + source weights
    rules/
      dsl.rs                     # minimal declarative DSL (serde_yaml)
      compiler.rs                # DSL → predicates over Common Event + features
      runtime.rs                 # evaluation & action hooks
      stdlib.rs                  # helpers: path regex, proc ancestry, etc.
      examples/
        *.rule.yaml              # seed rules (10+)
    correlation/
      evidence_model.rs          # host+network+EPP evidence union
      graph.rs                   # entity graph: proc↔file↔net↔container
      sessionizer.rs             # bundle events into incidents
    explain/
      why.rs                     # observed vs baseline, deltas, weights
      templates.rs               # human-facing narratives
    storage/
      event_log.rs               # append-only normalized events (parquet/sqlite)
      alert_store.rs             # alerts/incidents (sqlite)
      kv.rs                      # small config & checkpoints
    feedback/
      api.rs                     # operator feedback (label, suppress, tune)
      learner.rs                 # update source weights & rule bias safely
    api/
      uds.rs                     # Unix socket gRPC/JSON-RPC for UI/CLIs
      types.rs                   # request/response types
    ui/
      cli.rs                     # operators’ CLI (health, tail, ack)
      (web later)
    obs/
      metrics.rs                 # Prometheus exporter
      health.rs                  # attach/liveness/rb-drops/self-tests
    config/
      edr.yaml                   # central config (paths, thresholds, adapters)
      rules.yaml                 # list of enabled rules/bundles
    adapters/
      suricata.rs                # EVE JSON tailer → external_event
      zeek.rs                    # optional
      siem_forwarder.rs          # JSON/Loki/Splunk/OTLP
