// src/config.rs
//
// Signed policy. policy.json is signed with Ed25519, and the signature sits next to
// it in policy.json.sig as hex. The signature covers the exact bytes of the file,
// so there is no canonical form to agree on: the bytes that were checked are the
// bytes that get parsed. scripts/policy_signer.py makes the signature with the
// openssl CLI, the same library this crate links.
//
// The agent watches the file and applies a new policy while it runs (reload.rs in
// src/shield, judge_reload here). Every policy carries a serial. A policy with a
// lower serial than the one running is refused, so an old signed policy that
// allowed less cannot be replayed. The highest serial applied and a copy of that
// policy are kept in the state directory, so a restart does not reopen the door.
//
// The public key comes from EDR_POLICY_PUBKEY, else config/policy_pub.pem. A key
// inside policy.json would prove nothing, since whoever edits the file could
// replace it, so the key is pinned outside the file.
//
// The policy holds the settings an attacker would want to weaken, and the agent
// enforces each one. Environment variables can make a setting stricter, never looser:
//   guard_mode             the lowest mode for every guard. EDR_GUARD_MODE and the per
//                          guard variables can raise it (monitor to enforce), not lower it
//   redact_command_lines   true means command line secrets are always masked, and
//                          EDR_REDACT=0 is refused with a warning
//   response_actions       which of kill_proc, block_egress and isolate_host the API runs
//   management_ports       TCP ports that stay reachable while the host is isolated (the
//                          agent API port is always added)
//   logging.level          the log level when RUST_LOG is not set
//   serial                 anti-rollback counter, see above
//   guard_allow            more executables for the runtime socket and memfd allowlists
//   shields                limits for runtime shields (src/shield): the longest lifetime,
//                          the most active shields, the never-block list, and which kinds
//                          may be promoted or applied without an operator
use anyhow::{bail, Context};
use openssl::pkey::{Id, PKey};
use openssl::sign::Verifier;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub const POLICY_PUBKEY_ENV: &str = "EDR_POLICY_PUBKEY";
pub const DEFAULT_POLICY_PUBKEY: &str = "config/policy_pub.pem";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RawPolicy {
    /// Policies older than serial support read as 0.
    #[serde(default)]
    pub serial: u64,
    pub guard_mode: String,
    #[serde(default)]
    pub guard_allow: GuardAllow,
    pub redact_command_lines: bool,
    pub response_actions: Vec<String>,
    pub management_ports: Vec<u16>,
    pub logging: LoggingConfig,
    #[serde(default)]
    pub shields: ShieldPolicy,
}

/// Executables the policy adds to the guard allowlists, by path. They are found
/// on disk and compared by inode, like the built in ones.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GuardAllow {
    #[serde(default)]
    pub socket_clients: Vec<String>,
    #[serde(default)]
    pub memfd_allow: Vec<String>,
}

/// The longest a shield may last, in seconds, when the policy does not say.
pub const DEFAULT_SHIELD_LIFETIME_SEC: u64 = 86_400;
/// The most active shields when the policy does not say.
pub const DEFAULT_MAX_SHIELDS: usize = 256;
/// Nothing a policy may ask for beyond this: 30 days, and the size of the maps.
pub const MAX_SHIELD_LIFETIME_SEC: u64 = 30 * 86_400;
pub const MAX_SHIELDS: usize = crate::shield::abi::STATS_SLOTS as usize - 1;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ShieldPolicy {
    #[serde(default = "yes")]
    pub enabled: bool,
    #[serde(default = "default_shield_lifetime")]
    pub max_lifetime_sec: u64,
    #[serde(default = "default_max_shields")]
    pub max_active: usize,
    #[serde(default)]
    pub never_block: NeverBlockPolicy,
    #[serde(default)]
    pub auto_promote: AutoPromote,
    #[serde(default)]
    pub auto_apply: AutoApply,
}

