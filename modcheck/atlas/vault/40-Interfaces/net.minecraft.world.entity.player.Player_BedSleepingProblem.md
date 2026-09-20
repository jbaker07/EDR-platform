---
type: "interface"
fqcn: "net.minecraft.world.entity.player.Player$BedSleepingProblem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.player.Player$BedSleepingProblem

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `OTHER_PROBLEMLnet/minecraft/world/entity/player/Player$BedSleepingProblem` | `` | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.entity.player.Player$BedSleepingProblem extends java.lang.Record {
    private final net.minecraft.network.chat.Component message;
    public static final net.minecraft.world.entity.player.Player$BedSleepingProblem TOO_FAR_AWAY;
    public static final net.minecraft.world.entity.player.Player$BedSleepingProblem OBSTRUCTED;
    public static final net.minecraft.world.entity.player.Player$BedSleepingProblem OTHER_PROBLEM;
    public static final net.minecraft.world.entity.player.Player$BedSleepingProblem NOT_SAFE;
    public net.minecraft.world.entity.player.Player$BedSleepingProblem(net.minecraft.network.chat.Component);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.network.chat.Component message();
    static {};
}
```
