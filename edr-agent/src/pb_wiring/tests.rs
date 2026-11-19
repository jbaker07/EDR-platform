// Tests for the playbook path: decoder-shaped outputs go through the real adapter
// (router, /proc enrichment with a stand-in reader), the real mapper and the real engine
// loaded from the shipped playbooks/ directory.
//
// Each output below has the category, signal and data keys its decoder writes:
//   proc_lifecycle exec   ebpf_ingest.rs decode_proc_lifecycle
//   wx_exec memfd_exec    ebpf_ingest.rs decode_wx
//   file_access_monitor   ebpf_ingest.rs decode_file
//   raw_syscalls_catcher  ebpf_bridge.rs edr_event_to_output (376-byte struct edr_event)

use std::collections::HashMap;
use std::path::Path;

use super::*;
use crate::pb_adapter::{adapter_facts_with, clock, ProcInfo};
use crate::telemetry_types::TelemetryOutput;

const O_RDONLY: i64 = 0;
const O_WRONLY: i64 = 0o1;
const O_RDWR: i64 = 0o2;
const O_CREAT_: i64 = 0o100;
const O_TRUNC_: i64 = 0o1000;

struct Sim {
    eng: PlaybookEngine,
    mapper: Mapper,
    /// Unix seconds.
    t: u64,
    /// What /proc would show for each pid.
    procs: HashMap<i64, ProcInfo>,
    hits: Vec<EngineHit>,
}

fn out(category: &str, signal: &str, data: &[(&str, String)]) -> TelemetryOutput {
    TelemetryOutput {
        category: category.into(),
        signal: signal.into(),
        confidence: 0.95,
        data: data.iter().map(|(k, v)| (k.to_string(), v.clone())).collect(),
    }
}

impl Sim {
    fn new() -> Self {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("playbooks");
        let (eng, report) = PlaybookEngine::load_dir_report(&dir).unwrap();
        assert_eq!(report.loaded.len(), 13);
        let mounts = FixedMounts(vec![
            ("/".into(), "ext4".into()),
            ("/mnt/nfs".into(), "nfs4".into()),
            ("/mnt/local".into(), "ext4".into()),
        ]);
        Self {
            eng,
            mapper: Mapper::with_mounts(Box::new(mounts)),
            t: 1_800_000_000,
            procs: HashMap::new(),
            hits: Vec::new(),
        }
    }

    fn feed(&mut self, o: TelemetryOutput) {
        let Sim {
            eng,
            mapper,
            t,
            procs,
            hits,
        } = self;
        let read = |pid: i64| procs.get(&pid).cloned().unwrap_or_default();
        for af in adapter_facts_with(&o, *t, &read) {
            for f in mapper.facts(&af) {
                hits.extend(eng.ingest_fact(f));
            }
        }
    }

    fn advance(&mut self, secs: u64) {
        self.t += secs;
        self.eng.tick(self.t);
    }

    fn count(&self, id: &str) -> usize {
        self.hits.iter().filter(|h| h.playbook_id == id).count()
    }

    fn fired(&self) -> Vec<String> {
        self.hits.iter().map(|h| h.playbook_id.clone()).collect()
    }

    fn cooldown(&self, id: &str) -> u64 {
        self.eng.playbooks[id].cooldown_sec
    }

    fn ts_ns(&self) -> String {
        clock::unix_secs_to_mono_ns(self.t).to_string()
    }

    /// sched_process_exec from proc_lifecycle. /proc shows the new image and argv.
    fn exec(&mut self, pid: i64, ppid: i64, uid: u32, exe: &str, cmd: &str) {
        self.procs.insert(
            pid,
            ProcInfo {
                exe: Some(exe.into()),
                cmdline: Some(cmd.into()),
                cwd: Some("/root".into()),
            },
        );
        let comm: String = exe.rsplit('/').next().unwrap_or("").chars().take(15).collect();
        let o = out(
            "process",
            "process::exec",
            &[
                ("pid", pid.to_string()),
                ("ppid", ppid.to_string()),
                ("uid", uid.to_string()),
                ("comm", comm),
                ("ts_ns", self.ts_ns()),
                ("evt_type", "1001".into()),
                ("source", "proc_lifecycle".into()),
            ],
        );
        self.feed(o);
    }

    /// wx_exec kind 2: a process wrote to a memfd and then exec'd.
    fn memfd_exec(&mut self, pid: i64, exe: &str, cmd: &str) {
        self.procs.insert(
            pid,
            ProcInfo {
                exe: Some(exe.into()),
                cmdline: Some(cmd.into()),
                cwd: Some("/tmp".into()),
            },
        );
        let o = out(
            "memory",
            "memory::memfd_exec",
            &[
                ("pid", pid.to_string()),
                ("kind", "2".into()),
                ("ts_ns", self.ts_ns()),
                ("source", "wx_exec".into()),
                ("memfd_bytes", "68432".into()),
            ],
        );
        self.feed(o);
    }

