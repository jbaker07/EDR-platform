---
type: "contract"
subject: "event:net.fabricmc.fabric.api.event.player.UseBlockCallback.EVENT"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.event.player.UseBlockCallback.EVENT

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.event.player.UseBlockCallback.EVENT|EVENT]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Two publication sites -- @Inject at HEAD of ServerPlayerGameMode.useItemOn (server) and @Inject into MultiPlayerGameMode.useItemOn before startPrediction (client) -- so the same listener runs on both sides in single-player and multiplayer alike. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |
| The callback returns InteractionResult; returning anything other than PASS stops vanilla's own use logic at that site, which on the client also prevents the packet from being sent. | `analyst_inference` | `extracted/edges.json#callback_of`; `extracted/edges.json#injects_into` |
| Because the server site is at HEAD, a listener that returns a non-PASS result suppresses every other mod's block-use handling that vanilla would have invoked. | `analyst_inference` | `extracted/edges.json#injects_into` |

## Not established

- Which result values the client site treats as consumed versus failed (the redirected call's semantics are not extracted; [[80-Unresolved/q.injection_points_not_resolved|q.injection_points_not_resolved]]).

## Evidence

- `extracted/edges.json#publishes_event`

## Open questions

- [[80-Unresolved/q.contested_method_policy|q.contested_method_policy]]
