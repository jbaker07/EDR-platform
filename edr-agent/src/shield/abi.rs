//! Kernel ABI of src/ebpf/edr_shield.bpf.c: config slots, map keys, the rule
//! row and the event record. The tests at the bottom check every layout against
//! the C side byte by byte.

use std::net::IpAddr;

use crate::guard::abi::ExeId;

/// shield_cfg slots (SCFG_* in the C file).
pub const SCFG_SCOPE: u32 = 0;
pub const SCFG_EXEC_ROWS: u32 = 1;
pub const SCFG_FILE_ROWS: u32 = 2;
pub const SCFG_CONN_ROWS: u32 = 3;

pub const MODE_MONITOR: u8 = 1;
pub const MODE_ENFORCE: u8 = 2;

pub const KIND_EXEC: u8 = 1;
pub const KIND_FILE: u8 = 2;
pub const KIND_CONNECT: u8 = 3;

pub const ACTION_WOULD_BLOCK: u8 = 1;
pub const ACTION_BLOCKED: u8 = 2;

/// max_entries of each rule map.
pub const ROWS_MAX: usize = 16384;
/// max_entries of shield_stats. A shield's number (1 to STATS_SLOTS - 1) is its
/// index there, so at most STATS_SLOTS - 1 shields exist at once.
pub const STATS_SLOTS: u32 = 32768;
/// struct shield_stat: hits, then last_report_ns, per CPU.
pub const SHIELD_STAT_SIZE: usize = 16;

pub const TASK_COMM_LEN: usize = 16;
pub const EXE_NAME_LEN: usize = 32;
pub const TARGET_LEN: usize = 64;

/// Bits of the LPM key in front of the address: port (16), family (8), pad (8).
pub const CONN_KEY_HEAD_BITS: u32 = 32;
/// prefixlen (4), then 24 bytes of data: port, family, pad, address, and a tail
/// that pads the data to a multiple of 8 so the trie's value behind it is aligned.
pub const CONN_KEY_SIZE: usize = 28;
pub const FILE_KEY_SIZE: usize = 16;

/// struct file_key, the same 16 bytes as the guard's exe_id.
pub fn file_key(id: &ExeId) -> [u8; FILE_KEY_SIZE] {
    id.key()
}

/// struct conn_key for a rule: network address, prefix length in bits of the
/// address, and port (0 for any port).
pub fn conn_key(addr: &IpAddr, prefix: u8, port: u16) -> [u8; CONN_KEY_SIZE] {
    let mut k = [0u8; CONN_KEY_SIZE];
    let (family, bytes): (u8, Vec<u8>) = match addr {
        IpAddr::V4(a) => (4, a.octets().to_vec()),
        IpAddr::V6(a) => (6, a.octets().to_vec()),
    };
    let prefixlen = CONN_KEY_HEAD_BITS + prefix as u32;
    k[0..4].copy_from_slice(&prefixlen.to_ne_bytes());
    k[4..6].copy_from_slice(&port.to_ne_bytes());
    k[6] = family;
    k[8..8 + bytes.len()].copy_from_slice(&bytes);
    // Bits past the prefix must be zero, so two spellings of one network are one key.
    let addr_bits = bytes.len() * 8;
    for bit in prefix as usize..addr_bits {
        k[8 + bit / 8] &= !(0x80u8 >> (bit % 8));
    }
    k
}

/// Mirrors struct shield_rule. The BPF programs only read it. Hit counts live in
/// shield_stats.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ShieldRule {
    pub expires_ns: u64,
    pub not_before_ns: u64,
    pub cgroup_id: u64,
    pub id: u32,
    pub mode: u8,
    pub kind: u8,
}

pub const SHIELD_RULE_SIZE: usize = 32;

