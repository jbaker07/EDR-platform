---
type: "interface"
fqcn: "net.minecraft.world.level.WorldDataConfiguration"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.WorldDataConfiguration

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/level/DataPackConfig;Lnet/minecraft/wo` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.WorldDataConfiguration extends java.lang.Record {
    private final net.minecraft.world.level.DataPackConfig dataPacks;
    private final net.minecraft.world.flag.FeatureFlagSet enabledFeatures;
    public static final java.lang.String ENABLED_FEATURES_ID;
    public static final com.mojang.serialization.MapCodec<net.minecraft.world.level.WorldDataConfiguration> MAP_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.WorldDataConfiguration> CODEC;
    public static final net.minecraft.world.level.WorldDataConfiguration DEFAULT;
    public net.minecraft.world.level.WorldDataConfiguration(net.minecraft.world.level.DataPackConfig, net.minecraft.world.flag.FeatureFlagSet);
    public net.minecraft.world.level.WorldDataConfiguration expandFeatures(net.minecraft.world.flag.FeatureFlagSet);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.level.DataPackConfig dataPacks();
    public net.minecraft.world.flag.FeatureFlagSet enabledFeatures();
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
