// edr-agent/src/modules/dns_tunnel.rs
//
// User-space half of the DNS tunnel monitor. src/ebpf/dns_tunnel_monitor.c
// copies the first bytes of every UDP datagram sent to port 53 into the
// dns_events ring buffer. This file holds:
//   1. DnsSendEvent, the Rust mirror of the C event struct
//   2. parse_question, a bounds-checked parser for the DNS question
//   3. DnsTunnelDetector, the per-query rules and the per-process volume rule
//   4. start_dns_monitor, the libbpf-rs loader (Linux only)
// It uses nothing but std and libbpf-rs, so harness/dns_live compiles this
// same file for the live kernel test.

use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Bytes of each datagram the kernel side copies. Must match DNS_PAYLOAD_CAP in the C file.
pub const DNS_PAYLOAD_CAP: usize = 320;

const DNS_HEADER_LEN: usize = 12;
/// RFC 1035 section 2.3.4: a label is at most 63 bytes.
const MAX_LABEL_LEN: usize = 63;
/// RFC 1035 section 2.3.4: a name is at most 255 bytes on the wire, length bytes included.
const MAX_NAME_WIRE_LEN: usize = 255;

pub const QTYPE_A: u16 = 1;
pub const QTYPE_CNAME: u16 = 5;
pub const QTYPE_NULL: u16 = 10;
pub const QTYPE_MX: u16 = 15;
pub const QTYPE_TXT: u16 = 16;
pub const QTYPE_AAAA: u16 = 28;
pub const QTYPE_SRV: u16 = 33;
/// Private-use record type that iodine can tunnel over (its PRIVATE mode).
pub const QTYPE_PRIVATE_IODINE: u16 = 65399;

// ---- Scoring thresholds ----

/// Real hostnames rarely pass 80 characters. Tunnels pack data toward the 253-character limit.
pub const LONG_NAME_LEN: usize = 100;
/// Tunnel tools fill labels toward the 63-byte limit. Long real labels (cloud load balancer names) stay near 50.
pub const LONG_LABEL_LEN: usize = 56;
/// Bits per character. Random base32 or base64 passes this at 30+ characters, ordinary names sit near 3. Hex tops out at 4.0, so hex is left to the length and volume rules.
pub const HIGH_ENTROPY_BITS: f64 = 4.0;
/// Entropy of a short string is noisy, so the entropy rule needs at least this many subdomain characters.
pub const ENTROPY_MIN_LEN: usize = 30;
/// Record types that carry data back to the client. TXT, NULL and private types hold raw bytes, CNAME, MX and SRV return a name.
pub const TUNNEL_QTYPES: [u16; 6] = [
    QTYPE_CNAME,
    QTYPE_NULL,
    QTYPE_MX,
    QTYPE_TXT,
    QTYPE_SRV,
    QTYPE_PRIVATE_IODINE,
];
/// Unique subdomains of one base domain from one process before the volume rule fires. A tunnel needs a fresh name per chunk so resolver caches do not answer for it.
pub const VOLUME_UNIQUE_LIMIT: usize = 20;
/// Length of the volume window in seconds.
pub const VOLUME_WINDOW_SECS: u64 = 60;
/// Most (process, base domain) windows kept at once. Each holds at most VOLUME_UNIQUE_LIMIT 8-byte hashes, so this caps memory at well under 1 MB.
pub const MAX_TRACKED_WINDOWS: usize = 4096;

// ---- Kernel event ----

/// Mirror of struct dns_send_event in dns_tunnel_monitor.c. Field order, types
/// and size must match exactly (checked by event_layout_matches_c below).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DnsSendEvent {
    pub tgid: u32,
    pub tid: u32,
    pub uid: u32,
    pub comm: [u8; 16],
    pub family: u16,
    pub dport: u16,
    pub orig_len: u32,
    pub cap_len: u32,
    pub payload: [u8; DNS_PAYLOAD_CAP],
}

impl DnsSendEvent {
    /// Copies one ring buffer record into an event. None if the record is too short.
    pub fn from_bytes(buf: &[u8]) -> Option<DnsSendEvent> {
        if buf.len() < std::mem::size_of::<DnsSendEvent>() {
            return None;
        }
        // SAFETY: the length is checked above, read_unaligned has no alignment
        // requirement, and every field is an integer or a byte array, so any
        // bit pattern is a valid DnsSendEvent.
        Some(unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const DnsSendEvent) })
    }

    /// The captured bytes. cap_len comes from the kernel but is still clamped.
    pub fn payload(&self) -> &[u8] {
        let n = (self.cap_len as usize).min(DNS_PAYLOAD_CAP);
        &self.payload[..n]
    }

    pub fn comm(&self) -> String {
        let end = self.comm.iter().position(|&b| b == 0).unwrap_or(self.comm.len());
        String::from_utf8_lossy(&self.comm[..end]).into_owned()
    }

    pub fn family_name(&self) -> &'static str {
        match self.family {
            2 => "AF_INET",
            10 => "AF_INET6",
            _ => "AF_OTHER",
        }
    }
}

