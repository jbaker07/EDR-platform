// src/pkg_verify.rs
// Which installed package owns an executable, and whether the file still matches the
// checksum the package manager recorded for it.
//
// Linux executables carry no code signature the way Windows and macOS binaries do, so
// the closest real signal is this: the file was installed by a package (dpkg, or rpm on
// hosts without dpkg) and its bytes are the bytes the package shipped. A tool an
// attacker drops in /tmp and names `restic` has no package, and a packaged binary that
// was overwritten fails the checksum, so neither passes for the real tool.
//
// dpkg: /var/lib/dpkg/info/<pkg>.list names each file of a package and <pkg>.md5sums
// its MD5. Only paths in bin, sbin and libexec directories are indexed, which keeps the
// index to a few thousand entries. rpm: `rpm -qf` and `rpm -V`, once per file.
//
// Each file is checked once per (device, inode, size, mtime) and the answer is cached,
// so a binary is hashed again only after it changes.

use std::collections::HashMap;
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime};

/// Files larger than this are not hashed, and get verified = false.
const MAX_HASH_BYTES: u64 = 512 * 1024 * 1024;
/// How often the dpkg info directory is checked for installs and removals.
const RECHECK: Duration = Duration::from_secs(60);
const CACHE_CAP: usize = 8192;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub package: String,
    /// The file's checksum matches the package database.
    pub verified: bool,
}

type FileId = (u64, u64, u64, i64);

pub struct PkgDb {
    root: PathBuf,
    /// path -> package, for packaged files in bin directories
    index: HashMap<String, String>,
    /// package -> (relative path -> md5)
    sums: HashMap<String, HashMap<String, String>>,
    cache: HashMap<(String, FileId), Option<Verdict>>,
    info_mtime: Option<SystemTime>,
    checked: Option<Instant>,
    /// No dpkg database: ask rpm, when it is installed.
    use_rpm: bool,
}

static HASHED_FILES: AtomicU64 = AtomicU64::new(0);
static HASH_US: AtomicU64 = AtomicU64::new(0);

/// (files hashed, total time hashing in ms) since start.
pub fn stats() -> (u64, u64) {
    (HASHED_FILES.load(Ordering::Relaxed), HASH_US.load(Ordering::Relaxed) / 1000)
}

fn exec_dir(path: &str) -> bool {
    path.contains("/bin/") || path.contains("/sbin/") || path.contains("/libexec/")
}

/// On merged /usr systems /bin is /usr/bin, and a package may list either spelling.
fn spellings(path: &str) -> Vec<String> {
    let mut v = vec![path.to_string()];
    for top in ["/bin/", "/sbin/", "/lib/"] {
        if let Some(rest) = path.strip_prefix(top) {
            v.push(format!("/usr{top}{rest}"));
        }
        if let Some(rest) = path.strip_prefix(&format!("/usr{top}")) {
            v.push(format!("{top}{rest}"));
        }
    }
    v
}

fn file_md5(path: &Path) -> Option<String> {
    let started = Instant::now();
    let mut f = std::fs::File::open(path).ok()?;
    let mut h = openssl::hash::Hasher::new(openssl::hash::MessageDigest::md5()).ok()?;
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = f.read(&mut buf).ok()?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]).ok()?;
    }
    let digest = h.finish().ok()?;
    HASHED_FILES.fetch_add(1, Ordering::Relaxed);
    HASH_US.fetch_add(started.elapsed().as_micros() as u64, Ordering::Relaxed);
    Some(digest.iter().map(|b| format!("{b:02x}")).collect())
}

impl PkgDb {
    /// A database rooted at `root` ("/" for the host).
    pub fn new(root: &Path) -> Self {
        let mut db = Self {
            root: root.to_path_buf(),
            index: HashMap::new(),
            sums: HashMap::new(),
            cache: HashMap::new(),
            info_mtime: None,
            checked: None,
            use_rpm: false,
        };
        db.refresh();
        db
    }

    fn info_dir(&self) -> PathBuf {
        self.root.join("var/lib/dpkg/info")
    }

