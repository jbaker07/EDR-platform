// src/vault_exporter.rs

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn export_to_vault(session_id: &str, logs: &[String]) -> io::Result<()> {
    let path = format!("/tmp/{}_session_export.log", session_id);
    let mut file = File::create(Path::new(&path))?;

    for line in logs {
        writeln!(file, "{}", line)?;
    }

    println!("📦 Session {} exported to {}", session_id, path);
    Ok(())
}