// ---- DNS question parser ----

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnsParseError {
    /// Fewer than 12 bytes, so not even a header.
    TooShort,
    /// QDCOUNT is 0.
    NoQuestion,
    /// A length byte of 64 to 191, which is not a valid label.
    LabelTooLong(usize),
    /// A compression pointer (top two bits set). A client never needs one in
    /// the question, and following it would mean jumping around the packet.
    CompressionPointer,
    /// More than 255 bytes on the wire.
    NameTooLong,
    /// The name or the type and class run past the captured bytes.
    Truncated,
}

impl fmt::Display for DnsParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DnsParseError::TooShort => write!(f, "shorter than a DNS header"),
            DnsParseError::NoQuestion => write!(f, "QDCOUNT is 0"),
            DnsParseError::LabelTooLong(n) => write!(f, "label length byte {n} is over 63"),
            DnsParseError::CompressionPointer => write!(f, "compression pointer in the question"),
            DnsParseError::NameTooLong => write!(f, "name is over 255 bytes"),
            DnsParseError::Truncated => write!(f, "question runs past the captured bytes"),
        }
    }
}

/// The first question of a DNS message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    /// Labels in order, lowercased. DNS names are case-insensitive and some
    /// resolvers randomize case (DNS 0x20), which would inflate entropy.
    pub labels: Vec<Vec<u8>>,
    pub qtype: u16,
    pub qclass: u16,
}

/// Parses the header and the first question. Every read is bounds checked and
/// any problem returns an error, so malformed input cannot panic.
pub fn parse_question(packet: &[u8]) -> Result<DnsQuestion, DnsParseError> {
    if packet.len() < DNS_HEADER_LEN {
        return Err(DnsParseError::TooShort);
    }
    let qdcount = u16::from_be_bytes([packet[4], packet[5]]);
    if qdcount == 0 {
        return Err(DnsParseError::NoQuestion);
    }

    let mut pos = DNS_HEADER_LEN;
    let mut labels = Vec::new();
    let mut wire_len = 1; // the zero byte that ends the name
    loop {
        let len_byte = *packet.get(pos).ok_or(DnsParseError::Truncated)?;
        pos += 1;
        if len_byte == 0 {
            break;
        }
        if len_byte & 0xC0 == 0xC0 {
            return Err(DnsParseError::CompressionPointer);
        }
        let len = len_byte as usize;
        if len > MAX_LABEL_LEN {
            return Err(DnsParseError::LabelTooLong(len));
        }
        wire_len += 1 + len;
        if wire_len > MAX_NAME_WIRE_LEN {
            return Err(DnsParseError::NameTooLong);
        }
        let label = packet.get(pos..pos + len).ok_or(DnsParseError::Truncated)?;
        labels.push(label.to_ascii_lowercase());
        pos += len;
    }

    let fixed = packet.get(pos..pos + 4).ok_or(DnsParseError::Truncated)?;
    Ok(DnsQuestion {
        labels,
        qtype: u16::from_be_bytes([fixed[0], fixed[1]]),
        qclass: u16::from_be_bytes([fixed[2], fixed[3]]),
    })
}

impl DnsQuestion {
    /// Dotted name. Bytes outside letters, digits, '-' and '_' are written as
    /// \DDD (the zone file convention), so binary labels stay readable.
    pub fn name(&self) -> String {
        if self.labels.is_empty() {
            return ".".to_string();
        }
        let mut out = String::new();
        for (i, label) in self.labels.iter().enumerate() {
            if i > 0 {
                out.push('.');
            }
            push_escaped(&mut out, label);
        }
        out
    }

    /// The last two labels. This is naive: for names under a public suffix
    /// such as co.uk it returns the suffix itself, so every *.co.uk site
    /// shares one volume window. The Public Suffix List would fix that.
    pub fn base_domain(&self) -> String {
        let start = self.labels.len().saturating_sub(2);
        let mut out = String::new();
        for (i, label) in self.labels[start..].iter().enumerate() {
            if i > 0 {
                out.push('.');
            }
            push_escaped(&mut out, label);
        }
        out
    }

