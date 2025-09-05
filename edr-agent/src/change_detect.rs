/// Page–Hinkley test for change detection.
/// If (m_t - m_min) > lambda => drift.
pub struct PageHinkley {
    lambda: f64,
    delta: f64,   // small tolerance (insensitivity)
    alpha: f64,   // forgetting factor in (0,1]; 1=no forget
    mean: f64,
    cum: f64,
    min_cum: f64,
    initialized: bool,
}

pub struct Drift {
    pub magnitude: f64,
}

impl PageHinkley {
    pub fn new(lambda: f64, delta: f64, alpha: f64) -> Self {
        Self {
            lambda: lambda.max(1e-6),
            delta,
            alpha: alpha.clamp(0.5, 1.0),
            mean: 0.0,
            cum: 0.0,
            min_cum: 0.0,
            initialized: false,
        }
    }

    pub fn update(&mut self, x: f64) -> Option<Drift> {
        if !self.initialized {
            self.mean = x;
            self.cum = 0.0;
            self.min_cum = 0.0;
            self.initialized = true;
            return None;
        }
        // EW mean
        self.mean = self.alpha * self.mean + (1.0 - self.alpha) * x;

        // Cumulative sum of deviations
        let dev = x - self.mean - self.delta;
        self.cum = (self.cum + dev).min(1e6).max(-1e6);
        self.min_cum = self.min_cum.min(self.cum);

        let diff = self.cum - self.min_cum;
        if diff > self.lambda {
            // reset after drift (optional)
            self.cum = 0.0;
            self.min_cum = 0.0;
            return Some(Drift { magnitude: diff });
        }
        None
    }
}
