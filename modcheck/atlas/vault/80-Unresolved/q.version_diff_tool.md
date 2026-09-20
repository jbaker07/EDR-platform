---
type: "question"
id: "q.version_diff_tool"
kind: "unimplemented_automation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.version_diff_tool

**Question.** Should ModCheck diff two extracted member sets (two Minecraft versions, or two Fabric API versions) and emit migrates edges for renamed or removed members?

**Kind.** `unimplemented_automation` -- **Status.** open

**Why it matters.** Migration advice today comes from build failures; a diff would make it mechanical and version-pinned.

**Affects.** [[10-Workflows/wf.engineering.version_migration|wf.engineering.version_migration]], [[70-Requests/request.port_1_21_mod|request.port_1_21_mod]]

**Evidence already available.**
- `extracted/minecraft_members.json`
- `capability/subscribe_event.fabric_server_tick`

**Best remaining source.** A second corpus (q.older_minecraft_versions).

**Procedure.** Implement after the second corpus exists; output a generated 90-Coverage migration section.

**Done when.** A generated migration note lists removed and renamed members between two pinned versions.

**Conclusions affected while open.**
- wf.engineering.version_migration is manual.
