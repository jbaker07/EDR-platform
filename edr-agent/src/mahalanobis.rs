// src/mahalanobis.rs
//! Streaming Mahalanobis distance with exponential decay, shrinkage, and ridge.
//! Also provides a compact, stable Episode vectorizer to feed the model.
//!
//! Public surface used in main.rs:
//!   - MahalaCfg (load / defaults)
//!   - Mahala::new / ::default
//!   - Mahala::vectorize_episode(&Episode) -> Vec<f64>
//!   - Mahala::observe(context, x, now)
//!   - Mahala::distance_and_damp(context, x, now) -> (d2, damp, support)

use chrono::{DateTime, Utc};
use nalgebra::{DMatrix, DVector};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

/// Tunables for the streaming estimator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MahalaCfg {
    /// Exponential half-life for statistics (seconds). Larger ⇒ slower adapt.
    #[serde(default = "MahalaCfg::d_half")]
    pub half_life_s: f64,
    /// Minimum effective support before distances are trusted.
    #[serde(default = "MahalaCfg::d_support")]
    pub min_support: u64,
    /// Diagonal ridge added to covariance (absolute).
    #[serde(default = "MahalaCfg::d_ridge")]
    pub ridge: f64,
    /// Ledoit–Wolf style shrinkage toward diagonal (0..1).
    #[serde(default = "MahalaCfg::d_shrink")]
    pub shrinkage_lambda: f64,
    /// Lower/upper bounds for damping factor that maps MD² → weight.
    #[serde(default = "MahalaCfg::d_dmin")]
    pub damp_min: f32,
    #[serde(default = "MahalaCfg::d_dmax")]
    pub damp_max: f32,
    /// Rough center of sigmoid (as a multiple of df ≈ feature dimension).
    #[serde(default = "MahalaCfg::d_center")]
    pub center_df_factor: f32,
    /// Slope of sigmoid for damping.
    #[serde(default = "MahalaCfg::d_k")]
    pub slope_k: f32,
}

impl MahalaCfg {
    pub fn d_half() -> f64 {
        6.0 * 3600.0
    }
    pub fn d_support() -> u64 {
        200
    }
    pub fn d_ridge() -> f64 {
        1e-6
    }
    pub fn d_shrink() -> f64 {
        0.15
    }
    pub fn d_dmin() -> f32 {
        0.75
    }
    pub fn d_dmax() -> f32 {
        1.05
    }
    pub fn d_center() -> f32 {
        1.0
    }
    pub fn d_k() -> f32 {
        0.7
    }

    /// Load from a JSON file (returns None if read/parse fails).
    pub fn load<P: AsRef<Path>>(p: P) -> Option<Self> {
        fs::read_to_string(p)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
    }

    /// Save to a JSON file (best-effort).
    pub fn save<P: AsRef<Path>>(&self, p: P) -> std::io::Result<()> {
        let s = serde_json::to_string_pretty(self).unwrap_or_default();
        fs::write(p, s)
    }
}

impl Default for MahalaCfg {
    fn default() -> Self {
        Self {
            half_life_s: Self::d_half(),
            min_support: Self::d_support(),
            ridge: Self::d_ridge(),
            shrinkage_lambda: Self::d_shrink(),
            damp_min: Self::d_dmin(),
            damp_max: Self::d_dmax(),
            center_df_factor: Self::d_center(),
            slope_k: Self::d_k(),
        }
    }
}

/// Internal per-context statistics (kept compact).
#[derive(Default, Clone, Serialize, Deserialize)]
struct CtxStats {
    mu: Vec<f64>,  // mean (length d)
    cov: Vec<f64>, // row-major d×d (E[(x-μ)(x-μ)^T]) under EMA
    last_ns: i64,  // last observation wall clock (ns)
    support: f64,  // effective decayed count
}

impl CtxStats {
    fn dim(&self) -> usize {
        self.mu.len()
    }
    fn reset(&mut self, d: usize, now_ns: i64) {
        self.mu = vec![0.0; d];
        self.cov = vec![0.0; d * d];
        self.last_ns = now_ns;
        self.support = 0.0;
    }
}