    /// Rebuild the index when the dpkg info directory changed.
    fn refresh(&mut self) {
        let due = self.checked.map(|t| t.elapsed() >= RECHECK).unwrap_or(true);
        if !due {
            return;
        }
        self.checked = Some(Instant::now());
        let dir = self.info_dir();
        let mtime = std::fs::metadata(&dir).and_then(|m| m.modified()).ok();
        if mtime.is_none() {
            self.use_rpm = self.root == Path::new("/")
                && ["/usr/bin/rpm", "/bin/rpm"].iter().any(|p| Path::new(p).exists());
            return;
        }
        if mtime == self.info_mtime {
            return;
        }
        self.info_mtime = mtime;
        self.index.clear();
        self.sums.clear();
        self.cache.clear();
        let Ok(entries) = std::fs::read_dir(&dir) else {
            return;
        };
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            let Some(stem) = name.strip_suffix(".list") else {
                continue;
            };
            let pkg = stem.split(':').next().unwrap_or(stem).to_string();
            let Ok(text) = std::fs::read_to_string(e.path()) else {
                continue;
            };
            for line in text.lines().filter(|l| exec_dir(l)) {
                for p in spellings(line) {
                    self.index.entry(p).or_insert_with(|| pkg.clone());
                }
            }
        }
    }

    /// The md5sums of one package, read once.
    fn sums_of(&mut self, pkg: &str) -> &HashMap<String, String> {
        if !self.sums.contains_key(pkg) {
            let mut sums = HashMap::new();
            if let Ok(entries) = std::fs::read_dir(self.info_dir()) {
                for e in entries.flatten() {
                    let name = e.file_name().to_string_lossy().into_owned();
                    let Some(stem) = name.strip_suffix(".md5sums") else {
                        continue;
                    };
                    if stem.split(':').next() != Some(pkg) {
                        continue;
                    }
                    for line in std::fs::read_to_string(e.path()).unwrap_or_default().lines() {
                        if let Some((md5, rel)) = line.split_once(char::is_whitespace) {
                            sums.insert(rel.trim_start().to_string(), md5.to_string());
                        }
                    }
                }
            }
            self.sums.insert(pkg.to_string(), sums);
        }
        &self.sums[pkg]
    }

    /// The package that owns `path` and whether the file matches it. None when no
    /// package owns the path, or it is not an executable directory dpkg indexes.
    pub fn verdict(&mut self, path: &str) -> Option<Verdict> {
        if !path.starts_with('/') {
            return None;
        }
        self.refresh();
        let on_disk = self.root.join(path.trim_start_matches('/'));
        let md = std::fs::metadata(&on_disk).ok()?;
        let id: FileId = (md.dev(), md.ino(), md.size(), md.mtime());
        if let Some(v) = self.cache.get(&(path.to_string(), id)) {
            return v.clone();
        }
        let v = if self.use_rpm {
            rpm_verdict(path)
        } else {
            self.dpkg_verdict(path, &on_disk, md.size())
        };
        if self.cache.len() >= CACHE_CAP {
            self.cache.clear();
        }
        self.cache.insert((path.to_string(), id), v.clone());
        v
    }

    fn dpkg_verdict(&mut self, path: &str, on_disk: &Path, size: u64) -> Option<Verdict> {
        let pkg = spellings(path).iter().find_map(|p| self.index.get(p).cloned())?;
        let want: Option<String> = {
            let sums = self.sums_of(&pkg);
            spellings(path)
                .iter()
                .find_map(|p| sums.get(p.trim_start_matches('/')).cloned())
        };
        let verified = match want {
            Some(want) if size <= MAX_HASH_BYTES => file_md5(on_disk).as_deref() == Some(want.as_str()),
            _ => false,
        };
        Some(Verdict { package: pkg, verified })
    }
}

/// rpm -qf for the owner, then rpm -V for that package: the file is verified when
/// rpm reports no digest change and the file is not missing.
fn rpm_verdict(path: &str) -> Option<Verdict> {
    let out = Command::new("rpm")
        .args(["-qf", "--queryformat", "%{NAME}\\n", path])
        .output()
        .ok()?;
    let package = parse_rpm_owner(&String::from_utf8_lossy(&out.stdout), out.status.success())?;
    let v = Command::new("rpm").args(["-V", &package]).output().ok()?;
    let verified = rpm_file_ok(&String::from_utf8_lossy(&v.stdout), path);
    Some(Verdict { package, verified })
}

