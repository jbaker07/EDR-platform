# EDR Platform

A Linux endpoint detection and response prototype. A Rust agent reads eight eBPF sensors, matches the events against 22 detection rules and 13 correlation playbooks, and serves the resulting incidents on a local API behind a bearer token. An optional BPF LSM guard blocks three attack paths inside the kernel, and a separate FastAPI backend with a React dashboard stores and shows process telemetry and alerts.

**Status: prototype.** Built and tested on Linux 7.0.14 (linuxkit, arm64). Known limits are listed at the end.

| Path | What it is |
|---|---|
| `edr-agent/` | Rust agent (crate `forensic_hooks`, libbpf-rs), eBPF sensors and guard in `src/ebpf/`, guard user space in `src/guard/`, rules, playbooks, the agent UI |
| `backend/` | FastAPI app (`main.py`), stateless rules engine, `safe_expr` rule evaluator, SQLite by default |
| `dashboard/` | React and Vite console for the backend |
| `scripts/` | `policy_signer.py` (Ed25519 policy signing) and `send_real_telemetry.py` (psutil collector for the backend) |
| `tests/` | pytest suite for the backend and the policy signer |
| `server/` | Node.js publish service. The agent posts its decision engine incidents to it, and it forwards them to OpenSearch, Splunk HEC or syslog CEF per `config/connectors.json` |
| `ontology/` | JSON ontology the agent serves on `/ontology/*` |

## Architecture

### Agent and sensors

The `edr_attach_any` loader loads six sensors and pins their maps under `/sys/fs/bpf/edr`. raw_syscalls_catcher is loaded first so its ring `edr_events_rb` becomes the shared ring. The agent reads the pinned maps and loads dns_tunnel_monitor and suspicious_ipc itself. In all, 29 programs: 25 tracepoints and 4 kprobes.

| Sensor (`src/ebpf/*.bpf.c`) | Hook type | What it watches | Output |
|---|---|---|---|
| proc_lifecycle | 3 tracepoints (sched_process_fork, exec, exit) | Fork, exec with the filename and the first 256 bytes of argv, exit code | shared ring `edr_events_rb` |
| raw_syscalls_catcher | 2 tracepoints (raw_syscalls sys_enter, sys_exit) | Allowlisted syscalls with path, fd, return value and errno. Default allowlist: openat, openat2, write, close, renameat, renameat2, unlinkat. The agent's own tgid is skipped in the kernel | `edr_events_rb` |
| tcp_state | 2 kprobes (tcp_set_state, tcp_retransmit_skb) | TCP state changes and retransmits with addresses and ports | `edr_events_rb` |
| file_access_monitor | 1 tracepoint (sys_enter_openat) | Every openat with pid, uid, comm and path | perf array `events` |
| net_flow | 11 tracepoints (sys_enter_connect, sys_exit of sendto, sendmsg, recvfrom, recvmsg, 6 fd stashes) | connect destination (IPv4 or IPv6 and port) and bytes sent and received | ring `net_events` |
| wx_exec | 7 tracepoints (mmap, mprotect, memfd_create exit, write enter and exit, execve, execveat) | A writable mapping followed by an executable one within 5 s, and 4 KiB or more written to a memfd then executed within 120 s | ring `wx_events` |
| dns_tunnel_monitor | 2 kprobes (udp_sendmsg, udpv6_sendmsg) | The DNS question in UDP datagrams to port 53, scored in user space for long names, long labels, entropy and per domain volume | ring `dns_events`, loaded by the agent |
| suspicious_ipc | 1 tracepoint (sys_enter_connect) | connect() to 13 container runtime sockets (Docker, containerd, k3s, CRI-O, Podman, cri-dockerd) by a client that is not a known runtime tool | ring `ipc_events`, loaded by the agent |

### Prevention guard (BPF LSM)

`src/ebpf/edr_guard.bpf.c` holds five programs on LSM hooks, the checks the kernel makes right before it allows an action, so they can refuse it with EPERM. `src/guard/` loads them with libbpf-rs, fills the allowlists and turns decisions into TrustEvents and telemetry records.

| Guard | LSM hook | Refuses | MITRE |
|---|---|---|---|
| runtime_socket | `unix_stream_connect` | connect() to one of the 13 runtime sockets from an executable that is not an allowlisted client. The socket is matched by the name the daemon bound, the caller by executable inode and device, never by comm, so a python3 copy named `docker` is refused | T1611, T1610 |
| memfd_exec | `bprm_check_security` | execve of a memory only file made with memfd_create, unless the caller is allowlisted (runc is found by default) | T1620 |
| self_protect | `task_kill`, `ptrace_access_check`, `bpf` | Fatal signals, ptrace attach and lookups of the agent's BPF objects by any other process | T1562.001 |

