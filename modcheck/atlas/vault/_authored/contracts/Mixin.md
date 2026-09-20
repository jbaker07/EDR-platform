---
type: "contract"
subject: "mechanism:Mixin"
kind: "hook"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: Mixin

Subject: [[30-Mechanisms/Mixin|Mixin]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Mixin priority defaults to 1000 and the target list to empty; a mod that leaves priority at default has no ordering relationship with Fabric API's own mixins. | `direct_reference` | `extracted/corpus.json`; [[30-Mechanisms/Mixin|Mixin]] |
| Fabric API 0.161.0+26.3 uses no @Overwrite; a mod's @Overwrite of any of the 479 injected methods removes Fabric's injection and the events it fires. | `direct_reference` | `extracted/edges.json#injects_into`; `extracted/fabric_api.json` |

## Not established

- Application order between equal-priority mixins from different mods ([[80-Unresolved/q.mixin_docs_application_order|q.mixin_docs_application_order]]).
- Whether any given mixin applies at runtime ([[80-Unresolved/q.runtime_mixin_application|q.runtime_mixin_application]]).

## Evidence

- `extracted/corpus.json`
- `extracted/edges.json#injects_into`

## Open questions

- [[80-Unresolved/q.mixin_docs_application_order|q.mixin_docs_application_order]]
- [[80-Unresolved/q.runtime_mixin_application|q.runtime_mixin_application]]