fn parse_rpm_owner(stdout: &str, ok: bool) -> Option<String> {
    let name = stdout.lines().next()?.trim();
    (ok && !name.is_empty() && !name.contains(' ')).then(|| name.to_string())
}

/// rpm -V prints one line per changed file: nine flag characters (the third is 5 for a
/// digest change), an optional attribute marker, then the path. "missing" marks a
/// deleted file.
fn rpm_file_ok(verify_out: &str, path: &str) -> bool {
    !verify_out.lines().any(|l| {
        let l = l.trim_end();
        if !l.ends_with(path) {
            return false;
        }
        l.starts_with("missing") || l.as_bytes().get(2) == Some(&b'5')
    })
}

/// The host's database, shared by the mapper.
pub fn host_verdict(path: &str) -> Option<Verdict> {
    static DB: OnceLock<Mutex<PkgDb>> = OnceLock::new();
    let db = DB.get_or_init(|| Mutex::new(PkgDb::new(Path::new("/"))));
    db.lock().ok()?.verdict(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn md5_hex(b: &[u8]) -> String {
        openssl::hash::hash(openssl::hash::MessageDigest::md5(), b)
            .unwrap()
            .iter()
            .map(|x| format!("{x:02x}"))
            .collect()
    }

    fn fake_root(tag: &str) -> PathBuf {
        let r = std::env::temp_dir().join(format!("edr_pkg_{tag}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&r);
        fs::create_dir_all(r.join("var/lib/dpkg/info")).unwrap();
        fs::create_dir_all(r.join("usr/bin")).unwrap();
        fs::create_dir_all(r.join("tmp")).unwrap();
        r
    }

    #[test]
    fn packaged_file_verifies_until_it_changes() {
        let r = fake_root("dpkg");
        let body = b"restic binary";
        fs::write(r.join("usr/bin/restic"), body).unwrap();
        fs::write(r.join("var/lib/dpkg/info/restic.list"), "/.\n/usr\n/usr/bin\n/usr/bin/restic\n/usr/share/doc/restic/README\n").unwrap();
        fs::write(
            r.join("var/lib/dpkg/info/restic.md5sums"),
            format!("{}  usr/bin/restic\n", md5_hex(body)),
        )
        .unwrap();
        // A merged /usr package that lists /bin/ and a multiarch name.
        fs::write(r.join("usr/bin/curl"), b"curl").unwrap();
        fs::write(r.join("var/lib/dpkg/info/curl:arm64.list"), "/bin/curl\n").unwrap();
        fs::write(r.join("var/lib/dpkg/info/curl:arm64.md5sums"), format!("{}  bin/curl\n", md5_hex(b"curl"))).unwrap();
        fs::write(r.join("tmp/restic"), body).unwrap();

        let mut db = PkgDb::new(&r);
        assert_eq!(db.verdict("/usr/bin/restic"), Some(Verdict { package: "restic".into(), verified: true }));
        assert_eq!(db.verdict("/usr/bin/curl"), Some(Verdict { package: "curl".into(), verified: true }));
        // The same bytes under /tmp belong to no package.
        assert_eq!(db.verdict("/tmp/restic"), None);
        assert_eq!(db.verdict("relative/restic"), None);
        // Overwritten in place: a new size, so it is checked again and fails.
        fs::write(r.join("usr/bin/restic"), b"something else entirely").unwrap();
        assert_eq!(db.verdict("/usr/bin/restic"), Some(Verdict { package: "restic".into(), verified: false }));
        let _ = fs::remove_dir_all(&r);
    }

    #[test]
    fn rpm_output_is_read() {
        assert_eq!(parse_rpm_owner("restic\n", true), Some("restic".into()));
        assert_eq!(parse_rpm_owner("file /tmp/x is not owned by any package\n", false), None);
        assert!(rpm_file_ok("", "/usr/bin/restic"));
        assert!(rpm_file_ok(".M.......    /usr/bin/restic\n", "/usr/bin/restic"), "a mode change is not a digest change");
        assert!(!rpm_file_ok("S.5....T.    /usr/bin/restic\n", "/usr/bin/restic"));
        assert!(!rpm_file_ok("missing     /usr/bin/restic\n", "/usr/bin/restic"));
        assert!(rpm_file_ok("S.5....T.  c /etc/restic.conf\n", "/usr/bin/restic"), "another file changed");
    }
}
