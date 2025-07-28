use sha2::{Digest, Sha256};
use walkdir::WalkDir;
use mime_guess::from_path;
use users::get_user_by_uid;
use std::os::fd::AsRawFd;
use nix::sys::uio::pread;
use libc::iovec; // replaces `IoVec`
use serde::Serialize;
use serde::Deserialize;
use std::io::BufReader;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::telemetry::calculate_entropy;
use std::{fs::{self, File}, io::Read, path::{Path, PathBuf}, os::unix::fs::PermissionsExt};
use crate::utils::utils::{sha256_digest, estimate_entropy, get_permissions};
use std::os::unix::fs::MetadataExt;
use std::io::BufRead;
use std::fs::metadata;
use std::time::Instant;
use users::os::unix::UserExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FingerprintEntry {
    File {
        path: String,
        hash: String,
        owner: u32,
        trusted_uid: u32,
        permissions: Option<u32>,
        entropy: Option<f64>,
        exec_capable: Option<bool>,
        category: String,
        tags: Vec<String>,
        source_module: String,
        created_at: Option<u64>,
        description: Option<String>,
    },
    User {
        username: String,
        uid: u32,
        roles: String,
        category: String,
        tags: Vec<String>,
        source_module: String,
        created_at: Option<u64>,
        description: Option<String>,
    },


    UsbDevice {
        vendor_id: String,
        product_id: String,
        serial_number: Option<String>,
        category: String,
        tags: Vec<String>,
        source_module: String,
        created_at: Option<u64>,
        description: Option<String>,
    },
    Job {
        command: String,
        schedule: String,
        owner: u32,
        category: String,
        tags: Vec<String>,
        source_module: String,
        created_at: Option<u64>,
        description: Option<String>,
    },
    Script {
    path: String,
    hash: String,
    owner: u32,
    group: u32,
    trusted_uid: u32,
    permissions: Option<u32>,
    entropy: Option<f64>,
    exec_capable: Option<bool>,
    shebang: Option<String>,
    category: String,
    tags: Vec<String>,
    source_module: String,
    created_at: Option<u64>,
    description: Option<String>,
    },

    NetworkEndpoint {
        ip: String,
        port: u16,
        protocol: String,
        process: Option<String>,
        category: String,
        tags: Vec<String>,
        source_module: String,
        created_at: Option<u64>,
        description: Option<String>,
    },
    KernelModule {
        variant: String,
        value: String,
        category: String,
        tags: Vec<String>,
        source_module: String,
        created_at: Option<u64>,
        description: Option<String>,
        size_bytes: Option<u64>,
        permissions: Option<u32>,
        hash: Option<String>,
    },

    AuthChain {
        binary_path: String,
        hash: String,
        signer: Option<String>,
        verified: bool,
        trusted_uid: u32,
        category: String,
        tags: Vec<String>,
        source_module: String,
        created_at: Option<u64>,
        description: Option<String>,
    },
    CronShell {
    command: String,              // Full cron command
    shell_path: String,           // Shell binary used (e.g., /bin/bash)
    user: String,                 // Username (not UID)
    uid: u32,                     // UID for trust mapping
    home_dir: Option<String>,     // Useful for role mapping
    frequency: Option<String>,    // e.g., "@hourly", "0 5 * * *"
    exec_capable: Option<bool>,   // Indicates if the shell path is executable
    category: String,
    tags: Vec<String>,
    source_module: String,
    created_at: Option<u64>,
    description: Option<String>,
},    
BaselineSyscallRange {
        path: String,
        hash: String,
        owner: u32,
        trusted_uid: u32,
        permissions: u32,
        entropy: Option<f64>,
        exec_capable: Option<bool>,
        category: String,
        tags: Vec<String>, // syscall names or ranges
        source_module: String,
        created_at: Option<u64>,
        description: Option<String>,
    },

    GpgSshKey {
        path: String,
        hash: String,
        owner: u32,
        trusted_uid: u32,
        permissions: Option<u32>,
        entropy: Option<f64>,
        exec_capable: Option<bool>,
        category: String,
        tags: Vec<String>,
        source_module: String,
        created_at: Option<u64>,
        description: Option<String>,
    },


    
    ProcessLineageBaseline {
    root_process: String,
    expected_ancestry: Vec<String>,
    trusted_uid: u32,
    category: String,
    tags: Vec<String>,
    source_module: String,
    created_at: Option<u64>,
    description: Option<String>,
},

    KnownGoodBinary {
        binary_path: String,
        hash: String,
        signer: Option<String>,
        verified: bool,
        trusted_uid: u32,
        roles_allowed: Vec<String>,
        category: String,
        tags: Vec<String>,
        source_module: String,
        created_at: Option<u64>,
        description: Option<String>
    },
    Container {
    container_id: String,
    name: String,
    runtime: String,
    rootfs_path: String,
    trusted_uid: u32,
    category: String,
    tags: Vec<String>,
    source_module: String,
    created_at: Option<u64>,
    description: Option<String>,
    },

    MemoryRegion {
    path: String,
    pid: u32,
    perms: String,
    offset: Option<u64>,
    size: Option<u64>,
    entropy: Option<f64>,
    exec_capable: Option<bool>,
    trusted_uid: u32,
    category: String,
    tags: Vec<String>,
    source_module: String,
    created_at: Option<u64>,
    description: Option<String>,
},

    
}


pub fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}


