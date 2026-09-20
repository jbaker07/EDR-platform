#!/usr/bin/env python3
"""Assemble the runtime test installation, and pre-fill its capture manifest.

Everything up to pressing play. Run this on the machine that has a licensed
Stardew Valley with SMAPI and Content Patcher; it produces an isolated Mods
directory holding the complete packs for one case, plus a capture manifest with
every installed hash already filled in, so the operator's only manual step is
pasting console output and hashing it.

    python evaluation/runtime/provision.py --case sv_deliberate_conflict --out ./runtime-test

Three things it does that matter:

* **Complete packs, not parse-able ones.** The static walkthrough reconstructs a
  folder with only `content.json`, `manifest.json` and `LICENSE`. That is enough
  to parse and not enough to run: Content Patcher skips a `Load` whose
  `FromFile` is missing, logging a warning, and the patch then shows as not
  applied -- which looks exactly like a resolution outcome and is not one. So
  this fetches every asset the pack's patches reference, expanding
  `{{WhichBear}}` over the ConfigSchema's declared values.
* **It does not "tidy" the published pack.** Bear Mounts declares its config
  values in lower case while shipping capitalised filenames, so
  `assets/BearMount_{{WhichBear}}.png` expands to a name no file matches
  exactly. That is not a defect: Windows and macOS resolve it in the filesystem,
  and on Linux and Android SMAPI's own CaseInsensitiveFileLookup does. Renaming
  the files would mean testing something the author never shipped, so the
  resolution here is case-insensitive and the published names are preserved.
* **It never touches a real game install.** It writes only into `--out`. Point
  SMAPI at that directory with `--mods-path`, so the operator's own mods and
  saves are untouched.

Nothing here is redistributed: assets are fetched into the gitignored evidence
cache under the pack's MIT licence, with the LICENSE file carried alongside.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
import urllib.parse
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "src"))

import build_probe_pack  # noqa: E402

from modcheck import yamlio  # noqa: E402
from modcheck.acquire import fetch  # noqa: E402
from modcheck.jsonc import strip_jsonc  # noqa: E402
from modcheck.observe import load_predictions  # noqa: E402
from modcheck.paths import project_root  # noqa: E402

BEAR_COMMIT = "b3b227790164b68c8877544121092670ac71f644"
BEAR_ROOT = ("https://raw.githubusercontent.com/MouseyPounds/stardew-mods/"
             f"{BEAR_COMMIT}/Bear%20Mounts/Mods/%5BCP%5D%20Bear%20Mounts")
BEAR_LICENSE = ("https://raw.githubusercontent.com/MouseyPounds/stardew-mods/"
                f"{BEAR_COMMIT}/LICENSE")
BEAR_DIR = "[CP] Bear Mounts"
PROBE_DIR = "[CP] ModCheck Conflict Probe"

# Which packs each case installs, and at what probe priority. Taken from the
# predictions' `requires` blocks; kept here as data so provisioning cannot
# quietly disagree with what the prediction says was installed.
CASES = {
    "sv_baseline_bear_mounts": {"bear": True, "probe": None},
    "sv_deliberate_conflict": {"bear": True, "probe": "exclusive"},
    "sv_selected_replacement": {"bear": True, "probe": "High"},
    "sv_conditional_behaviour": {"bear": True, "probe": None,
                                 "config": {"UseSaddle": "true"}},
    "sv_composing_edit": {"bear": True, "probe": "Late", "probe_mode": "overlay"},
}

_TOKEN = re.compile(r"\{\{(?P<name>\w+)\}\}")


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _get(url: str) -> bytes:
    """Fetch through the project's own acquisition path, so it is cached."""
    result = fetch(url)
    if result.status != 200 or not result.path_bytes:
        raise SystemExit(f"could not fetch {url}: HTTP {result.status}")
    return result.path_bytes


def referenced_assets(content: dict) -> list[str]:
    """Every `FromFile` a patch names, with config tokens expanded.

    A token is expanded over its ConfigSchema `AllowValues`, so a single
    `{{WhichBear}}` reference yields one asset per bear. Any patch could be
    reached by a config the player chooses, so a complete install needs all of
    them -- fetching only the default would leave the other four cases unable to
    run at all.
    """
    schema = content.get("ConfigSchema") or {}
    allowed = {
        name: [v.strip() for v in str(spec.get("AllowValues", "")).split(",") if v.strip()]
        for name, spec in schema.items()
    }
    out: set[str] = set()
    for change in content.get("Changes") or []:
        raw = change.get("FromFile")
        if not raw:
            continue
        names = [raw]
        for token in _TOKEN.findall(raw):
            values = allowed.get(token)
            if not values:
                continue
            names = [n.replace("{{%s}}" % token, value) for n in names for value in values]
        out.update(names)
    return sorted(out)