    /// file_access_monitor: sys_enter_openat, no flags and no stamp.
    fn perf_open(&mut self, pid: i64, uid: u32, comm: &str, path: &str) {
        let o = out(
            "file",
            "file_access",
            &[
                ("pid", pid.to_string()),
                ("uid", uid.to_string()),
                ("comm", comm.into()),
                ("path", path.into()),
                ("source", "file_access_monitor".into()),
            ],
        );
        self.feed(o);
    }

    /// One 376-byte struct edr_event as edr_event_to_output writes it.
    fn raw(&mut self, signal: &str, evt: i64, sys: i64, pid: i64, fd: i64, ret: i64, flags: i64, aux64: u64, path: &str, path2: &str) {
        let mut data = vec![
            ("ts_ns", self.ts_ns()),
            ("timestamp", self.t.to_string()),
            ("tgid", pid.to_string()),
            ("ppid", "1".to_string()),
            ("uid", "1000".to_string()),
            ("fd", fd.to_string()),
            ("ret", ret.to_string()),
            ("evt_type", evt.to_string()),
            ("syscall_id", sys.to_string()),
            ("flags", flags.to_string()),
            ("aux_u32", "0".to_string()),
            ("aux_u64", aux64.to_string()),
            ("comm", "worker".to_string()),
        ];
        if !path.is_empty() {
            data.push(("path", path.to_string()));
        }
        if !path2.is_empty() {
            data.push(("path2", path2.to_string()));
        }
        let o = out("file", signal, &data);
        self.feed(o);
    }

    /// openat exit: ret is the fd, or -errno.
    fn sys_open(&mut self, pid: i64, path: &str, flags: i64, ret: i64) {
        let fd = if ret >= 0 { ret } else { -1 };
        self.raw("file_open", 10, 56, pid, fd, ret, flags, 0, path, "");
    }
    fn sys_rename(&mut self, pid: i64, old: &str, new: &str) {
        self.raw("file_rename", 11, 276, pid, -1, 0, 0, 0, old, new);
    }
    /// renameat (38 on arm64), what glibc rename() uses. Same frame, other syscall id.
    fn sys_renameat(&mut self, pid: i64, old: &str, new: &str) {
        self.raw("file_rename", 11, 38, pid, -1, 0, 0, 0, old, new);
    }

    /// proc_lifecycle exec of a process that exited before /proc was read. /proc has
    /// nothing for the pid, so only the filename and argv the kernel copied are left.
    fn exec_gone(&mut self, pid: i64, ppid: i64, filename: &str, argv: &str) {
        self.procs.remove(&pid);
        let comm: String = filename.rsplit('/').next().unwrap_or("").chars().take(15).collect();
        let o = out(
            "process",
            "process::exec",
            &[
                ("pid", pid.to_string()),
                ("ppid", ppid.to_string()),
                ("uid", "0".into()),
                ("comm", comm),
                ("ts_ns", self.ts_ns()),
                ("evt_type", "1001".into()),
                ("source", "proc_lifecycle".into()),
                ("exec_filename", filename.into()),
                ("exec_argv", argv.into()),
            ],
        );
        self.feed(o);
    }
    fn sys_unlink(&mut self, pid: i64, path: &str) {
        self.raw("file_unlink", 14, 35, pid, -1, 0, 0, 0, path, "");
    }
    fn sys_write(&mut self, pid: i64, fd: i64, count: u64) {
        self.raw("fd_write", 13, 64, pid, fd, 0, 0, count, "", "");
    }
    fn sys_close(&mut self, pid: i64, fd: i64) {
        self.raw("fd_close", 18, 57, pid, fd, 0, 0, 0, "", "");
    }
}

// ---- The 13 playbooks: a sequence that fires, and a near miss ----

const BREAKOUT: &str = "pb_container_breakout_precursors_linux";
fn breakout(s: &mut Sim) {
    s.exec(4100, 4000, 0, "/usr/bin/unshare", "unshare --mount --uts --ipc --fork /bin/sh");
    s.advance(20);
    s.perf_open(4101, 0, "head", "/proc/kallsyms");
}
fn breakout_miss(s: &mut Sim) {
    // The host touch comes 480 s later, after the 420 s TTL of the namespace step.
    s.exec(4100, 4000, 0, "/usr/bin/unshare", "unshare --mount --uts --ipc --fork /bin/sh");
    s.advance(480);
    s.perf_open(4101, 0, "head", "/proc/kallsyms");
}

