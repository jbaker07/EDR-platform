"""Tests for scripts/policy_signer.py, the signer side of the agent policy check.

The agent (edr-agent/src/config.rs) checks the same .sig format with the same
OpenSSL library, and its own test checks the shipped policy too.
"""

from __future__ import annotations

import importlib.util
import shutil
import subprocess
from pathlib import Path

import pytest

_REPO = Path(__file__).resolve().parent.parent
_SPEC = importlib.util.spec_from_file_location("policy_signer", _REPO / "scripts" / "policy_signer.py")
signer = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(signer)


def _openssl_has_rawin() -> bool:
    if not shutil.which("openssl"):
        return False
    out = subprocess.run(["openssl", "pkeyutl", "-help"], capture_output=True, text=True)
    return "-rawin" in (out.stdout + out.stderr)


pytestmark = pytest.mark.skipif(not _openssl_has_rawin(), reason="needs openssl 3.0+ (pkeyutl -rawin)")


def _keys(tmp_path: Path) -> tuple[Path, Path]:
    key, pub = tmp_path / "keys" / "signing.pem", tmp_path / "pub.pem"
    signer.keygen(key, pub)
    return key, pub


def test_sign_then_verify_round_trip(tmp_path):
    key, pub = _keys(tmp_path)
    assert key.stat().st_mode & 0o777 == 0o600
    policy = tmp_path / "policy.json"
    policy.write_text('{"policy": {"mode": "forensic"}}\n')
    signer.sign(policy, key, pub)
    sig = (tmp_path / "policy.json.sig").read_text().strip()
    assert len(sig) == 128  # 64 bytes as hex
    assert signer.verify(policy, pub)


def test_one_changed_byte_fails(tmp_path):
    key, pub = _keys(tmp_path)
    policy = tmp_path / "policy.json"
    policy.write_text('{"policy": {"collection_interval": 60}}\n')
    signer.sign(policy, key, pub)
    policy.write_text('{"policy": {"collection_interval": 61}}\n')
    assert not signer.verify(policy, pub)
    (tmp_path / "policy.json.sig").unlink()
    assert not signer.verify(policy, pub)  # missing signature


def test_private_key_rules(tmp_path):
    with pytest.raises(SystemExit, match="inside the repo"):
        signer.keygen(_REPO / "edr-agent" / "should_not_exist.pem", tmp_path / "pub.pem")
    assert not (_REPO / "edr-agent" / "should_not_exist.pem").exists()
    key, pub = _keys(tmp_path)
    with pytest.raises(SystemExit, match="already exists"):
        signer.keygen(key, pub)
    key.chmod(0o644)
    policy = tmp_path / "policy.json"
    policy.write_text("{}")
    with pytest.raises(SystemExit, match="readable by group or others"):
        signer.sign(policy, key, pub)


def test_shipped_policy_verifies_with_the_shipped_key():
    assert signer.verify(_REPO / "edr-agent" / "policy.json", signer.DEFAULT_PUB)
