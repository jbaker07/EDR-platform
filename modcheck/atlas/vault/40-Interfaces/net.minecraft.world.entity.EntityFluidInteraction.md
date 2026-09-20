---
type: "interface"
fqcn: "net.minecraft.world.entity.EntityFluidInteraction"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.EntityFluidInteraction

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `applyCurrentTo` | `(Lnet/minecraft/tags/TagKey;Lnet/minecraft/world/entity/Entity;D)V` | exact | invokevirtual@12 in `SimpleConfiguredFluidBehavior.handleFluidInteractionUpdate` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `isEyeInFluid` | `(Lnet/minecraft/tags/TagKey;)Z` | exact | invokevirtual@54 in `EntityMixin.checkIfUnderSwimmableFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `isInFluid` | `(Lnet/minecraft/tags/TagKey;)Z` | exact | invokevirtual@40 in `EntityMixin.handleCustomFluidInteractionUpdates` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `isInFluid` | `(Lnet/minecraft/tags/TagKey;)Z` | exact | invokevirtual@40 in `EntityMixin.checkIfInSwimmableFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `isInFluid` | `(Lnet/minecraft/tags/TagKey;)Z` | exact | invokevirtual@40 in `EntityMixin.checkCustomFluids` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (2 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final fluidTrackers : Ljava/util/List;
private final currentAccumulators : Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;
public <init>(Ljava/util/Set;)V
public update(Lnet/minecraft/world/entity/Entity;Z)Z
private static hasFluidAndLoaded(Lnet/minecraft/world/level/Level;IIIIII)Z
private getOrCreateTrackerFor(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/entity/EntityFluidInteraction$Tracker;
private getCurrentAccumulatorFor(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/entity/EntityFluidInteraction$CurrentAccumulator;
public applyCurrentTo(Lnet/minecraft/tags/TagKey;Lnet/minecraft/world/entity/Entity;D)V
public getFluidHeight(Lnet/minecraft/tags/TagKey;)D
public isInFluid(Lnet/minecraft/tags/TagKey;)Z
public isEyeInFluid(Lnet/minecraft/tags/TagKey;)Z
```