const ATTR: &str = "pb_outage_ransomware_linux_attr_locker";
fn attr_chattr(s: &mut Sim) {
    for i in 0..200 {
        let cmd = format!("chattr +i /home/u/f{i} /srv/d/f{i} /opt/a/f{i}");
        s.exec(5000 + i, 4999, 0, "/usr/bin/chattr", &cmd);
        if i % 10 == 9 {
            s.advance(1);
        }
    }
}
fn attr(s: &mut Sim) {
    attr_chattr(s);
    for i in 0..300 {
        s.sys_open(6000 + i % 5, &format!("/home/u/f{}", i % 200), O_RDWR, -13);
    }
}
fn attr_miss(s: &mut Sim) {
    // The opens fail with ENOENT, not EACCES or EPERM: no access-failure spike.
    attr_chattr(s);
    for i in 0..300 {
        s.sys_open(6000 + i % 5, &format!("/home/u/g{i}"), O_RDONLY, -2);
    }
}

const BACKUP: &str = "pb_outage_ransomware_linux_backup_wipe_first";
fn backup_burst(s: &mut Sim) {
    for i in 0..400 {
        let p = format!("/srv/data/f{i}.db");
        s.sys_rename(7100, &p, &format!("{p}.locked"));
    }
}
fn backup(s: &mut Sim) {
    s.exec(7000, 6999, 0, "/usr/bin/rm", "rm -rf /backup/daily");
    s.advance(60);
    backup_burst(s);
}
fn backup_miss(s: &mut Sim) {
    // The backup wipe is 2500 s before the burst, past its 2400 s TTL.
    s.exec(7000, 6999, 0, "/usr/bin/rm", "rm -rf /backup/daily");
    s.advance(2500);
    backup_burst(s);
}

const HOSTPATH: &str = "pb_outage_ransomware_linux_container_hostpath";
fn hostpath_burst(s: &mut Sim) {
    for i in 0..500 {
        let p = format!("/var/lib/docker/volumes/webdata/_data/f{i}");
        s.sys_open(7300, &p, O_WRONLY | O_TRUNC_, 3 + i % 100);
    }
}
fn hostpath(s: &mut Sim) {
    s.exec(7200, 7199, 0, "/usr/bin/docker", "docker exec -it web sh");
    s.advance(30);
    hostpath_burst(s);
}
fn hostpath_miss(s: &mut Sim) {
    // The same volume burst with no container tool run first.
    hostpath_burst(s);
}

const DB: &str = "pb_outage_ransomware_linux_db_impact";
fn db_churn(s: &mut Sim) {
    for i in 0..400 {
        s.sys_unlink(7500, &format!("/var/lib/mysql/shop/t{i}.ibd"));
    }
}
fn db(s: &mut Sim) {
    s.exec(7400, 7399, 0, "/usr/bin/mariadb", "mariadb shop -e DROP TABLE accounts");
    s.advance(10);
    db_churn(s);
}
fn db_miss(s: &mut Sim) {
    // A client runs, but no destructive SQL.
    s.exec(7400, 7399, 0, "/usr/bin/mariadb", "mariadb shop -e SELECT 1");
    s.advance(10);
    db_churn(s);
}

const FILEBURST: &str = "pb_outage_ransomware_linux_fileburst_crypto";
fn fileburst_renames(s: &mut Sim) {
    for i in 0..510 {
        let dir = ["/home/alice/docs", "/srv/share", "/opt/app/data"][i % 3];
        let p = format!("{dir}/f{i}.txt");
        s.sys_rename(7600, &p, &format!("{p}.enc"));
    }
}
fn openssl(s: &mut Sim) {
    s.exec(
        7601,
        7600,
        1000,
        "/usr/bin/openssl",
        "openssl enc -aes-256-cbc -pbkdf2 -in /home/alice/docs/f0.txt -out /home/alice/docs/f0.txt.enc",
    );
}
fn fileburst(s: &mut Sim) {
    fileburst_renames(s);
    s.advance(5);
    openssl(s);
}
fn fileburst_miss(s: &mut Sim) {
    // The crypto tool runs 700 s after the burst, past the burst's 600 s TTL.
    fileburst_renames(s);
    s.advance(700);
    openssl(s);
}

const FUSE: &str = "pb_outage_ransomware_linux_fuse_overlay";
fn fuse_burst(s: &mut Sim) {
    for i in 0..500 {
        let p = format!("/mnt/overlay/data/f{i}");
        s.sys_open(7701, &p, O_WRONLY | O_CREAT_ | O_TRUNC_, 5);
    }
}
fn fuse(s: &mut Sim) {
    s.exec(7700, 7699, 1000, "/usr/bin/fusermount", "fusermount -o rw /mnt/overlay");
    s.advance(15);
    fuse_burst(s);
}
fn fuse_miss(s: &mut Sim) {
    // The burst starts 700 s after the mount, past the mount step's 600 s TTL.
    s.exec(7700, 7699, 1000, "/usr/bin/fusermount", "fusermount -o rw /mnt/overlay");
    s.advance(700);
    fuse_burst(s);
}

