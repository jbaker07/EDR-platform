import json
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding, rsa
from cryptography.hazmat.primitives.asymmetric.utils import Prehashed
from pathlib import Path

MANIFEST_FILE = "update_manifest.json"
SIGNED_MANIFEST_FILE = "update_manifest.signed.json"
PRIVATE_KEY_FILE = "private_manifest_signing.pem"

def sign_manifest():
    with open(MANIFEST_FILE, "rb") as f:
        manifest_data = f.read()

    private_key = serialization.load_pem_private_key(
        Path(PRIVATE_KEY_FILE).read_bytes(), password=None
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
