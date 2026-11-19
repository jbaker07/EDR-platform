// src/pb_adapter/clock.rs
// One clock for every record: Unix seconds, wall clock.
//
// The BPF programs stamp events with bpf_ktime_get_ns(), which reads CLOCK_MONOTONIC:
// nanoseconds since boot, paused during suspend. Correlation windows compare stamps from
// several sensors, and some sensors carry no kernel stamp at all, so everything is moved
// to wall-clock seconds. The shift from monotonic to wall clock (REALTIME - MONOTONIC) is
// measured once and reused, so two events keep the same distance apart after conversion.
// The adapter lives in the library crate, so the eBPF decoders in the binary use it too.

use std::sync::OnceLock;

const NS_PER_SEC: i128 = 1_000_000_000;

/// Below this a nanosecond stamp cannot be Unix time (it would be before 2001-09-09),
/// so it is time since boot. Monotonic time reaches it only after 31 years of uptime.
const MONO_NS_LIMIT: u64 = 1_000_000_000_000_000_000;
/// The same cut for a stamp in seconds.
const MONO_SECS_LIMIT: u64 = 1_000_000_000;

fn read_clock(id: libc::clockid_t) -> i128 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // clock_gettime only fails for an unknown clock id. Both ids here always exist.
    unsafe { libc::clock_gettime(id, &mut ts) };
    ts.tv_sec as i128 * NS_PER_SEC + ts.tv_nsec as i128
}

/// REALTIME minus MONOTONIC in nanoseconds, measured on first use.
pub fn mono_offset_ns() -> i128 {
    static OFFSET: OnceLock<i128> = OnceLock::new();
    *OFFSET.get_or_init(|| {
        // Read monotonic on both sides of the wall clock and use the midpoint.
        let m1 = read_clock(libc::CLOCK_MONOTONIC);
        let wall = read_clock(libc::CLOCK_REALTIME);
        let m2 = read_clock(libc::CLOCK_MONOTONIC);
        wall - (m1 + m2) / 2
    })
}

/// Pure form of the conversion, used by the tests.
pub fn mono_ns_to_unix_secs_with(ns: u64, offset_ns: i128) -> u64 {
    let wall = ns as i128 + offset_ns;
    if wall <= 0 {
        0
    } else {
        (wall / NS_PER_SEC) as u64
    }
}

/// Kernel bpf_ktime_get_ns() stamp to Unix seconds.
pub fn mono_ns_to_unix_secs(ns: u64) -> u64 {
    mono_ns_to_unix_secs_with(ns, mono_offset_ns())
}

/// Inverse of mono_ns_to_unix_secs. Tests use it to build kernel stamps for a chosen time.
pub fn unix_secs_to_mono_ns(secs: u64) -> u64 {
    let ns = secs as i128 * NS_PER_SEC - mono_offset_ns();
    ns.max(0) as u64
}

/// A nanosecond stamp of either kind to Unix seconds.
pub fn ns_to_unix_secs(ns: u64) -> u64 {
    if ns < MONO_NS_LIMIT {
        mono_ns_to_unix_secs(ns)
    } else {
        ns / NS_PER_SEC as u64
    }
}

/// A stamp of unknown unit to Unix seconds. Seconds since boot, Unix seconds,
/// milliseconds, microseconds and nanoseconds are told apart by size.
pub fn any_to_unix_secs(v: u64) -> u64 {
    match v {
        0 => 0,
        v if v < MONO_SECS_LIMIT => mono_ns_to_unix_secs(v.saturating_mul(1_000_000_000)),
        v if v < 100_000_000_000 => v,
        v if v < 100_000_000_000_000 => v / 1_000,
        v if v < 100_000_000_000_000_000 => v / 1_000_000,
        v => ns_to_unix_secs(v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monotonic_ns_shifts_by_the_offset() {
        // Booted at Unix 1_700_000_000, event 90.5 s after boot.
        let off = 1_700_000_000i128 * NS_PER_SEC;
        assert_eq!(mono_ns_to_unix_secs_with(90_500_000_000, off), 1_700_000_090);
        // Two events keep their distance.
        let a = mono_ns_to_unix_secs_with(10_000_000_000, off);
        let b = mono_ns_to_unix_secs_with(610_000_000_000, off);
        assert_eq!(b - a, 600);
    }

    #[test]
    fn offset_matches_the_clocks_now() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mono_now = read_clock(libc::CLOCK_MONOTONIC) as u64;
        let got = mono_ns_to_unix_secs(mono_now);
        assert!(got.abs_diff(now) <= 1, "got {got}, now {now}");
        assert_eq!(mono_ns_to_unix_secs(unix_secs_to_mono_ns(now)), now);
    }

    #[test]
    fn stamps_are_told_apart_by_size() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // Unix seconds, milliseconds, microseconds and nanoseconds.
        assert_eq!(any_to_unix_secs(now), now);
        assert_eq!(any_to_unix_secs(now * 1_000 + 999), now);
        assert_eq!(any_to_unix_secs(now * 1_000_000), now);
        assert_eq!(any_to_unix_secs(now * 1_000_000_000), now);
        assert_eq!(ns_to_unix_secs(now * 1_000_000_000), now);
        // Seconds and nanoseconds since boot land near now, not in 1970.
        let mono_now_ns = read_clock(libc::CLOCK_MONOTONIC) as u64;
        assert!(ns_to_unix_secs(mono_now_ns).abs_diff(now) <= 1);
        assert!(any_to_unix_secs(mono_now_ns / 1_000_000_000).abs_diff(now) <= 1);
        assert_eq!(any_to_unix_secs(0), 0);
    }
}
