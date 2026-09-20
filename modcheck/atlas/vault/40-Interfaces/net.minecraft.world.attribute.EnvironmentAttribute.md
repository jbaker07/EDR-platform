---
type: "interface"
fqcn: "net.minecraft.world.attribute.EnvironmentAttribute"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.attribute.EnvironmentAttribute

System: [[20-Systems/net.minecraft.world.attribute|net.minecraft.world.attribute]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `defaultValue` | `()Ljava/lang/Object;` | exact | invokevirtual@15 in `BiomeSelectors.lambda$spawnsOneOf$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `defaultValue` | `()Ljava/lang/Object;` | exact | invokevirtual@47 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `defaultValue` | `()Ljava/lang/Object;` | exact | invokevirtual@60 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (7 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final type : Lnet/minecraft/world/attribute/AttributeType;
private final defaultValue : Ljava/lang/Object;
private final valueRange : Lnet/minecraft/world/attribute/AttributeRange;
private final isSyncable : Z
private final isPositional : Z
private final isSpatiallyInterpolated : Z
private final fullResolutionBiomes : Z
private <init>(Lnet/minecraft/world/attribute/AttributeType;Ljava/lang/Object;Lnet/minecraft/world/attribute/AttributeRange;ZZZZ)V
public static builder(Lnet/minecraft/world/attribute/AttributeType;)Lnet/minecraft/world/attribute/EnvironmentAttribute$Builder;
public type()Lnet/minecraft/world/attribute/AttributeType;
public defaultValue()Ljava/lang/Object;
public valueCodec()Lcom/mojang/serialization/Codec;
public sanitizeValue(Ljava/lang/Object;)Ljava/lang/Object;
public isSyncable()Z
public isPositional()Z
public isSpatiallyInterpolated()Z
public isFullResolutionBiomes()Z
public toString()Ljava/lang/String;
```
