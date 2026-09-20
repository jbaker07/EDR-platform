---
type: "contract"
subject: "event:net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.ALLOW_DEATH"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.ALLOW_DEATH

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.ALLOW_DEATH|ALLOW_DEATH]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Implemented as a @Redirect of LivingEntity.isDeadOrDying inside LivingEntity.hurtServer, so the listener's boolean stands in for the vanilla death check at that call site only. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#wraps` |
| A mod that redirects the same call site cannot coexist with this event (one redirect per call site). | `analyst_inference` | [[30-Mechanisms/Mixin|Mixin]]; `extracted/edges.json#wraps` |

## Not established

- Whether other death paths (void, /kill) reach hurtServer at all.

## Evidence

- `extracted/edges.json#wraps`

## Open questions

- [[80-Unresolved/q.mixin_docs_application_order|q.mixin_docs_application_order]]
