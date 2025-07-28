import os
import zipfile
import json
import time
import hashlib
import shutil
from datetime import datetime, timedelta
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler
from cryptography.hazmat.primitives.asymmetric import ed25519
from cryptography.exceptions import InvalidSignature

EXPORT_DIR = "forensic_exports/"
IMPORT_DIR = "airgap_imports/"
PROCESSED_DIR = "usb_processed/"
USB_MOUNT_DIR = "/media/usb"
KEY_PATH = "keys/bundle_signing_public.pem"
TTL_THRESHOLD_DAYS = 90

def load_public_key():
    with open(KEY_PATH, "rb") as f:
        return ed25519.Ed25519PublicKey.from_public_bytes(f.read())

def verify_sha256(filepath, expected_hash):
    sha = hashlib.sha256()
    with open(filepath, "rb") as f:
        sha.update(f.read())
    return sha.hexdigest() == expected_hash

def extract_and_validate(zip_path):
    if not zipfile.is_zipfile(zip_path):
        raise Exception("Not a valid zip archive.")

    extract_dir = os.path.join(IMPORT_DIR, os.path.splitext(os.path.basename(zip_path))[0])
    os.makedirs(extract_dir, exist_ok=True)

    with zipfile.ZipFile(zip_path, 'r') as zipf:
        zipf.extractall(extract_dir)

    meta_path = os.path.join(extract_dir, "bundle_metadata.json")
    sig_path = os.path.join(extract_dir, "signature.sig")

    if not os.path.exists(meta_path) or not os.path.exists(sig_path):
        raise Exception("Missing metadata or signature.")

    with open(meta_path, "r") as f:
        metadata = json.load(f)
    with open(sig_path, "rb") as f:
        signature = f.read()

    pubkey = load_public_key()
    try:
        pubkey.verify(signature, json.dumps(metadata).encode())
    except InvalidSignature:
        raise Exception("Invalid signature. Bundle may be tampered.")

    created = datetime.strptime(metadata["timestamp"], "%Y%m%d_%H%M%S")
    expiry = created + timedelta(days=metadata["ttl_days"])
    if datetime.utcnow() > expiry:
        raise Exception("Bundle expired.")

    for label, info in metadata["included_files"].items():
        file_path = os.path.join(extract_dir, info["filename"])
        if not verify_sha256(file_path, info["sha256"]):
            raise Exception(f"Hash mismatch for {info['filename']}")

    return extract_dir, metadata

def load_bundle(zip_path):
    try:
        extract_dir, meta = extract_and_validate(zip_path)
        print(f"[+] Loaded bundle for session {meta['session_id']}")
        move_to_processed(zip_path)
        return extract_dir
    except Exception as e:
        print(f"[!] Failed to load bundle: {e}")
        return None

def move_to_processed(zip_path):
    os.makedirs(PROCESSED_DIR, exist_ok=True)
    dest_path = os.path.join(PROCESSED_DIR, os.path.basename(zip_path))
    shutil.move(zip_path, dest_path)
    print(f"[→] Moved bundle to processed: {dest_path}")

# 🔍 USB Watcher
class USBReplayHandler(FileSystemEventHandler):
    def on_created(self, event):
        if not event.is_directory and event.src_path.endswith(".zip"):
            print(f"[USB] Detected new bundle: {event.src_path}")
            load_bundle(event.src_path)

def start_usb_watcher():
    observer = Observer()
    event_handler = USBReplayHandler()
    observer.schedule(event_handler, path=USB_MOUNT_DIR, recursive=False)
    observer.start()
    print(f"[🔌] Watching USB mount: {USB_MOUNT_DIR}")
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
    observer.join()

if __name__ == "__main__":
    os.makedirs(IMPORT_DIR, exist_ok=True)
    os.makedirs(USB_MOUNT_DIR, exist_ok=True)
    os.makedirs(PROCESSED_DIR, exist_ok=True)
    start_usb_watcher()
