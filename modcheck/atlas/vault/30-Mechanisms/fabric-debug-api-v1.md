---
type: "mechanism"
module: "fabric-debug-api-v1"
version: "1.0.4+3434d6d95d"
sha256: "1ef63bc24f6319cc43d2d4add8a2df5e8c3986b5bd1f39b920a5bd7738ec52e3"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-debug-api-v1

**Version** `1.0.4+3434d6d95d` -- **artifact sha256** `1ef63bc24f6319cc43d2d4add8a2df5e8c3986b5bd1f39b920a5bd7738ec52e3`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-registry-sync-v0": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-debug-api-v1.mixins.json", {"config": "fabric-debug-api-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-debug-api-v1.accesswidener`

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientDebugSubscriber|ClientDebugSubscriber]] | `requestedSubscriptions` | injects_into `@Inject at RETURN` | client | `ClientDebugSubscriberMixin.addSubscribers` |
| [[40-Interfaces/net.minecraft.client.renderer.debug.DebugRenderer|DebugRenderer]] | `refreshRendererList` | injects_into `@Inject at RETURN` | client | `DebugRendererMixin.registerRenderers` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]] | `registerDebugValues` | injects_into `@Inject at HEAD` | both | `EntityMixin.addDebugValues` |
| [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]] | `registerDebugValues` | injects_into `@Inject at HEAD` | both | `EntityMixin.addDebugValues` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.debug.v1.ClientDebugSubscriptionRegistry|ClientDebugSubscriptionRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.debug.v1.renderer.DebugRendererFactory|DebugRendererFactory]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.debug.v1.renderer.DebugRendererRegistry|DebugRendererRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.debug.v1.DebugValueFactory|DebugValueFactory]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.debug.v1.EntityDebugSubscriptionRegistry|EntityDebugSubscriptionRegistry]] (class, 3 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
