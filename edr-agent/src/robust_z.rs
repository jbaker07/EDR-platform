use super::quantiles::Quantiles;

/// Robust Z-score using streaming quantiles.
/// z = 0.6745 * (x - median) / MAD
/// MAD ≈ 0.5 * IQR / 0.6745 when MAD unstable.
pub struct RobustZ {
    qs: Quantiles,
    epsilon: f64,
}

impl Default for RobustZ {
    fn default() -> Self {
        Self {
            qs: Quantiles::default(),
            epsilon: 1e-6,
        }
    }
}

impl RobustZ {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update with a new observation and return current robust z for that observation.
    pub fn update(&mut self, x: f64) -> f64 {
        // Compute z against current distribution, then insert (prevents immediate self-damping).
        let (median, p25, p75) = (
            self.qs.p50().unwrap_or(x),
            self.qs.p(0.25).unwrap_or(x),
            self.qs.p(0.75).unwrap_or(x),
        );

        // Approximate MAD via IQR if needed.
        let mad_est = (p75 - p25) / 1.349; // IQR / 1.349 ~ sigma (normal)
        let denom = (mad_est.abs()).max(self.epsilon);
        let z = 0.6745 * (x - median) / denom;

        self.qs.observe(x);
        z
    }

    pub fn count(&self) -> usize {
        self.qs.count()
    }
}
