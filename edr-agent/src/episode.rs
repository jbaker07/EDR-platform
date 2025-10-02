// src/episode.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::event::{Event, EventType};
use crate::graph_builder::{GraphEdge, GraphNode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub host: String,
    pub context_id: Option<String>,
    pub role: Option<String>,
    pub start_ts: DateTime<Utc>,
    pub end_ts: DateTime<Utc>,
    pub events: Vec<Event>,
}

impl Episode {
    pub fn new(host: String) -> Self {
        let now = Utc::now();
        Self {
            host,
            context_id: None,
            role: None,
            start_ts: now,
            end_ts: now,
            events: Vec::with_capacity(128),
        }
    }

    #[inline]
    pub fn push(&mut self, ev: Event) {
        if ev.ts < self.start_ts {
            self.start_ts = ev.ts;
        }
        if ev.ts > self.end_ts {
            self.end_ts = ev.ts;
        }
        self.events.push(ev);
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn count_by(&self, et: EventType) -> usize {
        self.events.iter().filter(|e| e.event_type == et).count()
    }

    /// Build from TelemetryRecord batch (fast heuristic).
    pub fn from_records(host: String, records: &[crate::telemetry::TelemetryRecord]) -> Self {
        use crate::event::{Event, EventType};
        let mut ep = Episode::new(host);
        ep.context_id = Some("default".into());
        ep.role = Some("default".into());

        for r in records {
            if !r.binary_path.is_empty() || !r.command_line.is_empty() {
                ep.push(Event {
                    ts: Utc::now(),
                    event_type: EventType::Execve,
                    pid: Some(r.pid),
                    ppid: Some(r.ppid),
                    uid: Some(r.uid),
                    exe: Some(r.binary_path.clone()),
                    argv: if r.command_line.is_empty() {
                        None
                    } else {
                        Some(vec![r.command_line.clone()])
                    },
                    cwd: if r.cwd.is_empty() {
                        None
                    } else {
                        Some(r.cwd.clone())
                    },
                    ..Default::default()
                });
            }
            for t in &r.tags {
                let et = if t.contains("dll") {
                    EventType::DllInject
                } else if t.contains("ipc") {
                    EventType::EpollWait
                } else if t.contains("beacon") || t.contains("network") {
                    EventType::BeaconTick
                } else {
                    EventType::Other
                };
                ep.push(Event {
                    ts: Utc::now(),
                    event_type: et,
                    ..Default::default()
                });
            }
        }
        ep
    }

    /// Convert to a process graph (nodes = processes, edges = causal).
    /// This aligns with crate::graph_builder::{GraphNode, GraphEdge}.
    pub fn to_graph(&self) -> (Vec<GraphNode>, Vec<GraphEdge>) {
        // Collect processes we’ve seen
        let mut nodes_map: HashMap<i32, GraphNode> = HashMap::new();
        let mut edges: Vec<GraphEdge> = Vec::new();

        // Track within-pid last mmap/mprotect/memfd timestamps to make edges
        #[derive(Default)]
        struct PState {
            last_w_map: Option<DateTime<Utc>>,
            last_exec: Option<DateTime<Utc>>,
            memfd_open: Option<DateTime<Utc>>,
            memfd_written: bool,
        }
        let mut pstate: HashMap<i32, PState> = HashMap::new();

        // Helper to ensure node exists
        let mut ensure_node = |pid: i32,
                               ppid: Option<i32>,
                               exe: Option<String>,
                               argv: Option<Vec<String>>,
                               cwd: Option<String>,
                               uid: Option<u32>| {
            nodes_map.entry(pid).or_insert_with(|| {
                GraphNode {
                    id: format!("pid:{pid}"),
                    trust_score: 1.0, // neutral; scoring happens elsewhere
                    uid,
                    pid: Some(pid as u32),
                    ppid: ppid.map(|v| v as u32),
                    binary_path: exe,
                    command_line: argv.map(|a| a.join(" ")),
                    cwd,
                    tags: Vec::new(),
                    anchor_ids: Vec::new(),
                    ..Default::default()
                }
            });
        };

        // First pass: spawn edges via ppid→pid; and record local state for W→X/memfd chains.
        for ev in &self.events {
            let pid = match ev.pid {
                Some(p) => p,
                None => continue,
            };
            ensure_node(
                pid,
                ev.ppid,
                ev.exe.clone(),
                ev.argv.clone(),
                ev.cwd.clone(),
                ev.uid,
            );

            // Spawn edges (ppid→pid) on Execve/Fork/Clone
            match ev.event_type {
                EventType::Execve | EventType::Fexecve => {
                    if let Some(pp) = ev.ppid {
                        ensure_node(pp, None, None, None, None, ev.uid);
                        edges.push(GraphEdge {
                            source: format!("pid:{pp}"),
                            target: format!("pid:{pid}"),
                            ..Default::default()
                        });
                    }
                    pstate.entry(pid).or_default().last_exec = Some(ev.ts);
                }
                EventType::Mmap => {
                    // If PROT includes 'W' (we only have prot as String optionally)
                    if ev
                        .prot
                        .as_deref()
                        .unwrap_or("")
                        .to_ascii_uppercase()
                        .contains('W')
                    {
                        pstate.entry(pid).or_default().last_w_map = Some(ev.ts);
                    }
                }
                EventType::Mprotect => {
                    // W→X short-circuit within 5s → edge from pid to itself (flag)
                    if let Some(tw) = pstate.entry(pid).or_default().last_w_map {
                        if (ev.ts - tw).num_seconds().abs() <= 5 {
                            // self-edge to indicate W→X transition
                            edges.push(GraphEdge {
                                source: format!("pid:{pid}"),
                                target: format!("pid:{pid}"),
                                ..Default::default()
                            });
                        }
                    }
                }
                EventType::MemfdCreate => {
                    let s = pstate.entry(pid).or_default();
                    s.memfd_open = Some(ev.ts);
                    s.memfd_written = false;
                }
                EventType::MemfdWrite => {
                    let s = pstate.entry(pid).or_default();
                    s.memfd_written = true;
                }
                _ => {}
            }
        }

        // Second pass: memfd -> exec chains (pid-local)
        for (pid, st) in pstate.iter() {
            if st.memfd_open.is_some() && st.memfd_written {
                // If we saw an exec within 120s after memfd created, mark a self-edge (fileless exec)
                if let Some(exec_ts) = st.last_exec {
                    if let Some(open_ts) = st.memfd_open {
                        if (exec_ts - open_ts).num_seconds().abs() <= 120 {
                            edges.push(GraphEdge {
                                source: format!("pid:{pid}"),
                                target: format!("pid:{pid}"),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }

        // Convert nodes
        let mut nodes: Vec<GraphNode> = nodes_map.into_values().collect();

        // Optional: tag nodes by simple heuristics (beacon, dllinject)
        for ev in &self.events {
            if let Some(pid) = ev.pid {
                if let Some(n) = nodes.iter_mut().find(|n| n.pid == Some(pid as u32)) {
                    match ev.event_type {
                        EventType::BeaconTick => n.tags.push("beacon".into()),
                        EventType::DllInject => n.tags.push("dll_inject".into()),
                        _ => {}
                    }
                }
            }
        }

        (nodes, edges)
    }
}
