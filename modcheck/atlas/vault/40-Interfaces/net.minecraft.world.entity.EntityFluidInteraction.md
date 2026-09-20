---
type: "interface"
fqcn: "net.minecraft.world.entity.EntityFluidInteraction"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.EntityFluidInteraction

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `applyCurrentTo(Lnet/minecraft/tags/TagKey;Lnet/minecraft/world/entity/Enti` | `` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `isEyeInFluid(Lnet/minecraft/tags/TagKey;)Z` | `` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `isInFluid(Lnet/minecraft/tags/TagKey;)Z` | `` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.entity.EntityFluidInteraction {
    private final java.util.List<net.minecraft.world.entity.EntityFluidInteraction$Tracker> fluidTrackers;
    private final it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid>, net.minecraft.world.entity.EntityFluidInteraction$CurrentAccumulator> currentAccumulators;
    public net.minecraft.world.entity.EntityFluidInteraction(java.util.Set<net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid>>);
    public boolean update(net.minecraft.world.entity.Entity, boolean);
    private static boolean hasFluidAndLoaded(net.minecraft.world.level.Level, int, int, int, int, int, int);
    private net.minecraft.world.entity.EntityFluidInteraction$Tracker getOrCreateTrackerFor(net.minecraft.core.Holder<net.minecraft.world.level.material.Fluid>);
    private net.minecraft.world.entity.EntityFluidInteraction$CurrentAccumulator getCurrentAccumulatorFor(net.minecraft.core.Holder<net.minecraft.world.level.material.Fluid>);
    public void applyCurrentTo(net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid>, net.minecraft.world.entity.Entity, double);
    public double getFluidHeight(net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid>);
    public boolean isInFluid(net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid>);
    public boolean isEyeInFluid(net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid>);
}
```
