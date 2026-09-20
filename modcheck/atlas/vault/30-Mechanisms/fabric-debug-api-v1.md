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
- mixin classes: 4 found by annotation, 4 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientDebugSubscriber|ClientDebugSubscriber]].`requestedSubscriptions` | `()Ljava/util/Set;` | name_only | @Inject | RETURN | client | 1000 (default) | `ClientDebugSubscriberMixin.addSubscribers` |
| [[40-Interfaces/net.minecraft.client.renderer.debug.DebugRenderer|DebugRenderer]].`refreshRendererList` | `()V` | name_only | @Inject | RETURN | client | 1000 (default) | `DebugRendererMixin.registerRenderers` |
| [[40-Interfaces/net.minecraft.util.debug.ServerDebugSubscribers|ServerDebugSubscribers]].`hasRequiredPermissions` | `(Lnet/minecraft/server/level/ServerPlayer;)Z` | name_only | @WrapOperation | MIXINEXTRAS:EXPRESSION (selector_unsupported) | both | 1000 (default) | `ServerDebugSubscribersMixin.requireInIde` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`registerDebugValues` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/util/debug/DebugValueSource$Registration;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `EntityMixin.addDebugValues` |
| [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]].`registerDebugValues` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/util/debug/DebugValueSource$Registration;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `EntityMixin.addDebugValues` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.debug.v1.ClientDebugSubscriptionRegistry|ClientDebugSubscriptionRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.debug.v1.renderer.DebugRendererFactory|DebugRendererFactory]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.debug.v1.renderer.DebugRendererRegistry|DebugRendererRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.debug.v1.DebugValueFactory|DebugValueFactory]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.debug.v1.EntityDebugSubscriptionRegistry|EntityDebugSubscriptionRegistry]] (class, 3 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