def _resolve_case_insensitively(name: str, available: dict[str, str]) -> str | None:
    """Find the published filename for `name`, ignoring case.

    Mirrors what SMAPI's CaseInsensitiveFileLookup does on Linux and Android,
    and what NTFS and APFS do on Windows and macOS. Returns the file's real
    name, so the pack is assembled with the names its author shipped.
    """
    return available.get(name.lower())


def fetch_bear_mounts(out: Path) -> tuple[list[tuple[str, str]], list[str]]:
    """Fetch the complete published pack into `out`. Returns (files, warnings)."""
    out.mkdir(parents=True, exist_ok=True)
    (out / "assets").mkdir(exist_ok=True)

    files: list[tuple[str, str]] = []
    for name, url in (("manifest.json", f"{BEAR_ROOT}/manifest.json"),
                      ("content.json", f"{BEAR_ROOT}/content.json"),
                      ("LICENSE", BEAR_LICENSE)):
        data = _get(url)
        (out / name).write_bytes(data)
        files.append((name, _sha256(data)))

    content = json.loads(strip_jsonc((out / "content.json").read_bytes()))
    wanted = referenced_assets(content)

    # The pack's real filenames, discovered by probing rather than assumed: the
    # repository's tree API is not reachable from here, and guessing the
    # capitalisation is exactly the mistake this function exists to avoid.
    available: dict[str, str] = {}
    candidates = {w for w in wanted}
    for want in sorted(wanted):
        stem = want.rsplit("/", 1)[-1]
        for candidate in (stem, stem[:1].upper() + stem[1:], _capitalise_suffix(stem)):
            url = f"{BEAR_ROOT}/assets/{urllib.parse.quote(candidate)}"
            result = fetch(url)
            if result.status == 200 and result.path_bytes:
                available[f"assets/{stem}".lower()] = f"assets/{candidate}"
                (out / "assets" / candidate).write_bytes(result.path_bytes)
                files.append((f"assets/{candidate}", _sha256(result.path_bytes)))
                break
    del candidates

    warnings = []
    rescued = []
    for want in wanted:
        resolved = _resolve_case_insensitively(want, available)
        if resolved is None:
            warnings.append(
                f"{want} is referenced by a patch but no file with that name (in any "
                "case) could be fetched. Content Patcher will skip that Load with a "
                "warning, and the patch will show as not applied -- which is not a "
                "resolution outcome. Do not run a case that depends on it.")
        elif resolved != want:
            rescued.append(f"{want} -> {resolved}")
    if rescued:
        warnings.append(
            f"{len(rescued)} referenced name(s) match a published file only "
            "case-insensitively: " + "; ".join(rescued) + ". That is how it resolves "
            "on every supported platform -- Windows and macOS in the filesystem, "
            "Linux and Android via SMAPI's own CaseInsensitiveFileLookup -- so the "
            "files are kept under the names their author shipped rather than renamed "
            "to match. Renaming them would test something never released.")

    # One file can be reached by two references ({{WhichBear}}=skeleton and the
    # literal BearMount_Skeleton.png), and a manifest listing it twice is noise.
    seen: set[str] = set()
    unique = [(name, digest) for name, digest in files
              if not (name in seen or seen.add(name))]
    return unique, warnings


def _capitalise_suffix(stem: str) -> str:
    """'BearMount_brown.png' -> 'BearMount_Brown.png'."""
    if "_" not in stem:
        return stem
    head, tail = stem.rsplit("_", 1)
    return f"{head}_{tail[:1].upper()}{tail[1:]}"