impl ShieldRule {
    pub fn to_bytes(&self) -> [u8; SHIELD_RULE_SIZE] {
        let mut b = [0u8; SHIELD_RULE_SIZE];
        b[0..8].copy_from_slice(&self.expires_ns.to_ne_bytes());
        b[8..16].copy_from_slice(&self.not_before_ns.to_ne_bytes());
        b[16..24].copy_from_slice(&self.cgroup_id.to_ne_bytes());
        b[24..28].copy_from_slice(&self.id.to_ne_bytes());
        b[28] = self.mode;
        b[29] = self.kind;
        b
    }

    pub fn from_bytes(b: &[u8]) -> Option<Self> {
        if b.len() != SHIELD_RULE_SIZE {
            return None;
        }
        let u64_at = |i: usize| u64::from_ne_bytes(b[i..i + 8].try_into().unwrap());
        Some(Self {
            expires_ns: u64_at(0),
            not_before_ns: u64_at(8),
            cgroup_id: u64_at(16),
            id: u32::from_ne_bytes(b[24..28].try_into().unwrap()),
            mode: b[28],
            kind: b[29],
        })
    }
}

/// The hit count in one CPU's struct shield_stat.
pub fn stat_hits(b: &[u8]) -> u64 {
    b.get(0..8).map(|h| u64::from_ne_bytes(h.try_into().unwrap())).unwrap_or(0)
}

/// Mirrors struct shield_event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShieldEvent {
    pub timestamp_ns: u64,
    pub exe_ino: u64,
    pub target_ino: u64,
    pub cgroup_id: u64,
    pub exe_dev: u32,
    pub tgid: u32,
    pub tid: u32,
    pub uid: u32,
    pub shield_id: u32,
    pub target_dev: u32,
    pub port: u16,
    pub kind: u8,
    pub action: u8,
    pub family: u8,
    pub addr: [u8; 16],
    pub comm: String,
    pub exe_name: String,
    pub target: String,
}

pub const SHIELD_EVENT_SIZE: usize = 192;

fn c_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

impl ShieldEvent {
    /// One ring buffer record. None unless it is exactly one event.
    pub fn from_bytes(b: &[u8]) -> Option<Self> {
        if b.len() != SHIELD_EVENT_SIZE {
            return None;
        }
        let u64_at = |i: usize| u64::from_ne_bytes(b[i..i + 8].try_into().unwrap());
        let u32_at = |i: usize| u32::from_ne_bytes(b[i..i + 4].try_into().unwrap());
        let mut addr = [0u8; 16];
        addr.copy_from_slice(&b[64..80]);
        Some(Self {
            timestamp_ns: u64_at(0),
            exe_ino: u64_at(8),
            target_ino: u64_at(16),
            cgroup_id: u64_at(24),
            exe_dev: u32_at(32),
            tgid: u32_at(36),
            tid: u32_at(40),
            uid: u32_at(44),
            shield_id: u32_at(48),
            target_dev: u32_at(52),
            port: u16::from_ne_bytes([b[56], b[57]]),
            kind: b[58],
            action: b[59],
            family: b[60],
            addr,
            comm: c_string(&b[80..96]),
            exe_name: c_string(&b[96..128]),
            target: c_string(&b[128..192]),
        })
    }

    /// The destination of a connect_deny decision.
    pub fn dest(&self) -> Option<IpAddr> {
        match self.family {
            4 => Some(IpAddr::from([self.addr[0], self.addr[1], self.addr[2], self.addr[3]])),
            6 => Some(IpAddr::from(self.addr)),
            _ => None,
        }
    }

