import json
import hashlib
from datetime import datetime
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding, rsa
from cryptography.hazmat.backends import default_backend
import os

def load_private_key(path: str):
    with open(path, "rb") as f:
        return serialization.load_pem_private_key(f.read(), password=None, backend=default_backend())

def sign_data(data: bytes, private_key) -> bytes:
    return private_key.sign(
        data,
        padding.PSS(mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.MAX_LENGTH),
        hashes.SHA256()
    )

def generate_signed_intel_pack(data_dict: dict, private_key_path: str, output_path: str):
    private_key = load_private_key(private_key_path)
    payload = {
        "timestamp": datetime.utcnow().isoformat(),
        "intel": data_dict,
    }

    raw_bytes = json.dumps(payload, sort_keys=True).encode()
    payload["hash"] = hashlib.sha256(raw_bytes).hexdigest()
    payload["signature"] = sign_data(raw_bytes, private_key).hex()

    filename = f"intel_pack_{datetime.utcnow().strftime('%Y%m%d%H%M%S')}.json"
    full_path = os.path.join(output_path, filename)
    with open(full_path, "w") as f:
        json.dump(payload, f, indent=2)
    
    print(f"[✓] Intel pack signed and saved to: {full_path}")
    return full_path
