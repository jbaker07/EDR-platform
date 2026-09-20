---
type: "interface"
fqcn: "net.minecraft.world.level.gameevent.GameEvent$Context"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.gameevent.GameEvent$Context

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `of(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/min` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.gameevent.GameEvent$Context extends java.lang.Record {
    private final net.minecraft.world.entity.Entity sourceEntity;
    private final net.minecraft.world.level.block.state.BlockState affectedState;
    public net.minecraft.world.level.gameevent.GameEvent$Context(net.minecraft.world.entity.Entity, net.minecraft.world.level.block.state.BlockState);
    public static net.minecraft.world.level.gameevent.GameEvent$Context of(net.minecraft.world.entity.Entity);
    public static net.minecraft.world.level.gameevent.GameEvent$Context of(net.minecraft.world.level.block.state.BlockState);
    public static net.minecraft.world.level.gameevent.GameEvent$Context of(net.minecraft.world.entity.Entity, net.minecraft.world.level.block.state.BlockState);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.entity.Entity sourceEntity();
    public net.minecraft.world.level.block.state.BlockState affectedState();
}
```
