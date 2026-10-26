import json
import os
import sys
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding, rsa
from cryptography.hazmat.primitives.asymmetric.utils import Prehashed
from pathlib import Path

MANIFEST_FILE = "update_manifest.json"
SIGNED_MANIFEST_FILE = "update_manifest.signed.json"
# The signing key lives outside the repo. Point this variable at a PEM file.
PRIVATE_KEY_ENV = "EDR_SIGNING_KEY_PATH"

def sign_manifest():
    with open(MANIFEST_FILE, "rb") as f:
        manifest_data = f.read()

    key_path = os.environ.get(PRIVATE_KEY_ENV)
    if not key_path:
        sys.exit(f"Set {PRIVATE_KEY_ENV} to the path of your PEM private key.")

    private_key = serialization.load_pem_private_key(
        Path(key_path).read_bytes(), password=None
    )

    signature = private_key.sign(
        manifest_data,
        padding.PSS(
            mgf=padding.MGF1(hashes.SHA256()),
            salt_length=padding.PSS.MAX_LENGTH
        ),
        hashes.SHA256()
    )

    signed_payload = {
        "manifest": json.loads(manifest_data.decode()),
        "signature": signature.hex()
    }

    with open(SIGNED_MANIFEST_FILE, "w") as f:
        json.dump(signed_payload, f, indent=2)

    print("✅ Manifest signed successfully.")

if __name__ == "__main__":
    sign_manifest()