/// Streaming Mahalanobis with per-context stats.
pub struct Mahala {
    cfg: MahalaCfg,
    map: HashMap<String, CtxStats>, // context_id -> stats
}

impl Default for Mahala {
    fn default() -> Self {
        Self::new(MahalaCfg::default())
    }
}

impl Mahala {
    pub fn new(cfg: MahalaCfg) -> Self {
        Self {
            cfg,
            map: HashMap::new(),
        }
    }

    /// Replace runtime config (keeps current running stats).
    pub fn set_cfg(&mut self, cfg: MahalaCfg) {
        self.cfg = cfg;
    }

    /// Exponential decay factor for Δt seconds (half-life in cfg).
    fn exp_decay(&self, dt_s: f64) -> f64 {
        if self.cfg.half_life_s <= 0.0 {
            return 1.0;
        }
        let lambda = std::f64::consts::LN_2 / self.cfg.half_life_s;
        (-lambda * dt_s).exp()
    }

    /// Helper that doesn't borrow `self` (avoids borrow conflicts).
    #[inline]
    fn exp_decay_for(dt_s: f64, half_life_s: f64) -> f64 {
        if half_life_s <= 0.0 {
            return 1.0;
        }
        let lambda = std::f64::consts::LN_2 / half_life_s;
        (-lambda * dt_s).exp()
    }

    /// Observe one vector `x` into the named context at time `now`.
    /// Automatically handles dimension changes by reinitializing that context.
    pub fn observe(&mut self, context: &str, x: &[f64], now: DateTime<Utc>) {
        let d = x.len();
        if d == 0 {
            return;
        }
        let now_ns = now.timestamp_nanos_opt().unwrap_or(0);

        // Copy config scalar to avoid immutably borrowing `self` later.
        let half_life_s = self.cfg.half_life_s;

        // Get mutable entry
        let entry = self
            .map
            .entry(context.to_string())
            .or_insert_with(CtxStats::default);
        if entry.dim() != d {
            entry.reset(d, now_ns);
        }

        // Compute decay factor using locals only (no borrow of `self` now).
        let last_ns = entry.last_ns;
        let dt_s = ((now_ns - last_ns) as f64).max(0.0) / 1e9;
        let df = Mahala::exp_decay_for(dt_s, half_life_s);
        let alpha = 1.0 - df; // EMA weight for new sample

        // Decay historical support/cov, then add the new sample
        entry.support = entry.support * df + 1.0;

        // Update mean with EMA
        for j in 0..d {
            entry.mu[j] = (1.0 - alpha) * entry.mu[j] + alpha * x[j];
        }

        // Update covariance with EMA on (x-μ)(x-μ)^T
        // Using post-update μ is fine for EMA (consistent & stable).
        for i in 0..d {
            let di = x[i] - entry.mu[i];
            for j in 0..d {
                let dj = x[j] - entry.mu[j];
                let idx = i * d + j;
                entry.cov[idx] = (1.0 - alpha) * entry.cov[idx] + alpha * (di * dj);
            }
        }

        entry.last_ns = now_ns;
    }

