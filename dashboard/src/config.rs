// src/config.rs

use ed25519_dalek::{PublicKey, Signature, Verifier};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_512};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawPolicy {
    pub foo: String,
    pub bar: u32,
    // … your real policy fields …
}

#[derive(Serialize, Deserialize, Debug)]
struct SignedPolicy {
    policy: RawPolicy,
    signature: String,
    pubkey: String,
}

pub fn load_and_verify_policy<P: AsRef<Path>>(path: P) -> anyhow::Result<RawPolicy> {
    // 1. Read file
    let blob = fs::read_to_string(&path)?;
    let signed: SignedPolicy = serde_json::from_str(&blob)?;

    // 2. Recreate payload bytes (canonical JSON)
    let payload = serde_json::to_vec(&signed.policy)?;

    // 3. Verify signature
    let pk_bytes = hex::decode(signed.pubkey)?;
    let sig_bytes = hex::decode(signed.signature)?;
    let pk = PublicKey::from_bytes(&pk_bytes)?;
    let sig = Signature::from_bytes(&sig_bytes)?;
    pk.verify(&payload, &sig)
        .map_err(|_| anyhow::anyhow!("Policy signature invalid"))?;

    // 4. Tamper check: hash this binary on-disk
    let exe = std::env::current_exe()?;
    let bin = fs::read(&exe)?;
    let mut hasher = Sha3_512::new();
    hasher.update(&bin);
    let digest = hasher.finalize();
    // in real use you'd compare this digest to an approved baseline:
    println!("Agent binary SHA3-512: {:x}", digest);

    Ok(signed.policy)
}