    /// The labels left of the base domain.
    pub fn subdomain_labels(&self) -> &[Vec<u8>] {
        let end = self.labels.len().saturating_sub(2);
        &self.labels[..end]
    }
}

fn push_escaped(out: &mut String, label: &[u8]) {
    for &b in label {
        if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' {
            out.push(b as char);
        } else {
            out.push_str(&format!("\\{:03}", b));
        }
    }
}

pub fn qtype_name(qtype: u16) -> String {
    let name = match qtype {
        1 => "A",
        2 => "NS",
        5 => "CNAME",
        6 => "SOA",
        10 => "NULL",
        12 => "PTR",
        15 => "MX",
        16 => "TXT",
        28 => "AAAA",
        33 => "SRV",
        64 => "SVCB",
        65 => "HTTPS",
        255 => "ANY",
        65399 => "PRIVATE",
        _ => return format!("TYPE{qtype}"), // RFC 3597 style
    };
    name.to_string()
}

// ---- Features and rules ----

/// Shannon entropy in bits per byte: -sum(p * log2 p) over byte frequencies.
pub fn shannon_entropy(bytes: &[u8]) -> f64 {
    if bytes.is_empty() {
        return 0.0;
    }
    let mut counts = [0usize; 256];
    for &b in bytes {
        counts[b as usize] += 1;
    }
    let total = bytes.len() as f64;
    let mut bits = 0.0;
    for &count in counts.iter() {
        if count > 0 {
            let p = count as f64 / total;
            bits -= p * p.log2();
        }
    }
    bits
}

#[derive(Debug, Clone, PartialEq)]
pub struct QueryFeatures {
    /// Length of the dotted name without a trailing dot.
    pub name_len: usize,
    pub longest_label: usize,
    pub label_count: usize,
    /// Bytes in the subdomain labels, dots not counted.
    pub subdomain_len: usize,
    /// Shannon entropy of the subdomain bytes, dots not counted.
    pub subdomain_entropy: f64,
    pub qtype: u16,
}

impl QueryFeatures {
    pub fn from_question(q: &DnsQuestion) -> QueryFeatures {
        let label_bytes: usize = q.labels.iter().map(|l| l.len()).sum();
        let sub: Vec<u8> = q.subdomain_labels().concat();
        QueryFeatures {
            name_len: label_bytes + q.labels.len().saturating_sub(1),
            longest_label: q.labels.iter().map(|l| l.len()).max().unwrap_or(0),
            label_count: q.labels.len(),
            subdomain_len: sub.len(),
            subdomain_entropy: shannon_entropy(&sub),
            qtype: q.qtype,
        }
    }
}

/// Rules that judge one query on its own. Returns a reason for each rule that fires.
pub fn single_query_reasons(f: &QueryFeatures) -> Vec<String> {
    let mut reasons = Vec::new();
    if f.name_len >= LONG_NAME_LEN {
        reasons.push(format!(
            "name is {} characters (limit {})",
            f.name_len, LONG_NAME_LEN
        ));
    }
    if f.longest_label >= LONG_LABEL_LEN {
        reasons.push(format!(
            "a label is {} characters (limit {})",
            f.longest_label, LONG_LABEL_LEN
        ));
    }
    // Entropy alone also catches cloud load balancer names, which are long and
    // mixed, but those are asked for as A or AAAA. A tunnel that wants data
    // back asks for a type that can carry it.
    if TUNNEL_QTYPES.contains(&f.qtype)
        && f.subdomain_len >= ENTROPY_MIN_LEN
        && f.subdomain_entropy >= HIGH_ENTROPY_BITS
    {
        reasons.push(format!(
            "{} query with a high-entropy subdomain ({:.2} bits per character over {} characters)",
            qtype_name(f.qtype),
            f.subdomain_entropy,
            f.subdomain_len
        ));
    }
    reasons
}

/// The detector's view of one query.
#[derive(Debug, Clone)]
pub struct DnsAssessment {
    pub name: String,
    pub base_domain: String,
    pub qtype: u16,
    pub features: QueryFeatures,
    /// Why the query was flagged. Empty means benign.
    pub reasons: Vec<String>,
    /// Unique subdomains of base_domain from this process in the current
    /// window. Counting stops at VOLUME_UNIQUE_LIMIT.
    pub unique_subdomains: usize,
    /// This process and base domain already raised an alert in the current
    /// window, so this one is not reported again.
    pub repeat: bool,
}

