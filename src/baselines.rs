use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Stat {
    n: f64,
    mean: f64,
    m2: f64,      // sum of squares of differences from the current mean
}

impl Stat {
    fn push(&mut self, x: f64) -> (f64, f64) {
        // Welford’s online algorithm
        self.n += 1.0;
        let delta = x - self.mean;
        self.mean += delta / self.n;
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;

        // return (mean, std)
        let var = if self.n > 1.0 { self.m2 / (self.n - 1.0) } else { 1e-9 };
        (self.mean, var.sqrt())
    }

    fn mean_std(&self) -> (f64, f64) {
        if self.n <= 1.0 { return (self.mean, 1e-9); }
        (self.mean, (self.m2 / (self.n - 1.0)).sqrt())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaselineStore {
    stats: HashMap<String, Stat>,
    #[serde(skip)]
    path: Option<String>,
}

impl BaselineStore {
    pub fn in_memory() -> Self { Self { stats: HashMap::new(), path: None } }

    pub fn open<P: AsRef<Path>>(p: P) -> std::io::Result<Self> {
        let path = p.as_ref().to_string_lossy().to_string();
        if let Ok(txt) = fs::read_to_string(&path) {
            if let Ok(mut me) = serde_json::from_str::<BaselineStore>(&txt) {
                me.path = Some(path);
                return Ok(me);
            }
        }
        Ok(Self { stats: HashMap::new(), path: Some(path) })
    }

    pub fn update_and_z(&mut self, key: &str, value: f64) -> (f64, f64, f64) {
        let st = self.stats.entry(key.to_string()).or_default();
        let (mu, sigma) = st.push(value);
        let z = if sigma <= 1e-9 { 0.0 } else { (value - mu) / sigma };
        (value, mu, z)
    }

    pub fn mean_std(&self, key: &str) -> (f64, f64) {
        self.stats.get(key).map(|s| s.mean_std()).unwrap_or((0.0, 1.0))
    }

    pub fn flush(&self) -> std::io::Result<()> {
        if let Some(path) = &self.path {
            let mut clone = self.clone();
            clone.path = None;
            let txt = serde_json::to_string_pretty(&clone).unwrap_or_else(|_| "{}".into());
            fs::create_dir_all(std::path::Path::new(path).parent().unwrap_or_else(|| std::path::Path::new(".")))?;
            fs::write(path, txt)?;
        }
        Ok(())
    }
}
