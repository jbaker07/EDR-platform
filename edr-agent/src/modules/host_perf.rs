// src/modules/host_perf.rs
// CPU samples for the HostPerf playbook facts. Every batch loop pass (5 s) reads
// /proc/stat for the host and /proc/<pid>/stat for every process, and sends one sample
// for the host and one for each process that used at least EDR_HOST_PERF_MIN_PCT of a
// core (default 50) since the last pass. cpu_pct is 100 for one full core, so a miner
// pinning one core of a 24 core host shows 100 even though the host is mostly idle.
//
// GPUs: every pass also reads each DRM card's busy percent from sysfs
// (/sys/class/drm/card*/device/gpu_busy_percent, which the amdgpu driver exposes) and,
// when nvidia-smi is installed, the utilization it reports for each NVIDIA
// GPU. Each GPU gives one sample with scope gpu. Neither source says which process
// keeps the GPU busy, so gpu samples have no pid.
//
// The samples go straight to the playbook adapter, not to the telemetry files, because a
// busy host would add a row per process every 5 s and no reader uses them. The sampler
// runs on its own thread: the batch loop can take far longer than 5 s per pass under a
// burst (about 11 s on average in the live run), and a sustained slot breaks its run
// when samples are more than 15 s apart.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use crate::telemetry_types::TelemetryOutput;

/// CPU time of one process, in clock ticks.
fn proc_ticks(stat: &str) -> Option<(String, u64)> {
    // comm is in parentheses and may hold spaces, so split after the last ')'.
    let open = stat.find('(')?;
    let close = stat.rfind(')')?;
    let comm = stat.get(open + 1..close)?.to_string();
    let rest: Vec<&str> = stat.get(close + 2..)?.split_whitespace().collect();
    // After the comm: state is field 3, utime field 14 and stime field 15.
    let utime = rest.get(11)?.parse::<u64>().ok()?;
    let stime = rest.get(12)?.parse::<u64>().ok()?;
    Some((comm, utime + stime))
}

/// (busy, user, total) jiffies from the first line of /proc/stat.
fn host_ticks(stat: &str) -> Option<(u64, u64, u64)> {
    let line = stat.lines().find(|l| l.starts_with("cpu "))?;
    let v: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .filter_map(|x| x.parse().ok())
        .collect();
    if v.len() < 4 {
        return None;
    }
    let total: u64 = v.iter().take(8).sum();
    let idle = v[3] + v.get(4).copied().unwrap_or(0);
    let user = v[0] + v[1];
    Some((total.saturating_sub(idle), user, total))
}

/// Busy percent of each card under a DRM sysfs directory.
fn drm_gpu_busy(drm: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(drm) else {
        return out;
    };
    let mut cards: Vec<_> = entries
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        // card0, card1: not the connector entries such as card0-HDMI-A-1.
        .filter(|n| n.strip_prefix("card").map(|r| !r.is_empty() && r.bytes().all(|b| b.is_ascii_digit())).unwrap_or(false))
        .collect();
    cards.sort();
    for card in cards {
        if let Ok(v) = fs::read_to_string(drm.join(&card).join("device/gpu_busy_percent")) {
            if let Ok(pct) = v.trim().parse::<f64>() {
                out.push((card, format!("{pct:.1}")));
            }
        }
    }
    out
}

/// `nvidia-smi --query-gpu=index,utilization.gpu --format=csv,noheader,nounits` lines.
fn parse_nvidia_smi(out: &str) -> Vec<(String, String)> {
    out.lines()
        .filter_map(|l| {
            let (idx, pct) = l.split_once(',')?;
            let pct = pct.trim().parse::<f64>().ok()?;
            Some((format!("nvidia{}", idx.trim()), format!("{pct:.1}")))
        })
        .collect()
}

