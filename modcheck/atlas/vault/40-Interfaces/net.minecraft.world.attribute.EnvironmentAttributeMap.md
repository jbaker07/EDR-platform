---
type: "interface"
fqcn: "net.minecraft.world.attribute.EnvironmentAttributeMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.attribute.EnvironmentAttributeMap

System: [[20-Systems/net.minecraft.world.attribute|net.minecraft.world.attribute]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `applyModifier` | `(Lnet/minecraft/world/attribute/EnvironmentAttribute;Ljava/lang/Object` | exact | invokevirtual@21 in `BiomeSelectors.lambda$spawnsOneOf$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Builder;` | exact | invokestatic@0 in `BiomeModificationContextImpl$AttributesContextImpl.addAllRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Builder;` | exact | invokestatic@0 in `BiomeModificationContextImpl$AttributesContextImpl.setRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Builder;` | exact | invokestatic@0 in `BiomeModificationContextImpl$AttributesContextImpl.setModifierRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Builder;` | exact | invokestatic@13 in `DimensionModificationImpl.applyChanges` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `contains` | `(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Z` | exact | invokevirtual@4 in `BiomeModificationContextImpl$AttributesContextImpl.addAll` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `equals` | `(Ljava/lang/Object;)Z` | exact | invokevirtual@47 in `DimensionModificationImpl.applyChanges` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Lnet/minecraft/w` | exact | invokevirtual@13 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (5 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/world/attribute/EnvironmentAttributeMap;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final NETWORK_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC_ONLY_POSITIONAL : Lcom/mojang/serialization/Codec;
private final entries : Ljava/util/Map;
private static filterSyncable(Lnet/minecraft/world/attribute/EnvironmentAttributeMap;)Lnet/minecraft/world/attribute/EnvironmentAttributeMap;
private <init>(Ljava/util/Map;)V
public static builder()Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Builder;
public get(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Entry;
public applyModifier(Lnet/minecraft/world/attribute/EnvironmentAttribute;Ljava/lang/Object;)Ljava/lang/Object;
public contains(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Z
public keySet()Ljava/util/Set;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
private static synthetic lambda$static$2(Lnet/minecraft/world/attribute/EnvironmentAttributeMap;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$4(Ljava/util/List;)Ljava/lang/String;
private static synthetic lambda$static$3(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Z
private static synthetic lambda$static$0()Lcom/mojang/serialization/Codec;
private static synthetic lambda$static$1(Lnet/minecraft/world/attribute/EnvironmentAttributeMap;)Ljava/util/Map;
static <clinit>()V
```
