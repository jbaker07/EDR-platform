//! Kernel ABI of src/ebpf/edr_guard.bpf.c: config slots, map keys and the
//! event record. The tests at the bottom check every layout against the C side.

pub const TASK_COMM_LEN: usize = 16;
pub const UNIX_PATH_MAX: usize = 108;
pub const EXE_NAME_LEN: usize = 32;

/// guard_config slots (CFG_* in the C file).
pub const CFG_SOCKET_MODE: u32 = 0;
pub const CFG_MEMFD_MODE: u32 = 1;
pub const CFG_SELF_MODE: u32 = 2;
pub const CFG_AGENT_TGID: u32 = 3;
pub const CFG_SCOPE_CGROUP: u32 = 4;

/// bpf() commands that open an object by id (enum bpf_cmd in uapi/linux/bpf.h).
pub const BPF_PROG_GET_FD_BY_ID: u32 = 13;
pub const BPF_MAP_GET_FD_BY_ID: u32 = 14;
pub const BPF_LINK_GET_FD_BY_ID: u32 = 30;

/// An executable as the kernel names it: inode number and device of
/// mm->exe_file. Key of the socket_clients and memfd_allow maps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExeId {
    pub ino: u64,
    /// Kernel dev_t, major << 20 | minor.
    pub dev: u32,
}

impl ExeId {
    /// From stat() of a file. st_dev comes in the user space encoding and the
    /// kernel compares sb->s_dev, which packs major and minor differently.
    pub fn from_stat(st_dev: u64, st_ino: u64) -> Self {
        Self {
            ino: st_ino,
            dev: kernel_dev(st_dev),
        }
    }

    /// struct exe_id: ino (8 bytes), dev (4), pad (4, zero).
    pub fn key(&self) -> [u8; 16] {
        let mut key = [0u8; 16];
        key[..8].copy_from_slice(&self.ino.to_ne_bytes());
        key[8..12].copy_from_slice(&self.dev.to_ne_bytes());
        key
    }
}

/// User space dev_t (glibc encoding: minor low 8 bits, major 12 bits, rest of
/// minor, rest of major) to the kernel's internal dev_t (MKDEV, 20 bit minor).
pub fn kernel_dev(st_dev: u64) -> u32 {
    let major = ((st_dev >> 32) & 0xffff_f000) | ((st_dev >> 8) & 0x0fff);
    let minor = ((st_dev >> 12) & 0xffff_ff00) | (st_dev & 0x00ff);
    ((major << 20) | minor) as u32
}

/// struct obj_key: the bpf() command that opens the object, then its id.
pub fn obj_key(cmd: u32, id: u32) -> [u8; 8] {
    let mut key = [0u8; 8];
    key[..4].copy_from_slice(&cmd.to_ne_bytes());
    key[4..].copy_from_slice(&id.to_ne_bytes());
    key
}

/// What a guard decided about (KIND_* in the C file).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GuardKind {
    RuntimeSocket,
    MemfdExec,
    SignalToAgent,
    PtraceAgent,
    BpfAgentObject,
}

impl GuardKind {
    pub fn from_raw(kind: u8) -> Option<Self> {
        match kind {
            1 => Some(Self::RuntimeSocket),
            2 => Some(Self::MemfdExec),
            3 => Some(Self::SignalToAgent),
            4 => Some(Self::PtraceAgent),
            5 => Some(Self::BpfAgentObject),
            _ => None,
        }
    }

    /// The guard that owns this decision, as it appears in tags.
    pub fn guard(self) -> &'static str {
        match self {
            Self::RuntimeSocket => "runtime_socket",
            Self::MemfdExec => "memfd_exec",
            Self::SignalToAgent | Self::PtraceAgent | Self::BpfAgentObject => "self_protect",
        }
    }

    /// The LSM hook that made the decision.
    pub fn hook(self) -> &'static str {
        match self {
            Self::RuntimeSocket => "unix_stream_connect",
            Self::MemfdExec => "bprm_check_security",
            Self::SignalToAgent => "task_kill",
            Self::PtraceAgent => "ptrace_access_check",
            Self::BpfAgentObject => "bpf",
        }
    }

    /// T1611 Escape to Host and T1610 Deploy Container for the runtime socket,
    /// T1620 Reflective Code Loading for memfd exec, T1562.001 Impair Defenses:
    /// Disable or Modify Tools for tampering with the agent.
    pub fn mitre(self) -> &'static [&'static str] {
        match self {
            Self::RuntimeSocket => &["T1611", "T1610"],
            Self::MemfdExec => &["T1620"],
            Self::SignalToAgent | Self::PtraceAgent | Self::BpfAgentObject => &["T1562.001"],
        }
    }
}