const INTERMIT: &str = "pb_outage_ransomware_linux_intermit_encrypt";
fn intermit_files(s: &mut Sim, rename: bool) {
    for i in 0..800 {
        let p = format!("/home/bob/photos/img{i}.jpg");
        s.sys_open(7800, &p, O_RDWR, 7);
        s.sys_write(7800, 7, 4096);
        s.sys_write(7800, 7, 4096);
        s.sys_close(7800, 7);
        if rename {
            s.sys_rename(7800, &p, &format!("{p}.lock"));
        }
        if i % 100 == 99 {
            s.advance(10);
        }
    }
}
fn intermit(s: &mut Sim) {
    intermit_files(s, true);
}
fn intermit_miss(s: &mut Sim) {
    // The same short writes in place, with no renames.
    intermit_files(s, false);
}

const LUKS: &str = "pb_outage_ransomware_linux_luks_locker";
fn luks(s: &mut Sim) {
    s.exec(7900, 7899, 0, "/usr/sbin/cryptsetup", "cryptsetup luksFormat --batch-mode /dev/vdb");
    s.advance(120);
    s.exec(7901, 7899, 0, "/usr/bin/dash", "/bin/sh /usr/sbin/update-initramfs -u");
}
fn luks_miss(s: &mut Sim) {
    // The boot-chain change comes 1900 s later, past the 1800 s TTL of the locker step.
    s.exec(7900, 7899, 0, "/usr/sbin/cryptsetup", "cryptsetup luksFormat --batch-mode /dev/vdb");
    s.advance(1900);
    s.exec(7901, 7899, 0, "/usr/bin/dash", "/bin/sh /usr/sbin/update-initramfs -u");
}

const MEMFD: &str = "pb_outage_ransomware_linux_memfd_inplace";
fn memfd_burst(s: &mut Sim, rename: bool) {
    for i in 0..600 {
        let dir = ["/home/carol", "/srv/www", "/opt/data"][i % 3];
        let p = format!("{dir}/f{i}.dat");
        s.sys_open(8000, &p, O_RDWR, 9);
        if rename {
            s.sys_rename(8000, &p, &format!("{p}.x"));
        }
    }
}
fn memfd(s: &mut Sim) {
    s.memfd_exec(8000, "/memfd:payload (deleted)", "payload --run");
    s.advance(5);
    memfd_burst(s, false);
}
fn memfd_miss(s: &mut Sim) {
    // The in-place burst with no memfd or RWX exec.
    memfd_burst(s, false);
}

const NETSHARE: &str = "pb_outage_ransomware_linux_netshare_only";
fn share_burst(s: &mut Sim, root: &str) {
    for i in 0..400 {
        s.sys_open(8100, &format!("{root}/projects/f{i}.docx"), O_WRONLY | O_TRUNC_, 4);
    }
}
fn netshare(s: &mut Sim) {
    share_burst(s, "/mnt/nfs");
}
fn netshare_miss(s: &mut Sim) {
    // The same burst on a local ext4 mount.
    share_burst(s, "/mnt/local");
}

const VM: &str = "pb_outage_ransomware_linux_vm_image_datastore";
fn vm_images(s: &mut Sim, writes: usize, mib: u64) {
    for i in 0..20 {
        let p = format!("/var/lib/libvirt/images/vm{i}.qcow2");
        s.sys_open(8200, &p, O_RDWR, 11);
        for _ in 0..writes {
            s.sys_write(8200, 11, mib * 1024 * 1024);
        }
        s.sys_close(8200, 11);
        s.advance(10);
    }
}
fn vm(s: &mut Sim) {
    // 20 images, 600 MiB each, 11.7 GiB in all.
    vm_images(s, 2, 300);
}
fn vm_miss(s: &mut Sim) {
    // 20 images but 5 GiB in all, under the 10 GiB bar.
    vm_images(s, 1, 256);
}

const WIPER: &str = "pb_outage_wiper_linux_service_kill";
fn wiper(s: &mut Sim) {
    s.exec(8300, 8299, 0, "/usr/bin/systemctl", "systemctl stop postgresql");
    s.advance(30);
    s.exec(8301, 8299, 0, "/usr/bin/dd", "dd if=/dev/zero of=/dev/vdb bs=1M");
}
fn wiper_miss(s: &mut Sim) {
    // dd runs, but on files, not a device.
    s.exec(8300, 8299, 0, "/usr/bin/systemctl", "systemctl stop postgresql");
    s.advance(30);
    s.exec(8301, 8299, 0, "/usr/bin/dd", "dd if=/tmp/a of=/tmp/b bs=1M");
}