fn yes() -> bool {
    true
}
fn default_shield_lifetime() -> u64 {
    DEFAULT_SHIELD_LIFETIME_SEC
}
fn default_max_shields() -> usize {
    DEFAULT_MAX_SHIELDS
}
fn default_trial() -> u64 {
    600
}
fn default_auto_lifetime() -> u64 {
    3600
}

impl Default for ShieldPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            max_lifetime_sec: DEFAULT_SHIELD_LIFETIME_SEC,
            max_active: DEFAULT_MAX_SHIELDS,
            never_block: NeverBlockPolicy::default(),
            auto_promote: AutoPromote::default(),
            auto_apply: AutoApply::default(),
        }
    }
}

/// Added to the built in never-block list: executables (and files) no shield may
/// name or act on, and networks no connect_deny may cover.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NeverBlockPolicy {
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    pub cidrs: Vec<String>,
}

/// Kinds whose shields move from monitor to enforce by themselves after
/// trial_sec, when their document was signed for enforce.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AutoPromote {
    #[serde(default)]
    pub kinds: Vec<String>,
    #[serde(default = "default_trial")]
    pub trial_sec: u64,
}

impl Default for AutoPromote {
    fn default() -> Self {
        Self { kinds: Vec::new(), trial_sec: 600 }
    }
}

/// Kinds a playbook's proposal may apply by itself, unsigned, with at most this
/// lifetime. Empty means every proposal waits for a signed document.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AutoApply {
    #[serde(default)]
    pub kinds: Vec<String>,
    #[serde(default = "default_auto_lifetime")]
    pub max_lifetime_sec: u64,
}

impl Default for AutoApply {
    fn default() -> Self {
        Self { kinds: Vec::new(), max_lifetime_sec: 3600 }
    }
}

