use std::fs::{self, File};
use std::io::{Read};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct FileHash {
    pub path: String,
    pub hash: String,
}

pub fn scan_and_hash_files(dirs: &[&str]) -> Vec<FileHash> {
    let mut results = Vec::new();

    for dir in dirs {
        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if let Ok(hash) = hash_file(path) {
                results.push(FileHash {
                    path: path.display().to_string(),
                    hash,
                });
            }
        }
    }

    results
}

fn hash_file(path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 4096];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
