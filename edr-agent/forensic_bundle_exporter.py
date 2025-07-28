import os
import json
import zipfile
import hashlib
import shutil
from datetime import datetime
from cryptography.hazmat.primitives import serialization, hashes
from cryptography.hazmat.primitives.asymmetric import ed25519

EXPORT_DIR = "forensic_exports/"
KEY_PATH = "keys/bundle_signing_private.pem"

def load_signing_key():
    with open(KEY_PATH, "rb") as f:
        return ed25519.Ed25519PrivateKey.from_private_bytes(f.read())

def compute_sha256(filepath):
    hasher = hashlib.sha256()
    with open(filepath, "rb") as f:
        hasher.update(f.read())
    return hasher.hexdigest()

def export_bundle(session_id, replay_path, shap_path, trust_path, ttl_days=90):
    os.makedirs(EXPORT_DIR, exist_ok=True)
    timestamp = datetime.utcnow().strftime("%Y%m%d_%H%M%S")
    bundle_name = f"{session_id}_forensic_bundle_{timestamp}.zip"
    bundle_path = os.path.join(EXPORT_DIR, bundle_name)

    files_to_include = {
        "replay": replay_path,
        "shap": shap_path,
        "trust": trust_path,
    }

    temp_dir = os.path.join(EXPORT_DIR, f"temp_{session_id}")
    os.makedirs(temp_dir, exist_ok=True)

    bundle_meta = {
        "session_id": session_id,
        "timestamp": timestamp,
        "ttl_days": ttl_days,
        "included_files": {},
    }

    for label, path in files_to_include.items():
        if os.path.exists(path):
            dst = os.path.join(temp_dir, os.path.basename(path))
            shutil.copy2(path, dst)
            bundle_meta["included_files"][label] = {
                "filename": os.path.basename(path),
                "sha256": compute_sha256(path),
            }

    # Save metadata
    meta_path = os.path.join(temp_dir, "bundle_metadata.json")
    with open(meta_path, "w") as f:
        json.dump(bundle_meta, f, indent=2)

    # Sign metadata
    signing_key = load_signing_key()
    signature = signing_key.sign(json.dumps(bundle_meta).encode())
    with open(os.path.join(temp_dir, "signature.sig"), "wb") as sig_file:
        sig_file.write(signature)

    # Create ZIP
    with zipfile.ZipFile(bundle_path, 'w', zipfile.ZIP_DEFLATED) as zipf:
        for fname in os.listdir(temp_dir):
            zipf.write(os.path.join(temp_dir, fname), fname)

    shutil.rmtree(temp_dir)
    return bundle_path

if __name__ == "__main__":
    # Test run (paths should be adjusted for real system)
    bundle = export_bundle(
        session_id="session_abc123",
        replay_path="replays/session_abc123.json",
        shap_path="shap_values/session_abc123.json",
        trust_path="trust_logs/session_abc123.json"
    )
    print(f"Forensic bundle saved at: {bundle}")
