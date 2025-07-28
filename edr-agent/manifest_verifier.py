import os
import json
import hashlib
import sys
from pathlib import Path
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding

# === CONFIGURABLE PATHS ===
SIGNED_MANIFEST_PATH = "/opt/update/update_manifest.signed.json"
PUBLIC_KEY_PATH = "/opt/keys/public_manifest_key.pem"

# === PUBLIC KEY LOADING ===
def load_public_key():
    try:
        with open(PUBLIC_KEY_PATH, "rb") as f:
            return serialization.load_pem_public_key(f.read())
    except Exception as e:
        print(f"❌ Failed to load public key: {e}")
        sys.exit(1)

# === SIGNATURE VERIFICATION ===
def verify_manifest_signature(signed_manifest: dict) -> bool:
    try:
        signature = bytes.fromhex(signed_manifest["signature"])
        manifest_data = json.dumps(
            signed_manifest["manifest"],
            separators=(",", ":")
        ).encode("utf-8")

        public_key = load_public_key()
        public_key.verify(
            signature,
            manifest_data,
            padding.PSS(
                mgf=padding.MGF1(hashes.SHA256()),
                salt_length=padding.PSS.MAX_LENGTH
            ),
            hashes.SHA256()
        )
        return True
    except Exception as e:
        print(f"🚨 Signature verification failed: {e}")
        return False

# === MAIN MANIFEST LOADING FUNCTION ===
def load_verified_manifest() -> dict:
    if not os.path.exists(SIGNED_MANIFEST_PATH):
        print("❌ Signed manifest not found.")
        sys.exit(1)

    try:
        with open(SIGNED_MANIFEST_PATH, "r") as f:
            signed_manifest = json.load(f)
    except json.JSONDecodeError:
        print("❌ Manifest file is not valid JSON.")
        sys.exit(1)

    if "manifest" not in signed_manifest or "signature" not in signed_manifest:
        print("❌ Missing 'manifest' or 'signature' in signed file.")
        sys.exit(1)

    if not verify_manifest_signature(signed_manifest):
        print("❌ Invalid or tampered manifest.")
        sys.exit(1)

    print("✅ Manifest signature verified successfully.")
    return signed_manifest["manifest"]

# === OPTIONAL: CHECKSUM UTILITY ===
def get_manifest_checksum(manifest: dict) -> str:
    manifest_bytes = json.dumps(manifest, sort_keys=True).encode("utf-8")
    return hashlib.sha256(manifest_bytes).hexdigest()

# === OPTIONAL: CLI USAGE ===
if __name__ == "__main__":
    manifest = load_verified_manifest()
    print(f"🔐 Manifest Checksum: {get_manifest_checksum(manifest)}")
