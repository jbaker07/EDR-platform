---
type: "interface"
fqcn: "net.fabricmc.fabric.api.object.builder.v1.entity.FabricEntityType$Builder"
module: "fabric-object-builder-api-v1"
sha256: "3a5f0ccef440552828d9469420547dc3cdbe3e206af73fa98d0bfcfb5b75c1ba"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.object.builder.v1.entity.FabricEntityType$Builder

Module: [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] -- kind: interface

```java
public default net.minecraft.world.entity.EntityType$Builder<T> alwaysUpdateVelocity(boolean)
public default net.minecraft.world.entity.EntityType$Builder<T> canPotentiallyExecuteCommands(boolean)
public static <T extends net.minecraft.world.entity.LivingEntity> net.minecraft.world.entity.EntityType$Builder<T> createLiving(net.minecraft.world.entity.EntityType$EntityFactory<T>, net.minecraft.world.entity.MobCategory, java.util.function.UnaryOperator<net.fabricmc.fabric.api.object.builder.v1.entity.FabricEntityType$Builder$Living<T>>)
public static <T extends net.minecraft.world.entity.Mob> net.minecraft.world.entity.EntityType$Builder<T> createMob(net.minecraft.world.entity.EntityType$EntityFactory<T>, net.minecraft.world.entity.MobCategory, java.util.function.UnaryOperator<net.fabricmc.fabric.api.object.builder.v1.entity.FabricEntityType$Builder$Mob<T>>)
```
