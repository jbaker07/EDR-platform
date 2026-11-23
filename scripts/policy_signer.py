#!/usr/bin/env python3
# scripts/policy_signer.py
#
# Signs the agent policy with Ed25519. The signature covers the exact bytes of
# policy.json and goes next to it as policy.json.sig (hex), which is what
# edr-agent/src/config.rs checks at startup. Signing the file bytes means there
# is no canonical JSON form for two programs to agree on.
#
# It calls the openssl CLI (3.0 or newer, for pkeyutl -rawin), the same library
# the agent links, so there is no second crypto implementation to disagree with.
#
#   policy_signer.py keygen [--key K] [--pub P]
#   policy_signer.py sign <policy.json> [--key K] [--pub P]
#   policy_signer.py verify <policy.json> [--pub P]
#
# The private key defaults to ~/.edr/policy_signing_key.pem. It is created with
# mode 0600, refused if others can read it, and refused inside this repo so it
# cannot be committed by accident. Only the public key belongs in the repo.

import argparse
import os
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
DEFAULT_KEY = Path.home() / ".edr" / "policy_signing_key.pem"
DEFAULT_PUB = REPO / "edr-agent" / "config" / "policy_pub.pem"


def openssl(*args: str) -> None:
    res = subprocess.run(["openssl", *args], capture_output=True, text=True)
    if res.returncode != 0:
        sys.exit(f"openssl {args[0]} failed: {res.stderr.strip() or res.stdout.strip()}")


def inside_repo(path: Path) -> bool:
    try:
        path.resolve().relative_to(REPO)
        return True
    except ValueError:
        return False


def check_private_key(key: Path) -> None:
    if inside_repo(key):
        sys.exit(f"refusing a private key inside the repo: {key}")
    if not key.exists():
        sys.exit(f"no private key at {key} (run keygen first)")
    if key.stat().st_mode & 0o077:
        sys.exit(f"{key} is readable by group or others, chmod 600 it first")


def keygen(key: Path, pub: Path) -> None:
    if inside_repo(key):
        sys.exit(f"refusing a private key inside the repo: {key}")
    if key.exists():
        sys.exit(f"{key} already exists, not overwritten")
    key.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
    old = os.umask(0o077)  # the key file is never readable by others, not even briefly
    try:
        openssl("genpkey", "-algorithm", "ed25519", "-out", str(key))
    finally:
        os.umask(old)
    key.chmod(0o600)
    pub.parent.mkdir(parents=True, exist_ok=True)
    openssl("pkey", "-in", str(key), "-pubout", "-out", str(pub))
    print(f"private key {key} (mode 600, keep it out of the repo)")
    print(f"public key  {pub}")


def sig_path(policy: Path) -> Path:
    return policy.with_name(policy.name + ".sig")


def verify(policy: Path, pub: Path) -> bool:
    try:
        sig = bytes.fromhex(sig_path(policy).read_text().strip())
    except (OSError, ValueError):
        return False  # no .sig file, or not hex
    with tempfile.TemporaryDirectory() as d:
        raw = Path(d) / "sig.bin"
        raw.write_bytes(sig)
        res = subprocess.run(
            ["openssl", "pkeyutl", "-verify", "-pubin", "-inkey", str(pub),
             "-rawin", "-in", str(policy), "-sigfile", str(raw)],
            capture_output=True, text=True,
        )
    return res.returncode == 0


def sign(policy: Path, key: Path, pub: Path) -> None:
    check_private_key(key)
    with tempfile.TemporaryDirectory() as d:
        raw = Path(d) / "sig.bin"
        openssl("pkeyutl", "-sign", "-inkey", str(key), "-rawin",
                "-in", str(policy), "-out", str(raw))
        sig = raw.read_bytes()
    if len(sig) != 64:
        sys.exit(f"expected a 64-byte Ed25519 signature, got {len(sig)} bytes")
    sig_path(policy).write_text(sig.hex() + "\n")
    # Check against the public key the agent will use, so a key mix-up shows now.
    if pub.exists() and not verify(policy, pub):
        sys.exit(f"signed, but the signature does not verify with {pub}")
    print(f"wrote {sig_path(policy)}")


def main() -> None:
    ap = argparse.ArgumentParser(description="Sign the agent policy with Ed25519.")
    ap.add_argument("command", choices=["keygen", "sign", "verify"])
    ap.add_argument("policy", nargs="?", type=Path)
    ap.add_argument("--key", type=Path, default=DEFAULT_KEY)
    ap.add_argument("--pub", type=Path, default=DEFAULT_PUB)
    a = ap.parse_args()
    if a.command == "keygen":
        keygen(a.key.expanduser(), a.pub)
        return
    if a.policy is None:
        ap.error(f"{a.command} needs a policy path")
    if a.command == "sign":
        sign(a.policy, a.key.expanduser(), a.pub)
    elif verify(a.policy, a.pub):
        print(f"{a.policy}: signature OK")
    else:
        sys.exit(f"{a.policy}: signature does NOT verify with {a.pub}")


if __name__ == "__main__":
    main()
