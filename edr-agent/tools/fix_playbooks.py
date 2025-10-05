#!/usr/bin/env python3
from pathlib import Path
import re
import sys

try:
    from ruamel.yaml import YAML
    from ruamel.yaml.error import YAMLError
except ImportError as exc:
    print("ruamel.yaml is required. Install with `pip install ruamel.yaml`.", file=sys.stderr)
    raise

BASE_DIR = Path(__file__).resolve().parents[1]
PLAYBOOK_DIR = BASE_DIR / "playbooks"
ALLOWED_SEVERITIES = {"low", "medium", "high", "critical"}
DEFAULTS = {
    "version": 1,
    "severity": "medium",
    "window_sec": 900,
    "cooldown_sec": 300,
    "description": "n/a",
}

yaml = YAML()
yaml.preserve_quotes = True
yaml.indent(mapping=2, sequence=4, offset=2)
yaml.width = 4096


def derive_reason_code(identifier: str, fallback: str) -> str:
    ident = identifier or fallback or "generic"
    ident = re.sub(r"^pb_", "", ident, flags=re.IGNORECASE)
    ident = re.sub(r"[^0-9A-Za-z]+", "_", ident)
    ident = ident.strip("_") or "GENERIC"
    return ident.upper()


def ensure_string(value, default=""):
    if isinstance(value, str):
        return value
    if value is None:
        return default
    return str(value)


def process_file(path: Path) -> None:
    try:
        data = yaml.load(path.read_text())
    except YAMLError as exc:
        mark = getattr(exc, "problem_mark", None)
        if mark is not None:
            print(
                f"[WARN] Failed to parse {path}: line {mark.line + 1}, column {mark.column + 1}: {exc}",
                file=sys.stderr,
            )
        else:
            print(f"[WARN] Failed to parse {path}: {exc}", file=sys.stderr)
        return
    except Exception as exc:  # pragma: no cover - unexpected
        print(f"[WARN] Failed to parse {path}: {exc}", file=sys.stderr)
        return

    if data is None:
        data = {}
    if not isinstance(data, dict):
        print(f"[WARN] Skipping {path}: top-level YAML is not a mapping", file=sys.stderr)
        return

    changes = []
    filename_fallback = path.stem

    # Required id/name
    if "id" not in data:
        data["id"] = filename_fallback
        changes.append("id")
    data["id"] = ensure_string(data.get("id", filename_fallback))

    if "name" not in data:
        data["name"] = data["id"].replace("_", " ").title()
        changes.append("name")
    data["name"] = ensure_string(data.get("name"))

    # Defaults for other fields
    for key, default in DEFAULTS.items():
        if key not in data:
            data[key] = default
            changes.append(key)

    # Special handling for severity
    severity = ensure_string(data.get("severity", "")).lower()
    if severity not in ALLOWED_SEVERITIES:
        print(
            f"[INFO] {path}: severity '{data.get('severity')}' invalid; defaulting to 'medium'",
            file=sys.stderr,
        )
        severity = "medium"
        changes.append("severity")
    data["severity"] = severity

    # window_sec & cooldown_sec should be integers
    for key in ("window_sec", "cooldown_sec"):
        value = data.get(key)
        if not isinstance(value, int):
            try:
                data[key] = int(value)
            except (TypeError, ValueError):
                data[key] = DEFAULTS[key]
                print(f"[INFO] {path}: coerced {key} to {data[key]}", file=sys.stderr)
            else:
                if data[key] != value:
                    changes.append(key)

    version_val = data.get("version")
    parsed_version = None
    if isinstance(version_val, int) and version_val > 0:
        parsed_version = version_val
    else:
        try:
            parsed_version = int(str(version_val))
        except (TypeError, ValueError):
            parsed_version = 1
        if parsed_version <= 0:
            parsed_version = 1
    if data.get("version") != parsed_version:
        data["version"] = parsed_version
        changes.append("version")

    # description
    data["description"] = ensure_string(data.get("description", "n/a")) or "n/a"

    # title defaults to name
    if "title" not in data or not ensure_string(data.get("title")):
        data["title"] = data["name"]
        changes.append("title")

    # reason_code
    if "reason_code" not in data or not ensure_string(data.get("reason_code")):
        data["reason_code"] = derive_reason_code(data.get("id", filename_fallback), filename_fallback)
        changes.append("reason_code")

    # rules validation
    if "rules" not in data or not isinstance(data.get("rules"), list):
        data["rules"] = []
        print(f"[INFO] {path}: added empty 'rules' list", file=sys.stderr)
        changes.append("rules")

    # Special targeted overrides
    if path.name == "pb_lotl_cred.yaml":
        overrides = {
            "version": 1,
            "severity": "high",
            "window_sec": 900,
            "cooldown_sec": 600,
        }
        for key, value in overrides.items():
            if data.get(key) != value:
                data[key] = value
                changes.append(key)

    if path.name == "pb_Compress_exfil.yaml":
        overrides = {
            "version": 1,
            "severity": "medium",
            "window_sec": 900,
            "cooldown_sec": 600,
        }
        for key, value in overrides.items():
            if data.get(key) != value:
                data[key] = value
                changes.append(key)

    if not changes:
        print(f"{path}: no changes")
        return

    # Deduplicate change list for readability
    seen = set()
    ordered_changes = []
    for item in changes:
        if item not in seen:
            seen.add(item)
            ordered_changes.append(item)

    yaml.dump(data, path.open("w"))
    print(f"{path}: updated fields -> {', '.join(ordered_changes)}")


def main() -> None:
    if not PLAYBOOK_DIR.exists():
        print(f"Playbook directory {PLAYBOOK_DIR} not found", file=sys.stderr)
        sys.exit(1)

    for pb_file in sorted(PLAYBOOK_DIR.glob("*.yaml")):
        process_file(pb_file)


if __name__ == "__main__":
    main()
