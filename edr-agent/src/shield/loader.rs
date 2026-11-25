//! Loads edr_shield.bpf.o with its maps and links pinned under one bpffs
//! directory (default /sys/fs/bpf/edr_shield), so shields outlive the agent until
//! each row's deadline, and reads shield_events.
//!
//! At start the maps are reused when pins of a compatible layout exist, so rows
//! and hit counts survive an agent restart. The programs are always the ones in
//! the object file: each new link is pinned under a temporary name, the old pin
//! is removed, and the new pin is renamed into place. The new program runs before
//! the old one goes, so there is no gap.

use std::fs;
use std::os::fd::{AsFd, AsRawFd};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use libbpf_rs::{libbpf_sys, Link, MapFlags, MapHandle, MapInfo, Object, ObjectBuilder, Program, RingBufferBuilder};

use super::abi::{stat_hits, ShieldRule, SCFG_SCOPE, SHIELD_STAT_SIZE};
use super::{RuleMaps, ShieldKind};
use crate::guard::abi::{BPF_LINK_GET_FD_BY_ID, BPF_MAP_GET_FD_BY_ID, BPF_PROG_GET_FD_BY_ID};

pub const DEFAULT_PIN_DIR: &str = "/sys/fs/bpf/edr_shield";

const PROGRAMS: [&str; 4] = ["shield_exec_check", "shield_file_open", "shield_connect", "shield_sendmsg"];
const MAPS: [&str; 6] = ["shield_cfg", "shield_exec", "shield_file", "shield_conn", "shield_stats", "shield_events"];
const POLL: Duration = Duration::from_millis(200);

/// Duplicated handles of the rule maps. Send and Sync, unlike the object.
pub struct KernelMaps {
    cfg: MapHandle,
    exec: MapHandle,
    file: MapHandle,
    conn: MapHandle,
    stats: MapHandle,
}

impl KernelMaps {
    fn of(&self, kind: ShieldKind) -> &MapHandle {
        match kind {
            ShieldKind::ExecDeny => &self.exec,
            ShieldKind::FileDeny => &self.file,
            ShieldKind::ConnectDeny => &self.conn,
        }
    }
}

impl RuleMaps for KernelMaps {
    fn put(&self, kind: ShieldKind, key: &[u8], rule: &ShieldRule) -> Result<(), String> {
        self.of(kind)
            .update(key, &rule.to_bytes(), MapFlags::ANY)
            .map_err(|e| format!("{} update: {e}", kind.as_str()))
    }

    /// For connect_deny the kernel answers with the longest matching prefix, so
    /// callers compare the row's id with the shield they asked about.
    fn get(&self, kind: ShieldKind, key: &[u8]) -> Option<ShieldRule> {
        self.of(kind)
            .lookup(key, MapFlags::ANY)
            .ok()
            .flatten()
            .and_then(|v| ShieldRule::from_bytes(&v))
    }

    fn delete(&self, kind: ShieldKind, key: &[u8]) -> Result<(), String> {
        self.of(kind).delete(key).map_err(|e| format!("{} delete: {e}", kind.as_str()))
    }

    fn keys(&self, kind: ShieldKind) -> Vec<Vec<u8>> {
        self.of(kind).keys().collect()
    }

    fn set_slot(&self, slot: u32, value: u64) -> Result<(), String> {
        self.cfg
            .update(&slot.to_ne_bytes(), &value.to_ne_bytes(), MapFlags::ANY)
            .map_err(|e| format!("shield_cfg slot {slot}: {e}"))
    }

    fn hits(&self, num: u32) -> u64 {
        self.stats
            .lookup_percpu(&num.to_ne_bytes(), MapFlags::ANY)
            .ok()
            .flatten()
            .map(|per_cpu| per_cpu.iter().map(|v| stat_hits(v)).sum())
            .unwrap_or(0)
    }

