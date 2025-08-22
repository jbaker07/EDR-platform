import os
import json
import hashlib
import hmac
from pathlib import Path
from datetime import datetime

# Configuration
REPLAY_DIR = "/mnt/usb/replays"
VALID_EXT = ".replay.json"
OUTPUT_FILE = "checksums.sha256"
LOG_FILE = "checksum_audit.log"
SECRET_KEY = b"optional_hmac_key"  # Replace or load securely
ENABLE_HMAC = True
ENABLE_AUDIT_LOG = True
DRY_RUN = False

def calculate_sha256(filepath):
    """Returns SHA-256 hash of a file with normalized line endings."""
    h = hashlib.sha256()
    with open(filepath, "rb") as f:
        for chunk in iter(lambda: f.read(4096), b""):
            chunk = chunk.replace(b'\r\n', b'\n').replace(b'\r', b'\n')
            h.update(chunk)
    return h.hexdigest()

def validate_replay_format(filepath):
    """Ensures replay contains a minimally valid structure."""
    try:
        with open(filepath, "r", encoding="utf-8") as f:
            data = json.load(f)
        return isinstance(data, dict) and "session_id" in data and "events" in data
    except Exception:
        return False

def hmac_sign(hash_val, filename):
    """Signs hash with optional HMAC for integrity."""
    msg = f"{hash_val} {filename}".encode("utf-8")
    return hmac.new(SECRET_KEY, msg, hashlib.sha256).hexdigest()

def log_event(message):
    if ENABLE_AUDIT_LOG:
        with open(os.path.join(REPLAY_DIR, LOG_FILE), "a") as log:
            log.write(f"{datetime.utcnow().isoformat()} UTC - {message}\n")

def generate_checksums(directory):
    seen_hashes = set()
    checksum_lines = []

    replay_files = list(Path(directory).glob(f"*{VALID_EXT}"))
    if not replay_files:
        print("[!] No replay files found.")
        return

    for file in replay_files:
        if not validate_replay_format(file):
            log_event(f"⚠️ Skipped invalid replay format: {file.name}")
            continue

        hash_val = calculate_sha256(file)
        if hash_val in seen_hashes:
            log_event(f"⚠️ Duplicate replay detected: {file.name}")
            continue

        seen_hashes.add(hash_val)
        line = f"{hash_val} {file.name}"
        if ENABLE_HMAC:
            sig = hmac_sign(hash_val, file.name)
            line += f" {sig}"

        checksum_lines.append(line)
        log_event(f"[✓] Processed: {file.name}")

    if not DRY_RUN:
        with open(os.path.join(directory, OUTPUT_FILE), "w") as f:
            f.write("\n".join(checksum_lines))

    print(f"[✓] {len(checksum_lines)} replay checksums written to {OUTPUT_FILE}")

if __name__ == "__main__":
    generate_checksums(REPLAY_DIR)