pub fn crawl_files() -> Vec<FingerprintEntry> {
    use std::collections::HashSet;
    use std::os::unix::fs::PermissionsExt;
    use mime_guess::from_path;

    println!("[*] crawl_files() called");
    let mut entries = Vec::new();
    let mut seen_hashes = HashSet::new();
    let home = std::env::var("HOME").unwrap_or("/home/unknown".into());

    let config_path = format!("{}/.config", home);
    let local_path = format!("{}/.local", home);

    let dirs = vec![
        "/etc", "/usr/bin", "/bin", "/usr/sbin", "/opt", "/lib", "/lib64",
        "/tmp", "/var/tmp", config_path.as_str(), local_path.as_str(),
        "/var/lib/docker/overlay2", "/var/lib/containers", "/run/containers",
    ];

    for dir in dirs {
        println!("[DEBUG] Checking dir: {}", dir);
        if !Path::new(dir).exists() {
            println!("[WARN] Skipping non-existent dir: {}", dir);
            continue;
        }

        let mut file_count = 0;
        for entry in WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            file_count += 1;
            let path = entry.path().to_path_buf();
            let metadata = match fs::metadata(&path) {
                Ok(m) => m,
                Err(_) => {
                    if let Ok(fallback) = fs::symlink_metadata(&path) {
                        let uid = fallback.uid();
                        let permissions = fallback.permissions().mode();
                        entries.push(FingerprintEntry::File {
                            path: path.display().to_string(),
                            hash: "unreadable".into(),
                            owner: uid,
                            trusted_uid: uid,
                            permissions: Some(permissions),
                            entropy: None,
                            exec_capable: None,
                            category: "unknown".into(),
                            tags: vec!["unreadable".into()],
                            source_module: "file_hash_watcher".into(),
                            created_at: Some(current_timestamp()),
                            description: Some("Unreadable file: metadata only".into()),
                        });
                    }
                    continue;
                }
            };

            let permissions = metadata.permissions().mode();
            let exec_capable = permissions & 0o111 != 0;
            let suid = permissions & 0o4000 != 0;
            let sgid = permissions & 0o2000 != 0;
            let uid = metadata.uid();
            let size = metadata.len();

            let hash = match sha256_digest(path.to_string_lossy().as_ref()) {
                Some(h) => h,
                None => {
                    entries.push(FingerprintEntry::File {
                        path: path.display().to_string(),
                        hash: "hash_failed".into(),
                        owner: uid,
                        trusted_uid: uid,
                        permissions: Some(permissions),
                        entropy: None,
                        exec_capable: Some(exec_capable),
                        category: "unknown".into(),
                        tags: vec!["hash_failed".into()],
                        source_module: "file_hash_watcher".into(),
                        created_at: Some(current_timestamp()),
                        description: Some("Hashing failed".into()),
                    });
                    continue;
                }
            };

            if seen_hashes.contains(&hash) {
                continue;
            }
            seen_hashes.insert(hash.clone()); // keep original for insertion

            let entropy = estimate_entropy(path.to_string_lossy().as_bytes());
            let mime = from_path(&path).first_or_octet_stream().essence_str().to_string();

            let mut tags = vec!["baseline".to_string()];
            if exec_capable {
                tags.push("executable".to_string());
            }
            if suid {
                tags.push("suid".to_string());
            }
            if sgid {
                tags.push("sgid".to_string());
            }
            if mime.contains("desktop")
                || path.extension().map(|e| e == "desktop").unwrap_or(false)
            {
                tags.push("launcher".to_string());
            }

            let hash_clone = hash.clone(); // safe for later use

            entries.push(FingerprintEntry::File {
                path: path.display().to_string(),
                hash,
                owner: uid,
                trusted_uid: uid,
                permissions: Some(permissions),
                entropy: Some(entropy),
                exec_capable: Some(exec_capable),
                category: mime,
                tags,
                source_module: "file_hash_watcher".into(),
                created_at: Some(current_timestamp()),
                description: Some(format!("Baseline file in {} (size: {} bytes)", dir, size)),
            });

            if file_count <= 2 {
                println!(
                    "[DEBUG] Entry from {}: hash truncated = {}...",
                    path.display(),
                    &hash_clone[..8]
                );
            }
        }

        println!("[DEBUG] Files scanned in {}: {}", dir, file_count);
    }

    println!("[*] crawl_files() finished with {} entries", entries.len());
    entries
}



pub fn crawl_users() -> Vec<FingerprintEntry> {
    let mut entries = Vec::new();
    let passwd_file = File::open("/etc/passwd").expect("Failed to open /etc/passwd");
    let reader = BufReader::new(passwd_file);

    let mut shadow_map = std::collections::HashMap::new();
    if let Ok(shadow_file) = File::open("/etc/shadow") {
        let shadow_reader = BufReader::new(shadow_file);
        for line in shadow_reader.lines().flatten() {
            if let Some((user, hash)) = line.split_once(':') {
                shadow_map.insert(user.to_string(), hash.to_string());
            }
        }
    }

    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 7 {
            continue;
        }

        let username = parts[0].to_string();
        let uid = parts[2].parse::<u32>().unwrap_or(0);
        let gid = parts[3].parse::<u32>().unwrap_or(0);
        let home = parts[5].to_string();
        let shell = parts[6].to_string();
        let mut tags = vec![];
        let roles = "standard_user".to_string(); // or pull from actual user metadata

        let shadow_entry = shadow_map.get(&username);
        let locked = shadow_entry.map_or(false, |s| s.starts_with('!') || s.starts_with('*'));
        if locked { tags.push("locked".to_string()); }

        if uid == 0 { tags.push("root".to_string()); }
        if shell == "/usr/sbin/nologin" || shell == "/bin/false" {
            tags.push("non_interactive".to_string());
        } else {
            tags.push("shell".to_string());
        }

        if uid < 1000 {
            tags.push("system".to_string());
        } else {
            tags.push("human".to_string());
        }

        if username == "nobody" {
            tags.push("anonymous".to_string());
        }

        if username == "sudo" || username.contains("admin") || username.contains("wheel") {
            tags.push("sudo".to_string());
        }

        entries.push(FingerprintEntry::User {
            username: username.clone(),
            uid,
            roles: roles.clone(), // roles must be a String, not a Vec<String>
            category: "user_account".into(),
            tags: tags.clone(),   // assuming this is Vec<String>
            source_module: "user_tracker".into(),
            created_at: Some(current_timestamp()),
            description: Some(format!(
                "Baseline user account (uid={}, shell={}, home={})",
                uid, shell.trim(), home.trim()
            )),
        });

    }

    entries
}



