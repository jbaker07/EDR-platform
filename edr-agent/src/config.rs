// src/config.rs
//
// Signed policy. policy.json is signed with Ed25519, and the signature sits next to
// it in policy.json.sig as hex. The signature covers the exact bytes of the file,
// so there is no canonical form to agree on: the bytes that were checked are the
// bytes that get parsed, read once. scripts/policy_signer.py makes the signature
// with the openssl CLI, the same library this crate links.
//
// The public key comes from EDR_POLICY_PUBKEY, else config/policy_pub.pem. A key
// inside policy.json would prove nothing, since whoever edits the file could
// replace it, so the key is pinned outside the file.
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RawPolicy {
    pub collection_interval: u64,
    pub endpoint_role: String,
    pub mode: String,
    pub trust_threshold: f32,
    pub gnn_enabled: bool,
    pub logging: LoggingConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingConfig {
    pub level: String,
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
    Ok(file.policy)
}

/// The agent's entry point: key from EDR_POLICY_PUBKEY or the default path.
pub fn load_and_verify_policy<P: AsRef<Path>>(path: P) -> anyhow::Result<RawPolicy> {
    let pubkey =
        std::env::var(POLICY_PUBKEY_ENV).unwrap_or_else(|_| DEFAULT_POLICY_PUBKEY.to_string());
    load_policy(path.as_ref(), Path::new(&pubkey), crate::agent_api::dev_no_auth())
}

#[cfg(test)]
mod tests {
    use super::*;
    use openssl::pkey::Private;
    use openssl::sign::Signer;

    const POLICY: &[u8] = br#"{"policy":{"collection_interval":60,"endpoint_role":"server","mode":"forensic","trust_threshold":0.7,"gnn_enabled":true,"logging":{"level":"info"}}}"#;

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
        tampered[i] = b'9'; // collection_interval 60 -> 90
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
        assert_eq!(load_policy(&policy, &pubkey, true).unwrap().collection_interval, 60);

        fs::write(signature_path(&policy), sign(&key, POLICY)).unwrap();
        assert_eq!(load_policy(&policy, &pubkey, false).unwrap().endpoint_role, "server");

        // Edit the file after signing: refused.
        let edited = String::from_utf8(POLICY.to_vec())
            .unwrap()
            .replace("forensic", "passive");
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
        assert_eq!(p.mode, "forensic");
    }
}
