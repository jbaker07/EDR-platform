#!/usr/bin/env python3
from pathlib import Path
import sys

try:
    from ruamel.yaml import YAML
    from ruamel.yaml.error import YAMLError
except ImportError:
    print("ruamel.yaml is required. Install with `pip install ruamel.yaml`.", file=sys.stderr)
    sys.exit(1)

BASE_DIR = Path(__file__).resolve().parents[1]
PLAYBOOK_DIR = BASE_DIR / "playbooks"
ALLOWED_SEVERITIES = {"low", "medium", "high", "critical"}
REQUIRED_KEYS = {
    "id": str,
    "name": str,
    "version": int,
    "severity": str,
    "window_sec": int,
    "cooldown_sec": int,
    "description": str,
    "rules": list,
    "reason_code": str,
    "title": str,
}

yaml = YAML()
yaml.preserve_quotes = True

def validate_file(path: Path) -> list[str]:
    errors: list[str] = []
    try:
        data = yaml.load(path.read_text())
    except YAMLError as exc:
        mark = getattr(exc, "problem_mark", None)
        if mark is not None:
            errors.append(
                f"{path}: YAML parse error at line {mark.line + 1}, column {mark.column + 1}: {exc}"
            )
        else:
            errors.append(f"{path}: YAML parse error: {exc}")
        return errors
    except Exception as exc:  # pragma: no cover - unexpected
        errors.append(f"{path}: YAML parse error: {exc}")
        return errors

    if not isinstance(data, dict):
        errors.append(f"{path}: top-level YAML is not a mapping")
        return errors

    for key, expected_type in REQUIRED_KEYS.items():
        if key not in data:
            errors.append(f"{path}: missing required key '{key}'")
            continue
        value = data[key]
        if expected_type is int:
            if not isinstance(value, int):
                errors.append(f"{path}: key '{key}' must be an integer (found {type(value).__name__})")
        elif expected_type is list:
            if not isinstance(value, list):
                errors.append(f"{path}: key '{key}' must be a list")
        elif expected_type is str:
            if not isinstance(value, str):
                errors.append(f"{path}: key '{key}' must be a string")

    severity = data.get("severity")
    if isinstance(severity, str):
        if severity.lower() not in ALLOWED_SEVERITIES:
            errors.append(f"{path}: severity '{severity}' is not one of {sorted(ALLOWED_SEVERITIES)}")
    else:
        errors.append(f"{path}: severity must be a string")

    return errors


def main() -> None:
    if not PLAYBOOK_DIR.exists():
        print(f"Playbook directory {PLAYBOOK_DIR} not found", file=sys.stderr)
        sys.exit(1)

    all_errors: list[str] = []
    for pb_file in sorted(PLAYBOOK_DIR.glob("*.yaml")):
        all_errors.extend(validate_file(pb_file))

    if all_errors:
        for err in all_errors:
            print(err, file=sys.stderr)
        sys.exit(1)

    print("All playbooks validated successfully.")


if __name__ == "__main__":
    main()
