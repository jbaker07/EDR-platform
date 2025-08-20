use nalgebra::{DMatrix, DVector};
use rand::prelude::*;
use rand::seq::SliceRandom;

/// Minimal robust covariance via random subsampling + coordinate-wise medians.
pub struct EllipticEnvelope {
    pub center: DVector<f64>,
    pub cov_inv: DMatrix<f64>,
    /// decision boundary on squared Mahalanobis distance
    pub threshold: f64,
}

impl EllipticEnvelope {
    pub fn fit(baseline: &Vec<Vec<f64>>, subsamples: usize, alpha: f64) -> Option<Self> {
        if baseline.len() < 8 || baseline[0].is_empty() { return None; }
        let n = baseline.len();
        let d = baseline[0].len();
        if baseline.iter().any(|row| row.len() != d) { return None; }

        let mut rng = thread_rng();
        let mut covs = Vec::new();
        let mut means = Vec::new();
        let subsamples = subsamples.max(1);

        for _ in 0..subsamples {
            let mut idx: Vec<usize> = (0..n).collect();
            idx.shuffle(&mut rng);
            let take = ((n as f64) * 0.6).ceil() as usize;
            let take = take.clamp(2, n);
            let sub: Vec<&Vec<f64>> = idx.iter().take(take).map(|i| &baseline[*i]).collect();

            let m = mean_vec(&sub, d);
            let c = cov_mat(&sub, &m, d);
            if let Some(inv) = c.clone().try_inverse() {
                covs.push(inv);
                means.push(m);
            }
        }
        if covs.is_empty() { return None; }

        let cov_inv = median_matrix(&covs);
        let center  = median_vector(&means);

        let mut dists = Vec::with_capacity(n);
        for x in baseline {
            let v = DVector::from_vec(x.clone());
            let diff = &v - &center;
            let md2 = diff.transpose() * &cov_inv * diff;
            dists.push(md2[(0, 0)]);
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let alpha = alpha.clamp(0.0, 1.0);
        let k = ((dists.len() - 1) as f64 * alpha).round() as usize;
        let threshold = dists[k];

        Some(Self { center, cov_inv, threshold })
    }

    /// Positive => inlier, Negative => outlier
    pub fn decision_function(&self, x: &[f64]) -> f64 {
        let v = DVector::from_vec(x.to_vec());
        let diff = &v - &self.center;
        let md2 = diff.transpose() * &self.cov_inv * diff;
        self.threshold - md2[(0, 0)]
    }
}

// — helpers —
fn mean_vec(sub: &Vec<&Vec<f64>>, d: usize) -> DVector<f64> {
    let m = sub.len().max(1) as f64;
    let mut acc = vec![0.0f64; d];
    for row in sub {
        for j in 0..d { acc[j] += row[j]; }
    }
    for j in 0..d { acc[j] /= m; }
    DVector::from_vec(acc)
}

fn cov_mat(sub: &Vec<&Vec<f64>>, mean: &DVector<f64>, d: usize) -> DMatrix<f64> {
    let m = sub.len();
    let denom = ((m as isize) - 1).max(1) as f64;
    let mut c = DMatrix::<f64>::zeros(d, d);
    for row in sub {
        let v = DVector::from_vec(row.clone().to_vec());
        let diff = &v - mean;
        c += &diff * diff.transpose();
    }
    c /= denom;
    let ridge = 1e-8;
    for i in 0..d { c[(i, i)] += ridge; }
    c
}

fn median_vector(vs: &Vec<DVector<f64>>) -> DVector<f64> {
    let d = vs[0].len();
    let mut out = vec![0.0f64; d];
    for j in 0..d {
        let mut col: Vec<f64> = vs.iter().map(|v| v[j]).collect();
        out[j] = median_scalar(&mut col);
    }
    DVector::from_vec(out)
}

fn median_matrix(ms: &Vec<DMatrix<f64>>) -> DMatrix<f64> {
    let (r, c) = ms[0].shape();
    let mut out = DMatrix::<f64>::zeros(r, c);
    for i in 0..r {
        for j in 0..c {
            let mut vals: Vec<f64> = ms.iter().map(|m| m[(i, j)]).collect();
            out[(i, j)] = median_scalar(&mut vals);
        }
    }
    out
}

fn median_scalar(v: &mut [f64]) -> f64 {
    if v.is_empty() { return 0.0; }
    let mid = v.len() / 2;
    v.select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if v.len() % 2 == 1 { v[mid] } else {
        // average neighbors
        let mut lo = mid.saturating_sub(1);
        let mut hi = mid;
        if lo >= v.len() { lo = mid; }
        if hi >= v.len() { hi = mid; }
        (v[lo] + v[hi]) * 0.5
    }
}