* Each guard runs `off`, `monitor` (allow and log what would have been blocked, the default) or `enforce`, set by `EDR_GUARD_MODE` or a per guard override.
* The guard starts only when `bpf` is in `/sys/kernel/security/lsm`. Otherwise the agent logs one line and stays detect only. Self protection also needs the host pid namespace.
* Nothing is pinned, so the kernel detaches every guard when the agent exits for any reason.
* Off switch: create `/run/edr-guard/off` (holding the token if `EDR_GUARD_OFF_SHA256` is set). Within 200 ms self protection turns off and the agent can be stopped normally.
* Measured cost: a few hundred ns at most per unix stream connect. The hooks never run for TCP or file I/O.

### Rules and playbooks

* `edr-agent/src/rules.json` holds 22 rules (schema version 1, unknown keys rejected), reloaded when the file changes.
* `edr-agent/playbooks/` holds 31 YAML playbooks. 13 load, all Linux ransomware, wiper and container breakout chains. The other 18 use an older schema or fields the parser rejects, fact kinds the engine does not implement (NetFlow, RuleHit, K8sEvent, Net) or operators it lacks, and the loader logs the reason for each.
* Facts flow from the sensors through an adapter and a mapper into the playbook engine. Rule alerts and playbook hits become incidents on `/incidents`.
* Rule alerts, Mahalanobis and elliptic envelope outliers and KRIM-lite graph alerts also feed a decision engine (correlation, cooldowns, allow and suppress lists). Its incidents are posted to the publish service in `server/`.
* An optional graph scoring sidecar (`src/rgcn_service.py`) is off unless `RGCN_ENABLED=1`. Its default backend scores graph structure with NetworkX, and no trained model ships.

### Backend and dashboard

* `backend/` serves 20 routes under `/api`: telemetry ingest, stored process events and per host counts, alerts, agent registry, rules list and reload, rule evaluation, a dry run analyze, IOCs, feedback, collaboration, health, and `/api/predict`. The predict route serves a demo scikit-learn model trained on synthetic data. It is not a detector.
* The backend is fed by `scripts/send_real_telemetry.py`, not by the agent. The two halves do not talk to each other yet.
* `dashboard/` has six pages backed by stored data: Dashboard, Processes, Detection Rules, IOCs, Feedback Log, Collaboration.

## Security design

**Agent API.** It listens on 127.0.0.1 only. `agent_api::secure_router` puts the bearer token check on the whole protected router, SSE streams, nested `/playbooks` routes and the `/json_files` directory included, so a route added there later is covered. Only `/healthz` (which returns just `{"ok":true}`), `/` and `/ui/*` are public, because a browser cannot attach a header to a page load. They are static files from the repo and carry no data. The token comes from `EDR_AGENT_TOKEN` and must be 32 or more characters. The agent keeps only its SHA-256 digest and compares digests with OpenSSL's constant time memcmp. With no valid token the agent opens no port at all, logs why, and keeps sensing and detecting. The UI keeps the token in a closure in `ui/auth.js`, never in localStorage, a cookie or a URL, and reads streams with fetch because EventSource cannot send headers.

**CORS.** The agent sends no CORS headers to any origin unless it is listed exactly in `EDR_AGENT_CORS_ORIGINS`. `*` is refused and credentials are never allowed. The backend allows one origin, `EDR_DASHBOARD_ORIGIN`, with GET and POST and the Authorization and Content-Type headers, without credentials.

**Signed policy.** `edr-agent/policy.json` is signed with Ed25519. The detached signature in `policy.json.sig` (hex) covers the exact file bytes, so there is no canonical JSON form to agree on. The public key is pinned outside the signed file, in `config/policy_pub.pem` or the path in `EDR_POLICY_PUBKEY`. A bad, missing or wrong key signature makes the agent exit with code 1 before the guard or any sensor reader starts. `scripts/policy_signer.py` (keygen, sign, verify) calls the openssl CLI. It refuses a private key path inside the repo or a key that group or others can read, never overwrites a key, and checks each new signature against the public key. The private key is kept outside the repo.

