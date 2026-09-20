---
type: "interface"
fqcn: "net.minecraft.world.level.LevelAccessor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.LevelAccessor

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/level/CommonLevelAccessor`, `net/minecraft/world/level/ScheduledTickAccess`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getFluidState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/material/Flu` | inherited_exact | invokeinterface@31 in `FlowingFluidMixin.shouldSpreadLiquid` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getFluidState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/material/Flu` | inherited_exact | invokeinterface@19 in `LavaFluidMixin.shouldSpreadLiquid` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (0 fields, 22 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract nextSubTickCount()J
public createTick(Lnet/minecraft/core/BlockPos;Ljava/lang/Object;ILnet/minecraft/world/ticks/TickPriority;)Lnet/minecraft/world/ticks/ScheduledTick;
public createTick(Lnet/minecraft/core/BlockPos;Ljava/lang/Object;I)Lnet/minecraft/world/ticks/ScheduledTick;
public abstract getLevelData()Lnet/minecraft/world/level/storage/LevelData;
public getGameTime()J
public abstract getServer()Lnet/minecraft/server/MinecraftServer;
public getDifficulty()Lnet/minecraft/world/Difficulty;
public abstract getChunkSource()Lnet/minecraft/world/level/chunk/ChunkSource;
public hasChunk(II)Z
public abstract getRandom()Lnet/minecraft/util/RandomSource;
public updateNeighborsAt(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;)V
public neighborShapeChanged(Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;II)V
public playSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/BlockPos;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;)V
public abstract playSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/BlockPos;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FF)V
public abstract addParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V
public abstract levelEvent(Lnet/minecraft/world/entity/Entity;ILnet/minecraft/core/BlockPos;I)V
public levelEvent(ILnet/minecraft/core/BlockPos;I)V
public abstract gameEvent(Lnet/minecraft/core/Holder;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/level/gameevent/GameEvent$Context;)V
public gameEvent(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder;Lnet/minecraft/world/phys/Vec3;)V
public gameEvent(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder;Lnet/minecraft/core/BlockPos;)V
public gameEvent(Lnet/minecraft/core/Holder;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/gameevent/GameEvent$Context;)V
public gameEvent(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/gameevent/GameEvent$Context;)V
```
