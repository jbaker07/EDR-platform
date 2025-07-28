use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

/// Summary of a session's key telemetry traits
#[derive(Clone, Debug)]
pub struct SessionDigest {
    pub command_hash: String,
    pub connection_count: usize,
    pub privilege_escalation_count: usize,
    pub auth_event_count: usize,
    pub timestamp: u64,
}

/// Rolling cache to store recent session digests
pub struct DigestCache {
    cache: VecDeque<SessionDigest>,
    max_size: usize,
}

impl DigestCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    /// Add a new digest and evict old ones
    pub fn insert(&mut self, digest: SessionDigest) {
        if self.cache.len() >= self.max_size {
            self.cache.pop_front();
        }
        self.cache.push_back(digest);
    }

    /// Compute similarity to recent digests
    pub fn is_similar(&self, digest: &SessionDigest) -> bool {
        for prev in &self.cache {
            if digest.command_hash == prev.command_hash
                && digest.connection_count == prev.connection_count
                && digest.privilege_escalation_count == prev.privilege_escalation_count
            {
                return true;
            }
        }
        false
    }

    /// Get the most recent session digest (the last one in the queue).
    pub fn get_recent_session_digest(&self) -> Option<&SessionDigest> {
        self.cache.back() // Returns the last item (most recent) in the cache, or None if the cache is empty
    }
}

/// Build a SHA-256 hash from a set of commands
pub fn hash_command_sequence(commands: &[String]) -> String {
    let mut hasher = Sha256::new();
    for cmd in commands {
        hasher.update(cmd.as_bytes());
    }
    hex::encode(hasher.finalize())
}

/// Construct a digest from telemetry stats
pub fn generate_session_digest(
    commands: &[String],
    conn_count: usize,
    priv_esc_count: usize,
    auth_count: usize,
) -> SessionDigest {
    SessionDigest {
        command_hash: hash_command_sequence(commands),
        connection_count: conn_count,
        privilege_escalation_count: priv_esc_count,
        auth_event_count: auth_count,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    }
}
