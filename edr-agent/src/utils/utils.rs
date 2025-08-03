use std::fs;
use std::io::{self, Read};
use std::os::unix::fs::PermissionsExt;

pub fn sha256_digest(path: &str) -> Option<String> {
    use sha2::{Digest, Sha256};
    let mut file = fs::File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096];

    loop {
        let read = file.read(&mut buffer).ok()?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    Some(format!("{:x}", hasher.finalize()))
}

pub fn estimate_entropy(bytes: &[u8]) -> f64 {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for &byte in bytes {
        *freq.entry(byte).or_insert(0usize) += 1;
    }

    let len = bytes.len() as f64;
    freq.values()
        .map(|&count| {
            let p = count as f64 / len;
            -p * p.log2()
        })
        .sum()
}

pub fn get_permissions(path: &str) -> Option<String> {
    let metadata = fs::metadata(path).ok()?;
    let perms = metadata.permissions().mode();
    Some(format!("{:o}", perms))
}
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::modules::telemetry_fingerprint::MemoryRegion;

pub fn parse_proc_maps_for_addr(pid: i32, addr: u64) -> Option<MemoryRegion> {
    let path = format!("/proc/{}/maps", pid);
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            continue;
        }

        // Parse address range
        let range = parts[0];
        let perms = parts[1];
        let offset = u64::from_str_radix(parts[2], 16).unwrap_or(0);
        let pathname = parts.get(5).unwrap_or(&"").to_string();

        let mut region_start = 0;
        let mut region_end = 0;
        if let Some((start_str, end_str)) = range.split_once('-') {
            region_start = u64::from_str_radix(start_str, 16).ok()?;
            region_end = u64::from_str_radix(end_str, 16).ok()?;
        }

        if addr >= region_start && addr < region_end {
            let size = region_end - region_start;
            let exec_capable = perms.contains('x');
            let trusted_uid = false; // or implement UID lookup logic if needed

            let entropy = 0.0; // optionally implement entropy scan here
            let category = if perms.contains('x') { "executable" } else { "data" };

            return Some(MemoryRegion {
                path: pathname.to_string(),
                perms: perms.to_string(),
                offset,
                size,
                entropy,
                exec_capable,
                trusted_uid,
                category: category.to_string(),
            });
        }
    }

    None
}