pub fn crawl_jobs() -> Vec<FingerprintEntry> {
    let mut entries = Vec::new();

    // --- System-wide crontab (/etc/crontab) ---
    if let Ok(file) = File::open("/etc/crontab") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if line.trim().is_empty() || line.starts_with('#') { continue; }
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 7 {
                let user = fields[5];
                let command = fields[6..].join(" ");
                entries.push(FingerprintEntry::Job {
                    command,
                    schedule: "system_cron".into(),
                    owner: get_uid_for_user(user),
                    category: "job".into(),
                    tags: vec!["cron".into()],
                    source_module: "job_sched_monitor".into(),
                    created_at: Some(current_timestamp()),
                    description: Some("System crontab job".into()),
                });
            }
        }
    }

    // --- Per-user crontabs ---
    if let Ok(dir) = fs::read_dir("/var/spool/cron/crontabs") {
        for entry in dir.flatten() {
            let path = entry.path();
            if let Ok(file) = File::open(&path) {
                let reader = BufReader::new(file);
                let user = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                for line in reader.lines().flatten() {
                    if line.trim().is_empty() || line.starts_with('#') { continue; }
                    entries.push(FingerprintEntry::Job {
                        command: line.trim().to_string(),
                        schedule: "user_cron".into(),
                        owner: get_uid_for_user(&user),
                        category: "job".into(),
                        tags: vec!["cron".into(), "user_cron".into()],
                        source_module: "job_sched_monitor".into(),
                        created_at: Some(current_timestamp()),
                        description: Some(format!("Crontab for user {}", user)),
                    });
                }
            }
        }
    }

    // --- systemd timers ---
    if let Ok(output) = Command::new("systemctl").arg("list-timers").output() {
        for line in String::from_utf8_lossy(&output.stdout).lines().skip(1) {
            if line.trim().is_empty() || line.starts_with("---") { continue; }
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() >= 6 {
                let timer_name = cols[5];
                entries.push(FingerprintEntry::Job {
                    command: timer_name.to_string(),
                    schedule: cols[2].to_string(), // e.g. "daily", "hourly"
                    owner: 0,
                    category: "job".into(),
                    tags: vec!["systemd_timer".into()],
                    source_module: "job_sched_monitor".into(),
                    created_at: Some(current_timestamp()),
                    description: Some("Systemd timer job".into()),
                });
            }
        }
    }

    // --- at jobs ---
    if let Ok(output) = Command::new("atq").output() {
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if !line.trim().is_empty() {
                entries.push(FingerprintEntry::Job {
                    command: "<deferred shell command via at>".into(),
                    schedule: "at".into(),
                    owner: 0,
                    category: "job".into(),
                    tags: vec!["at_job".into()],
                    source_module: "job_sched_monitor".into(),
                    created_at: Some(current_timestamp()),
                    description: Some("Deferred job via atq".into()),
                });
            }
        }
    }

    entries
}


fn get_uid_for_user(username: &str) -> u32 {
    use users::get_user_by_name;
    get_user_by_name(username).map(|u| u.uid()).unwrap_or(0)
}

pub fn crawl_usb_devices() -> Vec<FingerprintEntry> {
    let mut entries = Vec::new();

    // Get all mounted devices
    let mounts = match File::open("/proc/mounts") {
        Ok(file) => BufReader::new(file)
            .lines()
            .flatten()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
        Err(_) => vec![],
    };

    // Scan all /dev/sd* block devices
    if let Ok(devices) = fs::read_dir("/sys/block/") {
        for dev in devices.flatten() {
            let dev_name = dev.file_name().to_string_lossy().into_owned();
            if !dev_name.starts_with("sd") { continue; } // Only look at real block devices

            let sys_path = format!("/sys/block/{}/device", dev_name);
            let uevent_path = format!("{}/uevent", sys_path);
            let dev_path = format!("/dev/{}", dev_name);

            // Confirm USB parent (via uevent)
            let uevent = fs::read_to_string(&uevent_path).unwrap_or_default();
            if !uevent.contains("DRIVER=usb") && !uevent.contains("MODALIAS=usb:") { continue; }

            // Try to get serial number
            let serial = get_serial_from_udevadm(&dev_path);
            let vendor_id = Some("1234".to_string());
            let product_id = Some("5678".to_string());
            let serial_number = Some("ABCDEF123456".to_string());

            // Find mount path, if any
            let mount_path = mounts
                .iter()
                .find(|(dev_match, _)| dev_match == &dev_path)
                .map(|(_, mnt)| mnt.to_string());

            // Estimate UID from mount path ownership
            let trusted_uid = match &mount_path {
                Some(p) => fs::metadata(p).map(|m| m.uid()).unwrap_or(0),
                None => 0,
            };
            entries.push(FingerprintEntry::UsbDevice {
                vendor_id: vendor_id.unwrap_or_else(|| "unknown".into()),
                product_id: product_id.unwrap_or_else(|| "unknown".into()),
                serial_number: serial,
                category: "usb_device".into(),
                tags: vec!["usb".into(), "block_device".into()],
                source_module: "usb_monitor".into(),
                created_at: Some(current_timestamp()),
                description: Some(format!("USB block device: {}", dev_path)),
            });

        }
    }

    entries
}

