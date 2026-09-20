---
type: "interface"
fqcn: "net.minecraft.world.flag.FeatureFlagSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.flag.FeatureFlagSet

System: [[20-Systems/net.minecraft.world.flag|net.minecraft.world.flag]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `isSubsetOf` | `(Lnet/minecraft/world/flag/FeatureFlagSet;)Z` | exact | invokevirtual@53 in `ResourceConditionsImpl.featuresEnabled` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `of` | `()Lnet/minecraft/world/flag/FeatureFlagSet;` | exact | invokestatic@19 in `GameRuleBuilder.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (4 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final EMPTY : Lnet/minecraft/world/flag/FeatureFlagSet;
public static final MAX_CONTAINER_SIZE : I
private final universe : Lnet/minecraft/world/flag/FeatureFlagUniverse;
private final mask : J
private <init>(Lnet/minecraft/world/flag/FeatureFlagUniverse;J)V
static create(Lnet/minecraft/world/flag/FeatureFlagUniverse;Ljava/util/Collection;)Lnet/minecraft/world/flag/FeatureFlagSet;
public static of()Lnet/minecraft/world/flag/FeatureFlagSet;
public static of(Lnet/minecraft/world/flag/FeatureFlag;)Lnet/minecraft/world/flag/FeatureFlagSet;
public static of(Lnet/minecraft/world/flag/FeatureFlag;[Lnet/minecraft/world/flag/FeatureFlag;)Lnet/minecraft/world/flag/FeatureFlagSet;
private static computeMask(Lnet/minecraft/world/flag/FeatureFlagUniverse;JLjava/lang/Iterable;)J
public contains(Lnet/minecraft/world/flag/FeatureFlag;)Z
public isEmpty()Z
public isSubsetOf(Lnet/minecraft/world/flag/FeatureFlagSet;)Z
public intersects(Lnet/minecraft/world/flag/FeatureFlagSet;)Z
public join(Lnet/minecraft/world/flag/FeatureFlagSet;)Lnet/minecraft/world/flag/FeatureFlagSet;
public subtract(Lnet/minecraft/world/flag/FeatureFlagSet;)Lnet/minecraft/world/flag/FeatureFlagSet;
public equals(Ljava/lang/Object;)Z
public hashCode()I
static <clinit>()V
```
