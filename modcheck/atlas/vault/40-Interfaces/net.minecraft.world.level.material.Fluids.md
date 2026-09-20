---
type: "interface"
fqcn: "net.minecraft.world.level.material.Fluids"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.material.Fluids

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `EMPTYLnet/minecraft/world/level/material/Fluid;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.material.Fluids {
    public static final net.minecraft.world.level.material.Fluid EMPTY;
    public static final net.minecraft.world.level.material.FlowingFluid FLOWING_WATER;
    public static final net.minecraft.world.level.material.FlowingFluid WATER;
    public static final net.minecraft.world.level.material.FlowingFluid FLOWING_LAVA;
    public static final net.minecraft.world.level.material.FlowingFluid LAVA;
    public net.minecraft.world.level.material.Fluids();
    private static <T extends net.minecraft.world.level.material.Fluid> T register(net.minecraft.resources.ResourceKey<net.minecraft.world.level.material.Fluid>, T);
    static {};
}
```