fn nvidia_gpu_busy() -> Vec<(String, String)> {
    static PATH: OnceLock<Option<&'static str>> = OnceLock::new();
    let Some(bin) = *PATH.get_or_init(|| {
        ["/usr/bin/nvidia-smi", "/usr/local/bin/nvidia-smi"]
            .into_iter()
            .find(|p| Path::new(p).exists())
    }) else {
        return Vec::new();
    };
    std::process::Command::new(bin)
        .args(["--query-gpu=index,utilization.gpu", "--format=csv,noheader,nounits"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| parse_nvidia_smi(&String::from_utf8_lossy(&o.stdout)))
        .unwrap_or_default()
}

pub struct Sampler {
    root: String,
    /// DRM sysfs directory for GPU busy percent, /sys/class/drm on a host.
    drm: String,
    /// Ask nvidia-smi too. Off in tests.
    nvidia: bool,
    min_pct: f64,
    tick_hz: f64,
    ncpu: usize,
    last_host: Option<(u64, u64, u64)>,
    last_procs: HashMap<u32, u64>,
    last_at: Option<Instant>,
}

impl Sampler {
    pub fn new(root: &str, min_pct: f64) -> Self {
        // SAFETY: sysconf has no preconditions.
        let hz = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
        let ncpu = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        Self {
            root: root.to_string(),
            drm: "/sys/class/drm".to_string(),
            nvidia: true,
            min_pct,
            tick_hz: if hz > 0 { hz as f64 } else { 100.0 },
            ncpu,
            last_host: None,
            last_procs: HashMap::new(),
            last_at: None,
        }
    }

    /// Samples since the last call. The first call only sets the baseline.
    pub fn sample(&mut self, skip_pid: u32) -> Vec<TelemetryOutput> {
        let now = Instant::now();
        let elapsed = self.last_at.map(|t| now.duration_since(t).as_secs_f64());
        self.last_at = Some(now);
        let mut out = Vec::new();

        let host = fs::read_to_string(format!("{}/stat", self.root))
            .ok()
            .and_then(|s| host_ticks(&s));
        if let (Some((busy, user, total)), Some((pb, pu, pt))) = (host, self.last_host) {
            let dt = total.saturating_sub(pt).max(1) as f64;
            let mut d = HashMap::new();
            d.insert("scope".to_string(), "host".to_string());
            d.insert("cpu_busy_pct".to_string(), format!("{:.1}", busy.saturating_sub(pb) as f64 * 100.0 / dt));
            d.insert("cpu_user_pct".to_string(), format!("{:.1}", user.saturating_sub(pu) as f64 * 100.0 / dt));
            d.insert("ncpu".to_string(), self.ncpu.to_string());
            out.push(output(d));
        }
        self.last_host = host;

        let mut gpus = drm_gpu_busy(Path::new(&self.drm));
        if self.nvidia {
            gpus.extend(nvidia_gpu_busy());
        }
        for (gpu, pct) in gpus {
            let mut d = HashMap::new();
            d.insert("scope".to_string(), "gpu".to_string());
            d.insert("gpu".to_string(), gpu);
            d.insert("gpu_util_pct".to_string(), pct);
            out.push(output(d));
        }

        let mut seen = HashMap::new();
        if let Ok(entries) = fs::read_dir(&self.root) {
            for e in entries.flatten() {
                let Some(pid) = e.file_name().to_str().and_then(|n| n.parse::<u32>().ok()) else {
                    continue;
                };
                if pid == skip_pid {
                    continue;
                }
                let Some((comm, ticks)) = fs::read_to_string(e.path().join("stat"))
                    .ok()
                    .and_then(|s| proc_ticks(&s))
                else {
                    continue;
                };
                seen.insert(pid, ticks);
                let (Some(secs), Some(prev)) = (elapsed, self.last_procs.get(&pid)) else {
                    continue;
                };
                let pct = ticks.saturating_sub(*prev) as f64 / self.tick_hz / secs.max(0.001) * 100.0;
                if pct < self.min_pct {
                    continue;
                }
                let mut d = HashMap::new();
                d.insert("scope".to_string(), "process".to_string());
                d.insert("pid".to_string(), pid.to_string());
                d.insert("comm".to_string(), comm);
                d.insert("cpu_pct".to_string(), format!("{pct:.1}"));
                if let Ok(exe) = fs::read_link(Path::new(&self.root).join(pid.to_string()).join("exe")) {
                    d.insert("binary_path".to_string(), exe.to_string_lossy().into_owned());
                }
                out.push(output(d));
            }
        }
        self.last_procs = seen;
        out
    }
}

fn output(mut d: HashMap<String, String>) -> TelemetryOutput {
    d.insert("source".to_string(), "host_perf".to_string());
    TelemetryOutput {
        category: "perf".into(),
        signal: "host_perf::sample".into(),
        confidence: 1.0,
        data: d,
    }
}

/// One pass of the agent's sampler, straight into the playbook adapter.
pub fn sample_and_emit() {
    static SAMPLER: OnceLock<Mutex<Sampler>> = OnceLock::new();
    let min = std::env::var("EDR_HOST_PERF_MIN_PCT")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(50.0);
    let s = SAMPLER.get_or_init(|| Mutex::new(Sampler::new("/proc", min)));
    let outputs = match s.lock() {
        Ok(mut s) => s.sample(std::process::id()),
        Err(_) => return,
    };
    for o in &outputs {
        crate::pb_adapter::emit_adapter_facts(o);
    }
}

/// Starts the sampler thread: one pass every 5 s.
pub fn spawn() {
    let _ = std::thread::Builder::new().name("host-perf".into()).spawn(|| loop {
        let t = std::time::Instant::now();
        sample_and_emit();
        std::thread::sleep(std::time::Duration::from_secs(5).saturating_sub(t.elapsed()));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stat_lines_parse() {
        let stat = "4242 (my miner) R 1 4242 4242 0 -1 4194560 100 0 0 0 750 250 0 0 20 0 1 0 12345 1000 10";
        assert_eq!(proc_ticks(stat), Some(("my miner".to_string(), 1000)));
        let host = "cpu  100 5 50 800 20 0 5 0 0 0\ncpu0 1 1 1 1 1 1 1 1\n";
        // busy = total 980 - idle 820, user = 105.
        assert_eq!(host_ticks(host), Some((160, 105, 980)));
    }

    #[test]
    fn gpu_busy_comes_from_drm_cards_and_nvidia_smi() {
        let drm = std::env::temp_dir().join(format!("edr_drm_{}", std::process::id()));
        let _ = fs::remove_dir_all(&drm);
        for (card, busy) in [("card0", Some("87\n")), ("card1", None), ("card0-HDMI-A-1", Some("5"))] {
            let d = drm.join(card).join("device");
            fs::create_dir_all(&d).unwrap();
            if let Some(b) = busy {
                fs::write(d.join("gpu_busy_percent"), b).unwrap();
            }
        }
        assert_eq!(drm_gpu_busy(&drm), vec![("card0".to_string(), "87.0".to_string())]);
        assert_eq!(
            parse_nvidia_smi("0, 91\n1, 3\nbad line\n"),
            vec![("nvidia0".to_string(), "91.0".to_string()), ("nvidia1".to_string(), "3.0".to_string())]
        );
        let root = std::env::temp_dir().join(format!("edr_perf_gpu_{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let mut s = Sampler::new(root.to_str().unwrap(), 50.0);
        s.drm = drm.to_string_lossy().into_owned();
        s.nvidia = false;
        let out = s.sample(0);
        let gpu: Vec<_> = out.iter().filter(|o| o.data["scope"] == "gpu").collect();
        assert_eq!(gpu.len(), 1);
        assert_eq!(gpu[0].data["gpu_util_pct"], "87.0");
        let _ = fs::remove_dir_all(&drm);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn a_busy_process_gives_a_sample_and_an_idle_one_does_not() {
        let root = std::env::temp_dir().join(format!("edr_perf_{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let write = |pid: u32, ticks: u64| {
            let d = root.join(pid.to_string());
            fs::create_dir_all(&d).unwrap();
            fs::write(
                d.join("stat"),
                format!("{pid} (w{pid}) R 1 1 1 0 -1 0 0 0 0 0 {ticks} 0 0 0 20 0 1 0 1 1 1"),
            )
            .unwrap();
        };
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("stat"), "cpu  0 0 0 1000 0 0 0 0\n").unwrap();
        write(10, 0);
        write(11, 0);
        let mut s = Sampler::new(root.to_str().unwrap(), 50.0);
        s.tick_hz = 100.0;
        s.drm = root.join("no-drm").to_string_lossy().into_owned();
        s.nvidia = false;
        assert!(s.sample(0).is_empty(), "the first pass only sets the baseline");
        // About one second later: pid 10 used 100 ticks (a full core), pid 11 none.
        std::thread::sleep(std::time::Duration::from_millis(1000));
        write(10, 100);
        fs::write(root.join("stat"), "cpu  400 0 0 1600 0 0 0 0\n").unwrap();
        let out = s.sample(0);
        let host = out.iter().find(|o| o.data["scope"] == "host").unwrap();
        assert_eq!(host.data["cpu_busy_pct"], "40.0");
        let procs: Vec<_> = out.iter().filter(|o| o.data["scope"] == "process").collect();
        assert_eq!(procs.len(), 1);
        assert_eq!(procs[0].data["pid"], "10");
        let pct: f64 = procs[0].data["cpu_pct"].parse().unwrap();
        assert!((80.0..=101.0).contains(&pct), "{pct}");
        let _ = fs::remove_dir_all(&root);
    }
}
