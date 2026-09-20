---
type: "contract"
subject: "event:net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback.EVENT"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback.EVENT

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback.EVENT|EVENT]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Fired from an @Inject into the Commands constructor at the INVOKE of CommandDispatcher.setConsumer, so every mod command is registered while the dispatcher is being built, on both sides. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |
| Callback receives the dispatcher, a CommandBuildContext and a CommandSelection; registration must be repeatable because Commands is constructed again on data pack reload. | `analyst_inference` | `extracted/edges.json#callback_of` |

## Not established

- Whether registering a command whose literal collides with a vanilla one replaces or errors.

## Evidence

- `extracted/edges.json#publishes_event`
