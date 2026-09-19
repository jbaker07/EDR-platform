"""ModCheck command line.

The CLI is the execution interface over the same core the reports and the
creator workflow use; it is not a separate product surface.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from . import sources_io
from .acquire import FetchError, fetch, verify
from .inspect import inspect_path
from .paths import CAPABILITIES, packs_dir
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
    store = _store(args)
    games = [args.game] if args.game else store.games
    drifted = 0
    total = 0
    for game in games:
        pack = store.pack(game)
        for sid, source in sorted(pack.sources.items()):
            if source.get("access") != "public" or not source.get("sha256"):
                continue
            total += 1
            ok, why = verify(source)
            if not ok:
                drifted += 1
                print(f"[drift] {game}/{sid}: {why}")
                if args.mark_stale:
                    print(f"         -> dependent records must be revalidated")
            elif args.verbose:
                print(f"[ok]    {game}/{sid}")
    print(f"\n{total} source(s) checked, {drifted} changed or unreachable")
    return 1 if drifted else 0


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

    sp = ssub.add_parser("list", help="list recorded sources")
    sp.add_argument("--game")
    sp.set_defaults(func=cmd_sources_list)

    return p


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