fn get_serial_from_udevadm(dev_path: &str) -> Option<String> {
    if let Ok(output) = Command::new("udevadm").arg("info").arg("-a").arg("-n").arg(dev_path).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.trim_start().starts_with("ATTRS{serial}==") {
                return line.split('=').nth(1).map(|s| s.trim_matches('"').to_string());
            }
        }
    }
    None
}


pub fn crawl_network() -> Vec<FingerprintEntry> {
    let mut entries = Vec::new();
    let now = current_timestamp();

    // Try to run `ss` with listening and process info
    let output = Command::new("ss")
        .arg("-lpnH") // listening, PID info, no headers
        .output()
        .or_else(|_| Command::new("netstat").arg("-anp").output());

    if let Ok(output) = output {
        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() < 5 { continue; }

            let proto = cols[0];
            let local_addr = cols[3];
            let mut process_name = "unknown".to_string();

            // Extract process info from last column (ss shows like: users:(("nginx",pid=1234,fd=5)))
            if let Some(users_col) = line.split("users:(").nth(1) {
                if let Some(proc_fragment) = users_col.split('"').nth(1) {
                    process_name = proc_fragment.to_string();
                }
            }

            // Handle IPv6 safely
            let (ip, port) = if local_addr.contains('[') && local_addr.contains(']') {
                // IPv6 format: [::1]:443
                let ip_part = local_addr.split(']').next().unwrap_or("").trim_start_matches('[');
                let port_part = local_addr.split(']').nth(1).unwrap_or("").trim_start_matches(':');
                (ip_part, port_part)
            } else {
                // IPv4 or bare format
                local_addr.rsplit_once(':').unwrap_or(("", ""))
            };

            if let Ok(port_num) = port.parse::<u16>() {
                entries.push(FingerprintEntry::NetworkEndpoint {
                    ip: ip.to_string(),
                    port: port_num,
                    protocol: proto.to_string(),
                    process: Some(process_name.clone()),
                    category: "network_endpoint".into(),
                    tags: vec!["network".into(), "baseline".into()],
                    source_module: "net_watch".into(),
                    created_at: Some(now),
                    description: Some(format!("Known-good {}:{} via {}", ip, port, process_name)),
                });
            }
        }
    } else {
        eprintln!("[!] Failed to run ss or netstat");
    }

    println!("[*] crawl_network() completed with {} entries", entries.len());
    entries
}


