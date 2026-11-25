// src/pb_adapter/queue.rs
// The bounded queue between the decoders and the playbook engine.
//
// Producers are the ring buffer reader threads and the 5 s telemetry sweep. When the
// queue is full a producer waits up to the push timeout for the engine thread to make
// room. While it waits it does not read its ring, so the kernel ring buffer (16 MiB for
// the shared ring) holds the next frames instead of the agent dropping facts. Only a
// fact that still finds no room after the timeout is dropped, and every drop is
// counted and shown on /metrics and /playbooks/status.
//
// The consumer is one OS thread (runner.rs) that takes up to BATCH facts at a time, so
// the mapper and the engine are locked once per batch, not once per fact.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};

use serde::Serialize;

pub const DEFAULT_CAPACITY: usize = 32_768;
pub const DEFAULT_PUSH_TIMEOUT: Duration = Duration::from_millis(250);
pub const BATCH: usize = 1024;

pub struct FactQueue<T> {
    items: Mutex<VecDeque<T>>,
    not_empty: Condvar,
    not_full: Condvar,
    capacity: usize,
    push_timeout: Duration,
    /// False until a consumer runs. Without one a full queue drops at once instead of
    /// making every producer wait for nothing.
    consumer_alive: AtomicBool,
    pushed: AtomicU64,
    dropped: AtomicU64,
    waited: AtomicU64,
    high_water: AtomicU64,
    batches: AtomicU64,
    /// Set when a push waited the whole timeout and found no room. While it is set a
    /// full queue drops at once instead of making every producer wait again, so a
    /// consumer that stalls costs one timeout, not one per item: a ring reader blocked
    /// 250 ms per record lets the kernel ring overflow, which loses the playbook facts
    /// on the same ring too. Cleared when the consumer has drained the queue to half.
    stalled: AtomicBool,
    stalls: AtomicU64,
}

/// Counters for the API.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct QueueStats {
    pub capacity: usize,
    pub depth: usize,
    pub high_water: u64,
    pub pushed_total: u64,
    pub dropped_total: u64,
    pub waited_total: u64,
    pub batches_total: u64,
    pub push_timeout_ms: u64,
    pub consumer_alive: bool,
    /// Times the consumer fell so far behind that a push timed out. Each one switches
    /// the queue to dropping at once until the consumer has drained it to half.
    pub stalls_total: u64,
}

impl<T> FactQueue<T> {
    pub fn new(capacity: usize, push_timeout: Duration) -> Self {
        Self {
            items: Mutex::new(VecDeque::with_capacity(capacity.min(4096))),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
            capacity: capacity.max(1),
            push_timeout,
            consumer_alive: AtomicBool::new(false),
            pushed: AtomicU64::new(0),
            dropped: AtomicU64::new(0),
            waited: AtomicU64::new(0),
            high_water: AtomicU64::new(0),
            batches: AtomicU64::new(0),
            stalled: AtomicBool::new(false),
            stalls: AtomicU64::new(0),
        }
    }

    pub fn set_consumer_alive(&self, alive: bool) {
        self.consumer_alive.store(alive, Ordering::Relaxed);
        self.not_full.notify_all();
    }

    /// Adds one item. Waits up to the push timeout while the queue is full and a
    /// consumer runs. Returns false, and counts a drop, when there was still no room.
    pub fn push(&self, item: T) -> bool {
        let mut q = self.items.lock().unwrap_or_else(|e| e.into_inner());
        if q.len() >= self.capacity {
            if !self.consumer_alive.load(Ordering::Relaxed) || self.stalled.load(Ordering::Relaxed) {
                self.dropped.fetch_add(1, Ordering::Relaxed);
                return false;
            }
            self.waited.fetch_add(1, Ordering::Relaxed);
            let deadline = Instant::now() + self.push_timeout;
            while q.len() >= self.capacity {
                let left = deadline.saturating_duration_since(Instant::now());
                if left.is_zero() {
                    self.dropped.fetch_add(1, Ordering::Relaxed);
                    if !self.stalled.swap(true, Ordering::Relaxed) {
                        self.stalls.fetch_add(1, Ordering::Relaxed);
                    }
                    return false;
                }
                q = self
                    .not_full
                    .wait_timeout(q, left)
                    .unwrap_or_else(|e| e.into_inner())
                    .0;
            }
        }
        q.push_back(item);
        let depth = q.len() as u64;
        drop(q);
        self.pushed.fetch_add(1, Ordering::Relaxed);
        self.high_water.fetch_max(depth, Ordering::Relaxed);
        self.not_empty.notify_one();
        true
    }

