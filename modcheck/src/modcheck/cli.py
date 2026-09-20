"""ModCheck command line.

The CLI is the execution interface over the same core the reports and the
creator workflow use; it is not a separate product surface.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from . import evaluate as evaluate_mod
from . import report as report_mod
from . import sources_io
from .acquire import FetchError, fetch, verify
from .analyze.config import Installation
from .creator.apply import ApplyError, GENERATORS, apply as apply_change
from .creator.build import BuildRefused, run_build, write_attestation
from .creator.scaffold import ScaffoldError, scaffold as run_scaffold
from .inspect import inspect_path
from .paths import CAPABILITIES, packs_dir, project_root, workspaces_dir
from .store import RECORD_DIRS, Store
from .validate import summarize, validate_store

STATUS_MARK = {
    "supported": "yes",
    "partial": "partial",
    "documented_only": "docs only",
    "unsupported": "no",
    "blocked": "blocked",
}


def _store(args) -> Store:
    return Store(Path(args.packs) if getattr(args, "packs", None) else None)


# --- commands ------------------------------------------------------------
def cmd_inspect(args) -> int:
    ins = inspect_path(args.path, game=args.game)
    if args.json:
        print(json.dumps(ins.as_dict(), indent=2, default=str))
        return 0
    print(f"{ins.path}")
    print(f"  sha256      {ins.sha256}")
    print(f"  size        {ins.bytes} bytes")
    print(f"  kind        {ins.kind}" + (f"   game {ins.game}" if ins.game else ""))
    if ins.loader:
        print(f"  loader      {ins.loader}")
    print("  facts:")
    for fact in ins.facts:
        value = fact.value
        if isinstance(value, list) and len(value) > 6:
            value = f"[{len(value)} items] {value[:3]} ..."
        print(f"    {fact.key:26} {value}   ({fact.evidence_class})")
    if ins.warnings:
        print("  warnings:")
        for w in ins.warnings:
            print(f"    ! {w}")
    print("  checked:")
    for c in ins.checked:
        print(f"    + {c}")
    print("  NOT checked (a clean result here is not a guarantee):")
    for c in ins.not_checked:
        print(f"    - {c}")
    return 0


def cmd_packs(args) -> int:
    store = _store(args)
    games = [args.game] if args.game else store.games
    if not games:
        print("no game packs found", file=sys.stderr)
        return 1
    width = max(len(g) for g in games) + 2
    short = [c.replace("_", " ") for c in CAPABILITIES]
    print("Capability support is tracked per capability. A game is never 'supported' as a whole.\n")
    header = "game".ljust(width) + "".join(s[:11].ljust(12) for s in short)
    print(header)
    print("-" * len(header))
    for game in games:
        pack = store.pack(game)
        row = game.ljust(width)
        for cap in CAPABILITIES:
            row += STATUS_MARK.get(pack.capability_status(cap), "?").ljust(12)
        print(row)
    print()
    for game in games:
        pack = store.pack(game)
        counts = pack.counts()
        print(f"{game:16} " + "  ".join(f"{k}={v}" for k, v in sorted(counts.items())))
    return 0


def cmd_validate(args) -> int:
    store = _store(args)
    games = [args.game] if args.game else store.games
    issues = validate_store(store, games)
    for issue in issues:
        if args.errors_only and issue.severity != "error":
            continue
        print(issue.format())
    counts = summarize(issues)
    print(f"\n{counts['error']} error(s), {counts['warning']} warning(s) "
          f"across {len(games)} pack(s)")
    return 1 if counts["error"] else 0


def cmd_sources_add(args) -> int:
    try:
        result = fetch(args.url, suffix=args.suffix or "")
    except FetchError as exc:
        print(f"fetch failed: {exc}", file=sys.stderr)
        return 1
    extra = {}
    if args.games:
        extra["games"] = args.games.split(",")
    if args.publisher:
        extra["publisher"] = args.publisher
    record = result.source_record(args.id, args.title, args.kind, **extra)
    if result.status != 200:
        record["access"] = "blocked"
        record["gap"] = args.gap or f"HTTP {result.status}: content not retrieved"
    path = packs_dir() / args.game / "sources.yaml"
    action = sources_io.upsert(path, record)
    print(f"{action} {args.id}: HTTP {result.status}"
          + (f", sha256 {result.sha256[:16]}, {result.bytes} bytes" if result.sha256 else ""))
    if result.status != 200:
        print("  recorded as a GAP: this is not acquired knowledge")
    return 0


def cmd_sources_verify(args) -> int:
    from . import staleness

    store = _store(args)
    games = [args.game] if args.game else store.games
    drifted = 0
    total = 0
    for game in games:
        pack = store.pack(game)
        changed: dict[str, str] = {}
        for sid, source in sorted(pack.sources.items()):
            if source.get("access") != "public" or not source.get("sha256"):
                continue
            total += 1
            ok, why = verify(source)
            if not ok:
                drifted += 1
                changed[sid] = why
                print(f"[drift] {game}/{sid}: {why}")
            elif args.verbose:
                print(f"[ok]    {game}/{sid}")
        if changed:
            marks = staleness.mark_stale(pack, changed, reasons=changed,
                                         write=args.mark_stale)
            verb = "marked stale" if args.mark_stale else "would be marked stale"
            for mark in marks:
                print(f"         {verb}: {mark.record_kind} {mark.record_id}"
                      f" (cites {', '.join(mark.sources)})")
            if marks and not args.mark_stale:
                print("         re-run with --mark-stale to record this in the files")
    print(f"\n{total} source(s) checked, {drifted} changed or unreachable")
    return 1 if drifted else 0


def cmd_sources_revalidate(args) -> int:
    from . import staleness

    store = _store(args)
    pack = store.pack(args.game)
    cleared = staleness.clear_stale(pack, args.record,
                                    verified_against=args.verified_against)
    if not cleared:
        print("nothing cleared: no matching record was marked stale", file=sys.stderr)
        return 1
    print("revalidated: " + ", ".join(cleared))
    return 0


def cmd_stale(args) -> int:
    from . import staleness

    store = _store(args)
    records = staleness.stale_records(store, args.game)
    for record in records:
        reason = (record.get("provenance") or {}).get("stale_reason", "")
        print(f"{record.game:16} {record.kind:12} {record.id:42} {reason}")
    print(f"\n{len(records)} record(s) awaiting revalidation")
    return 1 if records else 0


def cmd_sources_list(args) -> int:
    store = _store(args)
    for game in ([args.game] if args.game else store.games):
        pack = store.pack(game)
        if not pack.sources:
            continue
        print(f"\n{game}")
        for sid, s in sorted(pack.sources.items()):
            flag = "" if s.get("access") == "public" else f"  [{s.get('access')}]"
            print(f"  {sid:38} {s.get('kind','?'):18} {s['url']}{flag}")
    return 0


def cmd_search(args) -> int:
    store = _store(args)
    kinds = tuple(args.kind.split(",")) if args.kind else None
    hits = store.search(args.query, kinds=kinds, game=args.game)
    if not hits:
        print("no matches")
        return 1
    for rec in hits:
        title = rec.get("title") or rec.get("intent") or ""
        print(f"{rec.game:16} {rec.kind:12} {rec.id:38} {title[:60]}")
    print(f"\n{len(hits)} match(es)")
    return 0


def cmd_show(args) -> int:
    store = _store(args)
    for kind in ([args.kind] if args.kind else list(RECORD_DIRS)):
        rec = store.find(kind, args.id, game=args.game)
        if rec:
            import yaml
            print(f"# {rec.game}/{rec.kind} from {rec.path}")
            print(yaml.safe_dump(rec.data, sort_keys=False, allow_unicode=True, width=100))
            return 0
    print(f"no record with id {args.id!r}", file=sys.stderr)
    return 1


def cmd_guide(args) -> int:
    from .creator.guide import guide, render

    store = _store(args)
    result = guide(store, args.idea, args.game, game_version=args.game_version,
                   loader=args.loader, limit=args.limit)
    print(json.dumps(result.as_dict(), indent=2) if args.json else render(result))
    return 0 if result.applicable else 1


def cmd_scaffold(args) -> int:
    root = Path(args.into) if args.into else workspaces_dir() / args.mod_id
    try:
        result = run_scaffold(args.game, args.loader, root, args.mod_id,
                              package=args.package, minecraft_version=args.game_version,
                              mod_name=args.name)
    except (ScaffoldError, FetchError) as exc:
        print(f"scaffold failed: {exc}", file=sys.stderr)
        return 1
    if args.json:
        print(json.dumps(result.as_dict(), indent=2))
        return 0
    print(f"created {result.game}/{result.loader} project at {result.root}")
    for rel in result.files:
        print(f"  {rel}")
    print("\nversions resolved live from upstream:")
    for key, value in result.versions.items():
        print(f"  {key:22} {value}")
        if result.version_sources.get(key):
            print(f"  {'':22} from {result.version_sources[key]}")
    print("\nnotes:")
    for note in result.notes:
        print(f"  - {note}")
    print("\nbuild with:")
    for command in result.build_commands:
        print(f"  modcheck build {result.root} --allow-execute")
    return 0


def cmd_apply(args) -> int:
    kwargs = {}
    for item in args.set or []:
        if "=" not in item:
            print(f"--set expects key=value, got {item!r}", file=sys.stderr)
            return 2
        key, _, value = item.partition("=")
        if value.lower() in ("true", "false"):
            kwargs[key] = value.lower() == "true"
        else:
            kwargs[key] = value
    try:
        changeset = apply_change(args.generator, Path(args.project), **kwargs)
    except (ApplyError, TypeError) as exc:
        print(f"apply failed: {exc}", file=sys.stderr)
        return 1
    if args.json:
        print(json.dumps(changeset.as_dict(), indent=2))
    else:
        print(f"# {changeset.description}\n")
        print(changeset.diff())
        if changeset.notes:
            print("\nnotes:")
            for note in changeset.notes:
                print(f"  - {note}")
        if changeset.follow_up:
            print("\nfollow up:")
            for item in changeset.follow_up:
                print(f"  - {item}")
    if args.write:
        written = changeset.write(Path(args.project))
        print(f"\nwrote {len(written)} file(s): " + ", ".join(written))
    else:
        print("\n(nothing written; pass --write to apply this change)")
    return 0


def cmd_build(args) -> int:
    commands = args.command_line or ["gradle build"]
    try:
        result = run_build(Path(args.project), commands, allow_execute=args.allow_execute,
                           timeout=args.timeout)
    except (BuildRefused, FileNotFoundError) as exc:
        print(f"{exc}", file=sys.stderr)
        return 2
    if args.json:
        print(json.dumps(result.as_dict(), indent=2))
    else:
        print(f"build {'succeeded' if result.ok else 'FAILED'} in {args.project}")
        for run in result.runs:
            print(f"  $ {run.command}  -> exit {run.returncode} ({run.duration_seconds}s)")
            if not run.ok:
                print("\n".join(f"    {line}" for line in run.stderr_tail.splitlines()[-15:]))
                print("\n".join(f"    {line}" for line in run.stdout_tail.splitlines()[-15:]))
        for artifact in result.artifacts:
            print(f"  artifact {artifact.path}")
            print(f"           sha256 {artifact.sha256}  ({artifact.bytes} bytes)")
        print("\n  this build establishes:")
        for item in result.establishes:
            print(f"    + {item}")
        print("  it does NOT establish:")
        for item in result.does_not_establish:
            print(f"    - {item}")
    if args.attest and args.game:
        path = packs_dir() / args.game / "attestations" / f"{args.attest}.json"
        write_attestation(path, recipe_id=args.attest, game=args.game, result=result)
        print(f"\nattestation written to {path}")
    return 0 if result.ok else 1


def cmd_analyze(args) -> int:
    store = _store(args)
    if args.config:
        installation = Installation.from_json(args.config)
        rep = report_mod.analyze_installation(installation, store, deep=args.deep)
    elif args.artifact:
        installation = Installation.from_paths(
            args.game, args.artifact, game_version=args.game_version,
            files_known_complete=args.complete_file_list)
        rep = report_mod.analyze_installation(installation, store, deep=args.deep)
    else:
        print("give --config or one or more --artifact paths", file=sys.stderr)
        return 2
    print(report_mod.render_json(rep) if args.json
          else report_mod.render(rep, args.audience))
    return 1 if rep.by_severity("error") or rep.by_severity("blocker") else 0


def cmd_release_report(args) -> int:
    store = _store(args)
    build = json.loads(Path(args.build).read_text()) if args.build else None
    source = json.loads(args.source) if args.source else None
    rep = report_mod.release_report(artifact_path=args.artifact, game=args.game, store=store,
                                    build=(build or {}).get("build", build),
                                    source=source, recipes=args.recipe or [])
    text = report_mod.render_json(rep) if args.json else report_mod.render_text(rep)
    if args.out:
        Path(args.out).write_text(text, encoding="utf-8")
        print(f"wrote {args.out}")
    else:
        print(text)
    return 0


def cmd_impact(args) -> int:
    from .analyze import impact as impact_mod

    store = _store(args)
    before = Installation.from_json(args.before)
    if args.after:
        after = Installation.from_json(args.after)
    elif args.install:
        after = impact_mod.with_artifact(before, args.install)
    else:
        print("give --after <config.json> or --install <artifact>", file=sys.stderr)
        return 2
    result = report_mod.impact_of(before, after, store)
    print(json.dumps(result.as_dict(), indent=2, default=str) if args.json
          else impact_mod.render(result))
    return 0


def cmd_evaluate(args) -> int:
    store = _store(args)
    directory = Path(args.cases) if args.cases else project_root() / "evaluation" / "cases"
    results = evaluate_mod.run(directory, store, game=args.game)
    if not results:
        print(f"no evaluation cases found in {directory}", file=sys.stderr)
        return 2
    summary = evaluate_mod.summarize(results)
    if args.json:
        print(json.dumps({"summary": summary,
                          "results": [{"id": r.case.id, "kind": r.case.kind,
                                       "passed": r.passed, "missed": r.missed,
                                       "false_warnings": r.unexpected_present,
                                       "produced": r.produced,
                                       "seconds": round(r.seconds, 4)}
                                      for r in results]}, indent=2))
    else:
        print(evaluate_mod.render(results, summary))
    return 0 if summary["failed"] == 0 else 1


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="modcheck", description=__doc__)
    p.add_argument("--packs", help="override the packs/ directory")
    sub = p.add_subparsers(dest="command", required=True)

    sp = sub.add_parser("inspect", help="inspect a real mod artifact")
    sp.add_argument("path")
    sp.add_argument("--game")
    sp.add_argument("--json", action="store_true")
    sp.set_defaults(func=cmd_inspect)

    sp = sub.add_parser("packs", help="show the per-capability support matrix")
    sp.add_argument("--game")
    sp.set_defaults(func=cmd_packs)

    sp = sub.add_parser("validate", help="validate the knowledge base")
    sp.add_argument("--game")
    sp.add_argument("--errors-only", action="store_true")
    sp.set_defaults(func=cmd_validate)

    sp = sub.add_parser("search", help="search recipes, failures, interactions")
    sp.add_argument("query")
    sp.add_argument("--game")
    sp.add_argument("--kind", help="comma-separated: recipe,failure,interaction,resolution,example")
    sp.set_defaults(func=cmd_search)

    sp = sub.add_parser("show", help="print one record")
    sp.add_argument("id")
    sp.add_argument("--game")
    sp.add_argument("--kind")
    sp.set_defaults(func=cmd_show)

    sp = sub.add_parser("guide", help="turn an idea into the recipes that apply")
    sp.add_argument("idea", help="what you want to build, in your own words")
    sp.add_argument("--game", required=True)
    sp.add_argument("--game-version")
    sp.add_argument("--loader")
    sp.add_argument("--limit", type=int, default=5)
    sp.add_argument("--json", action="store_true")
    sp.set_defaults(func=cmd_guide)

    sp = sub.add_parser("scaffold", help="create a real, buildable mod project")
    sp.add_argument("mod_id")
    sp.add_argument("--game", required=True)
    sp.add_argument("--loader", required=True)
    sp.add_argument("--into", help="target directory (default: build_workspaces/<mod_id>)")
    sp.add_argument("--package", help="Java package, where applicable")
    sp.add_argument("--game-version", help="target game version (default: current release)")
    sp.add_argument("--name", help="display name")
    sp.add_argument("--json", action="store_true")
    sp.set_defaults(func=cmd_scaffold)

    sp = sub.add_parser("apply", help="generate a reviewable change to a project")
    sp.add_argument("generator", choices=sorted(GENERATORS))
    sp.add_argument("project")
    sp.add_argument("--set", action="append", metavar="KEY=VALUE",
                    help="generator argument (repeatable)")
    sp.add_argument("--write", action="store_true", help="apply the change to disk")
    sp.add_argument("--json", action="store_true")
    sp.set_defaults(func=cmd_apply)

    sp = sub.add_parser("build", help="run a project's own build (executes untrusted code)")
    sp.add_argument("project")
    sp.add_argument("command_line", nargs="*", help="commands (default: 'gradle build')")
    sp.add_argument("--allow-execute", action="store_true",
                    help="required: authorises executing the project's build script")
    sp.add_argument("--timeout", type=int, default=1800)
    sp.add_argument("--attest", help="recipe id to write a build attestation for")
    sp.add_argument("--game")
    sp.add_argument("--json", action="store_true")
    sp.set_defaults(func=cmd_build)

    sp = sub.add_parser("analyze", help="analyse a configuration or set of artifacts")
    sp.add_argument("--game", required=True)
    sp.add_argument("--artifact", action="append", help="artifact path (repeatable)")
    sp.add_argument("--config", help="a configuration JSON file")
    sp.add_argument("--game-version")
    sp.add_argument("--complete-file-list", action="store_true",
                    help="assert the artifact list is complete, so absence is evidence")
    sp.add_argument("--deep", action="store_true",
                    help="parse plugin records (Bethesda games; needs the esplugin helper)")
    sp.add_argument("--audience", choices=["player", "creator", "full"], default="full",
                    help="present the same findings for a player, a creator, or in full")
    sp.add_argument("--json", action="store_true")
    sp.set_defaults(func=cmd_analyze)

    sp = sub.add_parser("release-report", help="produce a creator release report")
    sp.add_argument("artifact")
    sp.add_argument("--game", required=True)
    sp.add_argument("--build", help="build result JSON from `modcheck build --json`")
    sp.add_argument("--source", help="source metadata as a JSON string (commit, repo)")
    sp.add_argument("--recipe", action="append")
    sp.add_argument("--out")
    sp.add_argument("--json", action="store_true")
    sp.set_defaults(func=cmd_release_report)

    sp = sub.add_parser("stale", help="records awaiting revalidation")
    sp.add_argument("--game")
    sp.set_defaults(func=cmd_stale)

    sp = sub.add_parser("impact", help="what an install or update would change")
    sp.add_argument("--before", required=True, help="current configuration JSON")
    sp.add_argument("--after", help="proposed configuration JSON")
    sp.add_argument("--install", help="an artifact to add to the current configuration")
    sp.add_argument("--json", action="store_true")
    sp.set_defaults(func=cmd_impact)

    sp = sub.add_parser("evaluate", help="run the evaluation cases")
    sp.add_argument("--game")
    sp.add_argument("--cases", help="cases directory (default: evaluation/cases)")
    sp.add_argument("--json", action="store_true")
    sp.set_defaults(func=cmd_evaluate)

    sources = sub.add_parser("sources", help="knowledge acquisition")
    ssub = sources.add_subparsers(dest="sources_command", required=True)

    sp = ssub.add_parser("add", help="fetch a URL and record it as a source")
    sp.add_argument("url")
    sp.add_argument("--game", required=True)
    sp.add_argument("--id", required=True)
    sp.add_argument("--title", required=True)
    sp.add_argument("--kind", required=True,
                    choices=["official_docs", "community_wiki", "source_repo", "api", "schema",
                             "release_artifact", "issue_tracker", "analyzer_output",
                             "forum_post", "spec"])
    sp.add_argument("--games", help="comma-separated game ids this source applies to")
    sp.add_argument("--publisher")
    sp.add_argument("--suffix", help="file suffix for the cached copy, e.g. .json")
    sp.add_argument("--gap", help="what we therefore do not know, if the fetch fails")
    sp.set_defaults(func=cmd_sources_add)

    sp = ssub.add_parser("verify", help="re-fetch sources and report drift")
    sp.add_argument("--game")
    sp.add_argument("--verbose", action="store_true")
    sp.add_argument("--mark-stale", action="store_true")
    sp.set_defaults(func=cmd_sources_verify)

    sp = ssub.add_parser("revalidate", help="clear the stale flag after rechecking")
    sp.add_argument("--game", required=True)
    sp.add_argument("--record", action="append", required=True)
    sp.add_argument("--verified-against", action="append")
    sp.set_defaults(func=cmd_sources_revalidate)

    sp = ssub.add_parser("list", help="list recorded sources")
    sp.add_argument("--game")
    sp.set_defaults(func=cmd_sources_list)

    return p


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