/// fires_once: the sequence fires the playbook exactly once.
/// cooldown: the same sequence again inside the cooldown does not fire, after it does.
/// near_miss: one required step missing or outside its time does not fire.
macro_rules! playbook_tests {
    ($name:ident, $id:expr, $seq:path, $miss:path) => {
        mod $name {
            use super::*;

            #[test]
            fn fires_once() {
                let mut s = Sim::new();
                $seq(&mut s);
                assert_eq!(s.count($id), 1, "fired: {:?}", s.fired());
            }

            #[test]
            fn cooldown_holds_then_ends() {
                let mut s = Sim::new();
                $seq(&mut s);
                assert_eq!(s.count($id), 1);
                let cd = s.cooldown($id);
                s.advance(cd / 2);
                $seq(&mut s);
                assert_eq!(s.count($id), 1, "fired again inside the cooldown");
                s.advance(cd);
                $seq(&mut s);
                assert_eq!(s.count($id), 2, "did not fire after the cooldown");
            }

            #[test]
            fn near_miss_does_not_fire() {
                let mut s = Sim::new();
                $miss(&mut s);
                assert_eq!(s.count($id), 0, "fired: {:?}", s.fired());
            }
        }
    };
}

playbook_tests!(container_breakout_precursors, BREAKOUT, breakout, breakout_miss);
playbook_tests!(attr_locker, ATTR, attr, attr_miss);
playbook_tests!(backup_wipe_first, BACKUP, backup, backup_miss);
playbook_tests!(container_hostpath, HOSTPATH, hostpath, hostpath_miss);
playbook_tests!(db_impact, DB, db, db_miss);
playbook_tests!(fileburst_crypto, FILEBURST, fileburst, fileburst_miss);
playbook_tests!(fuse_overlay, FUSE, fuse, fuse_miss);
playbook_tests!(intermit_encrypt, INTERMIT, intermit, intermit_miss);
playbook_tests!(luks_locker, LUKS, luks, luks_miss);
playbook_tests!(memfd_inplace, MEMFD, memfd, memfd_miss);
playbook_tests!(netshare_only, NETSHARE, netshare, netshare_miss);
playbook_tests!(vm_image_datastore, VM, vm, vm_miss);
playbook_tests!(wiper_service_kill, WIPER, wiper, wiper_miss);

// ---- What the hits carry, and the conditions the near misses do not cover ----

#[test]
fn fileburst_hit_carries_the_counts_and_the_tool() {
    let mut s = Sim::new();
    fileburst(&mut s);
    let hit = s.hits.iter().find(|h| h.playbook_id == FILEBURST).unwrap();
    let burst = &hit.slots["file_burst"].fields;
    assert_eq!(burst["__agg.kind"], "file_burst");
    // The slot keeps its newest hit, made by the last of the 510 renames.
    assert_eq!(burst["__agg.files"], "510");
    assert_eq!(burst["__agg.distinct_topdirs"], "3");
    assert_eq!(burst["__agg.window_sec"], "600");
    assert_eq!(hit.slots["crypto_exec"].fields["exe"], "/usr/bin/openssl");
    // The optional rename-extension slot was filled by the .enc renames.
    assert!(hit.filled_slots.contains(&"new_exts".to_string()), "{:?}", hit.filled_slots);
    assert_eq!(hit.ts, s.t);
    let alert = hit_alert_json(hit);
    assert_eq!(alert["event"]["exe"], "/usr/bin/openssl");
    assert_eq!(alert["technique"], "T1486");
    assert!(alert["findings"].as_array().unwrap().iter().any(|f| f["label"] == "T1486"));
}

#[test]
fn chroot_in_usr_sbin_fills_the_namespace_step() {
    let mut s = Sim::new();
    s.exec(4200, 4000, 0, "/usr/sbin/chroot", "chroot /host /bin/bash");
    s.perf_open(4201, 0, "cat", "/var/run/docker.sock");
    assert_eq!(s.count(BREAKOUT), 1);
}

#[test]
fn memfd_inplace_skips_bursts_with_renames() {
    let mut s = Sim::new();
    s.memfd_exec(8000, "/memfd:payload (deleted)", "payload --run");
    memfd_burst(&mut s, true);
    assert_eq!(s.count(MEMFD), 0, "rename ratio 1.0 is not in place");
}

#[test]
fn intermit_needs_short_writes() {
    let mut s = Sim::new();
    for i in 0..800 {
        let p = format!("/home/bob/v/clip{i}.mp4");
        s.sys_open(7800, &p, O_RDWR, 7);
        s.sys_write(7800, 7, 1024 * 1024);
        s.sys_close(7800, 7);
        s.sys_rename(7800, &p, &format!("{p}.lock"));
    }
    assert_eq!(s.count(INTERMIT), 0, "1 MiB writes are not partial encryption");
}

