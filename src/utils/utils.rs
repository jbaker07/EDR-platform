use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, BufRead, BufReader};
use std::os::unix::fs::PermissionsExt;

use crate::modules::telemetry_fingerprint::FingerprintEntry;
use crate::modules::telemetry_fingerprint::FingerprintEntry::MemoryRegion;
use crate::utils::time::now_ts;

/// Compute the SHA-256 digest of a file at `path`
pub fn sha256_digest(path: &str) -> Option<String> {
    use sha2::{Digest, Sha256};

    let mut file = fs::File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 4096];

    loop {
        let read = file.read(&mut buffer).ok()?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    Some(format!("{:x}", hasher.finalize()))
}

/// Shannon entropy estimate (0..8) for a byte slice
pub fn estimate_entropy(bytes: &[u8]) -> f64 {
    if bytes.is_empty() {
        return 0.0;
    }

    let mut freq: HashMap<u8, usize> = HashMap::new();
    for &b in bytes {
        *freq.entry(b).or_insert(0) += 1;
    }

    let len = bytes.len() as f64;
    freq.values()
        .map(|&count| {
            let p = count as f64 / len;
            -p * p.log2()
        })
        .sum()
}

/// Return octal permissions for a filesystem path (e.g., "100755")
pub fn get_permissions(path: &str) -> Option<String> {
    let metadata = fs::metadata(path).ok()?;
    let perms = metadata.permissions().mode();
    Some(format!("{:o}", perms))
}

/// Parse `/proc/<pid>/maps` into a simple vector of string maps.
/// Keys: range, perms, offset, start, end, size, path, exec
pub fn parse_proc_maps(pid: i32) -> Vec<HashMap<String, String>> {
    let path = format!("/proc/{pid}/maps");
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);

    let mut out = Vec::new();
    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            continue;
        }

        // columns: address perms offset dev inode [pathname...]
        let range = parts[0];
        let perms = parts[1];
        let offset_hex = parts[2];
        let pathname = parts.get(5).copied().unwrap_or("").to_string();

        let (mut start, mut end) = (0u64, 0u64);
        if let Some((a, b)) = range.split_once('-') {
            if let (Ok(s), Ok(e)) = (u64::from_str_radix(a, 16), u64::from_str_radix(b, 16)) {
                start = s;
                end = e;
            }
        }
        let size = end.saturating_sub(start);
        let offset = u64::from_str_radix(offset_hex, 16).unwrap_or(0);

        let mut m = HashMap::new();
        m.insert("range".into(), range.to_string());
        m.insert("perms".into(), perms.to_string());
        m.insert("offset".into(), format!("0x{:x}", offset));
        m.insert("start".into(), format!("0x{:x}", start));
        m.insert("end".into(), format!("0x{:x}", end));
        m.insert("size".into(), size.to_string());
        m.insert("path".into(), pathname);
        m.insert("exec".into(), perms.contains('x').to_string());
        out.push(m);
    }
    out
}

/// Parse `/proc/<pid>/maps` and, if `addr` falls within a region, return a
/// `FingerprintEntry::MemoryRegion` describing it.
pub fn parse_proc_maps_for_addr(pid: i32, addr: u64) -> Option<FingerprintEntry> {
    let path = format!("/proc/{}/maps", pid);
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        // Expected fields (at least): <range> <perms> <offset> <dev> <inode> [pathname...]
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            continue;
        }

        // Parse address range
        let range = parts[0];
        let perms = parts[1];
        let offset = u64::from_str_radix(parts[2], 16).unwrap_or(0);

        // pathname is optional (index 5)
        let pathname = parts.get(5).map(|s| (*s).to_string()).unwrap_or_default();

        let (region_start, region_end) = match range.split_once('-') {
            Some((start_str, end_str)) => {
                let s = match u64::from_str_radix(start_str, 16) { Ok(v) => v, Err(_) => continue };
                let e = match u64::from_str_radix(end_str, 16) { Ok(v) => v, Err(_) => continue };
                (s, e)
            }
            None => continue,
        };

        if addr >= region_start && addr < region_end {
            let size = region_end.saturating_sub(region_start);
            let exec_capable = perms.contains('x');
            let trusted_uid: u32 = 0; // Unknown here; fill if you have a lookup

            // If you later map and read memory, replace this placeholder.
            let entropy_val = 0.0_f64;
            let category = if exec_capable { "executable" } else { "data" }.to_string();

            return Some(MemoryRegion {
                // metadata
                created_at: Some(now_ts()),
                description: Some(String::new()),
                source_module: "utils::parse_proc_maps_for_addr".to_string(),
                tags: Vec::new(),

                // identity
                pid: pid as u32,
                trusted_uid,

                // region info
                path: pathname,
                perms: perms.to_string(),
                category,

                // optional numeric details
                offset: Some(offset),
                size: Some(size),
                entropy: Some(entropy_val),
                exec_capable: Some(exec_capable),
            });
        }
    }

    None
}