    /// Compute MD² and a damping weight (for penalty calibration).
    /// Returns (d2, damp, support).
    ///
    /// - MD² uses covariance "view" decayed forward to `now` to avoid underestimation.
    /// - Shrinkage toward diagonal and ridge regularization stabilize inverse.
    pub fn distance_and_damp(
        &mut self,
        context: &str,
        x: &[f64],
        now: DateTime<Utc>,
    ) -> (f32, f32, u64) {
        let d = x.len();
        if d == 0 {
            return (0.0, 1.0, 0);
        }

        let st = match self.map.get(context) {
            Some(s) if s.dim() == d => s.clone(),
            _ => {
                return (0.0, 1.0, 0);
            }
        };

        // Decay to "now" for a fair covariance magnitude
        let now_ns = now.timestamp_nanos_opt().unwrap_or(0);
        let dt_s = ((now_ns - st.last_ns) as f64).max(0.0) / 1e9;
        let df = self.exp_decay(dt_s);

        // Build μ and Σ (decayed), then shrink + ridge
        let mu = DVector::<f64>::from_vec(st.mu);
        let mut cov = DMatrix::<f64>::from_row_slice(d, d, &st.cov);

        cov *= df; // forward-decay
        if self.cfg.shrinkage_lambda > 0.0 {
            let lam = self.cfg.shrinkage_lambda;
            // Diagonal formed from covariance diagonal
            let diag = DMatrix::<f64>::from_diagonal(&DVector::from_iterator(
                d,
                (0..d).map(|i| cov[(i, i)]),
            ));
            cov = (1.0 - lam) * cov + lam * diag;
        }
        for i in 0..d {
            cov[(i, i)] += self.cfg.ridge;
        }

        // Safe inverse
        let inv = match cov.try_inverse() {
            Some(m) => m,
            None => {
                return (0.0, 1.0, st.support as u64);
            }
        };

        let xvec = DVector::<f64>::from_column_slice(x);
        let delta = xvec - mu;
        let d2 = (delta.transpose() * inv * delta)[(0, 0)]
            .max(0.0)
            .min(f64::MAX) as f32;

        // Damping sigmoid: map MD² to [damp_min, damp_max]
        let df_f = d as f32; // degrees of freedom proxy
        let center = self.cfg.center_df_factor * df_f;
        let k = self.cfg.slope_k;
        let sig = 1.0 / (1.0 + (-k * (center - d2)).exp());
        let damp = self.cfg.damp_min + (self.cfg.damp_max - self.cfg.damp_min) * sig;

        (d2, damp, st.support as u64)
    }

    /// Export a JSON snapshot of a context’s current stats (debug/inspection).
    pub fn export_context_snapshot(&self, context: &str) -> Option<String> {
        self.map
            .get(context)
            .and_then(|st| serde_json::to_string_pretty(st).ok())
    }

    /// Clear a context (e.g., if you rotate roles or trust that the baseline drifted).
    pub fn reset_context(&mut self, context: &str) {
        self.map.remove(context);
    }

    // -----------------------------------------------------------------------------------------
    // Episode vectorizer: stable, low-dimensional features for MD. Tweak as needed.
    // -----------------------------------------------------------------------------------------

