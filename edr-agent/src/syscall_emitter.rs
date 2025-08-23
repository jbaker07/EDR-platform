// src/syscall_emitter.rs
//! Syscall cadence emitter (collector only).
//!
//! Consumes `rawsys:*` tags on graph nodes and summarizes:
//!   - total syscall tags, unique syscall kinds
//!   - "interesting" syscall hits (execve/execveat, mprotect, ptrace, bpf, socket, connect, mmap, clone/fork)
//!   - how many nodes had syscall tags
//!   - simple 2-hop sequences: execve -> {ptrace|bpf|mprotect|mmap} , socket -> connect
//!
//! Returns an empty Vec until your FeatureObservation ctor is finalized.
//! Use `emit_with_graph` to feed the tag-carrying graph built in main.rs.

use std::collections::{HashMap, HashSet};

use crate::episode::Episode;
use crate::graph_builder::{GraphEdge, GraphNode};
use crate::baselines::BaselineStore;
use crate::traits::FeatureEmitter;

#[derive(Debug, Clone)]
pub struct SyscallEmitter {
    /// Cap distinct syscall names tracked (defensive).
    pub max_kinds: usize,
    /// Cap nodes visited when scanning for 2-hop sequences.
    pub max_nodes_for_sequences: usize,
}

impl Default for SyscallEmitter {
    fn default() -> Self {
        Self {
            max_kinds: 256,
            max_nodes_for_sequences: 20000,
        }
    }
}
impl SyscallEmitter {
    pub fn new() -> Self { Self::default() }

    #[inline]
    fn interesting_set() -> &'static [&'static str] {
        &[
            "execve", "execveat",
            "ptrace", "bpf",
            "mprotect", "mmap",
            "socket", "connect",
            "clone", "fork", "vfork",
            "open", "openat",
        ]
    }

    /// Preferred path for your setup: feed the tag-carrying graph from main.rs.
    pub fn emit_with_graph(
        &self,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
        _baselines: &mut BaselineStore,
    ) -> Vec<crate::traits::FeatureObservation> {
        self.emit_core(nodes, edges)
    }

    fn emit_core(
        &self,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
    ) -> Vec<crate::traits::FeatureObservation> {
        // ---- 1) Mine rawsys:* tags -----------------------------
        let mut per_kind: HashMap<String, u64> = HashMap::new();
        let mut nodes_with_syscalls = 0u64;
        let mut interest_hits = 0u64;

        let interest: HashSet<&str> = Self::interesting_set().iter().copied().collect();

        // per-node set for sequence checks
        let mut node_syscalls: HashMap<&str, HashSet<String>> = HashMap::new();

        for n in nodes {
            let mut local_any = false;
            let mut set: HashSet<String> = HashSet::new();

            for t in &n.tags {
                if let Some(rest) = t.strip_prefix("rawsys:") {
                    let name = rest.trim().to_lowercase();
                    if per_kind.len() < self.max_kinds || per_kind.contains_key(&name) {
                        *per_kind.entry(name.clone()).or_insert(0) += 1;
                    }
                    if interest.contains(name.as_str()) {
                        interest_hits += 1;
                    }
                    set.insert(name);
                    local_any = true;
                }
            }

            if local_any {
                nodes_with_syscalls += 1;
                node_syscalls.insert(n.id.as_str(), set);
            }
        }

        let total_syscall_tags = per_kind.values().copied().sum::<u64>();
        let unique_syscalls = per_kind.len() as u64;

        // ---- 2) 2-hop sequence scan ---------------------------
        let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
        for e in edges {
            adj.entry(e.source.as_str()).or_default().push(e.target.as_str());
        }

        let mut seq_exec_to_memctl = 0u64; // execve -> {ptrace|bpf|mprotect|mmap}
        let mut seq_socket_connect = 0u64; // socket -> connect

        let targets_memctl: HashSet<&str> = ["ptrace", "bpf", "mprotect", "mmap"].into_iter().collect();
        let targets_connect: HashSet<&str> = ["connect"].into_iter().collect();

        let mut scanned = 0usize;
        for n in nodes {
            if scanned >= self.max_nodes_for_sequences { break; }
            scanned += 1;

            if let Some(set) = node_syscalls.get(n.id.as_str()) {
                if set.contains("socket") {
                    if contains_any(n.id.as_str(), &targets_connect, &node_syscalls, &adj) {
                        seq_socket_connect += 1;
                    }
                }
                if set.contains("execve") || set.contains("execveat") {
                    if contains_any(n.id.as_str(), &targets_memctl, &node_syscalls, &adj) {
                        seq_exec_to_memctl += 1;
                    }
                }
            }
        }

        // ---- 3) TODO: map to FeatureObservation ----------------
        let _ = (
            total_syscall_tags,
            unique_syscalls,
            nodes_with_syscalls,
            interest_hits,
            seq_exec_to_memctl,
            seq_socket_connect,
            per_kind,
        );
        Vec::new()
    }
}

fn contains_any<'a>(
    nid: &'a str,
    targets: &HashSet<&str>,
    node_syscalls: &HashMap<&'a str, HashSet<String>>,
    adj: &HashMap<&'a str, Vec<&'a str>>,
) -> bool {
    if let Some(set) = node_syscalls.get(nid) {
        if set.iter().any(|s| targets.contains(s.as_str())) {
            return true;
        }
    }
    if let Some(neigh) = adj.get(nid) {
        for &m in neigh {
            if let Some(set) = node_syscalls.get(m) {
                if set.iter().any(|s| targets.contains(s.as_str())) {
                    return true;
                }
            }
        }
    }
    false
}

impl FeatureEmitter for SyscallEmitter {
    fn emit(
        &self,
        ep: &Episode,
        _baselines: &mut BaselineStore,
    ) -> Vec<crate::traits::FeatureObservation> {
        let (nodes, edges) = ep.to_graph();
        self.emit_core(&nodes, &edges)
    }
}
