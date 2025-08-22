import os
import json
import shutil
import gzip
import hashlib
import time
from datetime import datetime
from pathlib import Path
from cryptography.fernet import Fernet

# Vault configuration
EXPORT_DIR = "vault_exports"
SNAPSHOT_SOURCE = "telemetry_snapshots/"
SIGNING_KEY_PATH = "config/vault_signing.key"
TTL_DAYS = 30

def generate_filename(session_id: str, ts: int) -> str:
    return f"replay_{session_id}_{ts}.vault.gz"

def hash_file(filepath: str) -> str:
    sha256 = hashlib.sha256()
    with open(filepath, "rb") as f:
        for block in iter(lambda: f.read(65536), b""):
            sha256.update(block)
    return sha256.hexdigest()

def encrypt_file(filepath: str, key: bytes) -> str:
    fernet = Fernet(key)
    with open(filepath, "rb") as f:
        encrypted = fernet.encrypt(f.read())
    encrypted_path = filepath + ".enc"
    with open(encrypted_path, "wb") as ef:
        ef.write(encrypted)
    return encrypted_path

def load_signing_key() -> bytes:
    if not os.path.exists(SIGNING_KEY_PATH):
        key = Fernet.generate_key()
        with open(SIGNING_KEY_PATH, "wb") as f:
            f.write(key)
        return key
    with open(SIGNING_KEY_PATH, "rb") as f:
        return f.read()

def bundle_and_export(session_id: str, metadata: dict):
    timestamp = int(time.time())
    os.makedirs(EXPORT_DIR, exist_ok=True)

    snapshot_path = os.path.join(SNAPSHOT_SOURCE, f"{session_id}.json")
    if not os.path.exists(snapshot_path):
        print(f"[VaultExporter] No snapshot found for {session_id}")
        return

    # Compress snapshot
    gz_path = os.path.join(EXPORT_DIR, generate_filename(session_id, timestamp))
    with open(snapshot_path, "rb") as f_in, gzip.open(gz_path, "wb") as f_out:
        shutil.copyfileobj(f_in, f_out)

    # Encrypt and hash
    key = load_signing_key()
    enc_path = encrypt_file(gz_path, key)
    digest = hash_file(enc_path)

    # Write export manifest
    manifest = {
        "session_id": session_id,
        "timestamp": timestamp,
        "hash": digest,
        "metadata": metadata,
        "encrypted_file": os.path.basename(enc_path),
    }

    manifest_path = os.path.join(EXPORT_DIR, f"{session_id}_{timestamp}.manifest.json")
    with open(manifest_path, "w") as f:
        json.dump(manifest, f, indent=2)

    print(f"[VaultExporter] Vault export completed for session {session_id}")

def expire_old_exports():
    cutoff = time.time() - TTL_DAYS * 86400
    for file in Path(EXPORT_DIR).glob("*.manifest.json"):
        ts = int(file.name.split("_")[-1].split(".")[0])
        if ts < cutoff:
            enc_file = file.with_suffix(".vault.gz.enc")
            print(f"[VaultExporter] Expiring {file.name} and {enc_file.name}")
            file.unlink(missing_ok=True)
            enc_file.unlink(missing_ok=True)