#[test]
fn netshare_rsync_archive_is_vetoed() {
    let mut s = Sim::new();
    s.exec(8110, 8100, 1000, "/usr/bin/rsync", "rsync --archive /home/a/ /mnt/nfs/projects/");
    netshare(&mut s);
    assert_eq!(s.count(NETSHARE), 0, "a live forbidden slot blocks the playbook");
    // After the forbidden slot's 1800 s TTL the same burst fires.
    s.advance(1801);
    netshare(&mut s);
    assert_eq!(s.count(NETSHARE), 1);
}

#[test]
fn package_manager_vetoes_fileburst() {
    let mut s = Sim::new();
    s.exec(7650, 1, 0, "/usr/bin/apt-get", "apt-get install -y nginx");
    fileburst(&mut s);
    assert_eq!(s.count(FILEBURST), 0);
}

// ---- Mapper ----

fn mapper() -> Mapper {
    Mapper::with_mounts(Box::new(FixedMounts(vec![("/".into(), "ext4".into())])))
}

fn map_one(m: &mut Mapper, o: TelemetryOutput) -> Vec<Fact> {
    let read = |_pid: i64| ProcInfo {
        exe: Some("/usr/bin/python3.11".into()),
        cmdline: Some("python3 job.py".into()),
        cwd: Some("/home/u/work".into()),
    };
    adapter_facts_with(&o, 1_800_000_000, &read)
        .iter()
        .flat_map(|af| m.facts(af))
        .collect()
}

fn raw_out(evt: i64, sys: i64, pid: i64, fd: i64, ret: i64, flags: i64, aux64: u64, path: &str, path2: &str) -> TelemetryOutput {
    let mut d = vec![
        ("ts_ns", "5000000000".to_string()),
        ("timestamp", "1800000000".to_string()),
        ("tgid", pid.to_string()),
        ("uid", "0".to_string()),
        ("fd", fd.to_string()),
        ("ret", ret.to_string()),
        ("evt_type", evt.to_string()),
        ("syscall_id", sys.to_string()),
        ("flags", flags.to_string()),
        ("aux_u64", aux64.to_string()),
        ("comm", "python3".to_string()),
    ];
    if !path.is_empty() {
        d.push(("path", path.to_string()));
    }
    if !path2.is_empty() {
        d.push(("path2", path2.to_string()));
    }
    out("file", "file_open", &d)
}

#[test]
fn open_flags_and_return_pick_the_op() {
    let mut m = mapper();
    let op = |m: &mut Mapper, flags: i64, ret: i64| {
        let f = map_one(m, raw_out(10, 56, 900, ret.max(-1), ret, flags, 0, "/etc/hosts", ""));
        (f[0].fields["op"].clone(), f[0].fields.get("errno").cloned())
    };
    assert_eq!(op(&mut m, O_RDONLY, 3), ("open".into(), None));
    assert_eq!(op(&mut m, O_WRONLY, 3), ("write".into(), None));
    assert_eq!(op(&mut m, O_RDWR, 3), ("write".into(), None));
    assert_eq!(op(&mut m, O_CREAT_ | O_WRONLY, 3), ("write".into(), None));
    assert_eq!(op(&mut m, O_RDONLY, -13), ("open_fail".into(), Some("EACCES".into())));
    assert_eq!(op(&mut m, O_RDONLY, -1), ("open_fail".into(), Some("EPERM".into())));
    assert_eq!(op(&mut m, O_RDONLY, -2), ("open_fail".into(), Some("ENOENT".into())));
}

#[test]
fn write_bytes_follow_the_fd_until_close() {
    let mut m = mapper();
    map_one(&mut m, raw_out(10, 56, 900, 5, 5, O_RDWR, 0, "/srv/a.bin", ""));
    let w = map_one(&mut m, raw_out(13, 64, 900, 5, 0, 0, 4096, "", ""));
    assert_eq!(w[0].fields["path"], "/srv/a.bin");
    assert_eq!(w[0].fields["bytes"], "4096");
    // Same fd number in another process, and a write after close: no file known.
    assert!(map_one(&mut m, raw_out(13, 64, 901, 5, 0, 0, 10, "", "")).is_empty());
    map_one(&mut m, raw_out(18, 57, 900, 5, 0, 0, 0, "", ""));
    assert!(map_one(&mut m, raw_out(13, 64, 900, 5, 0, 0, 10, "", "")).is_empty());
}

#[test]
fn rename_relative_paths_and_skipped_frames() {
    let mut m = mapper();
    let r = map_one(&mut m, raw_out(11, 276, 900, -1, 0, 0, 0, "a.txt", "a.txt.LOCKED"));
    assert_eq!(r[0].fields["op"], "rename");
    // Relative to the cwd /proc reported for the pid.
    assert_eq!(r[0].fields["path"], "/home/u/work/a.txt");
    assert_eq!(r[0].fields["topdir"], "/home");
    assert_eq!(r[0].fields["ext_new"], ".locked");
    let d = map_one(&mut m, raw_out(14, 35, 900, -1, 0, 0, 0, "/var/www/x.php", ""));
    assert_eq!((d[0].fields["op"].as_str(), d[0].fields["topdir"].as_str()), ("delete", "/var/www"));
    // execve at syscall entry is not an exec fact.
    assert!(map_one(&mut m, raw_out(30, 221, 900, -1, 0, 0, 0, "/usr/bin/ls", "")).is_empty());
    // The agent's own events never become facts.
    let mut own = Mapper::new();
    let me = std::process::id() as i64;
    assert!(map_one(&mut own, raw_out(14, 35, me, -1, 0, 0, 0, "/tmp/x", "")).is_empty());
}

