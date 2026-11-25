#!/usr/bin/env python3
# scripts/shieldctl.py
#
# Operator helper for runtime shields on one agent. The agent only accepts a
# shield document signed with the policy key, so the usual flow is:
#
#   shieldctl.py propose <incident id> [index] > doc.json   the proposal on an incident
#   scripts/policy_signer.py sign doc.json                  writes doc.json.sig
#   shieldctl.py apply doc.json                             POST /shields, starts in monitor
#   shieldctl.py promote <shield id> "trial was clean"      monitor to enforce
#   shieldctl.py list
#   shieldctl.py lift <shield id> "incident closed"         DELETE /shields/<id>
#
# Or write a document by hand: shieldctl.py new connect_deny --cidr 203.0.113.10 --port 4444
# --ttl 3600 --reason "C2 from incident 12" > doc.json (exec_deny and file_deny take --path,
# and the document then names the file by the device and inode stat() gives now).
#
# The agent API token comes from $EDR_AGENT_TOKEN_FILE (a file that holds only the
# token) and the API from $EDR_API (default http://127.0.0.1:8080). Standard library only.

import argparse
import json
import os
import sys
import time
import urllib.error
import urllib.request

API = os.environ.get("EDR_API", "http://127.0.0.1:8080")


def token() -> str:
    path = os.environ.get("EDR_AGENT_TOKEN_FILE")
    if not path:
        sys.exit("set EDR_AGENT_TOKEN_FILE to the file that holds the agent API token")
    return open(path).read().strip()


def call(method: str, path: str, body=None):
    data = None if body is None else json.dumps(body).encode()
    req = urllib.request.Request(API + path, data=data, method=method)
    req.add_header("Authorization", "Bearer " + token())
    if data is not None:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=10) as r:
            return r.status, json.loads(r.read() or b"{}")
    except urllib.error.HTTPError as e:
        raw = e.read()
        try:
            return e.code, json.loads(raw or b"{}")
        except ValueError:
            return e.code, {"error": raw.decode(errors="replace")}


def show(status, body) -> None:
    print(json.dumps({"status": status, "body": body}, indent=2))
    if status >= 300:
        sys.exit(1)


def new_doc(a) -> dict:
    now = int(time.time())
    match = {}
    if a.kind == "connect_deny":
        if not a.cidr:
            sys.exit("connect_deny needs --cidr")
        match["cidr"] = a.cidr
        if a.port:
            match["port"] = a.port
    else:
        if not a.path:
            sys.exit(f"{a.kind} needs --path")
        st = os.stat(a.path)
        match = {"path": a.path, "dev": st.st_dev, "ino": st.st_ino}
    doc = {
        "id": a.id or f"manual.{a.kind}.{now}",
        "serial": a.serial or now,
        "kind": a.kind,
        "match": match,
        "scope": {"cgroup": a.cgroup} if a.cgroup else {},
        "mode": a.mode,
        "not_before": now,
        "expires_at": now + a.ttl,
    }
    if a.reason:
        doc["reason"] = a.reason
    if a.incident:
        doc["incident_id"] = a.incident
    if "reason" not in doc and "incident_id" not in doc:
        sys.exit("give --reason or --incident")
    return {"shield": doc}


def main() -> None:
    ap = argparse.ArgumentParser(description="Runtime shields on one EDR agent.")
    sub = ap.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("propose")
    p.add_argument("incident")
    p.add_argument("index", nargs="?", type=int, default=0)
    n = sub.add_parser("new")
    n.add_argument("kind", choices=["exec_deny", "file_deny", "connect_deny"])
    n.add_argument("--path")
    n.add_argument("--cidr")
    n.add_argument("--port", type=int)
    n.add_argument("--cgroup")
    n.add_argument("--ttl", type=int, default=3600)
    n.add_argument("--mode", choices=["monitor", "enforce"], default="enforce")
    n.add_argument("--id")
    n.add_argument("--serial", type=int)
    n.add_argument("--reason")
    n.add_argument("--incident")
    ap_ = sub.add_parser("apply")
    ap_.add_argument("doc")
    ap_.add_argument("--sig")
    for name in ("promote", "lift"):
        q = sub.add_parser(name)
        q.add_argument("id")
        q.add_argument("reason", nargs="?", default="")
    sub.add_parser("list")
    a = ap.parse_args()

    if a.cmd == "new":
        sys.stdout.write(json.dumps(new_doc(a), indent=2) + "\n")
    elif a.cmd == "propose":
        status, inc = call("GET", f"/incidents/{a.incident}")
        if status != 200:
            sys.exit(f"incident {a.incident}: {status}")
        props = inc.get("shield_proposals") or []
        if a.index >= len(props):
            sys.exit(f"incident {a.incident} has {len(props)} shield proposals")
        sys.stdout.write(props[a.index]["document"])
    elif a.cmd == "apply":
        # The signature covers the exact bytes, so the file is sent as it is.
        text = open(a.doc, encoding="utf-8").read()
        sig = open(a.sig or a.doc + ".sig").read().strip()
        show(*call("POST", "/shields", {"document": text, "signature": sig}))
    elif a.cmd == "promote":
        show(*call("POST", f"/shields/{a.id}/promote", {"reason": a.reason}))
    elif a.cmd == "lift":
        show(*call("DELETE", f"/shields/{a.id}", {"reason": a.reason}))
    else:
        show(*call("GET", "/shields"))


if __name__ == "__main__":
    main()