pub fn crawl_containers() -> Vec<FingerprintEntry> {
    let mut entries = Vec::new();

    // Utility: Extract UID from path
    let get_uid_from_path = |p: &str| -> u32 {
        fs::metadata(p).map(|m| m.uid()).unwrap_or(0)
    };

    // 1. Docker containers
    if let Ok(output) = Command::new("docker")
        .args(&["ps", "--format", "{{.ID}} {{.Names}}"])
        .output()
    {
        if output.status.success() {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    let id = parts[0];
                    let name = parts[1];

                    if let Ok(iout) = Command::new("docker")
                        .args(&["inspect", "--format", "{{.GraphDriver.Data.MergedDir}}", id])
                        .output()
                    {
                        if iout.status.success() {
                            let fs_root = String::from_utf8_lossy(&iout.stdout).trim().to_string();
                            let trusted_uid = get_uid_from_path(&fs_root);
                            entries.push(FingerprintEntry::Container {
                                container_id: id.to_string(),
                                name: name.to_string(),
                                runtime: "docker".into(),
                                rootfs_path: fs_root,
                                trusted_uid,
                                category: "container".into(),
                                tags: vec!["docker".into(), "container".into()],
                                source_module: "container_monitor".into(),
                                created_at: Some(current_timestamp()),
                                description: Some(format!("Docker container {}", name)),
                            });
                        }
                    }
                }
            }
        }
    }

    // 2. containerd containers
    let containerd_path = "/run/containerd/io.containerd.runtime.v2.task/";
    if Path::new(containerd_path).exists() {
        if let Ok(namespaces) = fs::read_dir(containerd_path) {
            for ns in namespaces.flatten() {
                let ns_path = ns.path();
                if ns_path.is_dir() {
                    if let Ok(containers) = fs::read_dir(&ns_path) {
                        for container in containers.flatten() {
                            let id = container.file_name().to_string_lossy().to_string();
                            let bundle = ns_path.join(&id).join("rootfs");
                            if bundle.exists() {
                                let trusted_uid = get_uid_from_path(bundle.to_str().unwrap_or(""));
                                entries.push(FingerprintEntry::Container {
                                    container_id: id.clone(),
                                    name: id.clone(),
                                    runtime: "containerd".into(),
                                    rootfs_path: bundle.display().to_string(),
                                    trusted_uid,
                                    category: "container".into(),
                                    tags: vec!["containerd".into(), "container".into()],
                                    source_module: "container_monitor".into(),
                                    created_at: Some(current_timestamp()),
                                    description: Some(format!("containerd container {}", id)),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Podman containers (root context)
    if let Ok(output) = Command::new("podman")
        .args(&["ps", "--format", "{{.ID}} {{.Names}}"])
        .output()
    {
        if output.status.success() {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    let id = parts[0];
                    let name = parts[1];

                    if let Ok(iout) = Command::new("podman")
                        .args(&["inspect", "--format", "{{.GraphDriver.Data.MergedDir}}", id])
                        .output()
                    {
                        if iout.status.success() {
                            let fs_root = String::from_utf8_lossy(&iout.stdout).trim().to_string();
                            let trusted_uid = get_uid_from_path(&fs_root);
                            entries.push(FingerprintEntry::Container {
                                container_id: id.to_string(),
                                name: name.to_string(),
                                runtime: "podman".into(),
                                rootfs_path: fs_root,
                                trusted_uid,
                                category: "container".into(),
                                tags: vec!["podman".into(), "container".into()],
                                source_module: "container_monitor".into(),
                                created_at: Some(current_timestamp()),
                                description: Some(format!("Podman container {}", name)),
                            });
                        }
                    }
                }
            }
        }
    }

    // 4. Rootless Podman (home dir)
    if let Some(home) = dirs::home_dir() {
        let overlay_root = home.join(".local/share/containers/storage/overlay");
        if overlay_root.exists() {
            for entry in fs::read_dir(overlay_root).unwrap_or_else(|_| fs::read_dir("/dev/null").unwrap()) {
                if let Ok(e) = entry {
                    let path = e.path();
                    let merged = path.join("merged");
                    if merged.exists() {
                        let id = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                        let trusted_uid = get_uid_from_path(merged.to_str().unwrap_or(""));
                        entries.push(FingerprintEntry::Container {
                            container_id: id.clone(),
                            name: id.clone(),
                            runtime: "podman_rootless".into(),
                            rootfs_path: merged.display().to_string(),
                            trusted_uid,
                            category: "container".into(),
                            tags: vec!["podman".into(), "rootless".into(), "container".into()],
                            source_module: "container_monitor".into(),
                            created_at: Some(current_timestamp()),
                            description: Some(format!("Rootless Podman container {}", id)),
                        });
                    }
                }
            }
        }
    }

    // Fallback log
    if entries.is_empty() {
        eprintln!("[!] crawl_containers(): no containers detected.");
    }

    entries
}
pub fn crawl_scripts() -> Vec<FingerprintEntry> {
    let mut entries = Vec::new();

    let script_extensions = vec![
        "sh", "bash", "zsh", "csh", "ksh", "py", "pl", "rb", "js", "lua", "php", "tcl",
    ];

    let home = std::env::var("HOME").unwrap_or_default();
    let config_path = format!("{}/.config", home);
    let local_path = format!("{}/.local", home);

    let search_dirs = vec![
        "/etc",
        "/opt",
        "/var/tmp",
        "/tmp",
        "/usr/local/bin",
        "/usr/local/sbin",
        "/usr/bin",
        "/bin",
        "/sbin",
        config_path.as_str(),
        local_path.as_str(),
    ];

    for dir in search_dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();

            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let exec = metadata.permissions().mode() & 0o111 != 0;

            let ext_match = path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| script_extensions.contains(&s))
                .unwrap_or(false);

            // Shebang logic
            let mut shebang = None;
            if let Ok(f) = File::open(path) {
                let mut reader = BufReader::new(f);
                let mut first_line = String::new();
                if reader.read_line(&mut first_line).is_ok() && first_line.trim().starts_with("#!") {
                    shebang = Some(first_line.trim().to_string());
                }
            }

            // Skip non-scripts
            if !ext_match && !exec && shebang.is_none() {
                continue;
            }

            let mut file = match File::open(path) {
                Ok(f) => f,
                Err(_) => continue,
            };

            let mut contents = Vec::new();
            if file.read_to_end(&mut contents).is_err() {
                continue;
            }

            let hash = Sha256::digest(&contents);
            let entropy = calculate_entropy(&contents);

            let mut tags = vec!["script_file".into()];
            let path_str = path.display().to_string();

            if path_str.starts_with("/tmp") || path_str.starts_with("/var/tmp") {
                tags.push("risky_location".into());
            }
            if path_str.contains("/mnt") {
                tags.push("usb_mount".into());
            }
            if entropy > 7.5 {
                tags.push("high_entropy".into());
            }

            entries.push(FingerprintEntry::Script {
                path: path_str,
                hash: format!("{:x}", hash),
                owner: metadata.uid(),
                group: metadata.gid(), // or `.unwrap_or(0)` if using Option<u32>
                trusted_uid: 0,
                permissions: Some(metadata.permissions().mode()),
                entropy: Some(entropy as f64),
                exec_capable: Some(exec),
                shebang,
                category: "script".into(),
                tags,
                source_module: "script_monitor".into(),
                created_at: Some(current_timestamp()),
                description: Some("Script file fingerprinted during baseline".into()),
            });
        }
    }

    entries
}

pub fn crawl_memory_regions() -> Vec<FingerprintEntry> {
    let mut entries = Vec::new();

    if let Ok(pids) = fs::read_dir("/proc") {
        for pid_entry in pids.flatten() {
            let pid_path = pid_entry.path();
            let pid_str = match pid_path.file_name().and_then(|s| s.to_str()) {
                Some(s) => s,
                None => continue,
            };
            if !pid_str.chars().all(char::is_numeric) {
                continue;
            }

            let pid: i32 = match pid_str.parse() {
                Ok(p) => p,
                Err(_) => continue,
            };

            let maps_path = format!("/proc/{}/maps", pid);
            let mem_path = format!("/proc/{}/mem", pid);

            let maps_file = match fs::File::open(&maps_path) {
                Ok(f) => f,
                Err(_) => continue,
            };

            let reader = BufReader::new(maps_file);
            for line in reader.lines().flatten() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 5 {
                    continue;
                }

                let addresses = parts[0];
                let perms = parts[1];
                let offset = parts[2];
                let _device = parts[3];
                let _inode = parts[4];
                let path = if parts.len() > 5 {
                    parts[5..].join(" ")
                } else {
                    "anonymous".to_string()
                };

                let exec = perms.contains('x');
                let write = perms.contains('w');
                let _private = perms.contains('p');

                let category = if path.starts_with('/') {
                    "file_backed_memory"
                } else if path.contains("[stack]") {
                    "stack_memory"
                } else if path.contains("[heap]") {
                    "heap_memory"
                } else if path.contains("[anon]") || path == "anonymous" {
                    "anonymous_memory"
                } else {
                    "other_memory"
                };

                let (start, end) = match addresses.split_once('-') {
                    Some((s, e)) => (
                        usize::from_str_radix(s, 16).unwrap_or(0),
                        usize::from_str_radix(e, 16).unwrap_or(0),
                    ),
                    None => continue,
                };

                let size_bytes = end.saturating_sub(start);
                let entropy = if exec || write {
                    if let Ok(mut mem_file) = fs::File::open(&mem_path) {
                        let mut buf = vec![0u8; size_bytes.min(4096)];
                        if let Ok(n) = pread(&mem_file, &mut buf, start as i64) {
                            Some(calculate_entropy(&buf[..n]))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                let trusted_uid = fs::metadata(format!("/proc/{}", pid))
                    .map(|m| m.uid())
                    .unwrap_or(0);

                let region_id = format!("{}:{}:{}:{}:{}", pid, addresses, perms, offset, path);
                let hash = format!("{:x}", Sha256::digest(region_id.as_bytes()));

                let mut tags = vec!["memory_region".into()];
                if exec && write {
                    tags.push("rwx".into());
                }
                if category == "anonymous_memory" {
                    tags.push("anonymous".into());
                }
                if entropy.unwrap_or(0.0) > 7.5 {
                    tags.push("high_entropy".into());
                }

                entries.push(FingerprintEntry::MemoryRegion {
                    path: path.to_string(),
                    pid: pid as u32,
                    perms: perms.to_string(),
                    offset: Some(u64::from_str_radix(offset, 16).unwrap_or(0)),
                    size: Some(size_bytes as u64),
                    entropy: entropy.map(|e| e as f64),
                    exec_capable: Some(exec),
                    trusted_uid,
                    category: category.to_string(),
                    tags,
                    source_module: "mem_scan".into(),
                    created_at: Some(current_timestamp()),
                    description: Some(format!(
                        "Memory region [{}] for pid {} with perms [{}]",
                        path, pid, perms
                    )),
                });
            }
        }
    }

    entries
}
pub fn crawl_known_cron_shells() -> Vec<FingerprintEntry> {
    let mut entries = Vec::new();

    let crontab_path = "/etc/crontab";

    let file = match File::open(crontab_path) {
        Ok(f) => f,
        Err(_) => return entries,
    };

    for line in BufReader::new(file).lines().flatten() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() < 7 {
            continue; // Not a valid cron job line
        }

        let (frequency, user, command_tokens) = if tokens[0].starts_with('@') {
            // Format: @daily root /command
            if tokens.len() < 3 {
                continue;
            }
            (Some(tokens[0].to_string()), tokens[1].to_string(), tokens[2..].to_vec())
        } else {
            // Format: m h dom mon dow user /command
            if tokens.len() < 7 {
                continue;
            }
            let freq_parts = tokens[0..5].join(" ");
            (Some(freq_parts), tokens[5].to_string(), tokens[6..].to_vec())
        };

        let command_str = command_tokens.join(" ");
        let shell_path = if command_tokens.is_empty() {
            "/bin/sh".to_string()
        } else {
            command_tokens[0].to_string()
        };

        let exec_capable = match fs::metadata(&shell_path) {
            Ok(m) => m.permissions().mode() & 0o111 != 0,
            Err(_) => false,
        };

        let uid = match users::get_user_by_name(&user) {
            Some(u) => u.uid(),
            None => 0,
        };

        let home_dir = users::get_user_by_name(&user)
            .and_then(|u| u.home_dir().to_str().map(|s| s.to_string()));

        entries.push(FingerprintEntry::CronShell {
            command: command_str.clone(),
            shell_path: shell_path.clone(),
            user: user.clone(),
            uid,
            home_dir,
            frequency,
            exec_capable: Some(exec_capable),
            category: "cron_shell".to_string(),
            tags: vec!["scheduled_task".to_string()],
            source_module: "job_sched_monitor".to_string(),
            created_at: Some(current_timestamp()),
            description: Some(format!("Cron job shell command: {}", command_str)),
        });
    }

    entries
}

pub fn crawl_binary_auth_chain() -> Vec<FingerprintEntry> {
    let mut entries = Vec::new();
    let auth_bins = vec![
        "/usr/bin/sudo",
        "/usr/bin/su",
        "/usr/bin/passwd",
        "/usr/bin/login",
        "/bin/login",
        "/usr/sbin/sshd",
        "/sbin/unix_chkpwd",
        "/usr/libexec/sssd/sssd",
        "/usr/lib/polkit-1/polkit-agent-helper-1",
    ];

    for path in auth_bins {
        let pb = PathBuf::from(path);
        if pb.exists() {
            let hash = sha256_digest(pb.to_string_lossy().as_ref());
            let metadata = fs::metadata(&pb).ok();
            let owner = metadata.map(|m| m.uid()).unwrap_or(0);

            entries.push(FingerprintEntry::AuthChain {
                binary_path: path.to_string(),
                hash: hash.unwrap_or_default(),
                signer: None,
                verified: false, // mark as false until we implement signature checking
                trusted_uid: owner,
                category: "auth_binary".into(),
                tags: vec!["auth".into(), "privilege".into(), "binary".into()],
                source_module: "privilege_monitor".into(),
                created_at: Some(current_timestamp()),
                description: Some("Baseline authentication-related binary".into()),
            });
        }
    }

    // PAM Modules
    let pam_dirs = vec!["/lib/security/", "/lib64/security/", "/usr/lib/security/"];
    for dir in pam_dirs {
        if let Ok(walker) = fs::read_dir(dir) {
            for entry in walker.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "so").unwrap_or(false) {
                    let hash = sha256_digest(path.to_string_lossy().as_ref());
                    let metadata = fs::metadata(&path).ok();
                    let owner = metadata.map(|m| m.uid()).unwrap_or(0);

                    entries.push(FingerprintEntry::KnownGoodBinary {
                        binary_path: path.display().to_string(),
                        hash: hash.unwrap_or_default(),
                        signer: None,
                        verified: false,
                        trusted_uid: owner,
                        roles_allowed: vec!["default".into()],
                        category: "pam_module".into(),
                        tags: vec!["auth".into(), "pam".into(), "module".into()],
                        source_module: "privilege_monitor".into(),
                        created_at: Some(current_timestamp()),
                        description: Some("Baseline PAM module for authentication".into()),
                    });
                }
            }
        }
    }

    entries
}


pub fn trace_ancestry(mut pid: i32) -> Vec<String> {
    let mut ancestry = Vec::new();
    let mut visited = std::collections::HashSet::new();

    while pid > 1 && visited.insert(pid) {
        let proc_dir = PathBuf::from(format!("/proc/{}", pid));
        let exe_path = proc_dir.join("exe");
        let status_path = proc_dir.join("status");

        // Try to resolve the executable path
        let binary_path = fs::read_link(&exe_path)
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| format!("[unknown_exe_{}]", pid));

        ancestry.push(binary_path);

        // Parse the PPid field from /proc/[pid]/status
        let ppid = fs::read_to_string(&status_path)
            .ok()
            .and_then(|content| {
                content.lines().find_map(|line| {
                    if line.starts_with("PPid:") {
                        line.split_whitespace().nth(1).and_then(|v| v.parse::<i32>().ok())
                    } else {
                        None
                    }
                })
            })
            .unwrap_or(1);

        pid = ppid;
    }

    ancestry
}pub fn crawl_process_lineage_baseline() -> Vec<FingerprintEntry> {
    let mut entries = Vec::new();
    let proc_root = Path::new("/proc");

    let Ok(proc_entries) = fs::read_dir(proc_root) else {
        return entries; // /proc is unreadable
    };

    for entry in proc_entries.flatten() {
        let pid_dir = entry.path();
        let pid_str = entry.file_name().to_string_lossy().into_owned();

        if let Ok(pid) = pid_str.parse::<i32>() {
            let status_path = pid_dir.join("status");
            let exe_path = pid_dir.join("exe");

            let ppid = fs::read_to_string(&status_path)
                .ok()
                .and_then(|content| {
                    content.lines().find_map(|line| {
                        if line.starts_with("PPid:") {
                            line.split_whitespace().nth(1).and_then(|ppid| ppid.parse().ok())
                        } else {
                            None
                        }
                    })
                })
                .unwrap_or(0);

            let exe = fs::read_link(&exe_path)
                .ok()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| format!("[unknown_exe_{}]", pid));

            let ancestry = trace_ancestry(pid);
            if ancestry.is_empty() {
                continue;
            }

            let trusted_uid = ancestry
                .first()
                .and_then(|bin| fs::metadata(bin).ok())
                .map(|m| m.uid())
                .unwrap_or(0);

            entries.push(FingerprintEntry::ProcessLineageBaseline {
                root_process: exe,
                expected_ancestry: ancestry,
                trusted_uid,
                category: "process_lineage".into(),
                tags: vec!["process".into(), "lineage".into(), "ancestry".into()],
                source_module: "process_monitor".into(),
                created_at: Some(current_timestamp()),
                description: Some(format!("Observed lineage for pid {} -> ppid {}", pid, ppid)),
            });
        }
    }

    entries
}

