use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

/// Streaming quantiles without external deps (reservoir sampling + nth-element).
#[derive(Clone)]
pub struct Quantiles {
    cap: usize,
    buf: Vec<f64>,
    seen: u64,
    rng: XorShift64,
}

impl Default for Quantiles {
    fn default() -> Self {
        Self::new(8192) // default reservoir size
    }
}

impl Quantiles {
    /// `compression` is treated as the reservoir capacity (min 512).
    pub fn new(compression: usize) -> Self {
        let cap = compression.max(512);
        let seed = {
            let t = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64;
            (t ^ (&cap as *const _ as u64)).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        };
        Self {
            cap,
            buf: Vec::with_capacity(cap),
            seen: 0,
            rng: XorShift64::new(seed),
        }
    }

    /// Observe one value (ignores NaN/±inf).
    pub fn observe(&mut self, x: f64) {
        if !x.is_finite() {
            return;
        }
        self.seen = self.seen.saturating_add(1);
        if self.buf.len() < self.cap {
            self.buf.push(x);
        } else {
            // Vitter's reservoir sampling (Algorithm R)
            let j = (self.rng.next_u64() % self.seen) as usize;
            if j < self.cap {
                self.buf[j] = x;
            }
        }
    }

    /// Quantile in [0,1]. Returns None if empty.
    pub fn p(&self, q: f64) -> Option<f64> {
        let mut v: Vec<f64> = self.buf.iter().copied().filter(|x| x.is_finite()).collect();
        if v.is_empty() {
            return None;
        }
        let q = q.clamp(0.0, 1.0);
        let k = ((v.len() as f64 - 1.0) * q).round() as usize;
        let (_, nth, _) =
            v.select_nth_unstable_by(k, |a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
        Some(*nth)
    }

    pub fn p50(&self) -> Option<f64> {
        self.p(0.50)
    }
    pub fn p75(&self) -> Option<f64> {
        self.p(0.75)
    }
    pub fn p90(&self) -> Option<f64> {
        self.p(0.90)
    }
    pub fn p95(&self) -> Option<f64> {
        self.p(0.95)
    }
    pub fn p99(&self) -> Option<f64> {
        self.p(0.99)
    }

    /// Number of finite samples currently in the reservoir.
    pub fn count(&self) -> usize {
        self.buf.iter().filter(|x| x.is_finite()).count()
    }
}

/// Minimal xorshift64 PRNG (no std rng dependency).
#[derive(Clone)]
struct XorShift64 {
    state: u64,
}
impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}
