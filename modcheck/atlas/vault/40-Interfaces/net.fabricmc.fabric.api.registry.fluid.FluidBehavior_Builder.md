---
type: "interface"
fqcn: "net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder"
module: "fabric-content-registries-v0"
sha256: "e83273ce3a8d06e08c00f31bdc38497d653f678af3693a2fc3ba094537fa8879"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder

Module: [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] -- kind: interface

```java
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder movementSpeed(float)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder movementSpeed(net.minecraft.util.ToFloatFunction)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder movementSlowdown(float)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder movementSlowdown(float, float)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder movementSlowdown(net.minecraft.util.ToFloatFunction)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder movementSlowdown(net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder$MovementSlowdownFunction)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder fallDistanceModifier(float)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder gravityMultiplier(float)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder flowingPushScale(double)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder allowMovingDown(boolean)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder allowBoats(boolean)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder allowSwimming(boolean)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder allowSprinting(boolean)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder allowSprinting(java.util.function.Predicate)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder allowSprinting(java.util.function.BiPredicate)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder makeMobsFloat(boolean)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder makeRiddenMobsFloat(boolean)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder enableDrowning(boolean)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder onEnteredFluid(net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder$OnEnter)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder onExitedFluid(net.fabricmc.fabric.api.registry.fluid.FluidBehavior$Builder$OnExit)
public abstract net.fabricmc.fabric.api.registry.fluid.FluidBehavior build()
```
