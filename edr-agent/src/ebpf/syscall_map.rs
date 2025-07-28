use std::collections::HashMap;

/// Returns a syscall name for a given syscall ID and platform.
/// Supports "linux" and "macos" explicitly. Falls back to "unknown_syscall_{id}".
pub fn get_syscall_name(syscall_id: u32, platform: &str) -> String {
    let name = match platform {
        "linux" => linux_syscall_map().get(&syscall_id).cloned(),
        "macos" => macos_syscall_map().get(&syscall_id).cloned(),
        _ => None,
    };

    name.unwrap_or_else(|| format!("unknown_syscall_{}", syscall_id))
}

fn linux_syscall_map() -> HashMap<u32, &'static str> {
    let mut map = HashMap::new();
    map.insert(0, "read");
    map.insert(1, "write");
    map.insert(2, "open");
    map.insert(3, "close");
    map.insert(57, "fork");
    map.insert(59, "execve");
    map.insert(60, "exit");
    map.insert(61, "wait4");
    map.insert(62, "kill");
    map.insert(63, "uname");
    map.insert(64, "semget");
    map.insert(72, "fcntl");
    map.insert(78, "gettimeofday");
    map.insert(79, "getrlimit");
    map.insert(87, "unlink");
    map.insert(89, "readlink");
    map.insert(102, "socketcall");
    map.insert(104, "setitimer");
    map.insert(105, "getitimer");
    map.insert(110, "waitid");
    map.insert(120, "clone");
    map.insert(137, "statfs");
    map.insert(138, "fstatfs");
    map.insert(174, "msync");
    map.insert(202, "ftruncate");
    map.insert(203, "fchmod");
    map.insert(257, "openat");
    map.insert(258, "mkdirat");
    map.insert(263, "unlinkat");
    map.insert(288, "pkey_alloc");
    map
}

fn macos_syscall_map() -> HashMap<u32, &'static str> {
    let mut map = HashMap::new();
    map.insert(0, "indir");
    map.insert(1, "exit");
    map.insert(2, "fork");
    map.insert(3, "read");
    map.insert(4, "write");
    map.insert(5, "open");
    map.insert(6, "close");
    map.insert(20, "getpid");
    map.insert(23, "setuid");
    map.insert(25, "getuid");
    map.insert(33, "access");
    map.insert(45, "brk");
    map.insert(73, "munmap");
    map.insert(85, "open_dprotected_np");
    map.insert(93, "fstat64");
    map.insert(106, "recvmsg");
    map.insert(120, "clone");
    map.insert(133, "mkdir");
    map.insert(134, "rmdir");
    map.insert(202, "lstat64");
    map.insert(220, "sendto");
    map.insert(221, "shutdown");
    map.insert(222, "socketpair");
    map.insert(233, "stat64");
    map.insert(286, "kqueue");
    map.insert(296, "lchown");
    map.insert(329, "kevent_qos");
    map
}