def write_config(pack: Path, values: dict[str, str]) -> tuple[str, str] | None:
    """Write the config.json Content Patcher would otherwise generate.

    A config change needs a full game restart, not `patch reload` -- Content
    Patcher's own documentation says a reload does not refresh ConfigSchema or
    dynamic tokens. Writing it before first launch avoids a reload entirely.
    """
    if not values:
        return None
    content = json.loads(strip_jsonc((pack / "content.json").read_bytes()))
    schema = content.get("ConfigSchema") or {}
    config = {name: spec.get("Default") for name, spec in schema.items()}
    config.update(values)
    data = json.dumps(config, indent=2).encode() + b"\n"
    (pack / "config.json").write_bytes(data)
    return "config.json", _sha256(data)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--case", required=True, choices=sorted(CASES))
    parser.add_argument("--out", required=True,
                        help="an EMPTY directory to use as SMAPI's --mods-path. "
                             "Never point this at a real Mods folder")
    parser.add_argument("--run-id", default="", help="defaults to <case>-run1")
    parser.add_argument("--force", action="store_true",
                        help="allow a non-empty --out directory")
    args = parser.parse_args()

    out = Path(args.out).resolve()
    if out.exists() and any(out.iterdir()) and not args.force:
        print(f"{out} is not empty. Point --out at a fresh directory, or pass --force "
              "if you are certain. This tool writes mod folders and must never be "
              "aimed at a real game installation.", file=sys.stderr)
        return 2
    out.mkdir(parents=True, exist_ok=True)

    plan = CASES[args.case]
    prediction = next((p for p in load_predictions(
        project_root() / "evaluation" / "runtime" / "predictions")
        if p.id == args.case), None)
    if prediction is None:
        print(f"no prediction named {args.case}", file=sys.stderr)
        return 2

    installed: list[dict[str, str]] = []
    warnings: list[str] = []

    if plan.get("bear"):
        pack = out / BEAR_DIR
        files, pack_warnings = fetch_bear_mounts(pack)
        warnings += pack_warnings
        config = write_config(pack, plan.get("config") or {})
        if config:
            files.append(config)
        installed += [{"path": f"{BEAR_DIR}/{name}", "sha256": digest}
                      for name, digest in files]

    if plan.get("probe") is not None:
        priority = None if plan["probe"] == "exclusive" else plan["probe"]
        digests = build_probe_pack.build(priority, out / PROBE_DIR,
                                         mode=plan.get("probe_mode", "load"))
        installed += [{"path": f"{PROBE_DIR}/{name}", "sha256": digest}
                      for name, digest in digests.items()]

    run_id = args.run_id or f"{args.case}-run1"
    manifest = {
        "prediction": prediction.id,
        "run_id": run_id,
        "captured_at": "<fill in at capture time, RFC3339>",
        "versions": {"stardew_valley": "<fill in>", "smapi": "<fill in>",
                     "content_patcher": "<fill in>"},
        "installed": installed,
        "transcripts": [
            {"command": command, "path": "SMAPI console",
             "local_path": f"evaluation/runtime/transcripts/{prediction.id}."
                           f"{_slug(command)}.txt",
             "sha256": "<sha256 of that file>"}
            for command in prediction.commands],
        "notes": "",
    }
    manifest_path = out / f"{prediction.id}.{run_id}.capture.yaml"
    manifest_path.write_text(yamlio.dump(manifest))

    print(f"provisioned {args.case} into {out}")
    for entry in installed:
        print(f"  {entry['sha256'][:16]}...  {entry['path']}")
    if warnings:
        print()
        print("warnings:")
        for warning in warnings:
            print(f"  - {warning}")
    print()
    print("next, on this machine:")
    print(f"  1. launch SMAPI with --mods-path {out}")
    print("     (a disposable save; the asset must be requested before it can apply --")
    print("      summon the horse, or the patch shows as not applied for a reason that")
    print("      is not a defect)")
    for key, value in (prediction.requires or {}).items():
        print(f"     {key}: {str(value).strip()}")
    print("  2. in the SMAPI console, run each command and save its output verbatim:")
    for command in prediction.commands:
        print(f"       $ {command}")
        print(f"         -> evaluation/runtime/transcripts/{prediction.id}."
              f"{_slug(command)}.txt")
    print(f"  3. fill in the versions and transcript hashes in {manifest_path.name},")
    print("     copy it into evaluation/runtime/transcripts/, then:")
    print(f"       modcheck runtime observe --id {prediction.id} \\")
    print(f"           --capture evaluation/runtime/transcripts/{manifest_path.name}")
    print()
    print("  If the run disagrees with the prediction, leave the prediction exactly as")
    print("  recorded. The disagreement is the result.")
    return 0


def _slug(command: str) -> str:
    text = command.lower().replace("patch ", "", 1)
    return re.sub(r"[^a-z0-9]+", "-", text).strip("-")[:40]


if __name__ == "__main__":
    raise SystemExit(main())
