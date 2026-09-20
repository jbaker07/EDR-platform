---
type: "interface"
fqcn: "net.minecraft.world.flag.FeatureFlags"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.flag.FeatureFlags

System: [[20-Systems/net.minecraft.world.flag|net.minecraft.world.flag]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `DEFAULT_FLAGS` | `Lnet/minecraft/world/flag/FeatureFlagSet;` | exact | getstatic@261 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/world/flag/FeatureFlagRegistry;` | exact | getstatic@4 in `FabricBlockLootSubProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/world/flag/FeatureFlagRegistry;` | exact | getstatic@1 in `FabricEntityLootSubProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/world/flag/FeatureFlagRegistry;` | exact | getstatic@4 in `ConditionBlockLootSubProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/world/flag/FeatureFlagRegistry;` | exact | getstatic@1 in `ConditionEntityLootSubProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/world/flag/FeatureFlagRegistry;` | exact | getstatic@8 in `ResourceConditionsImpl.featuresEnabled` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/world/flag/FeatureFlagRegistry;` | exact | getstatic@1 in `FeaturesEnabledResourceCondition.<init>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/world/flag/FeatureFlagRegistry;` | exact | getstatic@4 in `FeaturesEnabledResourceCondition.<init>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| reads | `VANILLA_SET` | `Lnet/minecraft/world/flag/FeatureFlagSet;` | exact | getstatic@2 in `ExtendedMenuType.<init>` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |

## Declared members (8 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final VANILLA : Lnet/minecraft/world/flag/FeatureFlag;
public static final TRADE_REBALANCE : Lnet/minecraft/world/flag/FeatureFlag;
public static final REDSTONE_EXPERIMENTS : Lnet/minecraft/world/flag/FeatureFlag;
public static final MINECART_IMPROVEMENTS : Lnet/minecraft/world/flag/FeatureFlag;
public static final REGISTRY : Lnet/minecraft/world/flag/FeatureFlagRegistry;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final VANILLA_SET : Lnet/minecraft/world/flag/FeatureFlagSet;
public static final DEFAULT_FLAGS : Lnet/minecraft/world/flag/FeatureFlagSet;
public <init>()V
public static printMissingFlags(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/flag/FeatureFlagSet;)Ljava/lang/String;
public static printMissingFlags(Lnet/minecraft/world/flag/FeatureFlagRegistry;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/flag/FeatureFlagSet;)Ljava/lang/String;
public static isExperimental(Lnet/minecraft/world/flag/FeatureFlagSet;)Z
private static synthetic lambda$printMissingFlags$0(Ljava/util/Set;Lnet/minecraft/resources/Identifier;)Z
static <clinit>()V
```