#[test]
fn exec_and_memory_facts_get_exe_and_argv_from_proc() {
    let mut m = mapper();
    let e = map_one(
        &mut m,
        out(
            "process",
            "process::exec",
            &[("pid", "77".into()), ("ppid", "1".into()), ("uid", "0".into()), ("comm", "python3".into()), ("ts_ns", "5000000000".into()), ("source", "proc_lifecycle".into())],
        ),
    );
    assert_eq!(e[0].kind, FactKind::Exec);
    assert_eq!(e[0].exe.as_deref(), Some("/usr/bin/python3.11"));
    assert_eq!(e[0].cmd.as_deref(), Some("python3 job.py"));
    assert_eq!(e[0].user.as_deref(), Some("root"));
    assert_eq!(e[0].fields["ppid"], "1");
    // Fork and exit are not exec facts.
    let fork = out("process", "process::fork", &[("pid", "77".into()), ("source", "proc_lifecycle".into())]);
    assert!(map_one(&mut m, fork).is_empty());
    // RWX_MAP is an Exec fact for the process that mapped it. w_then_x is not a fact.
    let rwx = out("memory", "memory::rwx_map", &[("pid", "78".into()), ("source", "wx_exec".into())]);
    assert_eq!(map_one(&mut m, rwx)[0].fields["tags"], "memory::rwx_map,RWX_MAP");
    let wtx = out("memory", "memory::w_then_x", &[("pid", "78".into()), ("source", "wx_exec".into())]);
    assert!(map_one(&mut m, wtx).is_empty());
}

#[test]
fn numeric_file_names_stay_text() {
    let mut m = mapper();
    // As numbers these would read back as 7 and 12.
    let f = map_one(
        &mut m,
        out("file", "file_access", &[("pid", "5".into()), ("path", "007".into()), ("comm", "0012".into()), ("source", "file_access_monitor".into())]),
    );
    assert_eq!(f[0].fields["path"], "/home/u/work/007");
    assert_eq!(f[0].fields["comm"], "0012");
}

#[test]
fn mountinfo_parse_and_lookup() {
    let text = "22 1 0:21 / / rw,relatime - ext4 /dev/vda1 rw\n\
                40 22 0:40 / /mnt/nfs rw - nfs4 srv:/export rw\n\
                41 22 0:41 / /mnt/my\\040share rw - cifs //srv/s rw\n\
                42 22 0:42 / /mnt/nfsx rw - ext4 /dev/vdb rw\n";
    let t = parse_mountinfo(text);
    assert_eq!(t.len(), 4);
    assert_eq!(t[2].0, "/mnt/my share");
    assert_eq!(mount_for(&t, "/mnt/nfs/a/b"), Some(("/mnt/nfs".into(), "nfs4".into())));
    // Whole components only: /mnt/nfsx is not under /mnt/nfs.
    assert_eq!(mount_for(&t, "/mnt/nfsx/a").unwrap().1, "ext4");
    assert_eq!(mount_for(&t, "/mnt/my share/f").unwrap().1, "cifs");
    assert_eq!(mount_for(&t, "/etc/hosts").unwrap().0, "/");
}

// ---- Time ----

#[test]
fn kernel_stamps_and_missing_stamps_become_unix_seconds() {
    let mut m = mapper();
    // A raw frame carries ts_ns from bpf_ktime_get_ns (time since boot).
    let ns = clock::unix_secs_to_mono_ns(1_800_000_123);
    let mut o = raw_out(14, 35, 900, -1, 0, 0, 0, "/srv/x", "");
    o.data.insert("ts_ns".into(), ns.to_string());
    // A stale boot-seconds "timestamp" next to it is ignored in favour of ts_ns.
    o.data.insert("timestamp".into(), (ns / 1_000_000_000).to_string());
    assert_eq!(map_one(&mut m, o)[0].ts, 1_800_000_123);
    // file_access_monitor frames have no stamp and get the time they reached the adapter.
    let o = out("file", "file_access", &[("pid", "5".into()), ("path", "/srv/y".into()), ("source", "file_access_monitor".into())]);
    assert_eq!(map_one(&mut m, o)[0].ts, 1_800_000_000);
}

