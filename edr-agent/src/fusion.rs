use std::collections::HashMap;

/// σ(x) helper
#[inline] fn sigmoid(x: f32) -> f32 { 1.0 / (1.0 + (-x).exp()) }

/// Monotonic logistic fusion: score = σ( Σ w_i * s_i + b ), w_i >= 0
#[derive(Clone)]
pub struct FusionModel {
    pub weights: HashMap<String, f32>,
    pub bias: f32,
    lr: f32,        // learning rate
    wmax: f32,      // clamp for guardrails
    l2: f32,        // weight decay
}

impl Default for FusionModel {
    fn default() -> Self {
        Self { weights: HashMap::new(), bias: 0.0, lr: 0.05, wmax: 3.0, l2: 0.0 }
    }
}

impl FusionModel {
    pub fn new(lr: f32, wmax: f32, l2: f32) -> Self {
        Self { lr: lr.clamp(1e-4, 0.5), wmax: wmax.max(0.1), l2: l2.max(0.0), ..Default::default() }
    }

    /// Compute fused risk from named signals in [0,1].
    pub fn score(&self, signals: &[(String, f32)]) -> f32 {
        let mut z = self.bias;
        for (name, s) in signals {
            let w = *self.weights.get(name).unwrap_or(&0.0);
            z += w * s.clamp(0.0, 1.0);
        }
        sigmoid(z).clamp(0.0, 1.0)
    }

    /// Online update from label y∈{0,1} with bounded gradient & non-negative weights.
    pub fn update(&mut self, signals: &[(String, f32)], y: f32) {
        let y = y.clamp(0.0, 1.0);
        let yhat = self.score(signals);
        let err = (yhat - y).clamp(-1.0, 1.0);

        // Bias update
        self.bias = (self.bias - self.lr * err).clamp(-5.0, 5.0);

        // Weight updates (monotonic: project to [0, wmax])
        for (name, s) in signals {
            let s = s.clamp(0.0, 1.0);
            let w = self.weights.entry(name.clone()).or_insert(0.0);
            let grad = err * s + self.l2 * *w; // L2 decay
            *w = (*w - self.lr * grad).max(0.0).min(self.wmax);
        }
    }

    /// Operator feedback: bump up/down named sources a little (bounded & monotonic).
    pub fn apply_operator_feedback(&mut self, up: &[String], down: &[String]) {
        let bump = 0.05 * self.lr.max(0.01);
        for k in up {
            let w = self.weights.entry(k.clone()).or_insert(0.0);
            *w = (*w + bump).min(self.wmax);
        }
        for k in down {
            let w = self.weights.entry(k.clone()).or_insert(0.0);
            *w = (*w - bump).max(0.0);
        }
    }

    /// Convenience: set hard weights (e.g., from config) but enforce constraints.
    pub fn set_weights(&mut self, w: &HashMap<String, f32>, bias: f32) {
        self.weights.clear();
        for (k, v) in w {
            self.weights.insert(k.clone(), v.max(0.0).min(self.wmax));
        }
        self.bias = bias.clamp(-5.0, 5.0);
    }
}