/// What the guard did (ACTION_* in the C file).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GuardAction {
    /// Monitor mode: the action went ahead and was reported.
    WouldBlock,
    /// Enforce mode: the kernel refused the action with EPERM.
    Blocked,
}

impl GuardAction {
    pub fn from_raw(action: u8) -> Option<Self> {
        match action {
            1 => Some(Self::WouldBlock),
            2 => Some(Self::Blocked),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WouldBlock => "would_block",
            Self::Blocked => "blocked",
        }
    }
}

/// Mirrors struct guard_event in edr_guard.bpf.c.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GuardEvent {
    pub timestamp_ns: u64,
    pub exe_ino: u64,
    pub target_ino: u64,
    pub exe_dev: u32,
    pub tgid: u32,
    pub tid: u32,
    pub uid: u32,
    pub target_pid: u32,
    pub detail: u32,
    pub kind: u8,
    pub action: u8,
    pub pad: u16,
    pub comm: [u8; TASK_COMM_LEN],
    pub exe_name: [u8; EXE_NAME_LEN],
    pub target: [u8; UNIX_PATH_MAX],
}

pub const GUARD_EVENT_SIZE: usize = 208;
const _: () = assert!(std::mem::size_of::<GuardEvent>() == GUARD_EVENT_SIZE);

impl GuardEvent {
    /// Copies one ring buffer record. None unless it is exactly one event, so
    /// a layout change on one side only is refused instead of misread.
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() != GUARD_EVENT_SIZE {
            return None;
        }
        // SAFETY: the length is checked above, read_unaligned has no alignment
        // requirement, and any bit pattern is valid for these integer and byte
        // array fields.
        Some(unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const Self) })
    }

    pub fn kind(&self) -> Option<GuardKind> {
        GuardKind::from_raw(self.kind)
    }

    pub fn action(&self) -> Option<GuardAction> {
        GuardAction::from_raw(self.action)
    }

    pub fn exe_id(&self) -> ExeId {
        ExeId {
            ino: self.exe_ino,
            dev: self.exe_dev,
        }
    }

    pub fn comm(&self) -> String {
        c_string(&self.comm)
    }

    pub fn exe_name(&self) -> String {
        c_string(&self.exe_name)
    }

    pub fn target(&self) -> String {
        c_string(&self.target)
    }
}

