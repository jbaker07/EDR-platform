---
type: "interface"
fqcn: "net.minecraft.world.flag.FeatureFlagRegistry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.flag.FeatureFlagRegistry

System: [[20-Systems/net.minecraft.world.flag|net.minecraft.world.flag]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `allFlags` | `()Lnet/minecraft/world/flag/FeatureFlagSet;` | exact | invokevirtual@7 in `FabricBlockLootSubProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `allFlags` | `()Lnet/minecraft/world/flag/FeatureFlagSet;` | exact | invokevirtual@4 in `FabricEntityLootSubProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `allFlags` | `()Lnet/minecraft/world/flag/FeatureFlagSet;` | exact | invokevirtual@7 in `ConditionBlockLootSubProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `allFlags` | `()Lnet/minecraft/world/flag/FeatureFlagSet;` | exact | invokevirtual@4 in `ConditionEntityLootSubProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `fromNames` | `(Ljava/lang/Iterable;Ljava/util/function/Consumer;)Lnet/minecraft/worl` | exact | invokevirtual@18 in `ResourceConditionsImpl.featuresEnabled` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `subset` | `([Lnet/minecraft/world/flag/FeatureFlag;)Lnet/minecraft/world/flag/Fea` | exact | invokevirtual@8 in `FeaturesEnabledResourceCondition.<init>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `toNames` | `(Lnet/minecraft/world/flag/FeatureFlagSet;)Ljava/util/Set;` | exact | invokevirtual@11 in `FeaturesEnabledResourceCondition.<init>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (4 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final universe : Lnet/minecraft/world/flag/FeatureFlagUniverse;
private final names : Ljava/util/Map;
private final allFlags : Lnet/minecraft/world/flag/FeatureFlagSet;
private <init>(Lnet/minecraft/world/flag/FeatureFlagUniverse;Lnet/minecraft/world/flag/FeatureFlagSet;Ljava/util/Map;)V
public isSubset(Lnet/minecraft/world/flag/FeatureFlagSet;)Z
public allFlags()Lnet/minecraft/world/flag/FeatureFlagSet;
public fromNames(Ljava/lang/Iterable;)Lnet/minecraft/world/flag/FeatureFlagSet;
public subset([Lnet/minecraft/world/flag/FeatureFlag;)Lnet/minecraft/world/flag/FeatureFlagSet;
public fromNames(Ljava/lang/Iterable;Ljava/util/function/Consumer;)Lnet/minecraft/world/flag/FeatureFlagSet;
public toNames(Lnet/minecraft/world/flag/FeatureFlagSet;)Ljava/util/Set;
public codec()Lcom/mojang/serialization/Codec;
private synthetic lambda$codec$2(Lnet/minecraft/world/flag/FeatureFlagSet;)Ljava/util/List;
private synthetic lambda$codec$0(Ljava/util/List;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$codec$1(Ljava/util/Set;)Ljava/lang/String;
private static synthetic lambda$toNames$0(Lnet/minecraft/world/flag/FeatureFlagSet;Ljava/util/Set;Lnet/minecraft/resources/Identifier;Lnet/minecraft/world/flag/FeatureFlag;)V
private static synthetic lambda$fromNames$0(Lnet/minecraft/resources/Identifier;)V
static <clinit>()V
```