impl DnsAssessment {
    pub fn is_flagged(&self) -> bool {
        !self.reasons.is_empty()
    }

    /// Flagged and not a repeat. Only these become alerts, which keeps a busy
    /// tunnel from producing thousands of identical alerts a minute.
    pub fn should_alert(&self) -> bool {
        self.is_flagged() && !self.repeat
    }

    pub fn reason_text(&self) -> String {
        self.reasons.join(", ")
    }
}

/// Volume state for one (process, base domain) pair.
struct VolumeWindow {
    started: u64,
    subdomains: HashSet<u64>,
    alerted: bool,
}

impl VolumeWindow {
    fn new(now_secs: u64) -> VolumeWindow {
        VolumeWindow {
            started: now_secs,
            subdomains: HashSet::new(),
            alerted: false,
        }
    }
}

/// Runs the per-query rules and keeps the per-process volume windows.
pub struct DnsTunnelDetector {
    windows: HashMap<(u32, String), VolumeWindow>,
}

impl Default for DnsTunnelDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl DnsTunnelDetector {
    pub fn new() -> DnsTunnelDetector {
        DnsTunnelDetector {
            windows: HashMap::new(),
        }
    }

    pub fn tracked_windows(&self) -> usize {
        self.windows.len()
    }

    /// Parses one kernel event and assesses its query.
    pub fn process(
        &mut self,
        event: &DnsSendEvent,
        now_secs: u64,
    ) -> Result<DnsAssessment, DnsParseError> {
        let question = parse_question(event.payload())?;
        Ok(self.assess(event.tgid, &question, now_secs))
    }

    /// Scores one query from process tgid. now_secs only needs to move forward.
    pub fn assess(&mut self, tgid: u32, q: &DnsQuestion, now_secs: u64) -> DnsAssessment {
        let features = QueryFeatures::from_question(q);
        let mut reasons = single_query_reasons(&features);
        let base_domain = q.base_domain();
        let subdomain = q.subdomain_labels();

        let key = (tgid, base_domain.clone());
        if !self.windows.contains_key(&key) {
            self.make_room(now_secs);
        }
        let window = self
            .windows
            .entry(key)
            .or_insert_with(|| VolumeWindow::new(now_secs));
        if now_secs.saturating_sub(window.started) >= VOLUME_WINDOW_SECS {
            *window = VolumeWindow::new(now_secs);
        }
        // Only a hash of each subdomain is kept, and only up to the limit.
        if !subdomain.is_empty() && window.subdomains.len() < VOLUME_UNIQUE_LIMIT {
            let mut hasher = DefaultHasher::new();
            subdomain.hash(&mut hasher);
            window.subdomains.insert(hasher.finish());
        }
        let unique_subdomains = window.subdomains.len();
        if unique_subdomains >= VOLUME_UNIQUE_LIMIT {
            reasons.push(format!(
                "{} unique subdomains of {} from this process within {} s",
                unique_subdomains, base_domain, VOLUME_WINDOW_SECS
            ));
        }

        let repeat = !reasons.is_empty() && window.alerted;
        if !reasons.is_empty() {
            window.alerted = true;
        }

        DnsAssessment {
            name: q.name(),
            base_domain,
            qtype: q.qtype,
            features,
            reasons,
            unique_subdomains,
            repeat,
        }
    }

    /// Keeps the map under MAX_TRACKED_WINDOWS: drop expired windows first,
    /// then the oldest live one if it is still full.
    fn make_room(&mut self, now_secs: u64) {
        if self.windows.len() < MAX_TRACKED_WINDOWS {
            return;
        }
        self.windows
            .retain(|_, w| now_secs.saturating_sub(w.started) < VOLUME_WINDOW_SECS);
        if self.windows.len() >= MAX_TRACKED_WINDOWS {
            let oldest = self
                .windows
                .iter()
                .min_by_key(|(_, w)| w.started)
                .map(|(k, _)| k.clone());
            if let Some(k) = oldest {
                self.windows.remove(&k);
            }
        }
    }
}

// ---- Loader (Linux only) ----

/// Program names in dns_tunnel_monitor.c and the kernel functions they probe.
#[cfg(target_os = "linux")]
const PROBES: [(&str, &str); 2] = [
    ("dns_udp_sendmsg", "udp_sendmsg"),
    ("dns_udpv6_sendmsg", "udpv6_sendmsg"),
];

