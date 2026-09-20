---
type: "contract"
subject: "event:net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.BEFORE"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.BEFORE

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.BEFORE|BEFORE]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Fired from an @Inject into ServerPlayerGameMode.destroyBlock at the INVOKE of Block.playerWillDestroy, so it runs on the server after the block has been resolved but before it is removed. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |
| Callback returns boolean and receives (Level, Player, BlockPos, BlockState, BlockEntity); returning false is the cancel path (the CANCELED event exists alongside). | `declared` | `extracted/edges.json#callback_of` |

## Not established

- Whether creative-mode breaks pass through the same site.

## Evidence

- `extracted/edges.json#publishes_event`
