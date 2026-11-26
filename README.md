# EDR Platform

A Linux endpoint detection and response prototype. A Rust agent reads eight eBPF sensors, matches the events against 22 detection rules and 29 correlation playbooks, and serves the resulting incidents on a local API behind a bearer token. An optional BPF LSM guard blocks three attack paths inside the kernel and turns every block into an incident. Runtime shields add prevention that changes while the agent runs: signed documents that refuse one executable, one file or one network destination, each a row in a BPF map with its own deadline in the kernel. The API can kill a process, block egress to an address or isolate the host with nftables, with an optional expiry the kernel enforces. The agent forwards its incidents to a FastAPI backend, whose React dashboard shows them next to the backend's own process telemetry and alerts.

**Status: prototype.** Built and tested live on Linux 7.0.14 (linuxkit, arm64), and the runtime shields, the root tamper table and the overhead numbers on Ubuntu 26.04.1 (kernel 7.0.0-43-generic, arm64) in a Multipass VM, where bpf is in the LSM list by default. On x86_64, `.github/workflows/x86_64.yml` builds the objects and the agent on GitHub's Ubuntu runner (kernel 7.0.0-1020-azure, 4 CPUs, bpf in the LSM list) and runs the live smoke test and the live shield test there. Known limits are listed at the end.

| Path | What it is |
|---|---|
| `edr-agent/` | Rust agent (crate `forensic_hooks`, libbpf-rs), eBPF sensors, guard and shields in `src/ebpf/`, guard user space in `src/guard/`, shields in `src/shield/`, rules, playbooks, the agent UI |
| `backend/` | FastAPI app (`main.py`), stateless rules engine, `safe_expr` rule evaluator, SQLite by default |
| `dashboard/` | React and Vite console for the backend |
| `scripts/` | `policy_signer.py` (Ed25519 signing of policies and shield documents), `shieldctl.py` (write, apply, promote and lift shields), `send_real_telemetry.py` (psutil collector for the backend), and `ci/` (the live smoke test, the shield test, the root tamper table, the shield benchmark and the BPF atomics check) |
| `tests/` | pytest suite for the backend, the agent to backend contract and the policy signer |
| `server/` | Node.js publish service. The agent posts its decision engine incidents to it, and it forwards them to OpenSearch, Splunk HEC or syslog CEF per `config/connectors.json` |
| `ontology/` | JSON ontology the agent serves on `/ontology/*` |

## Architecture

### Agent and sensors

The `edr_attach_any` loader loads six sensors and pins their maps under `/sys/fs/bpf/edr`. raw_syscalls_catcher is loaded first so its ring `edr_events_rb` becomes the shared ring. The agent reads the pinned maps and loads dns_tunnel_monitor and suspicious_ipc itself. In all, 22 programs: 18 tracepoints and 4 kprobes.

| Sensor (`src/ebpf/*.bpf.c`) | Hook type | What it watches | Output |
|---|---|---|---|
| proc_lifecycle | 3 tracepoints (sched_process_fork, exec, exit) | Fork, exec with the filename and the first 256 bytes of argv, exit code | shared ring `edr_events_rb` |
| raw_syscalls_catcher | 2 tracepoints (raw_syscalls sys_enter, sys_exit) | Allowlisted syscalls with path, fd, return value and errno. Default allowlist: openat, openat2, write, close, renameat, renameat2, unlinkat, plus rename and unlink on x86_64. The agent's own tgid is skipped in the kernel | `edr_events_rb` |
| tcp_state | 2 kprobes (tcp_set_state, tcp_retransmit_skb) | TCP state changes and retransmits with addresses and ports | `edr_events_rb` |
| file_access_monitor | 1 tracepoint (raw_syscalls sys_enter, openat only) | Every openat with pid, uid, comm and path. The agent's own tgid is skipped in the kernel | perf array `events` |
| net_flow | 6 tracepoints (raw_syscalls sys_enter for write and sendto, sys_enter_connect, sys_exit of sendto, sendmsg, recvfrom, recvmsg) | connect destination (IPv4 or IPv6 and port), uid, bytes sent and received, and a copy of the first 1,024 bytes a process writes on a socket it connected to port 443 or 8443 when they start a TLS handshake record. User space reads the server name (SNI) and ALPN from that ClientHello (`src/tls_hello.rs`) | ring `net_events` |
| wx_exec | 5 tracepoints (mmap, mprotect, memfd_create exit, write exit, and raw_syscalls sys_enter for write, execve and execveat) | A writable mapping followed by an executable one within 5 s, and 4 KiB or more written to a memfd then executed within 120 s | ring `wx_events` |
| dns_tunnel_monitor | 2 kprobes (udp_sendmsg, udpv6_sendmsg) | The DNS question in UDP datagrams to port 53, scored in user space for long names, long labels, entropy and per domain volume | ring `dns_events`, loaded by the agent |
| suspicious_ipc | 1 tracepoint (sys_enter_connect) | connect() to 13 container runtime sockets (Docker, containerd, k3s, CRI-O, Podman, cri-dockerd) by a client that is not a known runtime tool | ring `ipc_events`, loaded by the agent |

For an exec, the agent reads `/proc/<pid>` right after the event. When `/proc` already shows a later program (a shell that ran its last command in place within a millisecond, or a reused pid), the filename and argv proc_lifecycle copied in the kernel win. The same read takes a short allowlist of environment variables (CI markers such as `CI` and `GITHUB_ACTIONS`, set with `EDR_ENV_ALLOWLIST`, a name that looks like a secret is refused) and never the rest of the environment. Exec facts also carry the package that installed the executable and whether its bytes still match the package's checksum (`src/pkg_verify.rs`: dpkg's file lists and md5sums, or `rpm -qf` and `rpm -V`, checked once per file version), the account name from `/etc/passwd`, the local hour and weekday, and `EDR_HOST_ROLE` when the site sets one. The agent also reads two sources outside eBPF for the playbooks: CPU time from `/proc/stat` and `/proc/<pid>/stat` and GPU busy percent from `/sys/class/drm/card*/device/gpu_busy_percent` or `nvidia-smi` every 5 s (`src/modules/host_perf.rs`), and sshd login lines from the auth log (`src/modules/auth_log.rs`).

### Prevention guard (BPF LSM)

`src/ebpf/edr_guard.bpf.c` holds twelve programs on LSM hooks, the checks the kernel makes right before it allows an action, so they can refuse it with EPERM. `src/guard/` loads them with libbpf-rs, fills the allowlists and turns decisions into TrustEvents, telemetry records, RuleHit facts and incidents.

| Guard | LSM hook | Refuses | MITRE |
|---|---|---|---|
| runtime_socket | `unix_stream_connect` | connect() to one of the 13 runtime sockets from an executable that is not an allowlisted client. The socket is matched by the name the daemon bound, the caller by executable inode and device, never by comm, so a python3 copy named `docker` is refused | T1611, T1610 |
| memfd_exec | `bprm_check_security` | execve of a memory only file made with memfd_create, unless the caller is allowlisted (runc is found by default) | T1620 |
| self_protect | `task_kill`, `ptrace_access_check`, `bpf`, `bpf_map`, `bpf_prog`, `file_open`, `inode_unlink`, `inode_rename`, `inode_link`, `path_truncate` | Fatal signals, ptrace attach, any new handle to the agent's BPF programs, maps and links (by id, by a pinned path under any name, or through a bind mount for maps and programs), opening a shield pin by path, and writes, unlinks, renames, hard links and truncates of the agent's binary, public key, anti-rollback state and shield pins, by any other process | T1562.001 |