    /// Takes up to `max` items, waiting up to `wait` for the first one. An empty Vec
    /// means the wait ran out.
    pub fn pop_batch(&self, max: usize, wait: Duration) -> Vec<T> {
        let mut q = self.items.lock().unwrap_or_else(|e| e.into_inner());
        if q.is_empty() {
            q = self
                .not_empty
                .wait_timeout_while(q, wait, |q| q.is_empty())
                .unwrap_or_else(|e| e.into_inner())
                .0;
        }
        let n = q.len().min(max);
        let out: Vec<T> = q.drain(..n).collect();
        if q.len() <= self.capacity / 2 {
            self.stalled.store(false, Ordering::Relaxed);
        }
        drop(q);
        if !out.is_empty() {
            self.batches.fetch_add(1, Ordering::Relaxed);
            self.not_full.notify_all();
        }
        out
    }

    pub fn stats(&self) -> QueueStats {
        let depth = self.items.lock().map(|q| q.len()).unwrap_or(0);
        QueueStats {
            capacity: self.capacity,
            depth,
            high_water: self.high_water.load(Ordering::Relaxed),
            pushed_total: self.pushed.load(Ordering::Relaxed),
            dropped_total: self.dropped.load(Ordering::Relaxed),
            waited_total: self.waited.load(Ordering::Relaxed),
            batches_total: self.batches.load(Ordering::Relaxed),
            push_timeout_ms: self.push_timeout.as_millis() as u64,
            consumer_alive: self.consumer_alive.load(Ordering::Relaxed),
            stalls_total: self.stalls.load(Ordering::Relaxed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn a_full_queue_waits_for_the_consumer_instead_of_dropping() {
        let q = Arc::new(FactQueue::new(4, Duration::from_secs(5)));
        q.set_consumer_alive(true);
        let consumer = {
            let q = q.clone();
            std::thread::spawn(move || {
                let mut got = Vec::new();
                while got.len() < 10_000 {
                    // A slow consumer: small batches with a pause.
                    std::thread::sleep(Duration::from_micros(50));
                    got.extend(q.pop_batch(3, Duration::from_millis(100)));
                }
                got
            })
        };
        for i in 0..10_000u32 {
            assert!(q.push(i));
        }
        let got = consumer.join().unwrap();
        assert_eq!(got, (0..10_000).collect::<Vec<_>>(), "every item, in order");
        let st = q.stats();
        assert_eq!(st.dropped_total, 0);
        assert!(st.waited_total > 0, "the producer had to wait: {st:?}");
        assert!(st.high_water <= 4);
    }

    #[test]
    fn a_stuck_consumer_turns_into_counted_drops_after_the_timeout() {
        let q = FactQueue::new(2, Duration::from_millis(20));
        q.set_consumer_alive(true);
        assert!(q.push(1));
        assert!(q.push(2));
        let t = Instant::now();
        assert!(!q.push(3));
        assert!(t.elapsed() >= Duration::from_millis(20));
        assert_eq!(q.stats().dropped_total, 1);
        // Without a consumer a full queue drops at once.
        q.set_consumer_alive(false);
        let t = Instant::now();
        assert!(!q.push(4));
        assert!(t.elapsed() < Duration::from_millis(20));
        assert_eq!(q.stats().dropped_total, 2);
        assert_eq!(q.pop_batch(10, Duration::ZERO), vec![1, 2]);
    }

    #[test]
    fn after_one_timeout_a_full_queue_drops_at_once_until_drained() {
        let q = FactQueue::new(4, Duration::from_millis(30));
        q.set_consumer_alive(true);
        for i in 0..4 {
            assert!(q.push(i));
        }
        let t = Instant::now();
        assert!(!q.push(4), "the first push waits the timeout");
        assert!(t.elapsed() >= Duration::from_millis(30));
        let t = Instant::now();
        for i in 5..100 {
            assert!(!q.push(i));
        }
        assert!(t.elapsed() < Duration::from_millis(30), "later pushes do not wait");
        assert_eq!(q.stats().stalls_total, 1);
        assert_eq!(q.stats().dropped_total, 96);
        // Draining one item leaves it above half: still dropping at once.
        assert_eq!(q.pop_batch(1, Duration::ZERO), vec![0]);
        assert!(q.push(100), "there is room again");
        let t = Instant::now();
        assert!(!q.push(101));
        assert!(t.elapsed() < Duration::from_millis(30));
        // Drained to half: a full queue makes producers wait again.
        assert_eq!(q.pop_batch(2, Duration::ZERO), vec![1, 2]);
        assert!(q.push(102));
        assert!(q.push(103));
        let t = Instant::now();
        assert!(!q.push(104));
        assert!(t.elapsed() >= Duration::from_millis(30));
        assert_eq!(q.stats().stalls_total, 2);
    }
}
