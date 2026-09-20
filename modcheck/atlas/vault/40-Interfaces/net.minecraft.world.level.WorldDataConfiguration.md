---
type: "interface"
fqcn: "net.minecraft.world.level.WorldDataConfiguration"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.WorldDataConfiguration

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/DataPackConfig;Lnet/minecraft/world/flag/F` | exact | invokespecial@264 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (6 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final dataPacks : Lnet/minecraft/world/level/DataPackConfig;
private final enabledFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
public static final ENABLED_FEATURES_ID : Ljava/lang/String;
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final DEFAULT : Lnet/minecraft/world/level/WorldDataConfiguration;
public <init>(Lnet/minecraft/world/level/DataPackConfig;Lnet/minecraft/world/flag/FeatureFlagSet;)V
public expandFeatures(Lnet/minecraft/world/flag/FeatureFlagSet;)Lnet/minecraft/world/level/WorldDataConfiguration;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public dataPacks()Lnet/minecraft/world/level/DataPackConfig;
public enabledFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