impl ShieldPolicy {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.max_lifetime_sec == 0 || self.max_lifetime_sec > MAX_SHIELD_LIFETIME_SEC {
            bail!("shields.max_lifetime_sec must be 1 to {MAX_SHIELD_LIFETIME_SEC}");
        }
        if self.max_active > MAX_SHIELDS {
            bail!("shields.max_active is limited to {MAX_SHIELDS}, the size of the shield maps");
        }
        for k in self.auto_promote.kinds.iter().chain(&self.auto_apply.kinds) {
            if !crate::pb_engine::SHIELD_KINDS.contains(&k.as_str()) {
                bail!("shields lists unknown kind `{k}`");
            }
        }
        if self.auto_apply.max_lifetime_sec == 0 || self.auto_apply.max_lifetime_sec > self.max_lifetime_sec {
            bail!("shields.auto_apply.max_lifetime_sec must be 1 to shields.max_lifetime_sec");
        }
        for c in &self.never_block.cidrs {
            crate::response::Net::parse(c).map_err(|e| anyhow::anyhow!("shields.never_block.cidrs: {e}"))?;
        }
        for p in &self.never_block.paths {
            if !p.starts_with('/') {
                bail!("shields.never_block.paths needs absolute paths, not `{p}`");
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LoggingConfig {
    pub level: String,
}

impl RawPolicy {
    /// Values the agent cannot act on are errors, so a typo never weakens a setting.
    pub fn validate(&self) -> anyhow::Result<()> {
        if crate::guard::GuardMode::parse(&self.guard_mode).is_none() {
            bail!("guard_mode `{}` is not off, monitor or enforce", self.guard_mode);
        }
        for a in &self.response_actions {
            if !crate::response::ACTIONS.contains(&a.as_str()) {
                bail!("response_actions has unknown action `{a}`");
            }
        }
        if self.management_ports.contains(&0) {
            bail!("management_ports has port 0");
        }
        if !matches!(self.logging.level.as_str(), "error" | "warn" | "info" | "debug" | "trace") {
            bail!("logging.level `{}` is not error, warn, info, debug or trace", self.logging.level);
        }
        for p in self.guard_allow.socket_clients.iter().chain(&self.guard_allow.memfd_allow) {
            if !p.starts_with('/') {
                bail!("guard_allow needs absolute paths, not `{p}`");
            }
        }
        self.shields.validate()?;
        Ok(())
    }

    pub fn guard_floor(&self) -> crate::guard::GuardMode {
        crate::guard::GuardMode::parse(&self.guard_mode).unwrap_or(crate::guard::GuardMode::Monitor)
    }
}

#[derive(Deserialize)]
struct PolicyFile {
    policy: RawPolicy,
}

/// policy.json -> policy.json.sig
pub fn signature_path(policy: &Path) -> PathBuf {
    let mut s = policy.as_os_str().to_owned();
    s.push(".sig");
    PathBuf::from(s)
}

/// Checks a detached Ed25519 signature (hex) over `policy_bytes` with a PEM public key.
pub fn verify_policy_signature(
    policy_bytes: &[u8],
    sig_hex: &str,
    pubkey_pem: &[u8],
) -> anyhow::Result<()> {
    let sig = hex::decode(sig_hex.trim()).context("signature is not hex")?;
    if sig.len() != 64 {
        bail!("signature is {} bytes, an Ed25519 signature is 64", sig.len());
    }
    let key = PKey::public_key_from_pem(pubkey_pem).context("public key is not a PEM public key")?;
    if key.id() != Id::ED25519 {
        bail!("public key is not an Ed25519 key");
    }
    // Ed25519 signs the message itself, so there is no digest to choose.
    let mut verifier = Verifier::new_without_digest(&key)?;
    if !verifier.verify_oneshot(&sig, policy_bytes).unwrap_or(false) {
        bail!("signature does not match the policy bytes (edited, or signed with another key)");
    }
    Ok(())
}

fn check_policy_file(policy_bytes: &[u8], policy: &Path, pubkey: &Path) -> anyhow::Result<()> {
    let sig_path = signature_path(policy);
    let sig = fs::read_to_string(&sig_path)
        .with_context(|| format!("no signature at {}", sig_path.display()))?;
    let pem = fs::read(pubkey).with_context(|| {
        format!("no public key at {} (set {POLICY_PUBKEY_ENV})", pubkey.display())
    })?;
    verify_policy_signature(policy_bytes, &sig, &pem)
}

/// Loads policy.json only if its signature checks out. With `allow_unsigned` (dev mode,
/// EDR_DEV_NO_AUTH=1) a bad or missing signature is a loud warning instead of an error.
pub fn load_policy(policy: &Path, pubkey: &Path, allow_unsigned: bool) -> anyhow::Result<RawPolicy> {
    let bytes = fs::read(policy).with_context(|| format!("cannot read {}", policy.display()))?;
    if let Err(e) = check_policy_file(&bytes, policy, pubkey) {
        if !allow_unsigned {
            return Err(e.context(format!("policy {} refused", policy.display())));
        }
        let msg = format!(
            "WARNING: loading an UNVERIFIED policy from {} because EDR_DEV_NO_AUTH=1: {e:#}",
            policy.display()
        );
        eprintln!("{msg}");
        log::warn!("{msg}");
    }
    let file: PolicyFile = serde_json::from_slice(&bytes)
        .with_context(|| format!("{} is not a valid policy", policy.display()))?;
    file.policy
        .validate()
        .with_context(|| format!("{} is not a valid policy", policy.display()))?;
    Ok(file.policy)
}

/// The agent's entry point: key from EDR_POLICY_PUBKEY or the default path.
pub fn load_and_verify_policy<P: AsRef<Path>>(path: P) -> anyhow::Result<RawPolicy> {
    load_policy(path.as_ref(), &pubkey_path(), crate::agent_api::dev_no_auth())
}

pub fn pubkey_path() -> PathBuf {
    PathBuf::from(std::env::var(POLICY_PUBKEY_ENV).unwrap_or_else(|_| DEFAULT_POLICY_PUBKEY.to_string()))
}

/// Parses and checks policy bytes that were already verified (or allowed unsigned).
pub fn parse_policy_bytes(bytes: &[u8]) -> anyhow::Result<RawPolicy> {
    let file: PolicyFile = serde_json::from_slice(bytes).context("not a valid policy")?;
    file.policy.validate().context("not a valid policy")?;
    Ok(file.policy)
}

// ---- Anti-rollback ----

pub const STATE_DIR_ENV: &str = "EDR_STATE_DIR";

/// state/ by default, next to the agent's other files.
pub fn state_dir() -> PathBuf {
    PathBuf::from(std::env::var(STATE_DIR_ENV).unwrap_or_else(|_| "state".to_string()))
}

/// A policy the agent checked: the parsed settings and the exact signed bytes.
#[derive(Clone, Debug)]
pub struct LoadedPolicy {
    pub policy: RawPolicy,
    pub bytes: Vec<u8>,
    pub sig: String,
    /// Where it came from: policy.json, or the copy kept in the state directory.
    pub source: PathBuf,
}

/// The highest policy serial applied, and a copy of that policy, in the state
/// directory. The guard write protects these files while the agent runs.
pub struct PolicyStore {
    dir: PathBuf,
}

impl PolicyStore {
    pub fn new(dir: &Path) -> Self {
        Self { dir: dir.to_path_buf() }
    }

    pub fn serial_path(&self) -> PathBuf {
        self.dir.join("policy_serial")
    }

    pub fn copy_path(&self) -> PathBuf {
        self.dir.join("policy_last.json")
    }

    /// 0 when nothing was recorded yet.
    pub fn floor(&self) -> u64 {
        fs::read_to_string(self.serial_path())
            .ok()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0)
    }

    /// The kept copy, if it is there.
    pub fn last(&self) -> Option<(Vec<u8>, String)> {
        let bytes = fs::read(self.copy_path()).ok()?;
        let sig = fs::read_to_string(signature_path(&self.copy_path())).ok()?;
        Some((bytes, sig))
    }

    /// Writes the floor and the copy in place, so each file keeps its inode and
    /// with it the guard's write protection.
    pub fn record(&self, p: &LoadedPolicy) -> std::io::Result<()> {
        fs::create_dir_all(&self.dir)?;
        write_in_place(&self.copy_path(), &p.bytes)?;
        write_in_place(&signature_path(&self.copy_path()), p.sig.trim().as_bytes())?;
        write_in_place(&self.serial_path(), format!("{}\n", p.policy.serial).as_bytes())
    }

    /// The files whose inodes the guard protects.
    pub fn files(&self) -> Vec<PathBuf> {
        vec![self.serial_path(), self.copy_path(), signature_path(&self.copy_path())]
    }
}

/// Truncates and writes a file without replacing it, owner only, never through a
/// symlink.
pub fn write_in_place(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)?;
    f.write_all(bytes)?;
    f.sync_data()
}

fn signed(bytes: &[u8], sig: &str, pem: &[u8], allow_unsigned: bool, what: &str) -> anyhow::Result<RawPolicy> {
    if let Err(e) = verify_policy_signature(bytes, sig, pem) {
        if !allow_unsigned {
            return Err(e.context(format!("{what} refused")));
        }
        let msg = format!("WARNING: using an UNVERIFIED policy from {what} because EDR_DEV_NO_AUTH=1: {e:#}");
        eprintln!("{msg}");
        log::warn!("{msg}");
    }
    parse_policy_bytes(bytes).with_context(|| format!("{what} is not a valid policy"))
}

/// Startup with anti-rollback. policy.json is used when it verifies and its
/// serial is at least the recorded floor. Otherwise the copy kept at the last
/// apply is used, if it verifies and is at the floor. A rolled back policy.json
/// therefore cannot lower the settings across a restart.
pub fn load_with_floor(
    policy: &Path,
    pubkey: &Path,
    store: &PolicyStore,
    allow_unsigned: bool,
) -> anyhow::Result<(LoadedPolicy, Vec<String>)> {
    let pem = fs::read(pubkey)
        .with_context(|| format!("no public key at {} (set {POLICY_PUBKEY_ENV})", pubkey.display()))?;
    let floor = store.floor();
    let mut notes = Vec::new();
    let from_file = (|| -> anyhow::Result<LoadedPolicy> {
        let bytes = fs::read(policy).with_context(|| format!("cannot read {}", policy.display()))?;
        let sig_path = signature_path(policy);
        let sig = fs::read_to_string(&sig_path).unwrap_or_default();
        let p = signed(&bytes, &sig, &pem, allow_unsigned, &policy.display().to_string())?;
        if p.serial < floor {
            bail!(
                "{} has serial {}, lower than the last applied serial {floor}: rollback refused",
                policy.display(),
                p.serial
            );
        }
        Ok(LoadedPolicy { policy: p, bytes, sig, source: policy.to_path_buf() })
    })();
    let err = match from_file {
        Ok(p) => return Ok((p, notes)),
        Err(e) => e,
    };
    notes.push(format!("{err:#}"));
    let Some((bytes, sig)) = store.last() else {
        return Err(err);
    };
    let what = store.copy_path().display().to_string();
    let p = signed(&bytes, &sig, &pem, allow_unsigned, &what)?;
    if p.serial < floor {
        bail!("{what} has serial {} under the floor {floor}", p.serial);
    }
    notes.push(format!("using the kept copy {what} (serial {})", p.serial));
    Ok((LoadedPolicy { policy: p, bytes, sig, source: store.copy_path() }, notes))
}

/// What a changed policy file means for the running agent.
#[derive(Debug)]
pub enum Reload {
    /// The same bytes as the running policy.
    Unchanged,
    Apply(Box<LoadedPolicy>),
    Refused(String),
}

/// Decides about new policy bytes while the agent runs: the signature must
/// verify, the policy must be valid, and its serial must be above the running
/// one. The same serial with other bytes is refused too, since two policies
/// with one serial would make rollback ambiguous.
pub fn judge_reload(
    running: &LoadedPolicy,
    bytes: &[u8],
    sig: &str,
    pem: &[u8],
    allow_unsigned: bool,
    source: &Path,
) -> Reload {
    if bytes == running.bytes.as_slice() {
        return Reload::Unchanged;
    }
    let p = match signed(bytes, sig, pem, allow_unsigned, &source.display().to_string()) {
        Ok(p) => p,
        Err(e) => return Reload::Refused(format!("{e:#}")),
    };
    if p.serial < running.policy.serial {
        return Reload::Refused(format!(
            "serial {} is lower than the running serial {}: rollback refused",
            p.serial, running.policy.serial
        ));
    }
    if p.serial == running.policy.serial {
        return Reload::Refused(format!(
            "serial {} is the running serial with different content: give a new policy a higher serial",
            p.serial
        ));
    }
    Reload::Apply(Box::new(LoadedPolicy {
        policy: p,
        bytes: bytes.to_vec(),
        sig: sig.to_string(),
        source: source.to_path_buf(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use openssl::pkey::Private;
    use openssl::sign::Signer;
    use std::os::unix::fs::MetadataExt;

    const POLICY: &[u8] = br#"{"policy":{"guard_mode":"monitor","redact_command_lines":true,"response_actions":["kill_proc"],"management_ports":[22,60],"logging":{"level":"info"}}}"#;

    fn keypair() -> (PKey<Private>, Vec<u8>) {
        let key = PKey::generate_ed25519().unwrap();
        let pem = key.public_key_to_pem().unwrap();
        (key, pem)
    }

    fn sign(key: &PKey<Private>, bytes: &[u8]) -> String {
        let mut signer = Signer::new_without_digest(key).unwrap();
        hex::encode(signer.sign_oneshot_to_vec(bytes).unwrap())
    }

    #[test]
    fn good_signature_verifies() {
        let (key, pem) = keypair();
        verify_policy_signature(POLICY, &sign(&key, POLICY), &pem).unwrap();
        // A trailing newline in the .sig file is fine.
        verify_policy_signature(POLICY, &format!("{}\n", sign(&key, POLICY)), &pem).unwrap();
    }

    #[test]
    fn one_changed_byte_fails() {
        let (key, pem) = keypair();
        let sig = sign(&key, POLICY);
        let mut tampered = POLICY.to_vec();
        let i = tampered.windows(2).position(|w| w == b"60").unwrap();
        tampered[i] = b'9'; // management port 60 -> 90
        assert!(verify_policy_signature(&tampered, &sig, &pem).is_err());
        // A flipped signature bit fails too.
        let mut bad = hex::decode(&sig).unwrap();
        bad[10] ^= 1;
        assert!(verify_policy_signature(POLICY, &hex::encode(bad), &pem).is_err());
    }

    #[test]
    fn wrong_key_fails() {
        let (key, _) = keypair();
        let (_, other_pem) = keypair();
        assert!(verify_policy_signature(POLICY, &sign(&key, POLICY), &other_pem).is_err());
        // An RSA key is refused outright.
        let rsa = PKey::from_rsa(openssl::rsa::Rsa::generate(2048).unwrap()).unwrap();
        let rsa_pem = rsa.public_key_to_pem().unwrap();
        let err = verify_policy_signature(POLICY, &sign(&key, POLICY), &rsa_pem).unwrap_err();
        assert!(err.to_string().contains("not an Ed25519"), "{err}");
    }

    #[test]
    fn missing_or_placeholder_signature_fails() {
        let (_, pem) = keypair();
        assert!(verify_policy_signature(POLICY, "", &pem).is_err());
        // The old shipped value: 32 bytes of deadbeef, not a signature.
        let placeholder = "deadbeef".repeat(8);
        assert!(verify_policy_signature(POLICY, &placeholder, &pem).is_err());
        assert!(verify_policy_signature(POLICY, &"00".repeat(64), &pem).is_err());
        assert!(verify_policy_signature(POLICY, "not hex", &pem).is_err());
    }

    fn temp_dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("policy_{tag}_{}", std::process::id()));
        fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn load_refuses_unless_dev_mode() {
        let (key, pem) = keypair();
        let d = temp_dir("load");
        let policy = d.join("policy.json");
        let pubkey = d.join("policy_pub.pem");
        fs::write(&policy, POLICY).unwrap();
        fs::write(&pubkey, &pem).unwrap();

        // No .sig file: refused, and the error names the missing file.
        let err = load_policy(&policy, &pubkey, false).unwrap_err();
        assert!(format!("{err:#}").contains("no signature at"), "{err:#}");
        // Dev mode loads it anyway, with a warning.
        assert_eq!(load_policy(&policy, &pubkey, true).unwrap().management_ports, vec![22, 60]);

        fs::write(signature_path(&policy), sign(&key, POLICY)).unwrap();
        assert_eq!(load_policy(&policy, &pubkey, false).unwrap().guard_mode, "monitor");

        // Edit the file after signing: refused. Lowering the guard is the edit an
        // attacker would make.
        let edited = String::from_utf8(POLICY.to_vec())
            .unwrap()
            .replace("\"monitor\"", "\"off\"");
        fs::write(&policy, edited).unwrap();
        assert!(load_policy(&policy, &pubkey, false).is_err());

        // Missing public key: refused.
        assert!(load_policy(&policy, &d.join("nope.pem"), false).is_err());
        let _ = fs::remove_dir_all(&d);
    }

    // The shipped policy.json must verify against the shipped public key, so an edit
    // made without re-signing fails here before it fails at startup.
    #[test]
    fn shipped_policy_is_signed_with_the_shipped_key() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let p = load_policy(
            &root.join("policy.json"),
            &root.join(DEFAULT_POLICY_PUBKEY),
            false,
        )
        .unwrap();
        assert!(p.redact_command_lines);
        assert_eq!(p.response_actions, vec!["kill_proc", "block_egress", "isolate_host"]);
        // Serial 2: serial 0 (no field) was the policy before runtime shields.
        assert_eq!(p.serial, 2);
        assert_eq!((p.shields.max_active, p.shields.max_lifetime_sec), (256, 86_400));
        // Nothing applies or promotes itself unless the operator signs a policy that says so.
        assert!(p.shields.auto_apply.kinds.is_empty() && p.shields.auto_promote.kinds.is_empty());
    }

    fn signed_policy(key: &PKey<Private>, serial: u64, guard: &str) -> (Vec<u8>, String) {
        let text = format!(
            r#"{{"policy":{{"serial":{serial},"guard_mode":"{guard}","redact_command_lines":true,"response_actions":["kill_proc"],"management_ports":[22],"logging":{{"level":"info"}}}}}}"#
        );
        let sig = sign(key, text.as_bytes());
        (text.into_bytes(), sig)
    }

    #[test]
    fn reload_needs_a_valid_signature_and_a_higher_serial() {
        let (key, pem) = keypair();
        let (b2, s2) = signed_policy(&key, 2, "enforce");
        let running = LoadedPolicy {
            policy: parse_policy_bytes(&b2).unwrap(),
            bytes: b2.clone(),
            sig: s2.clone(),
            source: PathBuf::from("policy.json"),
        };
        let src = Path::new("policy.json");
        assert!(matches!(judge_reload(&running, &b2, &s2, &pem, false, src), Reload::Unchanged));
        // Rollback to serial 1, signed with the right key, is refused.
        let (b1, s1) = signed_policy(&key, 1, "off");
        match judge_reload(&running, &b1, &s1, &pem, false, src) {
            Reload::Refused(why) => assert!(why.contains("rollback refused"), "{why}"),
            other => panic!("{other:?}"),
        }
        // The same serial with other bytes is refused.
        let (b2b, s2b) = signed_policy(&key, 2, "monitor");
        assert!(matches!(judge_reload(&running, &b2b, &s2b, &pem, false, src), Reload::Refused(_)));
        // A forged signature is refused, even with a higher serial.
        let (b3, _) = signed_policy(&key, 3, "off");
        let (other, _) = keypair();
        let forged = sign(&other, &b3);
        match judge_reload(&running, &b3, &forged, &pem, false, src) {
            Reload::Refused(why) => assert!(why.contains("signature"), "{why}"),
            other => panic!("{other:?}"),
        }
        // A higher serial with a good signature applies.
        let (b3, s3) = signed_policy(&key, 3, "monitor");
        match judge_reload(&running, &b3, &s3, &pem, false, src) {
            Reload::Apply(p) => assert_eq!((p.policy.serial, p.policy.guard_mode.as_str()), (3, "monitor")),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn the_floor_survives_a_restart_and_the_kept_copy_wins_over_a_rolled_back_file() {
        let (key, pem) = keypair();
        let d = temp_dir("floor");
        let pubkey = d.join("policy_pub.pem");
        fs::write(&pubkey, &pem).unwrap();
        let policy = d.join("policy.json");
        let store = PolicyStore::new(&d.join("state"));
        assert_eq!(store.floor(), 0);

        let (b2, s2) = signed_policy(&key, 2, "enforce");
        fs::write(&policy, &b2).unwrap();
        fs::write(signature_path(&policy), &s2).unwrap();
        let (p, notes) = load_with_floor(&policy, &pubkey, &store, false).unwrap();
        assert_eq!(p.policy.serial, 2);
        assert!(notes.is_empty());
        store.record(&p).unwrap();
        assert_eq!(store.floor(), 2);
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(fs::metadata(store.serial_path()).unwrap().permissions().mode() & 0o777, 0o600);
        let ino = fs::metadata(store.serial_path()).unwrap().ino();

        // Put the older signed policy back and "restart": the kept copy is used.
        let (b1, s1) = signed_policy(&key, 1, "off");
        fs::write(&policy, &b1).unwrap();
        fs::write(signature_path(&policy), &s1).unwrap();
        let (p, notes) = load_with_floor(&policy, &pubkey, &store, false).unwrap();
        assert_eq!((p.policy.serial, p.policy.guard_mode.as_str()), (2, "enforce"));
        assert!(notes[0].contains("rollback refused"), "{notes:?}");
        assert_eq!(p.source, store.copy_path());

        // Recording again keeps the inode, so the guard's protection stays on it.
        store.record(&p).unwrap();
        assert_eq!(fs::metadata(store.serial_path()).unwrap().ino(), ino);

        // With the copy gone too, the agent refuses to start on the old policy.
        fs::remove_file(store.copy_path()).unwrap();
        assert!(load_with_floor(&policy, &pubkey, &store, false).is_err());
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn shield_settings_are_checked() {
        let base = String::from_utf8(POLICY.to_vec()).unwrap();
        let with = |shields: &str| base.replace("\"logging\"", &format!("\"shields\":{shields},\"logging\""));
        assert!(parse_policy_bytes(with("{}").as_bytes()).is_ok());
        let p = parse_policy_bytes(with(r#"{"max_active":100,"auto_apply":{"kinds":["connect_deny"],"max_lifetime_sec":600}}"#).as_bytes()).unwrap();
        assert_eq!((p.shields.max_active, p.shields.auto_apply.max_lifetime_sec), (100, 600));
        assert_eq!(p.shields.max_lifetime_sec, DEFAULT_SHIELD_LIFETIME_SEC);
        for bad in [
            r#"{"max_lifetime_sec":0}"#,
            r#"{"max_lifetime_sec":99999999}"#,
            r#"{"max_active":999999}"#,
            r#"{"auto_promote":{"kinds":["wipe_deny"]}}"#,
            r#"{"auto_apply":{"kinds":["exec_deny"],"max_lifetime_sec":172800}}"#,
            r#"{"never_block":{"cidrs":["10.0.0.0/99"]}}"#,
            r#"{"never_block":{"paths":["relative/path"]}}"#,
            r#"{"maximum":3}"#,
        ] {
            assert!(parse_policy_bytes(with(bad).as_bytes()).is_err(), "{bad}");
        }
        // No serial reads as 0, the value of every policy signed before serials.
        assert_eq!(parse_policy_bytes(POLICY).unwrap().serial, 0);
    }

    #[test]
    fn a_signed_policy_with_values_the_agent_cannot_use_is_refused() {
        let (key, pem) = keypair();
        let d = temp_dir("values");
        let pubkey = d.join("policy_pub.pem");
        fs::write(&pubkey, &pem).unwrap();
        let policy = d.join("policy.json");
        for (bad, want) in [
            ("\"guard_mode\":\"monitor\"", "\"guard_mode\":\"enforcing\""),
            ("\"response_actions\":[\"kill_proc\"]", "\"response_actions\":[\"wipe_disk\"]"),
            ("\"level\":\"info\"", "\"level\":\"loud\""),
            ("\"logging\"", "\"collection_interval\":60,\"logging\""),
        ] {
            let text = String::from_utf8(POLICY.to_vec()).unwrap().replace(bad, want);
            fs::write(&policy, &text).unwrap();
            fs::write(signature_path(&policy), sign(&key, text.as_bytes())).unwrap();
            let err = load_policy(&policy, &pubkey, false).unwrap_err();
            assert!(format!("{err:#}").contains("not a valid policy"), "{want}: {err:#}");
        }
        let _ = fs::remove_dir_all(&d);
    }
}
