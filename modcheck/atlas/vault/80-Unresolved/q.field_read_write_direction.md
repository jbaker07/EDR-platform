---
type: "question"
id: "q.field_read_write_direction"
kind: "incomplete_extraction"
status: "resolved"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.field_read_write_direction

**Question.** Of the 168 `reads` edges, which are actually writes (putfield/putstatic) rather than reads (getfield/getstatic)?

**Kind.** `incomplete_extraction` -- **Status.** resolved

**Why it matters.** Shared mutable state between mods is a write, not a read. The Fieldref scan does not distinguish them, so the atlas cannot yet say which vanilla fields Fabric API mutates.

**Affects.** [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]], [[10-Workflows/wf.state.persistence|wf.state.persistence]]

**Evidence already available.**
- `extracted/edges.json#reads`

**Best remaining source.** The same javap -c output; the opcode precedes the Fieldref comment.

**Procedure.** In atlas/extract/jvm.py class_refs, keep the opcode with each Fieldref and emit `writes` edges for putfield/putstatic.

**Done when.** edges.json counts contain both `reads` and `writes`.

**Conclusions affected while open.**
- No shares_mutable_state edge can be derived.

**Resolved by.** Opcodes are read from the Code attribute (atlas/extract/classfile.py); edges.json now separates `extracted/edges.json#reads` (get*) from `extracted/edges.json#writes` (put*), each with the caller method and instruction offset. Type references (new, anewarray, instanceof) are counted, not edges.
