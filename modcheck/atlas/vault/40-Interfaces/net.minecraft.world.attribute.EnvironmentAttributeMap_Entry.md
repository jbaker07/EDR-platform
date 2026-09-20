---
type: "interface"
fqcn: "net.minecraft.world.attribute.EnvironmentAttributeMap$Entry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.attribute.EnvironmentAttributeMap$Entry

System: [[20-Systems/net.minecraft.world.attribute|net.minecraft.world.attribute]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `applyModifier` | `(Ljava/lang/Object;)Ljava/lang/Object;` | exact | invokevirtual@66 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `modifier` | `()Lnet/minecraft/world/attribute/modifier/AttributeModifier;` | exact | invokevirtual@23 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (2 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final argument : Ljava/lang/Object;
private final modifier : Lnet/minecraft/world/attribute/modifier/AttributeModifier;
public <init>(Ljava/lang/Object;Lnet/minecraft/world/attribute/modifier/AttributeModifier;)V
private static createCodec(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Lcom/mojang/serialization/Codec;
private static createFullCodec(Lnet/minecraft/world/attribute/EnvironmentAttribute;Lnet/minecraft/world/attribute/modifier/AttributeModifier;)Lcom/mojang/serialization/MapCodec;
public applyModifier(Ljava/lang/Object;)Ljava/lang/Object;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public argument()Ljava/lang/Object;
public modifier()Lnet/minecraft/world/attribute/modifier/AttributeModifier;
private static synthetic lambda$createFullCodec$0(Lnet/minecraft/world/attribute/modifier/AttributeModifier;Lnet/minecraft/world/attribute/EnvironmentAttribute;Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$createFullCodec$1(Lnet/minecraft/world/attribute/modifier/AttributeModifier;Ljava/lang/Object;)Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Entry;
private static synthetic lambda$createCodec$4(Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Entry;)Lcom/mojang/datafixers/util/Either;
private static synthetic lambda$createCodec$1(Lcom/mojang/datafixers/util/Either;)Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Entry;
private static synthetic lambda$createCodec$3(Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Entry;)Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Entry;
private static synthetic lambda$createCodec$2(Ljava/lang/Object;)Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Entry;
private static synthetic lambda$createCodec$0(Lnet/minecraft/world/attribute/EnvironmentAttribute;Lnet/minecraft/world/attribute/modifier/AttributeModifier;)Lcom/mojang/serialization/MapCodec;
```
