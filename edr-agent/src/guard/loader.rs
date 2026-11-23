//! Loads edr_guard.bpf.o, fills its maps, attaches the five LSM programs and
//! reads guard_events. Everything that touches the libbpf object lives on one
//! thread, because the object is not Send.

use std::fs;
use std::os::fd::{AsFd, AsRawFd};
use std::os::unix::fs::MetadataExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use libbpf_rs::{
    libbpf_sys, Link, Map, MapFlags, MapInfo, Object, ObjectBuilder, Program, RingBufferBuilder,
};

use super::abi::*;
use super::{
    emit, exe_ids, find_programs, guard_finding, off_switch_finding, off_switch_thrown,
    EventCallback, Finding, GuardHandle, GuardMode, GuardSettings, GuardSummary, RepeatFilter,
    CLIENT_DIRS, MEMFD_EXEC_CLIENTS,
};
use crate::modules::suspicious_ipc::{path_key, RUNTIME_CLIENT_ALLOWLIST, RUNTIME_SOCKETS};

const LSM_LIST: &str = "/sys/kernel/security/lsm";

/// Inode number of the host's pid namespace, fixed by the kernel
/// (PROC_PID_INIT_INO in include/linux/proc_ns.h).
const INIT_PID_NS_INO: u64 = 0xEFFF_FFFC;

const PROGRAMS: [&str; 5] = [
    "guard_unix_connect",
    "guard_memfd_exec",
    "guard_task_kill",
    "guard_ptrace",
    "guard_bpf",
];

/// How often the loop wakes up to check for stop and for the off file.
const POLL: Duration = Duration::from_millis(200);

/// Fails unless "bpf" is on the kernel's active LSM list.
pub(super) fn check_bpf_lsm() -> Result<()> {
    let list = read_lsm_list()?;
    if !super::bpf_lsm_listed(&list) {
        bail!(
            "the bpf LSM is not active on this kernel (active: {}). Add bpf to the lsm= boot parameter to enable it",
            list.trim()
        );
    }
    Ok(())
}

fn read_lsm_list() -> Result<String> {
    if let Ok(list) = fs::read_to_string(LSM_LIST) {
        return Ok(list);
    }
    // Containers often start without securityfs mounted. Mounting it only
    // makes the list readable, and loading BPF needs root anyway.
    // SAFETY: every pointer is a NUL terminated literal and data is NULL.
    let rc = unsafe {
        libc::mount(
            c"securityfs".as_ptr(),
            c"/sys/kernel/security".as_ptr(),
            c"securityfs".as_ptr(),
            0,
            std::ptr::null(),
        )
    };
    if rc != 0 {
        log::debug!("guard: mount securityfs: {}", std::io::Error::last_os_error());
    }
    fs::read_to_string(LSM_LIST).with_context(|| format!("read {LSM_LIST}"))
}

/// The kernel's pids are host pids. std::process::id() matches them only in
/// the host pid namespace.
fn in_host_pid_ns() -> bool {
    fs::metadata("/proc/self/ns/pid")
        .map(|m| m.ino() == INIT_PID_NS_INO)
        .unwrap_or(false)
}

/// Starts the guard thread and returns once the programs are attached, or
/// with the reason they could not be.
pub(super) fn spawn(settings: GuardSettings, mut on_event: EventCallback) -> Result<GuardHandle> {
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = stop.clone();
    let (ready_tx, ready_rx) = mpsc::channel::<Result<GuardSummary>>();
    let thread = std::thread::Builder::new()
        .name("edr_guard".into())
        .spawn(move || {
            let guard = match Loaded::load(&settings) {
                Ok(g) => g,
                Err(e) => {
                    let _ = ready_tx.send(Err(e));
                    return;
                }
            };
            let _ = ready_tx.send(Ok(guard.summary.clone()));
            if let Err(e) = guard.run(&settings, &thread_stop, &mut *on_event) {
                log::warn!("guard: event loop stopped: {e:#}");
            }
            guard.shutdown();
        })
        .context("spawn the edr_guard thread")?;

    match ready_rx.recv() {
        Ok(Ok(summary)) => Ok(GuardHandle {
            stop,
            thread: Some(thread),
            summary,
        }),
        Ok(Err(e)) => {
            let _ = thread.join();
            Err(e)
        }
        Err(_) => {
            let _ = thread.join();
            Err(anyhow!("the edr_guard thread exited during setup"))
        }
    }
}

/// The loaded object and its attachments. Fields drop in order, so the links
/// (which detach the programs) go before the object.
struct Loaded {
    links: Vec<Link>,
    obj: Object,
    summary: GuardSummary,
}