**Backend auth.** `FastAPI(dependencies=[Depends(require_token)])` puts the token check on all 20 routes, including the ones defined in `main.py`. Without an `EDR_API_TOKEN` of 32 or more characters the app refuses to import, so uvicorn exits before binding a port. Tokens are compared with `hmac.compare_digest`. `/docs` and `/openapi.json` exist only in dev mode. The dashboard takes the token in a sign in form and holds it in a module variable, never in the bundle or localStorage.

**Dev flag.** `EDR_DEV_NO_AUTH=1` (exactly `1`) opens both APIs without a token and turns a bad policy signature into a loud warning. In the agent a set token wins over the flag, and a short token is refused even with it.

**Secrets at rest.** Integration credentials posted to the agent are stored in `state/integrations.json` at mode 0600, written through a temp file and a rename with O_NOFOLLOW. A file with a looser mode is tightened at startup. `GET /integrations` masks every credential field (`****` plus the last 4 characters for values of 12 or more characters). Tokens are read from the environment and never logged. `.gitignore` excludes every `.pem` except the public key.

**No eval().** `POST /api/rules/eval` used to run each rule with `eval()`, and an empty globals dict does not sandbox it, so a rule could run any command. `backend/app/rules/safe_expr.py` parses each rule into a Python syntax tree, checks every node against an allowlist (the caller's field names, literals, and, or, not, comparisons, `in`, arithmetic on numbers, five string methods) and walks the tree itself. Imports, other attribute access and calls, indexing, lambdas, comprehensions and dunder names are rejected before the rule runs. No `eval` or `exec` call is left in the backend.

## Build and run

Requirements: Linux with kernel BTF and root. Rust, clang, llvm, libbpf-dev, libelf-dev, zlib1g-dev, libssl-dev, pkg-config and bpftool (Debian package names). Python 3.10 or newer, Node.js 20, and the openssl CLI 3.0 or newer for the signer.

```bash
make -C edr-agent/src/ebpf      # 8 sensors plus edr_guard.bpf.o, built with -Werror
gcc -O2 -Wall -o edr-agent/src/ebpf/edr_attach_any edr-agent/src/ebpf/edr_attach_any.c -lbpf -lelf -lz
(cd edr-agent && cargo build --release)
```

Load the six pinned sensors, raw_syscalls_catcher first. bpffs and tracefs must be mounted.

```bash
cd edr-agent/src/ebpf
for s in raw_syscalls_catcher proc_lifecycle net_flow wx_exec tcp_state file_access_monitor
do sudo ./edr_attach_any -p /sys/fs/bpf/edr "$s.bpf.o"
done
```

Run the agent from `edr-agent/`, because the policy, key, rules, playbooks and UI are found relative to it. The guard starts in monitor mode unless `EDR_GUARD_MODE` says otherwise.

```bash
cd edr-agent
export EDR_AGENT_TOKEN="$(openssl rand -hex 32)"
export EDR_BPF_DNS_OBJ=src/ebpf/dns_tunnel_monitor.bpf.o
sudo -E ./target/release/forensic_hooks
```

Open `http://127.0.0.1:8080/` and sign in with the token. From another shell with the token set, keep it off the command line, since the sensors record every argv:

```bash
( umask 077 && printf 'Authorization: Bearer %s\n' "$EDR_AGENT_TOKEN" > "$HOME/.edr_auth_header" )
curl -s -H @"$HOME/.edr_auth_header" http://127.0.0.1:8080/incidents
```

With `EDR_GUARD_MODE=enforce`, other processes cannot signal the agent. Create the off file, wait for the log line, then send SIGINT. Ctrl+C in its own terminal still works. After editing `policy.json`, sign it again with `python3 scripts/policy_signer.py sign edr-agent/policy.json`.

Backend first, then the collector and the dashboard from a second shell at the repo root with the same `EDR_API_TOKEN`. Sign in to the dashboard with that token.

```bash
cd backend
python3 -m venv .venv && . .venv/bin/activate
pip install -r requirements.txt
export EDR_API_TOKEN="$(openssl rand -hex 32)"
uvicorn main:app --port 8000
```

```bash
pip install psutil requests
python3 scripts/send_real_telemetry.py     # one pass of process telemetry
cd dashboard && npm ci && npm run dev      # http://localhost:5173, proxies /api to port 8000
```

### Environment variables

| Variable | Meaning |
|---|---|
| `EDR_AGENT_TOKEN` | Agent API bearer token, 32+ characters. Unset means no API, sensing goes on |
| `EDR_DEV_NO_AUTH` | `1` opens both APIs without a token and accepts an unverified policy with a warning. Dev machines only |
| `EDR_AGENT_CORS_ORIGINS` | Comma separated exact origins allowed to call the agent API |
| `EDR_POLICY_PUBKEY` | Public key for `policy.json.sig` (default `config/policy_pub.pem`) |
| `EDR_PORT` | Agent API port on 127.0.0.1 (default 8080) |
| `EDR_GUARD_MODE` | `off`, `monitor` or `enforce` for every guard (default `monitor`) |
| `EDR_GUARD_SOCKET_MODE`, `EDR_GUARD_MEMFD_MODE`, `EDR_GUARD_SELF_MODE` | Overrides for runtime_socket, memfd_exec and self_protect |
| `EDR_BPF_GUARD_OBJ` | Guard object (default `$EDR_BPF_DIR/edr_guard.bpf.o`, else `src/ebpf/edr_guard.bpf.o`) |
| `EDR_GUARD_SOCKET_CLIENTS` | Extra runtime client executables, colon separated |
| `EDR_GUARD_MEMFD_ALLOW` | Extra executables allowed to run a memfd, colon separated |
| `EDR_GUARD_SCOPE_CGROUP` | A cgroup v2 directory. Guards then judge only tasks in exactly that cgroup |
| `EDR_GUARD_OFF_FILE` | Self protection off switch (default `/run/edr-guard/off`) |
| `EDR_GUARD_OFF_SHA256` | If set, the off file must hold a token with this SHA-256 |
| `EDR_PIN_PREFIX` | Pinned sensor maps (default `/sys/fs/bpf/edr`) |
| `EDR_FORCE_PINNED` | `1` selects pinned mode even when no pins are found yet |
| `EDR_BPF_DIR` | Folder for `suspicious_ipc.bpf.o` and `edr_guard.bpf.o` |
| `EDR_BPF_IPC_OBJ` | suspicious_ipc object (default `$EDR_BPF_DIR/suspicious_ipc.bpf.o`, else `src/ebpf/suspicious_ipc.bpf.o`) |
| `EDR_BPF_DNS_OBJ` | dns_tunnel_monitor object (default `/opt/edr-ebpf/dns_tunnel_monitor.bpf.o`, ignores `EDR_BPF_DIR`) |
| `EDR_RAW_SYSCALLS` | Raw sensor allowlist: unset or `default`, a comma list of syscall numbers, or `none` to leave the map alone |
| `EDR_PERF_PAGES` | Pages per CPU for the file_access_monitor perf buffer (default 256) |
| `PB_DIR`, `IOC_DIR`, `ONTOLOGY_DIR`, `KRIM_BASELINE` | Playbooks (default `playbooks`), IOC lists (`state/ioc`), ontology (searched, finds `../ontology`), KRIM-lite baseline (`state/krim_baseline.json`) |
| `HOSTNAME` | Host name in incidents (default `localhost`) |
| `EDR_BOOTSTRAP` | `1` skips trust scoring |
| `EDR_LOG_DIR` | Agent log folder (default `edr-agent/logs`, relative to the working directory) |
| `EDR_TELEMETRY_DIR`, `EDR_TELEMETRY_FILE_PREFIX`, `EDR_TELEMETRY_FLUSH_SECS`, `EDR_TELEMETRY_MAX_BUFFER`, `EDR_TELEMETRY_ROLLING` | Telemetry writer: folder (`telemetry_output`), file prefix (`telemetry`), flush interval (30 s), buffer (10,000 records), rolling by `ts`, `hour` or `day` |
| `EDR_DISABLE_KV` | `1` stops `kv_telemetry.jsonl` rows |
| `EDR_OUT_DIR`, `EDR_GNN_DIR`, `EDR_GNN_FIFO` | Graph vector logs: process monitor base folder (`/var/tmp/edr-agent`), graph hook folder (`edr-agent/json_files`), FIFO (`/tmp/gnn_input.pipe`) |
| `PUBLISH_BUS_URL`, `PUBLISH_BUS_DISABLED` | Where incidents are posted (default `http://127.0.0.1:8787/api/incidents`), and `1` to stop posting |
| `RGCN_ENABLED`, `RGCN_URL`, `RGCN_POLL_MS`, `RGCN_TIMEOUT_MS`, `RGCN_MAX_NODES`, `RGCN_MAX_EDGES` | Graph scoring sidecar: on switch, URL (default `http://127.0.0.1:9000/infer`), poll interval, timeout, graph size caps |
| `RUST_LOG`, `EDR_VERBOSE`, `EDR_DEBUG_RAW_EVENTS`, `EDR_PCB_DUMP_RAW` | Log filter (`telemetry=trace` prints raw records), `1` prints raw records, `1` adds a hex copy of every frame, `1` dumps PCB records to `state/pcb_raw` |
| `EDR_API_TOKEN` | Backend bearer token, 32+ characters. The collector sends it too |
| `EDR_DASHBOARD_ORIGIN` | The one browser origin the backend allows (default `http://localhost:5173`) |
| `DATABASE_URL` | Backend database (default `sqlite:///./edr.db`, relative to the working directory) |
| `PREDICT_MODEL_PATH` | Demo model for `/api/predict` (default `backend/ml/models/demo_classifier.joblib`) |
| `EDR_API_URL` | Collector target (default `http://localhost:8000/api/telemetry`) |
| `PUBLISH_HOST`, `PUBLISH_PORT`, `SPLUNK_HEC_TOKEN` | Publish service address (default 127.0.0.1) and port (default 8787, `PORT` also works), and the HEC token `config/connectors.json` reads when Splunk is on |

## Tests

```bash
(cd edr-agent && cargo test)         # 195 lib + 24 bin tests, on Linux
python3 -m pytest tests -q            # 75 tests, needs the backend requirements plus pytest and httpx
(cd dashboard && npm run build && npm run lint)   # builds, lint shows 1 warning and 0 errors
```

* Agent API: without a token a plain route, an SSE stream, a nested route and the static directory all answer 401, and so do a wrong token, the Basic scheme and an empty Bearer. A foreign origin gets no CORS headers. Stored secrets never appear in `GET /integrations`, and the secrets file ends 0600 with a planted symlink refused.
* Policy: one changed byte, a wrong key, an RSA key, a missing or placeholder signature all fail, and an unsigned edit to the shipped policy fails `cargo test`.
* Playbooks: each of the 13 has a fires once, a cooldown and a near miss test. Guard: event layout against the C struct, mode parsing, client lookup by inode, off switch.
* Backend: every route carries the token dependency and answers 401 without or with a wrong token, and `safe_expr` rejects injection attempts such as `__import__('os').system(...)`.
* Live end to end (privileged container, host pid namespace, guard enforcing): all 13 loaded playbooks fired from benign stand in activity and reached `/incidents` (db_impact in a run of its own). The DNS and IPC sensors raised their alerts. The guard refused python3, a python3 copy named docker and a python3 renamed with prctl on `docker.sock`, a memfd exec, and `kill -9` of the agent. Every protected route answered 401 without the token, and a policy with one changed byte stopped the agent. Unknown frames on the shared ring stayed at 0.
* The dashboard builds and lints, but its pages were not clicked through in a browser test.

## Known limits

1. The playbook adapter channel holds 4,096 facts and drops facts when full. Under big bursts a playbook sitting on its threshold can miss: db_impact missed in both final runs that fired eight triggers together with the guard enforcing (7 of 8 playbooks fired), and fired in a run of its own.
2. A guard block becomes a TrustEvent and a telemetry row, not an incident. No rule or playbook turns guard records into incidents yet.
3. Sensors store full command lines, unredacted. A secret passed as an argument lands in `kv_telemetry.jsonl`.
4. Each API uses one static shared token. Rotation needs a restart, root can read it from `/proc/<pid>/environ`, and the agent API is plain HTTP on loopback.
5. Integration credentials are plain text in a 0600 file, and no connector reads them yet. The signed policy is verified, but no code reads its fields yet.
6. `/actions/isolate_host`, `/actions/kill_proc` and `/actions/block_egress` only log and echo the request.
7. Name based playbooks (luks, hostpath, db, fuse) and the suspicious_ipc detector trust executable names or comm, so a renamed tool evades them. The guard matches by inode instead.
8. Root can still get past the guard: `cgroup.kill`, a kernel module, a reboot without `bpf` in `lsm=`, or the off file when `EDR_GUARD_OFF_SHA256` is not set. The socket guard checks at connect time only and does not cover the TCP Docker API.
9. Raw syscall numbers are arm64 only, so the agent fills the raw allowlist only on arm64. file_access_monitor has no kernel self filter, so the agent's own opens are dropped in user space.
10. The publish service in `server/` has no authentication, so it listens on 127.0.0.1 only unless `PUBLISH_HOST` says otherwise.
11. The agent UI loads Tailwind, Chart.js and D3 from public CDNs, so it needs internet access and trusts those hosts.
