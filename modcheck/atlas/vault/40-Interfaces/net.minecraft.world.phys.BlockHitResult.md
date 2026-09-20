---
type: "interface"
fqcn: "net.minecraft.world.phys.BlockHitResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.phys.BlockHitResult

System: [[20-Systems/net.minecraft.world.phys|net.minecraft.world.phys]]

`class` public; extends `net/minecraft/world/phys/HitResult`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBlockPos` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@17 in `BlockBehaviourBlockStateBaseMixin.callUseItemOnEvent` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getBlockPos` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@15 in `BlockBehaviourBlockStateBaseMixin.callUseWithoutItemEvent` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getBlockPos` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@5 in `BrushItemMixin.modifyBlockStateParticleOption` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (6 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private final direction : Lnet/minecraft/core/Direction;
private final blockPos : Lnet/minecraft/core/BlockPos;
private final miss : Z
private final inside : Z
private final worldBorderHit : Z
public static miss(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/BlockHitResult;
public <init>(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;Z)V
public <init>(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;ZZ)V
private <init>(ZLnet/minecraft/world/phys/Vec3;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;ZZ)V
public withDirection(Lnet/minecraft/core/Direction;)Lnet/minecraft/world/phys/BlockHitResult;
public withPosition(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/BlockHitResult;
public hitBorder()Lnet/minecraft/world/phys/BlockHitResult;
public getBlockPos()Lnet/minecraft/core/BlockPos;
public getDirection()Lnet/minecraft/core/Direction;
public getType()Lnet/minecraft/world/phys/HitResult$Type;
public isInside()Z
public isWorldBorderHit()Z
static <clinit>()V
```