impl Loaded {
    fn load(s: &GuardSettings) -> Result<Self> {
        let mut obj = ObjectBuilder::default()
            .open_file(&s.obj_path)
            .with_context(|| format!("open {}", s.obj_path))?
            // load() runs the CO-RE relocations against this kernel's BTF, then the verifier.
            .load()
            .with_context(|| format!("load {}", s.obj_path))?;

        // Runtime socket names, then the executables allowed to use them.
        let watched = map(&obj, "watched_sockets")?;
        for path in RUNTIME_SOCKETS {
            let key = path_key(path).ok_or_else(|| anyhow!("socket path {path:?} does not fit"))?;
            watched.update(&key, &[1], MapFlags::ANY)?;
        }
        let mut clients = find_programs(CLIENT_DIRS, RUNTIME_CLIENT_ALLOWLIST);
        clients.extend(s.extra_socket_clients.iter().cloned());
        let socket_clients = exe_ids(&clients);
        fill_exe_map(map(&obj, "socket_clients")?, &socket_clients)?;
        let mut memfd = find_programs(CLIENT_DIRS, MEMFD_EXEC_CLIENTS);
        memfd.extend(s.extra_memfd_allow.iter().cloned());
        let memfd_allow = exe_ids(&memfd);
        fill_exe_map(map(&obj, "memfd_allow")?, &memfd_allow)?;

        // Who the agent is, and which tasks the guards judge.
        let mut modes = s.modes;
        let agent_tgid = if in_host_pid_ns() {
            Some(std::process::id())
        } else {
            if modes.self_protect != GuardMode::Off {
                log::warn!(
                    "guard: self protection is off, the agent is not in the host pid namespace so its pid is not the kernel's"
                );
            }
            modes.self_protect = GuardMode::Off;
            None
        };
        let off_now = off_switch_thrown(&s.off_file, s.off_sha256.as_deref());
        if modes.self_protect != GuardMode::Off && off_now {
            log::warn!("guard: self protection starts off, {} is present", s.off_file.display());
            modes.self_protect = GuardMode::Off;
        }
        let scope_cgroup_id = match &s.scope_cgroup {
            // A cgroup v2 directory's inode number is the cgroup id the kernel reports.
            Some(dir) => fs::metadata(dir)
                .with_context(|| format!("scope cgroup {}", dir.display()))?
                .ino(),
            None => 0,
        };
        let config = map(&obj, "guard_config")?;
        set_slot(config, CFG_AGENT_TGID, agent_tgid.unwrap_or(0) as u64)?;
        set_slot(config, CFG_SCOPE_CGROUP, scope_cgroup_id)?;

        // Attach while every mode is still 0 (off).
        let mut links = Vec::new();
        for name in PROGRAMS {
            let prog = obj
                .prog_mut(name)
                .ok_or_else(|| anyhow!("program {name} missing from {}", s.obj_path))?;
            links.push(prog.attach_lsm().with_context(|| format!("attach {name}"))?);
        }

        // The agent's own objects, for the bpf guard.
        let ids = object_ids(&obj, &links)?;
        let protected = map(&obj, "protected_ids")?;
        for (cmd, id) in &ids {
            protected.update(&obj_key(*cmd, *id), &[1], MapFlags::ANY)?;
        }

        // Modes last, so nothing is enforced before the allowlists are in place.
        let config = map(&obj, "guard_config")?;
        set_slot(config, CFG_SOCKET_MODE, modes.socket.as_raw())?;
        set_slot(config, CFG_MEMFD_MODE, modes.memfd.as_raw())?;
        set_slot(config, CFG_SELF_MODE, modes.self_protect.as_raw())?;

        let summary = GuardSummary {
            obj_path: s.obj_path.clone(),
            modes,
            agent_tgid,
            scope_cgroup_id,
            socket_clients,
            memfd_allow,
            protected_ids: ids.len(),
        };
        Ok(Self { links, obj, summary })
    }

    /// Reads guard_events until stop is set. Also watches for the off file.
    fn run(
        &self,
        s: &GuardSettings,
        stop: &AtomicBool,
        on_event: &mut (dyn FnMut(&GuardEvent, &Finding) + Send),
    ) -> Result<()> {
        // Pids in events are host pids. /proc here shows them only in the host pid namespace.
        let resolve_proc = in_host_pid_ns();
        let mut repeats = RepeatFilter::default();
        let mut builder = RingBufferBuilder::new();
        builder.add(map(&self.obj, "guard_events")?, |data: &[u8]| {
            handle_record(data, resolve_proc, &mut repeats, on_event);
            0
        })?;
        let ring = builder.build()?;

        let mut self_on = self.summary.modes.self_protect != GuardMode::Off;
        while !stop.load(Ordering::Relaxed) {
            match ring.poll(POLL) {
                Ok(()) => {}
                // A signal interrupted the wait. Poll again.
                Err(libbpf_rs::Error::System(errno)) if errno == libc::EINTR => {}
                Err(e) => return Err(e.into()),
            }
            if self_on && off_switch_thrown(&s.off_file, s.off_sha256.as_deref()) {
                set_slot(map(&self.obj, "guard_config")?, CFG_SELF_MODE, 0)?;
                self_on = false;
                log::warn!("guard: self protection turned off by {}", s.off_file.display());
                emit(off_switch_finding(&s.off_file));
            }
        }
        // Report what the kernel already decided before the programs go away.
        ring.consume()?;
        Ok(())
    }

