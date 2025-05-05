// src/config.rs

use ed25519_dalek::{PublicKey, Signature, Verifier};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_512};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawPolicy {
    pub collection_interval: u64, // e.g., 5 = every 5 seconds
    pub endpoint_role: String,    // e.g., "workstation" or "server"
    pub mode: String,             // "minimal" or "forensic"
}

#[derive(Serialize, Deserialize, Debug)]
struct SignedPolicy {
    policy: RawPolicy,
    signature: String,
    pubkey: String,
}

pub fn load_and_verify_policy<P: AsRef<Path>>(path: P) -> anyhow::Result<RawPolicy> {
    // 1. Read and parse signed blob
    let blob = fs::read_to_string(&path)?;
    let signed: SignedPolicy = serde_json::from_str(&blob)?;

    // 2. Serialize policy for verification
    let payload = serde_json::to_vec(&signed.policy)?;

    // 3. Verify digital signature
    let pk_bytes = hex::decode(&signed.pubkey)?;
    let sig_bytes = hex::decode(&signed.signature)?;
    let pk = PublicKey::from_bytes(&pk_bytes)?;
    let sig = Signature::from_bytes(&sig_bytes)?;
    pk.verify(&payload, &sig)
        .map_err(|_| anyhow::anyhow!("❌ Invalid signature on policy"))?;

    // 4. Self-hash agent binary (optional integrity audit)
    let exe_path = std::env::current_exe()?;
    let binary = fs::read(&exe_path)?;
    let mut hasher = Sha3_512::new();
    hasher.update(&binary);
    let digest = hasher.finalize();
    println!("🔐 Agent SHA3-512 digest: {:x}", digest);
    // Optional: compare digest against whitelist of known-good hashes

    Ok(signed.policy)
}
