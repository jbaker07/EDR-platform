---
type: "interface"
fqcn: "net.fabricmc.fabric.api.registry.fluid.FluidBehavior"
module: "fabric-content-registries-v0"
sha256: "e83273ce3a8d06e08c00f31bdc38497d653f678af3693a2fc3ba094537fa8879"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.registry.fluid.FluidBehavior

Module: [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] -- kind: interface

```java
public static final net.fabricmc.fabric.api.registry.fluid.FluidBehavior WATER_LIKE
public abstract void handleFluidInteractionUpdate(net.minecraft.tags.TagKey, net.minecraft.world.entity.Entity, net.minecraft.world.entity.EntityFluidInteraction, boolean)
public abstract void travelInFluid(net.minecraft.tags.TagKey, net.minecraft.world.entity.LivingEntity, net.minecraft.world.phys.Vec3, double, boolean, double)
public void travelFlyingInFluid(net.minecraft.tags.TagKey, net.minecraft.world.entity.LivingEntity, net.minecraft.world.phys.Vec3, float, float, float)
public boolean canSwimInFluid(net.minecraft.tags.TagKey, net.minecraft.world.entity.Entity)
public boolean shouldTryFloatingInFluid(net.minecraft.tags.TagKey, net.minecraft.world.entity.Entity)
public boolean canMoveDownInFluid(net.minecraft.tags.TagKey, net.minecraft.world.entity.Entity)
public boolean canDrownInFluid(net.minecraft.tags.TagKey, net.minecraft.world.entity.LivingEntity)
public boolean canSupportBoat(net.minecraft.tags.TagKey, net.minecraft.world.entity.Entity)
public boolean canSprintInFluid(net.minecraft.tags.TagKey, net.minecraft.world.entity.LivingEntity)
public void onFluidEntered(net.minecraft.tags.TagKey, net.minecraft.world.entity.Entity, boolean)
public void onFluidExited(net.minecraft.tags.TagKey, net.minecraft.world.entity.Entity)
public static net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder simple()
```