* Each guard runs `off`, `monitor` (allow and log what would have been blocked) or `enforce`, set by `EDR_GUARD_MODE` or a per guard override. The signed policy's `guard_mode` is the lowest mode allowed: a variable can raise a guard from monitor to enforce, and a request for less is overruled with a warning. The policy's `guard_allow` adds allowlist entries by path.
* The modes and allowlists sit in two banks, and one config slot says which bank the hooks read. A new signed policy fills the other bank and flips that one slot, so a hook sees the old set or the new set, never a mix.
* Every block and every would block becomes an incident on `/incidents` (and in `logs/guard_alerts.jsonl`) with a `details` block that names the guard, the hook, the caller's exe, pid and comm, the target and the mode (`enforce` for a block, `monitor` for a would block). Blocks are high severity, would blocks medium.
* The guard starts only when `bpf` is in `/sys/kernel/security/lsm`. Otherwise the agent logs one line and stays detect only. Self protection also needs the host pid namespace.
* Nothing of the guard is pinned, so the kernel detaches every guard when the agent exits for any reason. The runtime shields are pinned instead, because each shield ends on its own.
* Off switch: create `/run/edr-guard/off` (holding the token if `EDR_GUARD_OFF_SHA256` is set). Within 200 ms self protection turns off and the agent can be stopped normally.
* Measured cost of the original three guards: a few hundred ns at most per unix stream connect. What the whole agent costs other programs (sensors, guard and shields together) is measured under Runtime shields below.
* While the agent runs, the guard also protects every BPF object it holds a descriptor for (the sensor maps its readers opened included) and the sensors pinned under `EDR_PIN_PREFIX`: their links, programs, maps and pin files. Removing a sensor's link pin would detach it, so this is what keeps root from blinding the agent with `rm`. In the live run that was 107 objects and 47 pins.

### Runtime shields

A shield is prevention an operator, or a playbook the policy trusts, adds while the agent runs. `src/ebpf/edr_shield.bpf.c` holds four general programs, loaded once. A shield is one row in one of their maps, so adding, promoting or lifting a shield is a map write and never loads new kernel code.

| Kind | LSM hook | Refuses | Map |
|---|---|---|---|
| exec_deny | `bprm_check_security` | execve of one file, named by device and inode | hash on (inode, device) |
| file_deny | `file_open` | opening one file, named the same way. An exec opens the file too, so this also stops running it, and it stops copying it to a new inode | hash on (inode, device) |
| connect_deny | `socket_connect`, `socket_sendmsg` | connect() to an address or network, on one port or all ports, and sendto() or sendmsg() with that destination on an unconnected socket. An IPv4 mapped IPv6 address is judged as IPv4 | LPM trie on (port, family, address) |

* Each row carries its deadline (and its start) in CLOCK_BOOTTIME nanoseconds, checked with `bpf_ktime_get_boot_ns()` on every match. The block ends on time while the agent is stopped, hung or dead, and setting the wall clock cannot stretch it. The agent also removes expired rows once a second.
* Each row can be limited to a cgroup v2 directory and the cgroups below it. `EDR_GUARD_SCOPE_CGROUP` limits every shield as well.
* The links and maps are pinned under `/sys/fs/bpf/edr_shield` (`EDR_SHIELD_PIN_DIR`), so shields outlive the agent until their deadlines. At start the agent reuses the pinned maps, rows and hit counts included, attaches the programs from the object file, and swaps the link pins with no gap. While it runs, the guard refuses other processes a handle to the shield programs and maps, and refuses to open, unlink, rename or hard link the pins.
* The programs never write to a rule map, and no BPF program here uses an atomic instruction. Hit counts and the report rate limit live in a per CPU array indexed by the shield's number. `scripts/ci/check_bpf_atomics.py` fails the build on any atomic instruction (see Known limits for why).
* Every hit in enforce mode is refused with EPERM. Every hit is counted in the kernel, and at most one report per shield per CPU per second becomes a shield incident with the shield, the caller, the target and the mode.
* Measured overhead (`scripts/ci/shield_bench.sh`, Ubuntu 26.04.1, kernel 7.0.0-43, 6 vCPUs, median of 9 rounds, November 24 after the sensors moved to raw_syscalls sys_enter). The shield rows are real signed shields that never match what the benchmark does:

| Setup | exec (posix_spawn and wait) | open and close | UDP connect() | TCP connect to loopback |
|---|---|---|---|---|
| No agent | 275 µs | 663 ns | 318 ns | 9.8 µs |
| Agent with sensors and guard, no shield programs | 538 µs | 1,911 ns | 547 ns | 14.3 µs |
| Shield programs, 0 rows | 422 µs | 1,858 ns | 646 ns | 15.1 µs |
| 100 rows of each kind | 334 µs | 1,859 ns | 638 ns | 15.3 µs |
| 10,000 rows of each kind | 518 µs | 1,937 ns | 662 ns | 15.3 µs |

  The number of rows does not change the cost beyond the run to run noise: a hash or trie lookup does not grow with the rows the way a list would. The shield programs themselves add about 100 ns to a UDP connect. Most of the agent's cost is the sensors and the agent's own work on each event, and the exec figures move by more than that between runs on the same setup. An open with shields loaded costs about 1.9 µs now, against about 1.3 µs before the move to raw_syscalls sys_enter. The older figure was too low: file_access_monitor only ran on CPU 0 then (see Known limits), so 5 of the 6 CPUs paid nothing for it. Adding 29,700 shields through the API took 41 s. Memory, measured on an idle VM with no benchmark running: the agent started at about 188 MB, and 30,000 shields (10,000 of each kind) took it to 303 MB. Before two fixes on November 25 the same 30,000 shields took it to 437 MB, and three full `GET /shields` listings added another 115 MB that never came back. A snapshot no longer copies every shield before writing it, and the agent calls `malloc_trim` every 5 s so freed heap goes back to the kernel. Three listings now add 9 MB.

**Documents.** A shield comes from a JSON document signed with the policy key, Ed25519 over the exact bytes like `policy.json` (`edr-agent/tests/fixtures/shield_doc.json` is the format, checked by a Rust test and a pytest):

```json
{"shield": {"id": "c2-from-inc-12", "serial": 3, "kind": "connect_deny",
  "match": {"cidr": "203.0.113.10", "port": 4444}, "scope": {"cgroup": "/sys/fs/cgroup/system.slice/nginx.service"},
  "mode": "enforce", "not_before": 1795500000, "expires_at": 1795503600, "incident_id": "5f48249046bf..."}}
```

exec_deny and file_deny name the file by `dev` and `ino` as `stat -c '%d %i' FILE` prints them (a `path` is a label only). `mode` is the highest mode the shield may reach. The agent refuses, and writes to `logs/shields_audit.jsonl`:

* a bad signature (403)
* a serial at or under the last one seen for that id, which also stops replaying a lifted shield (409)
* a lifetime over the policy's `shields.max_lifetime_sec` (403)
* a target on the never-block list (403): init, systemd, sshd, the agent's binary, key and state, the dynamic loader and libc, nft and iptables, loopback, the backend, a network of every address, and the policy's `shields.never_block` paths and networks
* more active shields than `shields.max_active` (429), or a second shield on the same target (409)

**Monitor first.** Every shield starts in monitor mode: the kernel lets the action through, counts it and reports would block. `POST /shields/{id}/promote` moves it to enforce, if its document says `enforce`. The policy can promote kinds it lists in `shields.auto_promote` after `trial_sec`.

**API.** `POST /shields` (`{"document": "<exact text>", "signature": "<hex>"}`), `GET /shields`, `POST /shields/{id}/promote` and `DELETE /shields/{id}`, behind the agent token. Every change and every refusal is a line in the audit log. The agent UI has a shields panel with the mode, hits, expiry and the linked incident, and promote and lift buttons. State is kept in `state/` as a snapshot plus a journal that is flushed before the API answers, so an applied shield and its serial survive a crash.

**Proposals.** A playbook can name `shield_templates`. When it fires, each template becomes an unsigned document on the incident (`shield_proposals`), ready to sign. The webshell reverse shell playbook proposes connect_deny of the shell's destination and port for the shell's cgroup, for one hour. The in-memory exec to child to egress playbook proposes exec_deny of the child when it was dropped in /tmp, /dev/shm, /var/tmp or /run/user. The agent applies a proposal by itself only for kinds the signed policy lists in `shields.auto_apply`, capped at its `max_lifetime_sec`, and it still starts in monitor mode.