    /// Self protection off first, so the agent can be stopped as soon as
    /// this returns, then the other guards, then the links are dropped,
    /// which detaches every program.
    fn shutdown(self) {
        if let Ok(config) = map(&self.obj, "guard_config") {
            for slot in [CFG_SELF_MODE, CFG_SOCKET_MODE, CFG_MEMFD_MODE] {
                if let Err(e) = set_slot(config, slot, 0) {
                    log::warn!("guard: could not turn slot {slot} off before detaching: {e:#}");
                }
            }
        }
        drop(self.links);
        drop(self.obj);
        log::info!("guard: stopped, programs detached");
    }
}

fn handle_record(
    data: &[u8],
    resolve_proc: bool,
    repeats: &mut RepeatFilter,
    on_event: &mut (dyn FnMut(&GuardEvent, &Finding) + Send),
) {
    let Some(event) = GuardEvent::from_bytes(data) else {
        log::warn!(
            "guard: dropped a {} byte record, expected {GUARD_EVENT_SIZE}",
            data.len()
        );
        return;
    };
    if !repeats.admit(&event) {
        return;
    }
    let exe = resolve_proc
        .then(|| proc_exe_if_same(event.tgid, event.exe_id()))
        .flatten();
    let Some(finding) = guard_finding(&event, exe.as_deref()) else {
        log::warn!(
            "guard: record with unknown kind {} or action {}",
            event.kind,
            event.action
        );
        return;
    };
    on_event(&event, &finding);
    emit(finding);
}

/// Full path from /proc/<pid>/exe, but only while it is still the file the
/// kernel reported. After an exec that monitor mode let through, or once the
/// pid belongs to another process, /proc names something else, and the
/// kernel's file name stands in.
fn proc_exe_if_same(pid: u32, id: ExeId) -> Option<String> {
    let link = format!("/proc/{pid}/exe");
    let meta = fs::metadata(&link).ok()?;
    if ExeId::from_stat(meta.dev(), meta.ino()) != id {
        return None;
    }
    fs::read_link(&link).ok().map(|p| p.display().to_string())
}

fn map<'a>(obj: &'a Object, name: &str) -> Result<&'a Map> {
    obj.map(name).ok_or_else(|| anyhow!("map {name} missing"))
}

fn set_slot(config: &Map, slot: u32, value: u64) -> Result<()> {
    config
        .update(&slot.to_ne_bytes(), &value.to_ne_bytes(), MapFlags::ANY)
        .with_context(|| format!("guard_config slot {slot}"))
}

fn fill_exe_map(m: &Map, ids: &[(std::path::PathBuf, ExeId)]) -> Result<()> {
    for (path, id) in ids {
        m.update(&id.key(), &[1], MapFlags::ANY)
            .with_context(|| format!("allow {}", path.display()))?;
    }
    Ok(())
}

/// (bpf() command that opens it, id) for every program, map and link.
fn object_ids(obj: &Object, links: &[Link]) -> Result<Vec<(u32, u32)>> {
    let mut ids = Vec::new();
    for prog in obj.progs_iter() {
        ids.push((BPF_PROG_GET_FD_BY_ID, Program::get_id_by_fd(prog.as_fd())?));
    }
    for m in obj.maps_iter() {
        ids.push((BPF_MAP_GET_FD_BY_ID, MapInfo::new(m.as_fd())?.info.id));
    }
    for link in links {
        ids.push((BPF_LINK_GET_FD_BY_ID, link_id(link)?));
    }
    Ok(ids)
}

fn link_id(link: &Link) -> Result<u32> {
    // SAFETY: bpf_link_info is plain kernel data, valid for any bit pattern.
    let mut info: libbpf_sys::bpf_link_info = unsafe { std::mem::zeroed() };
    let mut len = std::mem::size_of_val(&info) as u32;
    // SAFETY: info and len are live locals and len is the size of info.
    let rc = unsafe {
        libbpf_sys::bpf_obj_get_info_by_fd(
            link.as_fd().as_raw_fd(),
            &mut info as *mut libbpf_sys::bpf_link_info as *mut std::ffi::c_void,
            &mut len,
        )
    };
    if rc != 0 {
        bail!("link info: {}", std::io::Error::from_raw_os_error(-rc));
    }
    Ok(info.id)
}