fn get_syscalls_used(binary_path: &str) -> Option<Vec<String>> {
    let output = Command::new("strace")
        .arg("-c")
        .arg("-e")
        .arg("trace=all")
        .arg(binary_path)
        .arg("--version") // a safe, non-destructive argument
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stderr);
    let mut syscalls = Vec::new();

    for line in stdout.lines() {
        if let Some(syscall) = line.split_whitespace().last() {
            if syscall.chars().all(char::is_alphabetic) && syscall.len() < 32 {
                syscalls.push(syscall.to_string());
            }
        }
    }

    if syscalls.is_empty() {
        None
    } else {
        Some(syscalls)
    }
}

pub fn crawl_gpg_ssh_keys() -> Vec<FingerprintEntry> {
    let mut entries = Vec::new();

    if let Ok(file) = File::open("/etc/passwd") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() < 6 {
                continue;
            }

            let username = parts[0];
            let uid: u32 = parts[2].parse().unwrap_or(0);
            let home_dir = parts[5];

            let ssh_path = format!("{}/.ssh", home_dir);
            let gpg_path = format!("{}/.gnupg", home_dir);

            // SSH Keys
            if Path::new(&ssh_path).exists() {
                if let Ok(files) = fs::read_dir(&ssh_path) {
                    for file in files.flatten() {
                        let path = file.path();
                        if let Some(fname) = path.file_name().and_then(|f| f.to_str()) {
                            if fname.starts_with("id_") && !fname.ends_with(".pub") {
                                if let Ok(metadata) = fs::metadata(&path) {
                                    let mut file_contents = Vec::new();
                                    if File::open(&path).and_then(|mut f| f.read_to_end(&mut file_contents)).is_ok() {
                                        entries.push(FingerprintEntry::GpgSshKey {
                                            path: path.to_string_lossy().to_string(),
                                            hash: format!("{:x}", Sha256::digest(&file_contents)),
                                            owner: metadata.uid(),
                                            trusted_uid: uid,
                                            permissions: Some(metadata.permissions().mode()),
                                            entropy: Some(calculate_entropy(&file_contents)),
                                            exec_capable: Some(metadata.permissions().mode() & 0o111 != 0),
                                            category: "key".into(),
                                            tags: vec!["ssh_key".into()],
                                            source_module: "key_crawler".into(),
                                            created_at: Some(current_timestamp()),
                                            description: Some(format!("SSH key file for user '{}'", username)),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // GPG Keys
            if Path::new(&gpg_path).exists() {
                if let Ok(files) = fs::read_dir(&gpg_path) {
                    for file in files.flatten() {
                        let path = file.path();
                        if let Some(fname) = path.file_name().and_then(|f| f.to_str()) {
                            if fname.contains("key") || fname.ends_with(".gpg") || fname.ends_with(".asc") {
                                if let Ok(metadata) = fs::metadata(&path) {
                                    let mut file_contents = Vec::new();
                                    if File::open(&path).and_then(|mut f| f.read_to_end(&mut file_contents)).is_ok() {
                                        entries.push(FingerprintEntry::GpgSshKey {
                                            path: path.to_string_lossy().to_string(),
                                            hash: format!("{:x}", Sha256::digest(&file_contents)),
                                            owner: metadata.uid(),
                                            trusted_uid: uid,
                                            permissions: Some(metadata.permissions().mode()),
                                            entropy: Some(calculate_entropy(&file_contents)),
                                            exec_capable: Some(metadata.permissions().mode() & 0o111 != 0),
                                            category: "key".into(),
                                            tags: vec!["gpg_key".into()],
                                            source_module: "key_crawler".into(),
                                            created_at: Some(current_timestamp()),
                                            description: Some(format!("GPG key file for user '{}'", username)),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    entries
}

pub fn crawl_kernel_modules() -> Vec<FingerprintEntry> {
    let mut entries = Vec::new();

    let file = match File::open("/proc/modules") {
        Ok(f) => f,
        Err(_) => return entries,
    };

    let reader = BufReader::new(file);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs());

    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            continue;
        }

        let name = parts[0].to_string();
        let size_bytes = parts[1].parse::<u64>().ok();

        let is_builtin = {
            let path = format!("/sys/module/{}/initstate", name);
            if Path::new(&path).exists() {
                Some(false) // dynamically loaded
            } else {
                Some(true) // likely built-in
            }
        };

        entries.push(FingerprintEntry::KernelModule {
            variant: "kernel_module".to_string(),
            value: name.clone(),
            category: "kernel_module".to_string(),
            tags: vec![
                "kernel".to_string(),
                "module".to_string(),
                if is_builtin.unwrap_or(false) {
                    "builtin".to_string()
                } else {
                    "dynamic".to_string()
                },
            ],
            source_module: "crawl_kernel_modules".to_string(),
            created_at: now,
            description: Some("Kernel module listed in /proc/modules".to_string()),
            size_bytes,
            permissions: None,
            hash: None,
        });
    }

    entries
}


pub fn crawl_all_advanced_and_write(output_path: &str) {
    let start = Instant::now();
    let mut entries: Vec<FingerprintEntry> = Vec::new();

    // Core behavioral/system baselines
    entries.extend(crawl_files());                     // /etc, /usr/bin/, /bin/, etc.
    entries.extend(crawl_users());                     // All local users and metadata
    entries.extend(crawl_jobs());                      // Cron, systemd timers, job schedulers
    entries.extend(crawl_usb_devices());               // USB serials, mount points
    entries.extend(crawl_network());                   // Open ports, ss, ip, netstat
    entries.extend(crawl_containers());                // Docker/LXC/OCI presence + FS paths
    entries.extend(crawl_scripts());                   // /tmp and dropper-like script baselines
    entries.extend(crawl_memory_regions());            // /proc/[pid]/maps & smaps for trusted binaries

    // Advanced system invariants
    entries.extend(crawl_known_cron_shells());         // Known shells used in job scheds
    entries.extend(crawl_binary_auth_chain());         // Setuid-root exec chains
    entries.extend(crawl_process_lineage_baseline());  // Known-good ancestry of system procs
    entries.extend(crawl_gpg_ssh_keys());              // ~/.ssh and ~/.gnupg validation
    entries.extend(crawl_kernel_modules());            // Loaded kernel modules
    println!("[DEBUG] Total fingerprint entries collected: {}", entries.len());

    for (i, entry) in entries.iter().enumerate() {
        println!("[DEBUG] Entry {}: {:?}", i, entry);
    }

    println!("[*] Number of fingerprint entries collected: {}", entries.len());


    match serde_json::to_string_pretty(&entries) {
        Ok(json) => {
            if let Err(e) = fs::write(output_path, json) {
                eprintln!("[!] Failed to write fingerprint file: {}", e);
            } else {
                println!("[+] Telemetry fingerprint written to: {}", output_path);
            }
        }
        Err(e) => {
            eprintln!("[!] Failed to serialize fingerprint entries: {}", e);
        }
    }

    println!("[*] Finished in {:?}", start.elapsed());
}