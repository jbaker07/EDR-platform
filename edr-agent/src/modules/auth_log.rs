// src/modules/auth_log.rs
// sshd logins and failures for the Auth playbook facts. The auth log is followed like
// `tail -F`: new lines only, from the end of the file at start, and from the top again
// when the file is rotated (a new inode, or a file shorter than the read position).
//
// Paths: EDR_AUTH_LOG (colon separated), else /var/log/auth.log and /var/log/secure.
// sshd writes there through syslog, or straight to a file with `sshd -E <file>`.
//
// Each line becomes one Auth record: result success or failure, the user, the source
// address and port, its scope (loopback, private, link_local, public) and src_new, true
// for the first successful login from that address since the agent started.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::fs::MetadataExt;
use std::sync::{Mutex, OnceLock};

use regex::Regex;

use crate::telemetry_types::TelemetryOutput;
use crate::utils::time::now_ts;

/// Bytes read per file per pass, so a huge burst cannot stall the sweep.
const MAX_READ: u64 = 4 * 1024 * 1024;
const SEEN_CAP: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthLine {
    pub result: &'static str,
    pub auth_kind: String,
    pub user: String,
    pub src_ip: String,
    pub src_port: String,
}

fn patterns() -> &'static [(Regex, &'static str)] {
    static P: OnceLock<Vec<(Regex, &'static str)>> = OnceLock::new();
    P.get_or_init(|| {
        let r = |p: &str| Regex::new(p).expect("auth pattern");
        vec![
            (
                r(r"Accepted (\S+) for (\S+) from (\S+) port (\d+)"),
                "success",
            ),
            (
                r(r"Failed (\S+) for (?:invalid user )?(\S+) from (\S+) port (\d+)"),
                "failure",
            ),
            (r(r"Invalid user (\S*) from (\S+) port (\d+)"), "invalid_user"),
        ]
    })
}

/// One sshd line, or None for any other line.
pub fn parse_line(line: &str) -> Option<AuthLine> {
    for (re, kind) in patterns() {
        let Some(c) = re.captures(line) else {
            continue;
        };
        return Some(if *kind == "invalid_user" {
            AuthLine {
                result: "failure",
                auth_kind: "invalid_user".into(),
                user: c[1].to_string(),
                src_ip: c[2].to_string(),
                src_port: c[3].to_string(),
            }
        } else {
            AuthLine {
                result: kind,
                auth_kind: c[1].to_string(),
                user: c[2].to_string(),
                src_ip: c[3].to_string(),
                src_port: c[4].to_string(),
            }
        });
    }
    None
}

/// Follows a set of log files.
pub struct AuthLogReader {
    paths: Vec<String>,
    /// path -> (inode, offset)
    pos: HashMap<String, (u64, u64)>,
    seen_success: HashSet<String>,
    partial: HashMap<String, String>,
}

impl AuthLogReader {
    pub fn new(paths: Vec<String>) -> Self {
        Self {
            paths,
            pos: HashMap::new(),
            seen_success: HashSet::new(),
            partial: HashMap::new(),
        }
    }

    pub fn from_env() -> Self {
        let paths = std::env::var("EDR_AUTH_LOG")
            .map(|v| v.split(':').filter(|p| !p.is_empty()).map(str::to_string).collect())
            .unwrap_or_else(|_| vec!["/var/log/auth.log".into(), "/var/log/secure".into()]);
        Self::new(paths)
    }

    /// New complete lines since the last call, as Auth records.
    pub fn poll(&mut self) -> Vec<TelemetryOutput> {
        let mut out = Vec::new();
        for path in self.paths.clone() {
            for (offset, line) in self.read_new(&path) {
                if let Some(a) = parse_line(&line) {
                    out.push(self.output(&path, offset, a));
                }
            }
        }
        out
    }

    fn read_new(&mut self, path: &str) -> Vec<(u64, String)> {
        let Ok(mut f) = File::open(path) else {
            return Vec::new();
        };
        let Ok(meta) = f.metadata() else {
            return Vec::new();
        };
        let (ino, size) = (meta.ino(), meta.len());
        let start = match self.pos.get(path) {
            // First sight of the file: start at its end, so old history is not replayed.
            None => {
                self.pos.insert(path.to_string(), (ino, size));
                return Vec::new();
            }
            Some((old_ino, off)) if *old_ino == ino && *off <= size => *off,
            // Rotated or truncated: read the new file from the top.
            _ => {
                self.partial.remove(path);
                0
            }
        };
        let want = (size - start).min(MAX_READ);
        let mut buf = vec![0u8; want as usize];
        if f.seek(SeekFrom::Start(start)).is_err() || f.read_exact(&mut buf).is_err() {
            return Vec::new();
        }
        self.pos.insert(path.to_string(), (ino, start + want));
        let mut text = self.partial.remove(path).unwrap_or_default();
        text.push_str(&String::from_utf8_lossy(&buf));
        let mut lines = Vec::new();
        let mut line_start = start;
        let ends_complete = text.ends_with('\n');
        let mut parts: Vec<&str> = text.split('\n').collect();
        let tail = if ends_complete { None } else { parts.pop() };
        for p in parts {
            if !p.is_empty() {
                lines.push((line_start, p.to_string()));
            }
            line_start += p.len() as u64 + 1;
        }
        if let Some(t) = tail {
            if !t.is_empty() {
                self.partial.insert(path.to_string(), t.to_string());
            }
        }
        lines
    }

    fn output(&mut self, path: &str, offset: u64, a: AuthLine) -> TelemetryOutput {
        let scope = crate::pb_wiring::ip_scope(&a.src_ip);
        let new = if a.result == "success" {
            if self.seen_success.len() >= SEEN_CAP {
                self.seen_success.clear();
            }
            self.seen_success.insert(a.src_ip.clone())
        } else {
            false
        };
        let mut d = HashMap::new();
        d.insert("source".to_string(), "auth_log".to_string());
        d.insert("method".to_string(), "ssh".to_string());
        d.insert("result".to_string(), a.result.to_string());
        d.insert("auth_kind".to_string(), a.auth_kind);
        d.insert("user".to_string(), a.user);
        d.insert("src_ip".to_string(), a.src_ip);
        d.insert("src_port".to_string(), a.src_port);
        d.insert("src_scope".to_string(), scope.to_string());
        d.insert("src_private".to_string(), (scope != "public").to_string());
        d.insert("src_new".to_string(), new.to_string());
        d.insert("timestamp".to_string(), now_ts().to_string());
        // Keeps the per-minute dedup in telemetry.rs from folding lines together.
        d.insert("event_id".to_string(), format!("{path}:{offset}"));
        let signal = if a.result == "success" { "auth::ssh_login" } else { "auth::ssh_failure" };
        TelemetryOutput {
            category: "auth".into(),
            signal: signal.into(),
            confidence: 0.9,
            data: d,
        }
    }
}

/// One pass of the agent's reader. Called by the telemetry sweep.
pub fn scan_auth_log() -> Vec<TelemetryOutput> {
    static READER: OnceLock<Mutex<AuthLogReader>> = OnceLock::new();
    READER
        .get_or_init(|| Mutex::new(AuthLogReader::from_env()))
        .lock()
        .map(|mut r| r.poll())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn sshd_lines_parse() {
        let a = parse_line("Nov 23 10:00:01 h sshd[812]: Accepted publickey for alice from 203.0.113.7 port 50122 ssh2: ED25519 SHA256:x").unwrap();
        assert_eq!((a.result, a.auth_kind.as_str(), a.user.as_str(), a.src_ip.as_str(), a.src_port.as_str()), ("success", "publickey", "alice", "203.0.113.7", "50122"));
        let f = parse_line("sshd[9]: Failed password for invalid user admin from 198.51.100.4 port 4242 ssh2").unwrap();
        assert_eq!((f.result, f.user.as_str()), ("failure", "admin"));
        let i = parse_line("sshd[9]: Invalid user oracle from 198.51.100.4 port 4243").unwrap();
        assert_eq!((i.result, i.auth_kind.as_str(), i.user.as_str()), ("failure", "invalid_user", "oracle"));
        assert!(parse_line("sshd[9]: Connection closed by 10.0.0.1 port 22").is_none());
    }

    #[test]
    fn follows_new_lines_and_rotation() {
        let dir = std::env::temp_dir().join(format!("edr_auth_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("auth.log");
        std::fs::write(&path, "old: Accepted password for root from 203.0.113.1 port 1 ssh2\n").unwrap();
        let mut r = AuthLogReader::new(vec![path.to_string_lossy().into_owned()]);
        assert!(r.poll().is_empty(), "history before the start is not replayed");

        let mut f = std::fs::OpenOptions::new().append(true).open(&path).unwrap();
        writeln!(f, "sshd[1]: Accepted password for bob from 203.0.113.9 port 5000 ssh2").unwrap();
        write!(f, "sshd[1]: Failed password for bob from 203.0.113.9 port 50").unwrap();
        let got = r.poll();
        assert_eq!(got.len(), 1, "a line without its newline waits");
        assert_eq!(got[0].data["src_new"], "true");
        assert_eq!(got[0].data["src_scope"], "public");
        writeln!(f, "01 ssh2").unwrap();
        writeln!(f, "sshd[1]: Accepted password for bob from 203.0.113.9 port 5002 ssh2").unwrap();
        let got = r.poll();
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].data["result"], "failure");
        assert_eq!(got[0].data["src_port"], "5001");
        assert_eq!(got[1].data["src_new"], "false", "second success from the same address");

        // Rotation: a new file at the same path is read from the top.
        std::fs::remove_file(&path).unwrap();
        std::fs::write(&path, "sshd[2]: Failed password for eve from 10.1.1.1 port 7 ssh2\n").unwrap();
        let got = r.poll();
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].data["src_private"], "true");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
