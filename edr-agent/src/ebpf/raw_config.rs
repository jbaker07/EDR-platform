// src/ebpf/raw_config.rs
// The agent's side of raw_syscalls_catcher's two control maps, set once at startup in
// pinned mode:
//   edr_self_tgid       the agent's tgid. The sensor skips it before taking ring space,
//                       so the agent's own log opens and writes never come back as events.
//   edr_allow_syscalls  which syscalls the sensor reports. An empty map means a silent
//                       sensor, so when the agent finds it empty it fills the default set.
// The syscall numbers are arm64, the same ones raw_syscalls_catcher.bpf.c hard-codes.

#![cfg(target_os = "linux")]

use std::os::fd::{AsRawFd, OwnedFd};

use libbpf_rs::libbpf_sys as sys;

pub const SELF_MAP: &str = "edr_self_tgid";
pub const ALLOW_MAP: &str = "edr_allow_syscalls";

/// What the loaded playbooks read from the raw sensor.
pub const DEFAULT_ALLOW: &[(u32, &str)] = &[
    (56, "openat"),     // open flags and errors: op=write, open, open_fail with errno
    (437, "openat2"),   // same handler as openat
    (64, "write"),      // byte counts, tied to a path through the open
    (57, "close"),      // ends the fd's entry in both fd tables, so a reused fd is not mislabeled
    (38, "renameat"),   // glibc rename() and renameat2(..., 0)
    (276, "renameat2"), // mv with flags, raw syscall users
    (35, "unlinkat"),   // rm
];

/// How EDR_RAW_SYSCALLS asks for the allowlist to be set.
#[derive(Debug, PartialEq)]
pub enum AllowPlan {
    /// Unset or "default": DEFAULT_ALLOW.
    Default,
    /// "none": leave the map alone, even when empty.
    Leave,
    /// A comma-separated list of syscall numbers.
    List(Vec<u32>),
}

pub fn parse_allow_plan(v: Option<&str>) -> Result<AllowPlan, String> {
    let v = match v.map(str::trim) {
        None | Some("") => return Ok(AllowPlan::Default),
        Some(v) => v,
    };
    match v.to_ascii_lowercase().as_str() {
        "default" => return Ok(AllowPlan::Default),
        "none" => return Ok(AllowPlan::Leave),
        _ => {}
    }
    let mut out = Vec::new();
    for part in v.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        let n = part
            .parse::<u32>()
            .map_err(|_| format!("EDR_RAW_SYSCALLS: '{part}' is not a syscall number"))?;
        out.push(n);
    }
    Ok(AllowPlan::List(out))
}

/// The numbers a plan writes. Leave writes nothing.
pub fn plan_numbers(plan: &AllowPlan) -> Vec<u32> {
    match plan {
        AllowPlan::Default => DEFAULT_ALLOW.iter().map(|(n, _)| *n).collect(),
        AllowPlan::Leave => Vec::new(),
        AllowPlan::List(v) => v.clone(),
    }
}

fn put_u32_u8(fd: i32, key: u32, val: u8) -> Result<(), String> {
    let rc = unsafe {
        sys::bpf_map_update_elem(
            fd,
            &key as *const u32 as *const std::ffi::c_void,
            &val as *const u8 as *const std::ffi::c_void,
            sys::BPF_ANY as u64,
        )
    };
    if rc < 0 {
        return Err(std::io::Error::last_os_error().to_string());
    }
    Ok(())
}

/// A pinned map under the prefix, at <prefix>/<name> (edr_attach_any) or
/// <prefix>/maps/<name> (attach_tp_sys_enter).
fn open_pinned(pin_prefix: &str, name: &str) -> Result<(String, OwnedFd), String> {
    let base = pin_prefix.trim_end_matches('/');
    let mut last = String::new();
    for path in [format!("{base}/{name}"), format!("{base}/maps/{name}")] {
        match super::events_reader::bpf_fd_for_path(&path) {
            Ok(fd) => return Ok((path, fd)),
            Err(e) => last = e,
        }
    }
    Err(last)
}