    pub fn exe_id(&self) -> ExeId {
        ExeId {
            ino: self.exe_ino,
            dev: self.exe_dev,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_bytes_follow_the_c_offsets() {
        let r = ShieldRule {
            expires_ns: 11,
            not_before_ns: 22,
            cgroup_id: 33,
            id: 66,
            mode: MODE_ENFORCE,
            kind: KIND_CONNECT,
        };
        let b = r.to_bytes();
        assert_eq!(b.len(), 32);
        assert_eq!(&b[0..8], &11u64.to_ne_bytes());
        assert_eq!(&b[16..24], &33u64.to_ne_bytes());
        assert_eq!(&b[24..28], &66u32.to_ne_bytes());
        assert_eq!((b[28], b[29], b[30], b[31]), (2, 3, 0, 0));
        assert_eq!(ShieldRule::from_bytes(&b), Some(r));
        assert!(ShieldRule::from_bytes(&b[..31]).is_none());
        let mut stat = [0u8; SHIELD_STAT_SIZE];
        stat[..8].copy_from_slice(&9u64.to_ne_bytes());
        assert_eq!(stat_hits(&stat), 9);
    }

    /// The panic this layout prevents: the trie keeps a value right after the key
    /// data, so data of 20 bytes put every value on a 4 byte boundary. The data is
    /// now 24 bytes, and the rule value has no field a program writes.
    #[test]
    fn the_lpm_key_data_is_a_multiple_of_8_bytes() {
        assert_eq!((CONN_KEY_SIZE - 4) % 8, 0);
        assert_eq!(SHIELD_RULE_SIZE % 8, 0);
        // The longest prefix stops before the tail.
        assert!(CONN_KEY_HEAD_BITS + 128 <= ((CONN_KEY_SIZE - 4) * 8) as u32);
    }

    #[test]
    fn conn_keys_carry_prefix_port_family_and_a_masked_address() {
        let a: IpAddr = "203.0.113.77".parse().unwrap();
        let k = conn_key(&a, 24, 8000);
        assert_eq!(&k[0..4], &(32u32 + 24).to_ne_bytes());
        assert_eq!(&k[4..6], &8000u16.to_ne_bytes());
        assert_eq!(k[6], 4);
        // .77 is past the /24, so it is cleared.
        assert_eq!(&k[8..12], &[203, 0, 113, 0]);
        assert!(k[12..].iter().all(|b| *b == 0));
        assert_eq!(k, conn_key(&"203.0.113.0".parse().unwrap(), 24, 8000));

        let v6: IpAddr = "2001:db8::1".parse().unwrap();
        let k = conn_key(&v6, 128, 0);
        assert_eq!(&k[0..4], &160u32.to_ne_bytes());
        assert_eq!(k[6], 6);
        assert_eq!(&k[8..10], &[0x20, 0x01]);
        assert_eq!(k[23], 1);
        let k = conn_key(&v6, 33, 0);
        // 2001:0db8 with only the first bit of the third word kept.
        assert_eq!(&k[8..13], &[0x20, 0x01, 0x0d, 0xb8, 0x00]);
    }

    #[test]
    fn event_decodes_from_c_layout_bytes() {
        let mut b = vec![0u8; 192];
        b[0..8].copy_from_slice(&7u64.to_ne_bytes());
        b[16..24].copy_from_slice(&999u64.to_ne_bytes());
        b[36..40].copy_from_slice(&4242u32.to_ne_bytes());
        b[48..52].copy_from_slice(&17u32.to_ne_bytes());
        b[56..58].copy_from_slice(&8000u16.to_ne_bytes());
        b[58] = KIND_CONNECT;
        b[59] = ACTION_BLOCKED;
        b[60] = 4;
        b[64..68].copy_from_slice(&[203, 0, 113, 10]);
        b[80..84].copy_from_slice(b"bash");
        b[96..100].copy_from_slice(b"bash");
        b[128..135].copy_from_slice(b"payload");
        let e = ShieldEvent::from_bytes(&b).unwrap();
        assert_eq!((e.timestamp_ns, e.target_ino, e.tgid, e.shield_id, e.port), (7, 999, 4242, 17, 8000));
        assert_eq!((e.kind, e.action), (KIND_CONNECT, ACTION_BLOCKED));
        assert_eq!(e.dest(), Some("203.0.113.10".parse().unwrap()));
        assert_eq!((e.comm.as_str(), e.target.as_str()), ("bash", "payload"));
        assert!(ShieldEvent::from_bytes(&b[..191]).is_none());
    }
}