/// Text up to the first NUL.
fn c_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::offset_of;

    #[test]
    fn event_layout_matches_the_c_struct() {
        assert_eq!(std::mem::size_of::<GuardEvent>(), 208);
        assert_eq!(offset_of!(GuardEvent, exe_ino), 8);
        assert_eq!(offset_of!(GuardEvent, target_ino), 16);
        assert_eq!(offset_of!(GuardEvent, exe_dev), 24);
        assert_eq!(offset_of!(GuardEvent, tgid), 28);
        assert_eq!(offset_of!(GuardEvent, uid), 36);
        assert_eq!(offset_of!(GuardEvent, target_pid), 40);
        assert_eq!(offset_of!(GuardEvent, detail), 44);
        assert_eq!(offset_of!(GuardEvent, kind), 48);
        assert_eq!(offset_of!(GuardEvent, action), 49);
        assert_eq!(offset_of!(GuardEvent, comm), 52);
        assert_eq!(offset_of!(GuardEvent, exe_name), 68);
        assert_eq!(offset_of!(GuardEvent, target), 100);
    }

    /// A record written byte by byte at the C offsets, never through the Rust struct.
    #[test]
    fn event_decodes_from_c_layout_bytes() {
        let mut b = vec![0u8; 208];
        b[0..8].copy_from_slice(&123_456_789u64.to_ne_bytes());
        b[8..16].copy_from_slice(&0x1122_3344u64.to_ne_bytes());
        b[16..24].copy_from_slice(&777u64.to_ne_bytes());
        b[24..28].copy_from_slice(&0x0030_0001u32.to_ne_bytes());
        b[28..32].copy_from_slice(&4321u32.to_ne_bytes());
        b[32..36].copy_from_slice(&4322u32.to_ne_bytes());
        b[36..40].copy_from_slice(&1000u32.to_ne_bytes());
        b[44..48].copy_from_slice(&0u32.to_ne_bytes());
        b[48] = 1;
        b[49] = 2;
        b[52..58].copy_from_slice(b"docker");
        b[68..75].copy_from_slice(b"python3");
        b[100..120].copy_from_slice(b"/var/run/docker.sock");

        let e = GuardEvent::from_bytes(&b).unwrap();
        assert_eq!((e.timestamp_ns, e.tgid, e.tid, e.uid), (123_456_789, 4321, 4322, 1000));
        assert_eq!(
            e.exe_id(),
            ExeId {
                ino: 0x1122_3344,
                dev: 0x0030_0001
            }
        );
        assert_eq!(e.target_ino, 777);
        assert_eq!(e.kind(), Some(GuardKind::RuntimeSocket));
        assert_eq!(e.action(), Some(GuardAction::Blocked));
        // The comm says docker, the executable is python3. Decisions use the executable.
        assert_eq!(e.comm(), "docker");
        assert_eq!(e.exe_name(), "python3");
        assert_eq!(e.target(), "/var/run/docker.sock");
    }

    #[test]
    fn event_size_guard_refuses_other_lengths() {
        assert!(GuardEvent::from_bytes(&[0u8; 207]).is_none());
        assert!(GuardEvent::from_bytes(&[0u8; 209]).is_none());
        assert!(GuardEvent::from_bytes(&[0u8; 208]).is_some());
    }

    #[test]
    fn unknown_kind_and_action_values_decode_as_none() {
        assert_eq!(GuardKind::from_raw(0), None);
        assert_eq!(GuardKind::from_raw(6), None);
        assert_eq!(GuardAction::from_raw(0), None);
        assert_eq!(GuardAction::from_raw(3), None);
        assert_eq!(GuardKind::from_raw(5), Some(GuardKind::BpfAgentObject));
    }

    #[test]
    fn kernel_dev_converts_the_user_space_encoding() {
        // sda1, major 8 minor 1: st_dev 0x801, kernel 8 << 20 | 1.
        assert_eq!(kernel_dev(0x801), 0x0080_0001);
        // overlayfs and tmpfs use major 0. Minor 0x2e stays in the low byte.
        assert_eq!(kernel_dev(0x2e), 0x2e);
        // Minor over 255: glibc moves its high bits up by 12.
        assert_eq!(kernel_dev(0x0010_0023), 0x123);
        // Major over 255 (the kernel allows 12 bits): glibc keeps it at bit 8.
        assert_eq!(kernel_dev((0x123 << 8) | 0x05), (0x123 << 20) | 0x05);
    }

    #[test]
    fn exe_key_is_ino_then_dev_then_zero_pad() {
        let key = ExeId {
            ino: 0x0102_0304_0506_0708,
            dev: 0x0a0b_0c0d,
        }
        .key();
        assert_eq!(&key[..8], &0x0102_0304_0506_0708u64.to_ne_bytes());
        assert_eq!(&key[8..12], &0x0a0b_0c0du32.to_ne_bytes());
        assert_eq!(&key[12..], &[0, 0, 0, 0]);
        let from_stat = ExeId::from_stat(0x801, 42);
        assert_eq!(from_stat, ExeId { ino: 42, dev: 0x0080_0001 });
    }

    #[test]
    fn obj_key_is_cmd_then_id() {
        let key = obj_key(BPF_MAP_GET_FD_BY_ID, 677);
        assert_eq!(&key[..4], &14u32.to_ne_bytes());
        assert_eq!(&key[4..], &677u32.to_ne_bytes());
    }

    #[test]
    fn every_kind_maps_to_its_guard_hook_and_technique() {
        assert_eq!(GuardKind::RuntimeSocket.mitre(), &["T1611", "T1610"]);
        assert_eq!(GuardKind::MemfdExec.mitre(), &["T1620"]);
        for k in [
            GuardKind::SignalToAgent,
            GuardKind::PtraceAgent,
            GuardKind::BpfAgentObject,
        ] {
            assert_eq!(k.guard(), "self_protect");
            assert_eq!(k.mitre(), &["T1562.001"]);
        }
        assert_eq!(GuardKind::MemfdExec.hook(), "bprm_check_security");
    }
}