    /// Feature names for the current vectorizer (stable ordering).
    pub fn feature_names() -> &'static [&'static str] {
        &[
            "evt_total",
            "bytes_file_to_pipe_log1p",
            "bytes_pipe_to_sock_log1p",
            "mprotect_to_exec_score",
            "memfd_to_exec_score",
            "memfd_bytes_log1p",
            "setuid_count",
            "capset_count",
            "seccomp_relax_count",
            "ptrace_count",
            "kernel_cfg_flip_count",
        ]
    }

    /// Build a compact numeric representation from an Episode.
    /// Keep this small and robust; log-scale large magnitudes.
    pub fn vectorize_episode(ep: &crate::episode::Episode) -> Vec<f64> {
        use crate::event::EventType;
        let mut total = 0_u32;
        let mut f2p_bytes = 0_u64;
        let mut p2s_bytes = 0_u64;
        let mut memfd_bytes = 0_u64;

        let mut wtox_peak = 0.0_f64; // heat for W→X near exec
        let mut memfd2exec_score = 0.0_f64;

        let mut setuid = 0_u32;
        let mut capset = 0_u32;
        let mut seccomp = 0_u32;
        let mut ptrace = 0_u32;
        let mut kflip = 0_u32;

        let mut last_wtox_ts = None;
        let mut memfd_open_ts: Option<chrono::DateTime<chrono::Utc>> = None;
        let mut memfd_write_acc: u64 = 0;
        let mut memfd_exec_scored = false;

        for ev in &ep.events {
            total += 1;

            match &ev.event_type {
                EventType::FileToPipeBytes => {
                    f2p_bytes += ev.bytes.unwrap_or(0);
                }
                EventType::PipeToSocketBytes => {
                    p2s_bytes += ev.bytes.unwrap_or(0);
                }

                // Memory perms flips can precede exec
                EventType::Mprotect | EventType::Mmap => {
                    if let Some(p) = &ev.prot {
                        let px = p.contains('X') || p.to_ascii_uppercase().contains("PROT_EXEC");
                        if px {
                            last_wtox_ts = Some(ev.ts);
                        }
                    }
                }

                EventType::Execve | EventType::Fexecve => {
                    if let Some(t) = last_wtox_ts {
                        let dt = (ev.ts - t).num_seconds().abs() as f64;
                        let score = (5.0 - dt).max(0.0);
                        if score > wtox_peak {
                            wtox_peak = score;
                        }
                    }
                    if let Some(t0) = memfd_open_ts {
                        let dt = (ev.ts - t0).num_seconds().abs() as f64;
                        if dt <= 120.0 && !memfd_exec_scored {
                            let bytes = memfd_write_acc as f64;
                            let byte_boost = if bytes > 0.0 {
                                bytes.ln().max(1.0)
                            } else {
                                1.0
                            };
                            let score = ((120.0 - dt).max(0.0) / 120.0) * byte_boost.min(16.0);
                            memfd2exec_score = memfd2exec_score.max(score);
                            memfd_exec_scored = true;
                        }
                    }
                }

                EventType::MemfdCreate => {
                    memfd_open_ts = Some(ev.ts);
                    memfd_write_acc = 0;
                    memfd_exec_scored = false;
                }
                EventType::MemfdWrite => {
                    let b = ev.bytes.unwrap_or(0);
                    memfd_write_acc += b;
                    memfd_bytes += b;
                }

                EventType::Setuid => setuid += 1,
                EventType::Capset => capset += 1,
                EventType::SeccompRelax => seccomp += 1,
                EventType::Ptrace => ptrace += 1,
                EventType::KernelCfgFlip => kflip += 1,

                _ => {}
            }
        }

        fn log1p_u(v: u64) -> f64 {
            (v as f64 + 1.0).ln()
        }

        vec![
            total as f64,
            log1p_u(f2p_bytes),
            log1p_u(p2s_bytes),
            wtox_peak,
            memfd2exec_score,
            log1p_u(memfd_bytes),
            setuid as f64,
            capset as f64,
            seccomp as f64,
            ptrace as f64,
            kflip as f64,
        ]
    }

    // -----------------------------------------------------------------------------------------
    // Optional helpers
    // -----------------------------------------------------------------------------------------

    /// Local chi-square quantile approximation (optional; you can prefer the one in elliptic_envelope).
    /// Returns q such that P[ChiSq(df) <= q] ≈ p. Valid for df >= 1, 0<p<1.
    pub fn chi_square_quantile_approx_local(df: usize, p: f64) -> f64 {
        // Wilson–Hilferty / Beasley-Springer-Moro style approximation on normal quantile
        // Simple, fast, good enough for gates.
        fn invnorm(u: f64) -> f64 {
            // Acklam's rational approximation
            let a: [f64; 6] = [
                -3.969683028665376e+01,
                2.209460984245205e+02,
                -2.759285104469687e+02,
                1.383577518672690e+02,
                -3.066479806614716e+01,
                2.506628277459239e+00,
            ];
            let b: [f64; 5] = [
                -5.447609879822406e+01,
                1.615858368580409e+02,
                -1.556989798598866e+02,
                6.680131188771972e+01,
                -1.328068155288572e+01,
            ];
            let c: [f64; 6] = [
                -7.784894002430293e-03,
                -3.223964580411365e-01,
                -2.400758277161838e+00,
                -2.549732539343734e+00,
                4.374664141464968e+00,
                2.938163982698783e+00,
            ];
            let d: [f64; 4] = [
                7.784695709041462e-03,
                3.224671290700398e-01,
                2.445134137142996e+00,
                3.754408661907416e+00,
            ];
            let pl = 0.02425;
            let pu = 1.0 - pl;
            let q = if u < pl {
                // lower region
                let q = (-2.0 * u.ln()).sqrt();
                (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
                    / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
            } else if u > pu {
                let q = (-2.0 * (1.0 - u).ln()).sqrt();
                -(((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
                    / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
            } else {
                let q = u - 0.5;
                let r = q * q;
                (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q
                    / (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0)
            };
            q
        }

        let k = df as f64;
        let z = invnorm(p.clamp(1e-9, 1.0 - 1e-9));
        // Wilson–Hilferty transform
        let a = 2.0 / (9.0 * k);
        k * (1.0 - a + z * (a).sqrt()).powi(3)
    }
}