#[cfg(target_os = "linux")]
fn load_and_attach(obj_path: &str) -> Result<(libbpf_rs::Object, Vec<libbpf_rs::Link>), String> {
    let open = libbpf_rs::ObjectBuilder::default()
        .open_file(obj_path)
        .map_err(|e| format!("open {obj_path}: {e}"))?;
    // load() runs the CO-RE relocations against this kernel's BTF and then the verifier.
    let mut obj = open.load().map_err(|e| format!("load {obj_path}: {e}"))?;

    let mut links = Vec::new();
    for (prog_name, kernel_fn) in PROBES {
        let prog = obj
            .prog_mut(prog_name)
            .ok_or_else(|| format!("program {prog_name} not found in {obj_path}"))?;
        match prog.attach_kprobe(false, kernel_fn) {
            Ok(link) => links.push(link),
            // IPv6 can be compiled out or not loaded. IPv4 still works.
            Err(e) if kernel_fn == "udpv6_sendmsg" && !links.is_empty() => {
                eprintln!("[dns_tunnel] {kernel_fn} not attached, IPv6 DNS is not covered: {e}")
            }
            Err(e) => return Err(format!("attach kprobe {kernel_fn}: {e}")),
        }
    }
    Ok((obj, links))
}

/// Loads the BPF object, attaches both kprobes and starts a thread that polls
/// the dns_events ring buffer and calls on_event for every record. Returns
/// once the probes are attached, or with the reason they could not be.
#[cfg(target_os = "linux")]
pub fn start_dns_monitor<F>(obj_path: &str, mut on_event: F) -> Result<(), String>
where
    F: FnMut(&DnsSendEvent) + Send + 'static,
{
    use std::time::Duration;
    const POLL_TIMEOUT: Duration = Duration::from_millis(100);

    let obj_path = obj_path.to_string();
    let (ready_tx, ready_rx) = std::sync::mpsc::channel::<Result<(), String>>();

    std::thread::Builder::new()
        .name("dns_tunnel_rb".into())
        .spawn(move || {
            // The object, the links and the ring buffer all live on this
            // thread. They are never dropped, so the probes stay attached.
            let (obj, _links) = match load_and_attach(&obj_path) {
                Ok(v) => v,
                Err(e) => {
                    let _ = ready_tx.send(Err(e));
                    return;
                }
            };
            let map = match obj.map("dns_events") {
                Some(m) => m,
                None => {
                    let _ = ready_tx.send(Err(format!("map dns_events not found in {obj_path}")));
                    return;
                }
            };
            let mut builder = libbpf_rs::RingBufferBuilder::new();
            let added = builder.add(map, move |data: &[u8]| {
                if let Some(event) = DnsSendEvent::from_bytes(data) {
                    on_event(&event);
                }
                0
            });
            if let Err(e) = added {
                let _ = ready_tx.send(Err(format!("ring buffer add: {e}")));
                return;
            }
            let ring = match builder.build() {
                Ok(r) => r,
                Err(e) => {
                    let _ = ready_tx.send(Err(format!("ring buffer build: {e}")));
                    return;
                }
            };
            let _ = ready_tx.send(Ok(()));
            loop {
                if let Err(e) = ring.poll(POLL_TIMEOUT) {
                    eprintln!("[dns_tunnel] ring buffer poll: {e}");
                    std::thread::sleep(POLL_TIMEOUT);
                }
            }
        })
        .map_err(|e| format!("spawn dns_tunnel_rb thread: {e}"))?;

    ready_rx
        .recv()
        .unwrap_or_else(|_| Err("dns_tunnel_rb thread exited during setup".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a query packet: 12-byte header with QDCOUNT 1, the name, qtype, class IN.
    fn build_query(name: &str, qtype: u16) -> Vec<u8> {
        let mut p = vec![0x12, 0x34, 0x01, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0];
        for label in name.split('.').filter(|l| !l.is_empty()) {
            p.push(label.len() as u8);
            p.extend_from_slice(label.as_bytes());
        }
        p.push(0);
        p.extend_from_slice(&qtype.to_be_bytes());
        p.extend_from_slice(&1u16.to_be_bytes());
        p
    }

    fn question(name: &str, qtype: u16) -> DnsQuestion {
        parse_question(&build_query(name, qtype)).expect("test query parses")
    }

    /// Deterministic pseudo-random text from an alphabet (xorshift), so tests need no rand crate.
    fn pseudo_random(alphabet: &[u8], len: usize, seed: u64) -> String {
        let mut x = seed;
        let mut out = String::new();
        for _ in 0..len {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            out.push(alphabet[(x % alphabet.len() as u64) as usize] as char);
        }
        out
    }

    const BASE32: &[u8] = b"abcdefghijklmnopqrstuvwxyz234567";
    const HEX: &[u8] = b"0123456789abcdef";

    fn split_labels(data: &str, size: usize) -> String {
        let bytes = data.as_bytes();
        let mut parts = Vec::new();
        let mut i = 0;
        while i < bytes.len() {
            let end = (i + size).min(bytes.len());
            parts.push(String::from_utf8_lossy(&bytes[i..end]).into_owned());
            i = end;
        }
        parts.join(".")
    }

    // ---- event layout ----

    #[test]
    fn event_layout_matches_c() {
        use std::mem::{align_of, offset_of, size_of};
        assert_eq!(size_of::<DnsSendEvent>(), 360);
        assert_eq!(align_of::<DnsSendEvent>(), 4);
        assert_eq!(offset_of!(DnsSendEvent, tgid), 0);
        assert_eq!(offset_of!(DnsSendEvent, tid), 4);
        assert_eq!(offset_of!(DnsSendEvent, uid), 8);
        assert_eq!(offset_of!(DnsSendEvent, comm), 12);
        assert_eq!(offset_of!(DnsSendEvent, family), 28);
        assert_eq!(offset_of!(DnsSendEvent, dport), 30);
        assert_eq!(offset_of!(DnsSendEvent, orig_len), 32);
        assert_eq!(offset_of!(DnsSendEvent, cap_len), 36);
        assert_eq!(offset_of!(DnsSendEvent, payload), 40);
    }

    #[test]
    fn event_from_bytes_checks_length_and_clamps_cap_len() {
        assert!(DnsSendEvent::from_bytes(&[0u8; 359]).is_none());
        let mut raw = vec![0u8; 360];
        raw[0..4].copy_from_slice(&7u32.to_ne_bytes());
        raw[12..19].copy_from_slice(b"python3");
        raw[36..40].copy_from_slice(&9999u32.to_ne_bytes()); // cap_len larger than the buffer
        let ev = DnsSendEvent::from_bytes(&raw).expect("full record");
        assert_eq!(ev.tgid, 7);
        assert_eq!(ev.comm(), "python3");
        assert_eq!(ev.payload().len(), DNS_PAYLOAD_CAP);
    }

    // ---- parser ----

    #[test]
    fn parses_normal_query() {
        let q = question("www.example.com", QTYPE_A);
        assert_eq!(q.name(), "www.example.com");
        assert_eq!(q.base_domain(), "example.com");
        assert_eq!(q.subdomain_labels(), &[b"www".to_vec()][..]);
        assert_eq!(q.qtype, QTYPE_A);
        assert_eq!(q.qclass, 1);
    }

    #[test]
    fn lowercases_names() {
        let q = question("WwW.ExAmPlE.CoM", QTYPE_A);
        assert_eq!(q.name(), "www.example.com");
    }

    #[test]
    fn parses_root_name() {
        let mut p = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0];
        p.extend_from_slice(&[0, 2, 0, 1]); // NS, IN
        let q = parse_question(&p).expect("root query");
        assert!(q.labels.is_empty());
        assert_eq!(q.name(), ".");
    }

    #[test]
    fn escapes_binary_labels() {
        let mut p = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];
        p.extend_from_slice(&[3, b'a', 0xff, b'.', 3, b'c', b'o', b'm', 0, 0, 1, 0, 1]);
        let q = parse_question(&p).expect("binary label");
        assert_eq!(q.name(), "a\\255\\046.com");
    }

    #[test]
    fn rejects_short_header() {
        assert_eq!(parse_question(&[]), Err(DnsParseError::TooShort));
        assert_eq!(parse_question(&[0u8; 11]), Err(DnsParseError::TooShort));
    }

    #[test]
    fn rejects_zero_questions() {
        let mut p = build_query("www.example.com", QTYPE_A);
        p[4] = 0;
        p[5] = 0;
        assert_eq!(parse_question(&p), Err(DnsParseError::NoQuestion));
    }

    #[test]
    fn rejects_truncated_question() {
        let p = build_query("www.example.com", QTYPE_A);
        // Cut inside the "example" label.
        assert_eq!(parse_question(&p[..18]), Err(DnsParseError::Truncated));
        // Name complete but qtype and qclass missing.
        assert_eq!(parse_question(&p[..p.len() - 2]), Err(DnsParseError::Truncated));
        // Header only.
        assert_eq!(parse_question(&p[..12]), Err(DnsParseError::Truncated));
    }

    #[test]
    fn rejects_label_over_63() {
        let mut p = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 64];
        p.extend_from_slice(&[b'a'; 64]);
        p.extend_from_slice(&[0, 0, 1, 0, 1]);
        assert_eq!(parse_question(&p), Err(DnsParseError::LabelTooLong(64)));
    }

    #[test]
    fn rejects_pointer_in_question() {
        let p = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0xC0, 0x0C, 0, 1, 0, 1];
        assert_eq!(parse_question(&p), Err(DnsParseError::CompressionPointer));
    }

    #[test]
    fn rejects_name_over_255() {
        let label = "a".repeat(63);
        let name = format!("{label}.{label}.{label}.{label}.{label}");
        // build_query does not enforce the limit, so this is 321 bytes on the wire.
        assert_eq!(
            parse_question(&build_query(&name, QTYPE_A)),
            Err(DnsParseError::NameTooLong)
        );
    }

    #[test]
    fn garbage_never_panics() {
        let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
        for round in 0..5000 {
            let len = round % 400;
            let mut buf = Vec::with_capacity(len);
            for _ in 0..len {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                buf.push(x as u8);
            }
            if buf.len() > 5 {
                buf[5] |= 1; // force QDCOUNT >= 1 so the name walk runs
            }
            let _ = parse_question(&buf);
        }
        assert!(parse_question(&[0xFF; 300]).is_err());
        assert_eq!(parse_question(&[0x00; 300]), Err(DnsParseError::NoQuestion));
    }

    // ---- scoring ----

    #[test]
    fn google_a_is_benign() {
        let mut d = DnsTunnelDetector::new();
        let a = d.assess(100, &question("www.google.com", QTYPE_A), 0);
        assert!(!a.is_flagged(), "unexpected reasons: {:?}", a.reasons);
        assert_eq!(a.features.name_len, 14);
        assert_eq!(a.features.label_count, 3);
        assert_eq!(a.features.longest_label, 6);
    }

    #[test]
    fn ordinary_names_are_benign() {
        let names = [
            ("mail.google.com", QTYPE_A),
            ("ec2-54-210-167-204.compute-1.amazonaws.com", QTYPE_A),
            ("my-loadbalancer-1234567890.us-west-2.elb.amazonaws.com", QTYPE_A),
            ("dualstack.k8s-prod-ingress-0a1b2c3d4e-1234567890.us-east-1.elb.amazonaws.com", QTYPE_AAAA),
            ("e1234.dscb.akamaiedge.net", QTYPE_A),
            ("r3---sn-ab5l6nzr.googlevideo.com", QTYPE_AAAA),
            ("my-service.my-namespace.svc.cluster.local", QTYPE_A),
            ("4.3.2.1.zen.spamhaus.org", QTYPE_A),
            ("f.e.d.c.b.a.9.8.7.6.5.4.3.2.1.0.f.e.d.c.b.a.9.8.7.6.5.4.3.2.1.0.ip6.arpa", 12),
            ("_dmarc.example.com", QTYPE_TXT),
            ("selector1._domainkey.example.com", QTYPE_TXT),
            ("_3f7a1c9e2b4d6f8a0c1e3b5d7f9a2c4e.example.com", QTYPE_CNAME),
            ("_ldap._tcp.dc._msdcs.corp.example.com", QTYPE_SRV),
        ];
        let mut d = DnsTunnelDetector::new();
        for (name, qtype) in names {
            let a = d.assess(100, &question(name, qtype), 0);
            assert!(!a.is_flagged(), "{name} flagged: {:?}", a.reasons);
        }
    }

    #[test]
    fn iodine_style_base32_txt_is_flagged() {
        // About 170 bytes of base32 data in 57-character labels under t.tunnel-test.net.
        let data = pseudo_random(BASE32, 170, 7);
        let name = format!("{}.t.tunnel-test.net", split_labels(&data, 57));
        let mut d = DnsTunnelDetector::new();
        let a = d.assess(200, &question(&name, QTYPE_TXT), 0);
        assert!(a.should_alert());
        assert_eq!(a.base_domain, "tunnel-test.net");
        assert!(a.reasons.iter().any(|r| r.starts_with("name is")));
        assert!(a.reasons.iter().any(|r| r.starts_with("a label is 57")));
        assert!(a.reasons.iter().any(|r| r.starts_with("TXT query with a high-entropy")));
    }

    #[test]
    fn dnscat_style_hex_is_flagged() {
        // dnscat2 style: hex data in full 63-character labels.
        let data = pseudo_random(HEX, 189, 11);
        let name = format!("{}.tunnel-test.net", split_labels(&data, 63));
        let mut d = DnsTunnelDetector::new();
        let a = d.assess(201, &question(&name, QTYPE_CNAME), 0);
        assert!(a.should_alert());
        // Hex cannot pass 4 bits per character, so length is what catches it.
        assert!(a.features.subdomain_entropy < HIGH_ENTROPY_BITS);
        assert!(a.reasons.iter().any(|r| r.starts_with("name is")));
    }

    #[test]
    fn entropy_rule_needs_a_data_carrying_type() {
        let label = pseudo_random(BASE32, 40, 3);
        let name = format!("{label}.tunnel-test.net");
        let mut d = DnsTunnelDetector::new();
        let txt = d.assess(202, &question(&name, QTYPE_TXT), 0);
        assert!(txt.features.subdomain_entropy >= HIGH_ENTROPY_BITS);
        assert_eq!(txt.reasons.len(), 1, "{:?}", txt.reasons);
        let a = d.assess(203, &question(&name, QTYPE_A), 0);
        assert!(!a.is_flagged(), "{:?}", a.reasons);
    }

    #[test]
    fn burst_of_unique_subdomains_alerts_once() {
        let mut d = DnsTunnelDetector::new();
        let mut alerts = 0;
        let mut first_flag_at = None;
        for i in 0..30 {
            let name = format!("{:02}{}.burst-test.net", i, pseudo_random(HEX, 8, i as u64 + 1));
            let a = d.assess(300, &question(&name, QTYPE_A), 10);
            if a.is_flagged() && first_flag_at.is_none() {
                first_flag_at = Some(i);
            }
            if a.should_alert() {
                alerts += 1;
                assert!(a.reason_text().contains("20 unique subdomains of burst-test.net"));
            }
        }
        assert_eq!(first_flag_at, Some(VOLUME_UNIQUE_LIMIT - 1));
        assert_eq!(alerts, 1);
    }

    #[test]
    fn same_name_repeated_is_not_volume() {
        let mut d = DnsTunnelDetector::new();
        for t in 0..100 {
            let a = d.assess(400, &question("www.example.com", QTYPE_A), t / 10);
            assert!(!a.is_flagged());
            assert_eq!(a.unique_subdomains, 1);
        }
    }

    #[test]
    fn volume_is_counted_per_process() {
        let mut d = DnsTunnelDetector::new();
        for i in 0..15 {
            let name = format!("host{i}.shared-test.net");
            assert!(!d.assess(501, &question(&name, QTYPE_A), 0).is_flagged());
            assert!(!d.assess(502, &question(&name, QTYPE_A), 0).is_flagged());
        }
    }

    #[test]
    fn volume_window_expires() {
        let mut d = DnsTunnelDetector::new();
        for i in 0..19 {
            let name = format!("n{i}.slow-test.net");
            assert!(!d.assess(600, &question(&name, QTYPE_A), 0).is_flagged());
        }
        let late = d.assess(600, &question("n19.slow-test.net", QTYPE_A), VOLUME_WINDOW_SECS + 1);
        assert!(!late.is_flagged());
        assert_eq!(late.unique_subdomains, 1);
    }

    #[test]
    fn tracked_windows_stay_bounded() {
        let mut d = DnsTunnelDetector::new();
        for i in 0..(MAX_TRACKED_WINDOWS + 500) {
            let name = format!("www.site{i}.test");
            d.assess(700, &question(&name, QTYPE_A), 0);
        }
        assert!(d.tracked_windows() <= MAX_TRACKED_WINDOWS);
    }

    #[test]
    fn process_runs_parser_and_scorer_on_an_event() {
        let packet = build_query("www.example.com", QTYPE_A);
        let mut raw = vec![0u8; 360];
        raw[0..4].copy_from_slice(&42u32.to_ne_bytes());
        raw[36..40].copy_from_slice(&(packet.len() as u32).to_ne_bytes());
        raw[40..40 + packet.len()].copy_from_slice(&packet);
        let ev = DnsSendEvent::from_bytes(&raw).expect("record");
        let mut d = DnsTunnelDetector::new();
        let a = d.process(&ev, 0).expect("parses");
        assert_eq!(a.name, "www.example.com");
        assert!(!a.is_flagged());
    }

    #[test]
    fn qtype_names() {
        assert_eq!(qtype_name(QTYPE_TXT), "TXT");
        assert_eq!(qtype_name(QTYPE_NULL), "NULL");
        assert_eq!(qtype_name(4242), "TYPE4242");
    }
}
