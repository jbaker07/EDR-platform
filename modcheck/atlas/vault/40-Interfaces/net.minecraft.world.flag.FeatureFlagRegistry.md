---
type: "interface"
fqcn: "net.minecraft.world.flag.FeatureFlagRegistry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.flag.FeatureFlagRegistry

System: [[20-Systems/net.minecraft.world.flag|net.minecraft.world.flag]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `allFlags()Lnet/minecraft/world/flag/FeatureFlagSet;` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `allFlags()Lnet/minecraft/world/flag/FeatureFlagSet;` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `fromNames(Ljava/lang/Iterable;Ljava/util/function/Consumer;)Lnet/mine` | `` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `subset([Lnet/minecraft/world/flag/FeatureFlag;)Lnet/minecraft/worl` | `` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `toNames(Lnet/minecraft/world/flag/FeatureFlagSet;)Ljava/util/Set;` | `` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (18, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.flag.FeatureFlagRegistry {
    private static final org.slf4j.Logger LOGGER;
    private final net.minecraft.world.flag.FeatureFlagUniverse universe;
    private final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.world.flag.FeatureFlag> names;
    private final net.minecraft.world.flag.FeatureFlagSet allFlags;
    private net.minecraft.world.flag.FeatureFlagRegistry(net.minecraft.world.flag.FeatureFlagUniverse, net.minecraft.world.flag.FeatureFlagSet, java.util.Map<net.minecraft.resources.Identifier, net.minecraft.world.flag.FeatureFlag>);
    public boolean isSubset(net.minecraft.world.flag.FeatureFlagSet);
    public net.minecraft.world.flag.FeatureFlagSet allFlags();
    public net.minecraft.world.flag.FeatureFlagSet fromNames(java.lang.Iterable<net.minecraft.resources.Identifier>);
    public net.minecraft.world.flag.FeatureFlagSet subset(net.minecraft.world.flag.FeatureFlag...);
    public net.minecraft.world.flag.FeatureFlagSet fromNames(java.lang.Iterable<net.minecraft.resources.Identifier>, java.util.function.Consumer<net.minecraft.resources.Identifier>);
    public java.util.Set<net.minecraft.resources.Identifier> toNames(net.minecraft.world.flag.FeatureFlagSet);
    public com.mojang.serialization.Codec<net.minecraft.world.flag.FeatureFlagSet> codec();
    private java.util.List lambda$codec$2(net.minecraft.world.flag.FeatureFlagSet);
    private com.mojang.serialization.DataResult lambda$codec$0(java.util.List);
    private static java.lang.String lambda$codec$1(java.util.Set);
    private static void lambda$toNames$0(net.minecraft.world.flag.FeatureFlagSet, java.util.Set, net.minecraft.resources.Identifier, net.minecraft.world.flag.FeatureFlag);
    private static void lambda$fromNames$0(net.minecraft.resources.Identifier);
    static {};
}
```