#[test]
fn facts_from_two_clocks_land_in_one_window() {
    // A wx frame (kernel stamp) and a perf open (no stamp, adapter time) 20 s apart
    // complete a playbook only if both are on the same clock.
    let mut s = Sim::new();
    s.exec(4100, 4000, 0, "/usr/bin/unshare", "unshare --mount --fork /bin/sh");
    s.advance(20);
    s.perf_open(4101, 0, "cat", "/proc/kallsyms");
    let hit = s.hits.iter().find(|h| h.playbook_id == BREAKOUT).unwrap();
    let a = hit.slots["ns_priv_tools"].ts;
    let b = hit.slots["host_fs_touch"].ts;
    assert_eq!(b - a, 20);
}

/// Prints the rationale of each playbook's hit. Run with --nocapture to read it.
#[test]
fn every_playbook_explains_its_hit() {
    let cases: [(&str, fn(&mut Sim)); 13] = [
        (BREAKOUT, breakout),
        (ATTR, attr),
        (BACKUP, backup),
        (HOSTPATH, hostpath),
        (DB, db),
        (FILEBURST, fileburst),
        (FUSE, fuse),
        (INTERMIT, intermit),
        (LUKS, luks),
        (MEMFD, memfd),
        (NETSHARE, netshare),
        (VM, vm),
        (WIPER, wiper),
    ];
    for (id, seq) in cases {
        let mut s = Sim::new();
        seq(&mut s);
        let hit = s.hits.iter().find(|h| h.playbook_id == id).expect(id);
        eprintln!("== {id} (also fired: {:?})", s.fired().iter().filter(|x| *x != id).collect::<Vec<_>>());
        for line in &hit.rationale {
            eprintln!("   {line}");
        }
    }
}

// ---- Sensor gaps closed in the kernel: renameat and the exec /proc race ----

#[test]
fn glibc_renameat_frames_fill_the_burst() {
    // glibc rename() enters renameat (38), not renameat2 (276). Same playbook result.
    let mut s = Sim::new();
    for i in 0..510 {
        let dir = ["/home/alice/docs", "/srv/share", "/opt/app/data"][i % 3];
        let p = format!("{dir}/f{i}.txt");
        s.sys_renameat(7600, &p, &format!("{p}.enc"));
    }
    s.advance(5);
    openssl(&mut s);
    assert_eq!(s.count(FILEBURST), 1, "fired: {:?}", s.fired());
    let hit = s.hits.iter().find(|h| h.playbook_id == FILEBURST).unwrap();
    assert_eq!(hit.slots["file_burst"].fields["__agg.files"], "510");
}

#[test]
fn attr_locker_fires_when_every_chattr_was_gone_from_proc() {
    // chattr exits in about a millisecond, so live every one of the 200 runs can be gone
    // before its frame is read. The kernel copy of filename and argv is what is left.
    let mut s = Sim::new();
    for i in 0..200 {
        let argv = format!("chattr +i /home/u/f{i} /srv/d/f{i} /opt/a/f{i}");
        s.exec_gone(5000 + i, 4999, "/usr/bin/chattr", &argv);
        if i % 10 == 9 {
            s.advance(1);
        }
    }
    for i in 0..300 {
        s.sys_open(6000 + i % 5, &format!("/home/u/f{}", i % 200), O_RDWR, -13);
    }
    assert_eq!(s.count(ATTR), 1, "fired: {:?}", s.fired());
}

#[test]
fn exec_gone_from_proc_without_a_kernel_copy_cannot_match() {
    // The same run with frames from an older sensor (no filename, no argv) gives Exec
    // facts with no exe or cmd, so the chattr slot never fills.
    let mut s = Sim::new();
    for i in 0..200 {
        s.exec_gone(5000 + i, 4999, "", "");
    }
    for i in 0..300 {
        s.sys_open(6000 + i % 5, &format!("/home/u/f{}", i % 200), O_RDWR, -13);
    }
    assert_eq!(s.count(ATTR), 0, "fired: {:?}", s.fired());
}

#[test]
fn proc_wins_over_the_kernel_copy_when_both_exist() {
    // A script: the kernel reports the script path, /proc/<pid>/exe the interpreter.
    // /proc is read first, so the exe the playbooks were written against is kept.
    let mut m = mapper();
    let e = map_one(
        &mut m,
        out(
            "process",
            "process::exec",
            &[
                ("pid", "77".into()),
                ("comm", "systemctl".into()),
                ("source", "proc_lifecycle".into()),
                ("exec_filename", "/usr/local/bin/systemctl".into()),
                ("exec_argv", "/bin/sh /usr/local/bin/systemctl stop x".into()),
            ],
        ),
    );
    assert_eq!(e[0].exe.as_deref(), Some("/usr/bin/python3.11"));
    assert_eq!(e[0].cmd.as_deref(), Some("python3 job.py"));
}
