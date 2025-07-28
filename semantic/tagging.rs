use std::collections::HashMap;

/// Contextual type of input to tag
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagContext {
    Syscall,
    Cmdline,
    FilePath,
    EnvVar,
    Unknown,
}

/// Main tagging function: takes input + context → returns tags
pub fn tag_from_context(input: &str, context: TagContext) -> Vec<String> {
    match context {
        TagContext::Syscall => tag_syscall(input),
        TagContext::Cmdline => tag_cmdline(input),
        TagContext::FilePath => tag_filepath(input),
        TagContext::EnvVar => tag_env_var(input),
        TagContext::Unknown => vec![],
    }
}

/// Syscall → tags
fn tag_syscall(syscall: &str) -> Vec<String> {
    let mut tags = vec![];
    match syscall {
        "ptrace" | "process_vm_readv" => tags.push("evasion::anti_debug".to_string()),
        "mprotect" | "mmap" => tags.push("execution::memory_manipulation".to_string()),
        "execve" => tags.push("execution::process_spawn".to_string()),
        _ => {}
    }
    tags
}

/// Cmdline → tags
fn tag_cmdline(cmdline: &str) -> Vec<String> {
    let mut tags = vec![];
    if cmdline.contains("whoami") {
        tags.push("privilege::enumeration".to_string());
    }
    if cmdline.contains("curl") && cmdline.contains("http") {
        tags.push("exfiltration::remote_fetch".to_string());
    }
    if cmdline.contains("nc") || cmdline.contains("netcat") {
        tags.push("network::reverse_shell".to_string());
    }
    tags
}

/// File path → tags
fn tag_filepath(path: &str) -> Vec<String> {
    let mut tags = vec![];
    if path.contains("/tmp/") && path.ends_with(".so") {
        tags.push("evasion::shared_object_inject".to_string());
    }
    if path.contains("/usr/bin/nc") {
        tags.push("network::backdoor_tool".to_string());
    }
    tags
}

/// Env var → tags
fn tag_env_var(env: &str) -> Vec<String> {
    let mut tags = vec![];
    if env.contains("LD_PRELOAD") {
        tags.push("evasion::ld_preload".to_string());
    }
    tags
}