    fn reset_hits(&self, num: u32) -> Result<(), String> {
        let cpus = libbpf_rs::num_possible_cpus().map_err(|e| format!("cpus: {e}"))?;
        let zero = vec![vec![0u8; SHIELD_STAT_SIZE]; cpus];
        self.stats
            .update_percpu(&num.to_ne_bytes(), &zero, MapFlags::ANY)
            .map_err(|e| format!("shield_stats {num}: {e}"))
    }
}

/// What start() set up.
pub struct ShieldStart {
    pub maps: KernelMaps,
    /// (bpf() command that opens it, id) of every program, map and link, for the
    /// guard's protected_ids.
    pub ids: Vec<(u32, u32)>,
    /// The pin directories and pins, for the guard's file protection.
    pub pins: Vec<PathBuf>,
    pub notes: Vec<String>,
    pub thread: ShieldThread,
}

/// The event reader. Stopping it leaves the pinned programs attached.
pub struct ShieldThread {
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl ShieldThread {
    pub fn stop(mut self) {
        self.shutdown();
    }

    fn shutdown(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

impl Drop for ShieldThread {
    fn drop(&mut self) {
        self.shutdown();
    }
}

type Ready = (KernelMaps, Vec<(u32, u32)>, Vec<PathBuf>, Vec<String>);

/// Loads, pins and attaches, then reads shield_events on its own thread and
/// hands every record to `on_event`.
pub fn start(
    obj_path: &str,
    pin_dir: &Path,
    scope_cgroup_id: u64,
    mut on_event: Box<dyn FnMut(&[u8]) + Send>,
) -> Result<ShieldStart> {
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = stop.clone();
    let (tx, rx) = mpsc::channel::<Result<Ready>>();
    let obj_path = obj_path.to_string();
    let pin_dir = pin_dir.to_path_buf();
    let thread = std::thread::Builder::new()
        .name("edr_shield".into())
        .spawn(move || {
            let (obj, links, notes) = match load_pinned(&obj_path, &pin_dir) {
                Ok(v) => v,
                Err(e) => {
                    let _ = tx.send(Err(e));
                    return;
                }
            };
            let ready = (|| -> Result<Ready> {
                let h = |name: &str| -> Result<MapHandle> {
                    Ok(MapHandle::try_clone(obj.map(name).ok_or_else(|| anyhow!("map {name} missing"))?)?)
                };
                let maps = KernelMaps {
                    cfg: h("shield_cfg")?,
                    exec: h("shield_exec")?,
                    file: h("shield_file")?,
                    conn: h("shield_conn")?,
                    stats: h("shield_stats")?,
                };
                maps.set_slot(SCFG_SCOPE, scope_cgroup_id).map_err(|e| anyhow!(e))?;
                let ids = object_ids(&obj, &links)?;
                let mut pins = vec![pin_dir.clone(), pin_dir.join("maps"), pin_dir.join("links")];
                pins.extend(MAPS.iter().map(|m| pin_dir.join("maps").join(m)));
                pins.extend(PROGRAMS.iter().map(|p| pin_dir.join("links").join(p)));
                Ok((maps, ids, pins, notes))
            })();
            let ok = ready.is_ok();
            let _ = tx.send(ready);
            if !ok {
                return;
            }
            let events = match obj.map("shield_events") {
                Some(m) => m,
                None => return,
            };
            let mut builder = RingBufferBuilder::new();
            if builder
                .add(events, |data: &[u8]| {
                    on_event(data);
                    0
                })
                .is_err()
            {
                return;
            }
            let Ok(ring) = builder.build() else { return };
            while !thread_stop.load(Ordering::Relaxed) {
                match ring.poll(POLL) {
                    Ok(()) => {}
                    Err(libbpf_rs::Error::System(errno)) if errno == libc::EINTR => {}
                    Err(e) => {
                        log::warn!("shields: event loop stopped: {e}");
                        break;
                    }
                }
            }
            let _ = ring.consume();
            // The links close here. The pins keep the programs attached.
            drop(links);
            drop(obj);
        })
        .context("spawn the edr_shield thread")?;
    match rx.recv() {
        Ok(Ok((maps, ids, pins, notes))) => Ok(ShieldStart {
            maps,
            ids,
            pins,
            notes,
            thread: ShieldThread { stop, thread: Some(thread) },
        }),
        Ok(Err(e)) => {
            let _ = thread.join();
            Err(e)
        }
        Err(_) => {
            let _ = thread.join();
            Err(anyhow!("the edr_shield thread exited during setup"))
        }
    }
}

fn open_with_pins(obj_path: &str, maps_dir: &Path) -> Result<libbpf_rs::OpenObject> {
    let mut open = ObjectBuilder::default()
        .open_file(obj_path)
        .with_context(|| format!("open {obj_path}"))?;
    for name in MAPS {
        let m = open.map_mut(name).ok_or_else(|| anyhow!("map {name} missing from {obj_path}"))?;
        m.set_pin_path(maps_dir.join(name))?;
    }
    Ok(open)
}

/// Loads the object, reusing pinned maps of the same layout, and attaches and
/// pins its programs, replacing the pins of an earlier run.
fn load_pinned(obj_path: &str, pin_dir: &Path) -> Result<(Object, Vec<Link>, Vec<String>)> {
    let mut notes = Vec::new();
    let maps_dir = pin_dir.join("maps");
    let links_dir = pin_dir.join("links");
    for d in [pin_dir, &maps_dir, &links_dir] {
        if !d.exists() {
            fs::create_dir(d).with_context(|| format!("mkdir {} (is bpffs mounted?)", d.display()))?;
        }
    }
    let reused = MAPS.iter().filter(|m| maps_dir.join(m).exists()).count();
    let mut obj = match open_with_pins(obj_path, &maps_dir)?.load() {
        Ok(o) => {
            if reused > 0 {
                notes.push(format!("reused {reused} pinned shield maps, rows and hit counts kept"));
            }
            o
        }
        Err(e) => {
            // A map pinned by an older build with another layout. Start over.
            notes.push(format!("pinned shield maps not reusable ({e}), loading fresh ones"));
            for m in MAPS {
                let _ = fs::remove_file(maps_dir.join(m));
            }
            open_with_pins(obj_path, &maps_dir)?
                .load()
                .with_context(|| format!("load {obj_path}"))?
        }
    };
    let mut links = Vec::new();
    for name in PROGRAMS {
        let prog = obj
            .prog_mut(name)
            .ok_or_else(|| anyhow!("program {name} missing from {obj_path}"))?;
        let mut link = prog.attach_lsm().with_context(|| format!("attach {name}"))?;
        let pin = links_dir.join(name);
        // bpffs refuses dots in names, so no "name.new".
        let tmp = links_dir.join(format!("{name}_new"));
        let _ = fs::remove_file(&tmp);
        link.pin(&tmp).with_context(|| format!("pin {}", tmp.display()))?;
        if pin.exists() {
            // The earlier run's link. Removing its last reference detaches it,
            // after the new program is already attached.
            fs::remove_file(&pin).with_context(|| format!("unpin {}", pin.display()))?;
            notes.push(format!("replaced the pinned {name}"));
        }
        fs::rename(&tmp, &pin).with_context(|| format!("rename {} to {}", tmp.display(), pin.display()))?;
        links.push(link);
    }
    Ok((obj, links, notes))
}

fn object_ids(obj: &Object, links: &[Link]) -> Result<Vec<(u32, u32)>> {
    let mut ids = Vec::new();
    for prog in obj.progs_iter() {
        ids.push((BPF_PROG_GET_FD_BY_ID, Program::get_id_by_fd(prog.as_fd())?));
    }
    for m in obj.maps_iter() {
        ids.push((BPF_MAP_GET_FD_BY_ID, MapInfo::new(m.as_fd())?.info.id));
    }
    for link in links {
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
        ids.push((BPF_LINK_GET_FD_BY_ID, info.id));
    }
    Ok(ids)
}
