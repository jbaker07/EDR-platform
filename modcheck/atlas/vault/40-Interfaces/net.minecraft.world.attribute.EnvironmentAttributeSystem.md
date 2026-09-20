---
type: "interface"
fqcn: "net.minecraft.world.attribute.EnvironmentAttributeSystem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.attribute.EnvironmentAttributeSystem

System: [[20-Systems/net.minecraft.world.attribute|net.minecraft.world.attribute]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/attribute/EnvironmentAttributeReader`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getDimensionValue` | `(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Ljava/lang/Objec` | exact | invokevirtual@11 in `FluidVariantAttributes$3.getViscosity` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (1 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final attributeSamplers : Ljava/util/Map;
private <init>(Ljava/util/Map;)V
private bakeLayerSampler(Lnet/minecraft/world/attribute/EnvironmentAttribute;Ljava/util/List;)Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$ValueSampler;
public static builder()Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$Builder;
private static addStaticLayers(Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$Builder;Lnet/minecraft/world/level/LevelAccessor;)V
private static addDynamicLayers(Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$Builder;Lnet/minecraft/world/level/Level;)V
private static addDimensionLayer(Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$Builder;Lnet/minecraft/world/level/dimension/DimensionType;)V
private static addBiomeLayer(Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$Builder;Lnet/minecraft/core/HolderLookup;Lnet/minecraft/world/level/biome/BiomeManager;)V
private static addBiomeLayerForAttribute(Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$Builder;Lnet/minecraft/world/attribute/EnvironmentAttribute;Lnet/minecraft/world/level/biome/BiomeManager;)V
public invalidateTickCache()V
private getValueSampler(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$ValueSampler;
public getDimensionValue(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Ljava/lang/Object;
public getValue(Lnet/minecraft/world/attribute/EnvironmentAttribute;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/attribute/SpatialAttributeInterpolator;)Ljava/lang/Object;
 getConstantBaseValue(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Ljava/lang/Object;
 isAffectedByPosition(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Z
private static synthetic lambda$addBiomeLayerForAttribute$0(Lnet/minecraft/world/attribute/EnvironmentAttribute;Lnet/minecraft/world/level/biome/BiomeManager;Ljava/lang/Object;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/attribute/SpatialAttributeInterpolator;)Ljava/lang/Object;
private static synthetic lambda$addBiomeLayer$1(Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$Builder;Lnet/minecraft/world/level/biome/BiomeManager;Lnet/minecraft/world/attribute/EnvironmentAttribute;)V
private static synthetic lambda$addBiomeLayer$0(Lnet/minecraft/core/Holder$Reference;)Ljava/util/stream/Stream;
private static synthetic lambda$addDynamicLayers$0(Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$Builder;Lnet/minecraft/world/clock/ClockManager;Lnet/minecraft/core/Holder;)V
private static synthetic lambda$bakeLayerSampler$0(Lnet/minecraft/world/attribute/EnvironmentAttributeLayer;)Z
private synthetic lambda$new$0(Lnet/minecraft/world/attribute/EnvironmentAttribute;Ljava/util/List;)V
```