```bash
python3 scripts/shieldctl.py propose <incident id> > doc.json     # or: shieldctl.py new exec_deny --path /tmp/x --reason ...
python3 scripts/policy_signer.py sign doc.json                     # writes doc.json.sig
EDR_AGENT_TOKEN_FILE=$HOME/.edr_agent_token python3 scripts/shieldctl.py apply doc.json
EDR_AGENT_TOKEN_FILE=$HOME/.edr_agent_token python3 scripts/shieldctl.py promote <shield id> "trial clean"
```

### Rules and playbooks

* `edr-agent/src/rules.json` holds 22 rules (schema version 1, unknown keys rejected), reloaded when the file changes. A rule that matches is also a RuleHit fact for the playbooks.
* `edr-agent/playbooks/` holds 29 YAML playbooks and all 29 load. The 13 ransomware, wiper and container chains that loaded before are joined by 16 that were ported from older schemas or rejected fields: archive then exfiltration, cloud CLI exfiltration, cryptominer, in-memory encryptor, sensitive reads to a tmpfs archive to a new peer, SSH port forward pivot, Linux credential dumping, curl to `/dev/shm` to systemd-run, in-memory process to child to egress, EDR tamper, egress flood, cloud object store impact, exfiltration then encryption, SSH valid credentials pivot, terminal tooling, and webshell reverse shell.
* Two playbooks were moved out of the repo because they need data the agent does not collect at all: Kubernetes cluster sabotage (Kubernetes audit events) and cloud IAM lockout (cloud IAM audit logs, and the file had no slots).
* Fact kinds the engine matches: Exec, FileIO, NetFlow (net_flow connects, send bytes and TLS hellos with their server name), Net (tcp_state connections established, closed and still open after 60 s), Auth (sshd lines), RuleHit (rules.json hits and guard decisions), HostPerf (CPU and GPU samples) and VendorAlert. `src/pb_wiring.rs` lists the fields of each.
* Condition operators: eq, ne or neq, gt, gte, lt, lte, contains, not_contains, contains_any, regex, not_regex, in, not_in, any_in, in_cidr, not_in_cidr, exists, in_slot (a field equal to the same field of another slot's live hit, so steps can be tied to one process), and `any_of` lists. A condition on a field the fact does not carry is false. Unknown keys in a condition or a slot are load errors.
* Counting slots: file_burst, partial_encrypt_burst, file_read_burst, errno_spike, exec_burst, proc_burst_workers, egress_volume, conn_burst, sustained (an unbroken run of HostPerf samples) and auth_fail_burst.
* `links` set order and timing between two slots: `follows_within_sec` (the second slot's hit is at or after the first's, at most N seconds later) or `within_sec` (either order), plus `requires` (`same_host`, `same_user`, `same_pid`, `same_exe`). A link counts once both slots have a hit. A broken link between two required slots keeps the chain open, and an optional slot whose hit breaks a link is left out of the evidence, with a line in the rationale. The pair in fileburst_crypto that asked for burst then crypto within 300 s and crypto then burst within 600 s, which held only when both fell in the same second, is now one `within_sec: 600` link.
* `context_keys` split a playbook's evidence into chains, such as one per `[host, user]`. Counting windows, hypotheses and cooldowns are all per chain, and a fact with no value for a key joins the chain with `-` for it. net_flow frames now carry the uid, and frames without one take the user of the process's exec.
* `emit_ground_truth: true` writes each hit to `logs/ground_truth.jsonl` (mode 0600, `EDR_GROUND_TRUTH_PATH` moves it) as a line labelled `attack` with the playbook, its techniques, the chain's context, and per slot the time, pid, exe, command line and path, so a model trainer can join it to the telemetry.
* The loader reads every key the shipped playbooks use and logs no warnings for them. An unknown top level key is still a warning, an unknown key inside a slot, a condition or a link is a load error.
* Real signals replaced the stand-ins the ports had: the TLS server name for pool and object store names (cryptominer, cloud object store, staging), the package check for the backup tools that veto three playbooks (a look-alike named `restic` in /tmp does not veto), `env.CI` and friends for fetches inside a CI job, GPU busy percent for the miner's optional GPU slot, account names for the jump box exception, the local weekday and hour for the weekend backup window, and `EDR_HOST_ROLE=bastion` for port forwarding on a bastion.
* Stand-ins remain only where a host agent cannot see the data, and each is explained in its YAML: ASN (the first successful login from an address since the agent started), reputation (a public destination on a web or DNS port), HTTP methods inside TLS (upload volume, since reading them needs a hook inside each client's own TLS library and the cloud CLIs bundle theirs), the source address of a web request (a known maintenance tool started by the web server), and change tickets and site policies, which live in other systems.
* Facts flow from the sensors into a bounded queue, then one engine thread maps and matches them in batches. Rule alerts, playbook hits and guard decisions become incidents on `/incidents`.
* Rule alerts, Mahalanobis and elliptic envelope outliers and KRIM-lite graph alerts also feed a decision engine (correlation, cooldowns, allow and suppress lists). Its incidents are posted to the publish service in `server/`.
* An optional graph scoring sidecar (`src/rgcn_service.py`) is off unless `RGCN_ENABLED=1`. Its default backend scores graph structure with NetworkX, and no trained model ships.

### Fact queue

The decoders push facts into one bounded queue (`src/pb_adapter/queue.rs`, 32,768 facts by default). When it is full the reader thread waits up to 250 ms for room instead of dropping, so the kernel ring buffer holds the next frames meanwhile. A fact that still finds no room is dropped and counted. Frames the mapper cannot use (fork, exit, the loader's W then X, recv byte counts) are left out before the queue. The engine runs on its own OS thread, not on the tokio runtime, and takes up to 1,024 facts per batch with one mapper lock and one engine lock. The engine clock tick and the long lived connection sweep run on the engine thread, and the CPU sampler on a thread of its own, because the batch loop can take far longer than 5 s per pass under a burst.

Telemetry records (for the rules, the anomaly taggers, the live graph, the trust scores and the telemetry files) go through a second queue of the same kind (`src/record_pipeline.rs`, 131,072 records, `EDR_RECORD_QUEUE`) with the same 250 ms backpressure. One OS thread takes chunks of up to 4,096 records and runs every stage on each chunk, so rule evaluation no longer waits for a 5 s loop on the tokio runtime. The side-effect monitors (auth log, sockets, scripts) run on a thread of their own and queue their records like the ring readers.

What made the record path slow, each found by timing the stages:

* The trust scoring read two JSON files, wrote a trust state file, appended two GNN log lines and printed four log lines for every record, 95% of the time in a replay of a live burst. It now reads its inputs once (and again when they change) and writes the newest value per endpoint once a second. The replay went from about 7,000 to about 90,000 records a second.
* Every incident update read all of `state/pcb_snaps.ndjson` and parsed 20,000 lines of it for the neighborhood graph, so a burst of rule hits stalled the thread for minutes. The graph now comes from the newest 20,000 process snapshots kept in memory, and an incident rebuilds it at most every 30 s of alert time.
* The process snapshot read about ten `/proc` files, the socket tables among them, for every record. It now samples a pid at most once per 5 s.
* The Mahalanobis and elliptic envelope taggers marked every record of an outlier chunk, so the tag_flags rule raised one alert per record (11,103 rule hits and about 2,300 incidents in one burst). The chunk is the outlier, so one record of it is tagged.

Both queues also stop waiting once a push has timed out: until the consumer has drained the queue to half, a full queue drops at once and counts it (`stalls_total`). A stalled consumer then costs one 250 ms wait instead of 250 ms per item, which kept a ring reader from blocking long enough for the kernel ring to overflow and take playbook facts with it.

Counting windows store a 64-bit hash of each counted key (path, directory, destination) instead of the string. Distinct counts only need equality, and a window can hold 200,000 events.

`GET /pipeline/status` shows both queues (depth, high water mark, waits, drops, stalls), `records_dropped_total`, the record thread's counts (records, chunks, rules evaluated, rule hits) and time per stage, the engine's counting windows and the events they hold, the ground truth lines written, the loaded playbooks, the redaction count, the exec frames taken from the kernel copy, and the forwarder counters. `/metrics` carries the fact queue counters.

### Response actions

`POST /actions/kill_proc`, `/actions/block_egress`, `/actions/isolate_host` and `/actions/release`, and `GET /actions/state`, all behind the agent token (`src/response.rs`).

* Every request needs a `reason` or an `incident_id` the agent knows, takes `"dry_run": true`, must be listed in the signed policy's `response_actions`, and is written to `logs/actions_audit.jsonl` (mode 0600), refused ones included.
* kill_proc sends SIGKILL. It refuses pid 0 and 1 and negative pids, the agent, a thread id that is not its process's leader, and kernel threads (PF_KTHREAD). It opens the process with pidfd_open, checks its start time again, and signals through pidfd_send_signal, so a pid that was reused after the checks is never hit.
* block_egress drops traffic to an address or network, optionally one port and protocol. isolate_host drops all traffic except loopback addresses, the management ports (the policy's `management_ports` plus the agent API port) and the backend address. release removes one block, the isolation, or with `"all": true` the whole table.
* The rules live in one nftables table, `inet edr_agent`, replaced as a whole in one `nft -f` transaction on every change. iptables (chains `EDR_EGRESS`, `EDR_ISO_OUT`, `EDR_ISO_IN`) is the fallback when nft is not installed. The rules outlive the agent on purpose, so killing the agent does not end an isolation.
* `"expires_in_sec": N` on block_egress or isolate_host ends the rule on time in the kernel. Each block's address is an element of a named nftables set of its own with an element timeout, and isolation drops traffic matching the sets `iso4` and `iso6` (0.0.0.0/0 and ::/0). When the element times out the rule matches nothing, whether the agent runs or not. A rebuild gives each element only the time it has left. isolate_host keeps no expiry unless asked, so an isolation still outlives the agent. iptables has no element timeouts, so it answers 501 to an expiry.

### Incident forwarding

With `EDR_BACKEND_URL` set, the agent sends every new or updated incident to `POST /api/incidents` on the backend every 5 s with the backend token (`src/forwarder.rs`). The newest version of an incident wins, a failed post is retried, and the graph snapshot stays on the agent. Plain http is refused for a host that is not loopback, because the token would cross the network in clear text, unless `EDR_FORWARD_ALLOW_HTTP=1` is set for a test network. `edr-agent/tests/fixtures/agent_incident_batch.json` is the payload contract: a Rust test checks the agent builds exactly that payload, and a pytest posts it to the backend.

### Backend and dashboard

* `backend/` serves 23 routes under `/api`: telemetry ingest, stored process events and per host counts, alerts, agent incidents (post, list, one), agent registry, rules list and reload, rule evaluation, a dry run analyze, IOCs, feedback, collaboration, health, and `/api/predict`. The predict route serves a demo scikit-learn model trained on synthetic data. It is not a detector.
* The backend is fed by the agent's incident forwarder and by `scripts/send_real_telemetry.py`.
* `dashboard/` has seven pages backed by stored data: Dashboard, Agent Incidents, Processes, Detection Rules, IOCs, Feedback Log, Collaboration.

## Security design

**Agent API.** It listens on 127.0.0.1 only. `agent_api::secure_router` puts the bearer token check on the whole protected router, SSE streams, nested `/playbooks` routes, the response actions and the `/json_files` directory included, so a route added there later is covered. Only `/healthz` (which returns just `{"ok":true}`), `/` and `/ui/*` are public, because a browser cannot attach a header to a page load. They are static files from the repo and carry no data. The UI's scripts and styles are served from `ui/vendor`, so the page loads nothing from a public CDN. The token comes from `EDR_AGENT_TOKEN_FILE` (an owner-only file, re-read every 2 s, so the token can be rotated without a restart) or `EDR_AGENT_TOKEN`, and must be 32 or more characters. The agent keeps only its SHA-256 digest and compares digests with OpenSSL's constant time memcmp. With no valid token the agent opens no port at all, logs why, and keeps sensing and detecting. The UI keeps the token in a closure in `ui/auth.js`, never in localStorage, a cookie or a URL, and reads streams with fetch because EventSource cannot send headers.

**CORS.** The agent sends no CORS headers to any origin unless it is listed exactly in `EDR_AGENT_CORS_ORIGINS`. `*` is refused and credentials are never allowed. The backend allows one origin, `EDR_DASHBOARD_ORIGIN`, with GET and POST and the Authorization and Content-Type headers, without credentials.

**Signed policy.** `edr-agent/policy.json` is signed with Ed25519 and carries a `serial`. The detached signature in `policy.json.sig` (hex) covers the exact file bytes, so there is no canonical JSON form to agree on. The public key is pinned outside the signed file, in `config/policy_pub.pem` or the path in `EDR_POLICY_PUBKEY`. A bad, missing or wrong key signature makes the agent exit with code 1 before the guard or any sensor reader starts. The policy holds the settings an attacker would want to weaken, and the agent enforces each one: `guard_mode` (the lowest guard mode), `redact_command_lines` (when true, `EDR_REDACT=0` is refused), `response_actions` (which actions the API may run), `management_ports` (what isolation keeps open) and `logging.level` (when `RUST_LOG` is unset). Unknown fields and values the agent cannot act on are refused at load. The policy also holds `guard_allow` and the `shields` limits. The agent checks `policy.json` and its signature once a second and applies a new one while it runs: the signature must verify and the serial must be above the running one, so an older signed policy is refused (anti-rollback), and so is a second policy with the same serial. A refusal keeps the running policy, goes to `logs/policy_audit.jsonl` and raises a high severity incident. The highest serial applied and a copy of that policy are kept in `state/policy_serial` and `state/policy_last.json`, and at startup a `policy.json` under that serial is refused in favour of the copy, so a restart does not reopen the door. `GET /policy/status` shows the serial in force and the last reload or refusal. `scripts/policy_signer.py` (keygen, sign, verify) calls the openssl CLI. It refuses a private key path inside the repo or a key that group or others can read, never overwrites a key, and checks each new signature against the public key. The private key is kept outside the repo.

**Command line secrets.** Every command line is masked before it is stored, logged, matched or sent (`src/redact.rs`): at the sources (the kernel's argv copy, every `/proc/<pid>/cmdline` read) and again at the sinks (`kv_telemetry.jsonl`, the telemetry files, the GNN log, TrustEvent logs). Masked: password, token, secret, API key and similar flags and `key=value` or JSON pairs, Authorization and Bearer values and other secret headers, tool specific short flags (`mysql -pX`, `sshpass -p`, `curl -u user:X` and others), AWS access key ids and secret keys, PEM private key blocks, `user:password@` in URLs, and GitHub, Slack, Stripe and JSON Web Token shapes. The flag or key name stays, so a detection can still see that a password was passed. `EDR_REDACT_FLAGS` and `EDR_REDACT_KEYS` add names. `redactions_total` on `/metrics` counts masked command lines.

**Backend auth.** `FastAPI(dependencies=[Depends(require_token)])` puts the token check on all 23 routes, including the ones defined in `main.py`. Without a token of 32 or more characters, from `EDR_API_TOKEN_FILE` (owner-only, checked on every request and re-read when it changes) or `EDR_API_TOKEN`, the app refuses to import, so uvicorn exits before binding a port. Tokens are compared with `hmac.compare_digest`. `/docs` and `/openapi.json` exist only in dev mode. The dashboard takes the token in a sign in form and holds it in a module variable, never in the bundle or localStorage. `POST /api/incidents` checks every field with size limits and accepts at most 500 incidents per post.

**Dev flag.** `EDR_DEV_NO_AUTH=1` (exactly `1`) opens both APIs without a token and turns a bad policy signature into a loud warning. In the agent a set token wins over the flag, and a short token is refused even with it.

**Secrets at rest.** Integration credentials posted to the agent are stored in `state/integrations.json` at mode 0600, written through a temp file and a rename with O_NOFOLLOW. A file with a looser mode is tightened at startup. `GET /integrations` masks every credential field (`****` plus the last 4 characters for values of 12 or more characters). Tokens are read from files or the environment and never logged. `.gitignore` excludes every `.pem` except the public key.

**No eval().** `POST /api/rules/eval` used to run each rule with `eval()`, and an empty globals dict does not sandbox it, so a rule could run any command. `backend/app/rules/safe_expr.py` parses each rule into a Python syntax tree, checks every node against an allowlist (the caller's field names, literals, and, or, not, comparisons, `in`, arithmetic on numbers, five string methods) and walks the tree itself. Imports, other attribute access and calls, indexing, lambdas, comprehensions and dunder names are rejected before the rule runs. No `eval` or `exec` call is left in the backend.

## Build and run

Requirements: Linux with kernel BTF and root. Rust, clang, llvm, libbpf-dev, libelf-dev, zlib1g-dev, libssl-dev, pkg-config and bpftool (Debian package names). nftables or iptables for the response actions. Python 3.10 or newer, Node.js 20, and the openssl CLI 3.0 or newer for the signer.

```bash
make -C edr-agent/src/ebpf      # 8 sensors plus edr_guard.bpf.o and edr_shield.bpf.o, built with -Werror
python3 scripts/ci/check_bpf_atomics.py edr-agent/src/ebpf   # no atomic instruction in any program
gcc -O2 -Wall -o edr-agent/src/ebpf/edr_attach_any edr-agent/src/ebpf/edr_attach_any.c -lbpf -lelf -lz
(cd edr-agent && cargo build --release)
```

`make` builds for the machine it runs on, with the arm64 `vmlinux.h` next to the sources. For another CPU, name it and its kernel types. The objects land in `build/<arch>/`:

```bash
make -C edr-agent/src/ebpf TARGET_ARCH=x86 VMLINUX_H=include/x86_64/vmlinux.h
```

`include/x86_64/vmlinux.h` was generated with `bpftool btf dump` from the BTF inside Debian's `linux-image-6.1.0-56-amd64` package (6.1.192). CO-RE relocations adjust the objects to the running kernel at load. To cross build the agent from an arm64 Debian machine, add the `x86_64-unknown-linux-gnu` target with rustup, install `gcc-x86-64-linux-gnu` and the amd64 `libelf-dev`, `zlib1g-dev` and `libssl-dev`, and point `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER`, `CC_x86_64_unknown_linux_gnu` and `PKG_CONFIG_LIBDIR` at them (with `PKG_CONFIG_ALLOW_CROSS=1`).

`.github/workflows/x86_64.yml` runs on GitHub's `ubuntu-latest` (x86_64): it builds the objects and the agent, runs `cargo test`, and then `scripts/ci/live_smoke.sh` loads the sensors with sudo, starts the agent, runs `scripts/ci/triggers.py` and checks that four playbooks became incidents and that a TLS hello's server name reached `kv_telemetry.jsonl`. When `bpf` is in the LSM list it also checks that the guard refuses a memfd exec. GitHub's runners boot without it, so there the script checks that the agent says it runs detect-only and skips the guard check.

Load the six pinned sensors, raw_syscalls_catcher first. bpffs and tracefs must be mounted.

```bash
cd edr-agent/src/ebpf
for s in raw_syscalls_catcher proc_lifecycle net_flow wx_exec tcp_state file_access_monitor
do sudo ./edr_attach_any -p /sys/fs/bpf/edr "$s.bpf.o"
done
```

Run the agent from `edr-agent/`, because the policy, key, rules, playbooks and UI are found relative to it. The guard starts in the policy's `guard_mode` (monitor as shipped) unless `EDR_GUARD_MODE` raises it.

```bash
cd edr-agent
( umask 077 && openssl rand -hex 32 > "$HOME/.edr_agent_token" )
export EDR_AGENT_TOKEN_FILE="$HOME/.edr_agent_token"
export EDR_BPF_DNS_OBJ=src/ebpf/dns_tunnel_monitor.bpf.o
sudo -E ./target/release/forensic_hooks
```

Open `http://127.0.0.1:8080/` and sign in with the token. From another shell, keep the token off the command line, since the sensors record every argv (secrets on command lines are masked, but a file is better):

```bash
( umask 077 && printf 'Authorization: Bearer %s\n' "$(cat "$HOME/.edr_agent_token")" > "$HOME/.edr_auth_header" )
curl -s -H @"$HOME/.edr_auth_header" http://127.0.0.1:8080/incidents
curl -s -H @"$HOME/.edr_auth_header" -H 'Content-Type: application/json' \
  -d '{"dst":"203.0.113.10","port":443,"reason":"c2 beacon","dry_run":true}' http://127.0.0.1:8080/actions/block_egress
```

To rotate the token, write the new one to a temp file and `mv` it over the token file. With `EDR_GUARD_MODE=enforce`, other processes cannot signal the agent. Create the off file, wait for the log line, then send SIGINT. Ctrl+C in its own terminal still works. After editing `policy.json`, sign it again with `python3 scripts/policy_signer.py sign edr-agent/policy.json`.

Backend first, then the collector and the dashboard from a second shell at the repo root with the same backend token. Sign in to the dashboard with that token. To have the agent forward its incidents, start it with `EDR_BACKEND_URL=http://127.0.0.1:8000` and `EDR_API_TOKEN_FILE` pointing at the backend token.

```bash
cd backend
python3 -m venv .venv && . .venv/bin/activate
pip install -r requirements.txt
( umask 077 && openssl rand -hex 32 > "$HOME/.edr_api_token" )
export EDR_API_TOKEN_FILE="$HOME/.edr_api_token"
uvicorn main:app --port 8000
```

```bash
pip install psutil requests
EDR_API_TOKEN="$(cat "$HOME/.edr_api_token")" python3 scripts/send_real_telemetry.py   # one pass of process telemetry
cd dashboard && npm ci && npm run dev      # http://localhost:5173, proxies /api to port 8000
```

`edr-agent/ui/vendor/build.sh` rebuilds the agent UI's vendored files (Tailwind CSS, a D3 subset, Chart.js and its date adapter) from the dashboard's installed packages.

### Environment variables

| Variable | Meaning |
|---|---|
| `EDR_AGENT_TOKEN_FILE` | Agent API bearer token in an owner-only file, re-read every 2 s. Wins over `EDR_AGENT_TOKEN` |
| `EDR_AGENT_TOKEN` | Agent API bearer token, 32+ characters. Unset (and no file) means no API, sensing goes on |
| `EDR_DEV_NO_AUTH` | `1` opens both APIs without a token and accepts an unverified policy with a warning. Dev machines only |
| `EDR_AGENT_CORS_ORIGINS` | Comma separated exact origins allowed to call the agent API |
| `EDR_POLICY_PUBKEY` | Public key for `policy.json.sig` (default `config/policy_pub.pem`) |
| `EDR_PORT` | Agent API port on 127.0.0.1 (default 8080). Isolation keeps it open |
| `EDR_GUARD_MODE` | `off`, `monitor` or `enforce` for every guard, never below the policy's `guard_mode` |
| `EDR_GUARD_SOCKET_MODE`, `EDR_GUARD_MEMFD_MODE`, `EDR_GUARD_SELF_MODE` | Overrides for runtime_socket, memfd_exec and self_protect, with the same floor |
| `EDR_BPF_GUARD_OBJ` | Guard object (default `$EDR_BPF_DIR/edr_guard.bpf.o`, else `src/ebpf/edr_guard.bpf.o`) |
| `EDR_GUARD_SOCKET_CLIENTS` | Extra runtime client executables, colon separated |
| `EDR_GUARD_MEMFD_ALLOW` | Extra executables allowed to run a memfd, colon separated |
| `EDR_GUARD_SCOPE_CGROUP` | A cgroup v2 directory. Guards and shields then judge only tasks in it and the cgroups below it |
| `EDR_BPF_SHIELD_OBJ` | Shield object (default `$EDR_BPF_DIR/edr_shield.bpf.o`, else `src/ebpf/edr_shield.bpf.o`) |
| `EDR_SHIELD_PIN_DIR` | Where the shield links and maps are pinned (default `/sys/fs/bpf/edr_shield`) |
| `EDR_SHIELDS_AUDIT` | Shield audit log (default `logs/shields_audit.jsonl`) |
| `EDR_POLICY_AUDIT` | Policy reload audit log (default `logs/policy_audit.jsonl`) |
| `EDR_STATE_DIR` | Anti-rollback floor, kept policy copy and shield state (default `state`) |
| `EDR_GUARD_OFF_FILE` | Self protection off switch (default `/run/edr-guard/off`) |
| `EDR_GUARD_OFF_SHA256` | If set, the off file must hold a token with this SHA-256 |
| `EDR_REDACT` | `0` turns command line masking off, refused while the policy's `redact_command_lines` is true |
| `EDR_REDACT_FLAGS`, `EDR_REDACT_KEYS` | More flag names and `key=value` names to mask, comma separated |
| `EDR_ADAPTER_QUEUE`, `EDR_ADAPTER_PUSH_TIMEOUT_MS` | Fact queue size (default 32768) and how long a reader waits on a full queue (default 250) |
| `EDR_RECORD_QUEUE`, `EDR_RECORD_PUSH_TIMEOUT_MS` | Record queue size (default 131072) and how long a reader waits on a full one (default 250) |
| `EDR_ENV_ALLOWLIST` | Environment variables read at exec, comma separated (default CI, GITHUB_ACTIONS, GITLAB_CI, JENKINS_URL, BUILDKITE, TF_BUILD, CIRCLECI, TEAMCITY_VERSION, DRONE). Names that look like secrets are refused |
| `EDR_HOST_ROLE` | What the host is for, such as `bastion`, as `host_role` on exec facts |
| `EDR_GROUND_TRUTH_PATH` | Labelled hits of playbooks with `emit_ground_truth` (default `logs/ground_truth.jsonl`) |
| `EDR_AUTH_LOG` | Auth logs to follow, colon separated (default `/var/log/auth.log:/var/log/secure`) |
| `EDR_HOST_PERF_MIN_PCT` | A process becomes a HostPerf sample at this share of one core (default 50) |
| `EDR_FIREWALL` | `nft` or `iptables` for the response actions (default nft when installed) |
| `EDR_ACTIONS_AUDIT` | Response action audit log (default `logs/actions_audit.jsonl`) |
| `EDR_BACKEND_URL` | Backend base URL for incident forwarding. Unset means no forwarding |
| `EDR_FORWARD_ALLOW_HTTP` | `1` allows plain http to a backend that is not loopback |
| `EDR_FORWARD_INTERVAL_SECS` | Seconds between forwarding posts (default 5) |
| `EDR_PIN_PREFIX` | Pinned sensor maps (default `/sys/fs/bpf/edr`) |
| `EDR_FORCE_PINNED` | `1` selects pinned mode even when no pins are found yet |
| `EDR_BPF_DIR` | Folder for `suspicious_ipc.bpf.o` and `edr_guard.bpf.o` |
| `EDR_BPF_IPC_OBJ` | suspicious_ipc object (default `$EDR_BPF_DIR/suspicious_ipc.bpf.o`, else `src/ebpf/suspicious_ipc.bpf.o`) |
| `EDR_BPF_DNS_OBJ` | dns_tunnel_monitor object (default `/opt/edr-ebpf/dns_tunnel_monitor.bpf.o`, ignores `EDR_BPF_DIR`) |
| `EDR_RAW_SYSCALLS` | Raw sensor allowlist: unset or `default` (arm64 or x86_64 numbers), a comma list of syscall numbers, or `none` to leave the map alone |
| `EDR_PERF_PAGES` | Pages per CPU for the file_access_monitor perf buffer (default 256) |
| `PB_DIR`, `IOC_DIR`, `ONTOLOGY_DIR`, `KRIM_BASELINE` | Playbooks (default `playbooks`), IOC lists (`state/ioc`), ontology (searched, finds `../ontology`), KRIM-lite baseline (`state/krim_baseline.json`) |
| `HOSTNAME` | Host name in incidents (default `localhost`) |
| `EDR_BOOTSTRAP` | `1` skips trust scoring |
| `EDR_LOG_DIR` | Agent log folder (default `edr-agent/logs`, relative to the working directory) |
| `EDR_TELEMETRY_DIR`, `EDR_TELEMETRY_FILE_PREFIX`, `EDR_TELEMETRY_FLUSH_SECS`, `EDR_TELEMETRY_MAX_BUFFER`, `EDR_TELEMETRY_ROLLING` | Telemetry writer: folder for the telemetry files and `kv_telemetry.jsonl` (`telemetry_output`), file prefix (`telemetry`), flush interval (30 s), buffer (10,000 records), rolling by `ts`, `hour` or `day` |
| `EDR_DISABLE_KV` | `1` stops `kv_telemetry.jsonl` rows |
| `EDR_OUT_DIR`, `EDR_GNN_DIR`, `EDR_GNN_FIFO` | Graph vector logs: process monitor base folder (`/var/tmp/edr-agent`), graph hook folder (`edr-agent/json_files`), FIFO (`/tmp/gnn_input.pipe`) |
| `PUBLISH_BUS_URL`, `PUBLISH_BUS_DISABLED` | Where decision engine incidents are posted (default `http://127.0.0.1:8787/api/incidents`), and `1` to stop posting |
| `RGCN_ENABLED`, `RGCN_URL`, `RGCN_POLL_MS`, `RGCN_TIMEOUT_MS`, `RGCN_MAX_NODES`, `RGCN_MAX_EDGES` | Graph scoring sidecar: on switch, URL (default `http://127.0.0.1:9000/infer`), poll interval, timeout, graph size caps |
| `RUST_LOG`, `EDR_VERBOSE`, `EDR_DEBUG_RAW_EVENTS`, `EDR_PCB_DUMP_RAW` | Log filter (default the policy's `logging.level`, `telemetry=trace` prints raw records), `1` prints every record and trust result (off by default, it is three lines per event), `1` adds a hex copy of every frame, `1` dumps PCB records to `state/pcb_raw` |
| `EDR_API_TOKEN_FILE`, `EDR_API_TOKEN` | Backend bearer token (file wins, re-read when it changes), 32+ characters. The agent's forwarder and the collector send it |
| `EDR_DASHBOARD_ORIGIN` | The one browser origin the backend allows (default `http://localhost:5173`) |
| `DATABASE_URL` | Backend database (default `sqlite:///./edr.db`, relative to the working directory) |
| `PREDICT_MODEL_PATH` | Demo model for `/api/predict` (default `backend/ml/models/demo_classifier.joblib`) |
| `EDR_API_URL` | Collector target (default `http://localhost:8000/api/telemetry`) |
| `PUBLISH_HOST`, `PUBLISH_PORT`, `SPLUNK_HEC_TOKEN` | Publish service address (default 127.0.0.1) and port (default 8787, `PORT` also works), and the HEC token `config/connectors.json` reads when Splunk is on |

## Tests

```bash
(cd edr-agent && cargo test)         # 343 lib + 30 bin tests on Linux arm64, 2 ignored (the replay and one doc test)
python3 -m pytest tests -q            # 92 tests with BPF_OBJ_DIR set to the built objects, needs the backend requirements plus pytest
(cd dashboard && npm run build && npm run lint)   # builds, lint shows 1 warning and 0 errors
```

* Playbooks: all 29 load with no warnings, and each has a fires once, a cooldown and a near miss test driven through the real adapter, mapper and engine. The engine tests cover every operator, `any_of`, `in_slot`, load errors for bad conditions and bad links, the counting kinds, link order and gaps, optional steps left out for a broken link, either order links, per user chains, windows and cooldowns, and the ground truth file (labelled, 0600).
* Real signals: the TLS hello parser on a ClientHello OpenSSL itself produced (name, ALPN, a cut copy, junk), the decoder on a TLS sized frame, the mapper giving the name to later sends, the package check on a fake dpkg root (verified, overwritten, merged /usr, not packaged) and rpm's output, the environment allowlist, GPU busy percent from DRM cards and nvidia-smi output, /etc/passwd names, and each restored exception (bastion, admin jump box, weekend backup window, verified backup package).
* Record path: `cargo test --release --bin forensic_hooks replay -- --ignored` replays a `kv_telemetry.jsonl` through every stage and prints the time per stage and the peak memory. The queue tests cover waiting, counted drops after a timeout, and dropping at once after a stall until the consumer has drained the queue to half.
* Fact queue: a full queue makes the producer wait and loses nothing with a slow consumer, and a stuck consumer turns into counted drops after the timeout.
* Exec enrichment: `/proc` wins when it agrees with the kernel's argv copy, and the kernel copy wins when `/proc` already shows a later program. tcp_state addresses and ports decode in network order.
* Redaction: each secret pattern, the cases that must stay (paths, git hashes, `mkdir -p`, `--password-file`), and a test that decodes a proc_lifecycle exec frame with six secrets in its argv and checks the kv row, the telemetry file, the playbook fact and the incident for the raw values.
* Guard incidents: a block and a would block become incidents with the guard name, caller exe, target and mode.
* Response actions: every kill refusal (pid 0 and 1, negative, the agent, its threads, another process's thread, a kernel thread, no reason, unknown incident), a real SIGKILL of a child, a reused pid refused, the nftables script for block, isolation and release, refusals for loopback, the backend and 0.0.0.0/0, dry runs that run nothing, the iptables commands, the policy gate and the audit log mode.
* Forwarding: the payload equals the shared fixture, plain http to a remote host is refused, failed batches come back without overwriting newer versions, and a real post to a local listener carries the bearer token. pytest posts the same fixture to the backend and reads it back, and checks 401 without the token, 422 for bad batches and replacement by a newer version.
* Tokens: a rotated agent token works at once and the old one gets 401, token files must be owner-only, and the backend token file rotates without a restart.
* Agent API: without a token a plain route, an SSE stream, a nested route and the static directory all answer 401, and so do a wrong token, the Basic scheme and an empty Bearer. A foreign origin gets no CORS headers. Stored secrets never appear in `GET /integrations`, and the secrets file ends 0600 with a planted symlink refused.
* Policy: one changed byte, a wrong key, an RSA key, a missing or placeholder signature all fail, a signed policy with unknown fields or values fails, the guard floor raises modes and never lowers them, and an unsigned edit to the shipped policy fails `cargo test`.
* Backend: every route carries the token dependency and answers 401 without or with a wrong token, and `safe_expr` rejects injection attempts such as `__import__('os').system(...)`.
* Live end to end on Linux 7.0.14 arm64 (privileged container, host pid namespace, guard enforcing, one agent run): the triggers for all 29 playbooks ran in one burst of about 100 s, with a 2.2 GB upload behind two real TLS ClientHellos for S3 bucket names, a 300,116 connect flood and a 330 s CPU burn among them, and all 29 fired (30 hits, 21 playbook incidents, since incidents group by exe, technique and 15 minutes). The six linked playbooks fired with every link held, per `[host, user]` chain, and wrote 7 ground truth lines. The fact queue pushed 785,944 facts with a high water mark of 14,648 and no drops. The record queue took all 804,431 records with a high water mark of 3,752, no waits, no stalls and **0 drops**, and the rules ran on all 803,584 records left after the filter that removes empty records (1,425 rule hits). Both TLS hellos reached the telemetry with their server name, and the cloud object store playbook fired on `bucket-x.s3.amazonaws.com`. The guard refused python3, a python3 copy named docker and a python3 renamed with prctl on `docker.sock`, a memfd exec and `kill -9` of the agent, and each became an incident with its guard, caller, target and mode. kill_proc killed a process through a pidfd and refused pid 1, pid 2, the agent and requests without a reason. block_egress, isolate_host and release changed real nftables rules, kept loopback, the management ports and the backend open, and the forwarder kept posting during isolation. A token file change rotated the agent token in 1.6 s. Six kinds of secret passed on command lines were found 0 times in 66 files (790 MB) and the incident list. All 57 agent incidents reached the backend. The agent's peak memory was 836 MB. Unknown frames stayed at 0 and nothing was left loaded afterwards.
* The same burst before this round's record path work: the record channel dropped 200,466 records and the peak memory was 994 MB. Two runs in between kept every record but came before the fixes above. In the first the record thread stalled for minutes on incident graphs, its backpressure let the kernel ring overflow, and 9 playbooks missed. In the second the counting windows grew the agent to 2.4 GB.
* `scripts/ci/live_smoke.sh`, the script the x86_64 workflow runs, passes on this arm64 kernel with the guard enforcing: four playbooks became incidents, the memfd exec was refused, and the hello's server name reached the telemetry.
* x86_64: the nine objects build with `-D__TARGET_ARCH_x86`, the cross built agent and its tests (313 lib + 29 bin, before the shields) pass on an amd64 Ubuntu 22.04 image under emulation, and the agent there verifies the policy, loads 29 of 29 playbooks and falls back to detect only. The ten objects with the shields compile for x86_64 and have no atomic instruction.
* Runtime shields, live (`scripts/ci/shield_live.sh`, Ubuntu 26.04.1, kernel 7.0.0-43, bpf in the LSM list, the guard enforcing): 29 of 29 checks pass in one run. For each of exec_deny, file_deny and connect_deny, monitor mode let the action through and counted it, enforce mode refused it with EPERM (connect_deny for TCP, UDP sendto and an IPv4 mapped IPv6 address), and other programs, files, ports and addresses were untouched. A copy of a denied file ran (a new inode) until a file_deny on the original stopped `cp`. A file_deny was promoted by the policy's trial period. A connect_deny scoped to one cgroup refused inside it and allowed outside. A tampered document (403), a replayed serial (409), a lifetime over the maximum (403), five never-block targets (403 each) and a sixth shield over the policy's five (429) were refused. A newer signed policy changed the memfd allowlist in the kernel in one bank switch, and an older signed policy and an edited one were refused, with an incident. The webshell playbook proposed and applied by itself a connect_deny of its destination in monitor mode, and the in-memory exec playbook proposed an exec_deny of the dropped child, which signed, applied and promoted then refused the child (exit 126). An isolation with a 6 s expiry ended on time. Three shields and an egress block that expire in 25 s were still refused before their deadline and ended on time while the agent was stopped with SIGSTOP, and after SIGCONT the sweep removed them and audited 3 expiries.
* x86_64, live (GitHub's ubuntu-latest runner, kernel 7.0.0-1020-azure, 4 CPUs, bpf in the LSM list, guard enforcing): `scripts/ci/live_smoke.sh` and `scripts/ci/shield_live.sh` pass. Three runs in a row passed after the change below.
* Tracepoints by CPU: a test program on several syscall tracepoints and a workload pinned to each CPU in turn showed that on kernel 7.0 a BPF program on `syscalls/sys_enter_write` or `syscalls/sys_enter_openat` ran on CPU 0 only, on both the x86_64 runner (7.0.0-1020-azure) and the arm64 VM (7.0.0-43-generic), while `sys_enter_connect`, `sys_enter_mmap`, `sys_enter_getpid` and every `sys_exit` tracepoint ran on all CPUs. Before the fix, the TLS hello was copied only when the client wrote it on CPU 0, so the smoke test's TLS check passed about 1 run in 3 on the 4 CPU runner. net_flow, file_access_monitor and wx_exec now read write, sendto, openat, execve and execveat on `raw_syscalls/sys_enter` with a syscall number check, which ran on every CPU. After the change the arm64 VM passed the live smoke test and all 29 shield checks again.
* Root tamper table (`scripts/ci/root_tamper.sh`, same VM, the agent as a systemd unit): of 30 attempts, 21 were blocked and 9 succeeded. The ones that work: reading the agent's environment, using the API token file to lift a shield, `cgroup.freeze`, `cgroup.kill` (the pinned shield still refused afterwards), removing the shield pins once the agent is dead, restarting on an older policy after deleting the state directory, restarting with root's own key or with `EDR_DEV_NO_AUTH=1`, and the off switch when no token is configured. Known limit 2 has the list.
* The 29 playbook burst with the shield programs loaded (same VM, one enforcing shield of each kind on targets the burst never touches): in the first run 28 of 29 fired. The wiper playbook missed because Ubuntu 26.04's Rust coreutils puts `dd` at `/usr/lib/cargo/bin/coreutils/dd` and the playbook only matched `/usr/bin` and `/usr/sbin`. It now matches the program name in any directory, and in the second run all 29 fired. In that run the fact queue took 807,514 facts with 0 drops, the record queue took 842,085 records with 0 drops, the rules ran on 841,302 records (483 hits), all 4 guard blocks became incidents, all 42 agent incidents reached the backend, kill_proc, block_egress, isolate_host and release worked with the backend kept open during isolation, the token rotated in 600 ms, 0 secrets were found, unknown frames stayed at 0 and the agent's peak memory was 976 MiB. The three shields had 0 hits, as they should.
* Dashboard: every page was clicked through in a browser against a live backend with test data (the agent incidents of a live run, telemetry, an agent, IOCs, feedback, notes): every API call answered 200, the console showed no errors, a wrong token was rejected, and a token rotated on the backend sent the page back to sign in. Times stored without a zone (agent registry, feedback, notes) showed 6 hours off and are now read as UTC.

## Known limits

1. Name based playbooks and the suspicious_ipc detector trust executable names, comm or command lines, so a renamed tool evades them. The guard matches by inode instead.
2. Root can still get past the guard. Measured with `scripts/ci/root_tamper.sh` (30 attempts, 21 blocked): `cgroup.freeze` hangs the agent (the kernel shields keep working), `cgroup.kill` kills it (the pinned shields keep refusing until they expire or root removes their pins), root can read the agent's environment and its API token file and lift shields over the API, and once the agent is stopped it can delete the anti-rollback state, or restart the agent with its own key (`EDR_POLICY_PUBKEY`) or `EDR_DEV_NO_AUTH=1`. The off file works for root when `EDR_GUARD_OFF_SHA256` is not set. `systemctl stop` returns after its timeout and calls the unit inactive, but the agent keeps running, outside systemd's view. Not tried, because each changes the kernel itself: a kernel module, /dev/mem, kexec, or a reboot without `bpf` in `lsm=`. The socket guard checks at connect time only and does not cover the TCP Docker API. The socket guard checks at connect time only and does not cover the TCP Docker API.
3. A few playbook steps still stand in for data no host agent can see: ASN, reputation, HTTP methods inside TLS, the source address of a web request, and change tickets (each explained in its YAML). The TLS server name comes from the first write on a socket to port 443 or 8443 only, a hello split over two writes or sent with sendmsg is missed, and Encrypted Client Hello shows its public name. The sensor copies the first 1,024 bytes, and OpenSSL 3.5 (Ubuntu 26.04) sends a 1,585 byte hello, most of it a 1,258 byte post-quantum key share. OpenSSL puts the server name at byte 147 and ALPN at byte 215, so both are in the copy, but a client that sends them after the key share would be missed. The package check covers dpkg files in bin, sbin and libexec directories, and the rpm path has unit tests only. Environment variables are read from `/proc` at exec, so a process that is already gone has none, and a process can set CI=true itself. GPU samples do not say which process is busy. VendorAlert slots have no feed, since nothing calls `satisfy_slot_from_vendor`.
4. NetFlow counts only bytes passed to send, sendto and sendmsg. A program that writes to its socket with write(2), such as most Go tools, has its upload under counted. Net facts take the process from the SYN_SENT state change, so inbound connections carry no process. `dst_new` and `src_new` remember peers only since the agent started.
5. Auth facts come only from sshd lines in log files. A host that logs only to journald needs `sshd -E <file>` or a forwarding rule, and the path in `EDR_AUTH_LOG`.
6. Each API uses one shared bearer token. Token files allow rotation without a restart and keep the token out of the process environment, but root can read the file, the agent API is plain HTTP on loopback, and any holder of the backend token can post incidents for any host name.
7. Command line masking is pattern based. A secret in an unusual flag or a bare value with no known shape is stored as it is, unless its name is added with `EDR_REDACT_FLAGS` or `EDR_REDACT_KEYS`.
8. kill_proc refuses init, the agent, threads and kernel threads, but not other system services such as sshd or journald. The agent keeps its firewall state in memory, so after a restart it does not know an old isolation is in place, and `release` with `"all": true` is the way to clear it.
9. The sensors still use per-syscall tracepoints for connect, mmap, mprotect and every syscall exit. Those ran on all CPUs in the test above, but a kernel change could restrict them the same way sys_enter_write was, and only the live smoke test would notice. The root tamper table and the overhead numbers were measured on arm64 only.
10. Under a burst the record and fact queues push back on the ring readers instead of dropping. If both stay full past the 250 ms timeout, records are dropped and counted, and a ring buffer that fills meanwhile loses frames in the kernel, which nothing counts yet. The trust scorer still keeps one state entry per pid.
11. Integration credentials are plain text in a 0600 file, and no connector reads them yet.
12. The publish service in `server/` has no authentication, so it listens on 127.0.0.1 only unless `PUBLISH_HOST` says otherwise.
13. Shields name files by inode and device. A copy of a denied file is a new inode, which file_deny on the original closes but which a file written again from scratch gets past. An inode number freed by a deleted file can be reused by a new one until the shield expires. exec_deny judges the file execve runs, so `sh script` or `ld.so ./binary` is a file_deny matter. connect_deny keeps one shield per exact target, and when two networks overlap the longer prefix decides, even when it is outside its time or scope. Hit counts on one CPU can miss a few when two tasks on it race.
14. A BPF program the verifier accepts can still crash the kernel. The first version of connect_deny did a 64 bit atomic add on a hit counter inside an LPM trie value. The trie stores the value right after a 20 byte key, so the value was only 4 byte aligned, and arm64 faults on an unaligned atomic in kernel mode: the first live run panicked the test VM's kernel. The programs now use no atomics (per CPU counters instead), the trie key's data is padded to 24 bytes, and `check_bpf_atomics.py` fails the build on any atomic instruction. x86_64 tolerates unaligned atomics, so an x86_64 CI run alone would never have shown it. With the fix, the same connect_deny checks ran on arm64 in the Ubuntu 26.04 VM with no fault, through the connect_deny steps of five runs of `shield_live.sh` and a benchmark that put 10,000 rows in the trie and sent 9 rounds of 200,000 UDP connects through it, and `check_bpf_atomics.py` finds 0 atomic instructions in the 10 objects built there with clang 21.
15. The agent's memory still grows with sensor events: about 30 MB per million file opens in the idle VM test (326 MB to 389 MB over 2,000,000 opens), with `malloc_trim` already returning freed heap. Which structure keeps it is not found yet. A long run should watch the agent's RSS.
