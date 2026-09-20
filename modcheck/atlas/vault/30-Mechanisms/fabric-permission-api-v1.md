---
type: "mechanism"
module: "fabric-permission-api-v1"
version: "1.0.8+fcdff87f5d"
sha256: "a3a82771b3fd9f2eb36e1098c5f90759ac877982b9ac297831a1fce9b2185bf8"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-permission-api-v1

**Version** `1.0.8+fcdff87f5d` -- **artifact sha256** `a3a82771b3fd9f2eb36e1098c5f90759ac877982b9ac297831a1fce9b2185bf8`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `{}`
- mixin configs: `["fabric-permission-api-v1.mixins.json"]`
- access widener: `fabric-permission-api-v1.classtweaker`
- mixin classes: 2 found by annotation, 2 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.permission.v1.PermissionEvents.ON_REQUEST|PermissionEvents.ON_REQUEST]]
- [[50-Interactions/events/net.fabricmc.fabric.api.permission.v1.PermissionEvents.PREPARE_OFFLINE_PLAYER|PermissionEvents.PREPARE_OFFLINE_PLAYER]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.commands.CommandSourceStack|CommandSourceStack]].`/^with/ desc=/CommandSourceStack;$/` | `?` | selector_unsupported | @ModifyReturnValue | RETURN | both | 1000 (default) | `CommandSourceStackMixin.copyOriginalOwner` |
| [[40-Interfaces/net.minecraft.commands.CommandSourceStack|CommandSourceStack]].`<init>` | `(Lnet/minecraft/commands/CommandSource;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec2;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/permissions/PermissionSet;Lnet/minecraft/commands/CommandSourceStack$NamesProvider;Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/world/entity/Entity;)V` | exact | @Inject | TAIL | both | 1000 (default) | `CommandSourceStackMixin.storeOriginalSource` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`<init>` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `EntityMixin.createPermissionContext` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.permission.v1.MutablePermissionContext|MutablePermissionContext]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.permission.v1.PermissionContext|PermissionContext]] (interface, 19 members)
- [[40-Interfaces/net.fabricmc.fabric.api.permission.v1.PermissionContextOwner|PermissionContextOwner]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.permission.v1.PermissionEvents|PermissionEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.permission.v1.PermissionNode|PermissionNode]] (interface, 13 members)
- [[40-Interfaces/net.fabricmc.fabric.api.permission.v1.PermissionPredicates|PermissionPredicates]] (class, 6 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
