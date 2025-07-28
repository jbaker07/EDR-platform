#!/usr/bin/env python3
# scripts/policy_signer.py

import json
import sys
from pathlib import Path
from nacl.signing import SigningKey

def main(policy_path: Path, key_path: Path, out_path: Path):
    # load or generate key
    if not key_path.exists():
        key_path.write_bytes(SigningKey.generate().encode())
        print(f"Generated new key → {key_path}")

    sk = SigningKey(key_path.read_bytes())
    policy = json.loads(policy_path.read_text(encoding="utf-8"))

    payload = json.dumps(policy, sort_keys=True).encode("utf-8")
    sig = sk.sign(payload).signature.hex()

    signed = {
        "policy": policy,
        "signature": sig,
        "pubkey": sk.verify_key.encode().hex(),
    }
    out_path.write_text(json.dumps(signed, indent=2))
    print(f"Wrote signed policy → {out_path}")

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Usage: policy_signer.py <policy.json> <signing.key> <signed_policy.json>")
        sys.exit(1)

    main(Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3]))
