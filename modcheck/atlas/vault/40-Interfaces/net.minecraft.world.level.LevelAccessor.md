---
type: "interface"
fqcn: "net.minecraft.world.level.LevelAccessor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.LevelAccessor

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getFluidState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/ma` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getFluidState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/ma` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.level.LevelAccessor extends net.minecraft.world.level.CommonLevelAccessor,net.minecraft.world.level.ScheduledTickAccess {
    public abstract long nextSubTickCount();
    public default <T> net.minecraft.world.ticks.ScheduledTick<T> createTick(net.minecraft.core.BlockPos, T, int, net.minecraft.world.ticks.TickPriority);
    public default <T> net.minecraft.world.ticks.ScheduledTick<T> createTick(net.minecraft.core.BlockPos, T, int);
    public abstract net.minecraft.world.level.storage.LevelData getLevelData();
    public default long getGameTime();
    public abstract net.minecraft.server.MinecraftServer getServer();
    public default net.minecraft.world.Difficulty getDifficulty();
    public abstract net.minecraft.world.level.chunk.ChunkSource getChunkSource();
    public default boolean hasChunk(int, int);
    public abstract net.minecraft.util.RandomSource getRandom();
    public default void updateNeighborsAt(net.minecraft.core.BlockPos, net.minecraft.world.level.block.Block);
    public default void neighborShapeChanged(net.minecraft.core.Direction, net.minecraft.core.BlockPos, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, int, int);
    public default void playSound(net.minecraft.world.entity.Entity, net.minecraft.core.BlockPos, net.minecraft.sounds.SoundEvent, net.minecraft.sounds.SoundSource);
    public abstract void playSound(net.minecraft.world.entity.Entity, net.minecraft.core.BlockPos, net.minecraft.sounds.SoundEvent, net.minecraft.sounds.SoundSource, float, float);
    public abstract void addParticle(net.minecraft.core.particles.ParticleOptions, double, double, double, double, double, double);
    public abstract void levelEvent(net.minecraft.world.entity.Entity, int, net.minecraft.core.BlockPos, int);
    public default void levelEvent(int, net.minecraft.core.BlockPos, int);
    public abstract void gameEvent(net.minecraft.core.Holder<net.minecraft.world.level.gameevent.GameEvent>, net.minecraft.world.phys.Vec3, net.minecraft.world.level.gameevent.GameEvent$Context);
    public default void gameEvent(net.minecraft.world.entity.Entity, net.minecraft.core.Holder<net.minecraft.world.level.gameevent.GameEvent>, net.minecraft.world.phys.Vec3);
    public default void gameEvent(net.minecraft.world.entity.Entity, net.minecraft.core.Holder<net.minecraft.world.level.gameevent.GameEvent>, net.minecraft.core.BlockPos);
    public default void gameEvent(net.minecraft.core.Holder<net.minecraft.world.level.gameevent.GameEvent>, net.minecraft.core.BlockPos, net.minecraft.world.level.gameevent.GameEvent$Context);
    public default void gameEvent(net.minecraft.resources.ResourceKey<net.minecraft.world.level.gameevent.GameEvent>, net.minecraft.core.BlockPos, net.minecraft.world.level.gameevent.GameEvent$Context);
}
```
