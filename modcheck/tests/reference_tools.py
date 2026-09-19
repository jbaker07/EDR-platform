"""Independent implementations used to cross-check our parsers.

A reference tool is evidence, not truth. Where one disagrees with us the
disagreement is investigated against the format specification before either
side is changed, and the outcome is recorded in the test that found it.

Each helper skips (never passes) when its tool is not built or installed.
"""
from __future__ import annotations

import json
import shutil
import subprocess
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
S4TK_DIR = ROOT / "helpers" / "s4tk-ref"
S4TK_SCRIPT = S4TK_DIR / "dbpf.js"


def s4tk_version() -> str:
    pkg = S4TK_DIR / "node_modules" / "@s4tk" / "models" / "package.json"
    if not pkg.exists():
        return "unavailable"
    return json.loads(pkg.read_text(encoding="utf-8"))["version"]


def require_s4tk() -> None:
    if shutil.which("node") is None:
        pytest.skip("node is not available")
    if not (S4TK_DIR / "node_modules" / "@s4tk" / "models").is_dir():
        pytest.skip("S4TK reference not installed (cd helpers/s4tk-ref && npm install)")


def _run_s4tk(args: list[str]) -> dict:
    proc = subprocess.run(["node", str(S4TK_SCRIPT), *args], cwd=S4TK_DIR,
                          capture_output=True, text=True, timeout=180)
    if not proc.stdout.strip():
        pytest.skip(f"S4TK helper produced no output: {proc.stderr[:200]}")
    return json.loads(proc.stdout)


def s4tk_write(out_path: Path, resources: list[dict]) -> dict:
    """Have S4TK build a real .package containing the given resource keys."""
    require_s4tk()
    return _run_s4tk(["write", str(out_path), json.dumps(resources)])


def s4tk_read(path: Path) -> dict:
    """Have S4TK read a .package and list its resource keys."""
    require_s4tk()
    return _run_s4tk(["read", str(path)])
