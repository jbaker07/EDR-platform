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