fn map_is_empty(fd: i32) -> bool {
    let mut next: u32 = 0;
    let rc = unsafe {
        sys::bpf_map_get_next_key(
            fd,
            std::ptr::null(),
            &mut next as *mut u32 as *mut std::ffi::c_void,
        )
    };
    rc < 0
}

/// Fill both maps. Every failure is logged and skipped, so an older sensor object
/// (no edr_self_tgid map) still runs. The agent then relies on the user-space drop in
/// ebpf_ingest.rs.
pub fn configure(pin_prefix: &str) {
    // The kernel compares host tgids. std::process::id() is the host tgid only when the
    // agent runs in the host pid namespace, as it does in pinned mode with --pid=host.
    let me = std::process::id();
    match open_pinned(pin_prefix, SELF_MAP) {
        Ok((path, fd)) => match put_u32_u8(fd.as_raw_fd(), me, 1) {
            Ok(()) => log::info!("raw sensor: skipping the agent's own tgid {me} ({path})"),
            Err(e) => log::warn!("raw sensor: could not write tgid {me} to {path}: {e}"),
        },
        Err(e) => log::warn!(
            "raw sensor: {e}. The sensor has no self filter, the user-space drop still applies"
        ),
    }

    let plan = match parse_allow_plan(std::env::var("EDR_RAW_SYSCALLS").ok().as_deref()) {
        Ok(p) => p,
        Err(e) => {
            log::warn!("raw sensor: {e}, using the default allowlist");
            AllowPlan::Default
        }
    };
    if plan == AllowPlan::Leave {
        log::info!("raw sensor: EDR_RAW_SYSCALLS=none, allowlist left as it is");
        return;
    }
    if !cfg!(target_arch = "aarch64") {
        log::warn!("raw sensor: syscall numbers are arm64 only, allowlist left as it is");
        return;
    }
    let (path, fd) = match open_pinned(pin_prefix, ALLOW_MAP) {
        Ok(v) => v,
        Err(e) => {
            log::warn!("raw sensor: {e}");
            return;
        }
    };
    // An operator who already ran set_allow keeps that choice.
    if !map_is_empty(fd.as_raw_fd()) {
        log::info!("raw sensor: allowlist at {path} already set, left as it is");
        return;
    }
    let mut on = Vec::new();
    for n in plan_numbers(&plan) {
        match put_u32_u8(fd.as_raw_fd(), n, 1) {
            Ok(()) => on.push(n.to_string()),
            Err(e) => log::warn!("raw sensor: could not enable syscall {n}: {e}"),
        }
    }
    log::info!("raw sensor: allowlist was empty, enabled syscalls {}", on.join(","));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allow_plan_from_env() {
        assert_eq!(parse_allow_plan(None), Ok(AllowPlan::Default));
        assert_eq!(parse_allow_plan(Some(" ")), Ok(AllowPlan::Default));
        assert_eq!(parse_allow_plan(Some("Default")), Ok(AllowPlan::Default));
        assert_eq!(parse_allow_plan(Some("none")), Ok(AllowPlan::Leave));
        assert_eq!(parse_allow_plan(Some("56, 64,38")), Ok(AllowPlan::List(vec![56, 64, 38])));
        assert!(parse_allow_plan(Some("56,openat")).is_err());
        assert!(plan_numbers(&AllowPlan::Leave).is_empty());
    }

    #[test]
    fn default_allowlist_covers_the_file_playbooks() {
        let on = plan_numbers(&AllowPlan::Default);
        // open flags and errno (attr_locker, memfd_inplace), write sizes (intermit_encrypt,
        // vm_image_datastore), every rename entry point and unlink (the burst playbooks)
        for n in [56, 64, 57, 38, 276, 35] {
            assert!(on.contains(&n), "syscall {n} missing");
        }
        // nothing per-packet or per-read, those would flood the ring
        for n in [63, 206, 207, 211, 212] {
            assert!(!on.contains(&n), "syscall {n} should stay off");
        }
    }
}
