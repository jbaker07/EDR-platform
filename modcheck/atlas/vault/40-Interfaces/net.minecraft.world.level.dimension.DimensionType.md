---
type: "interface"
fqcn: "net.minecraft.world.level.dimension.DimensionType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.dimension.DimensionType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `attributes()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |

## Declared members (56, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.dimension.DimensionType extends java.lang.Record {
    private final boolean hasFixedTime;
    private final boolean hasSkyLight;
    private final boolean hasCeiling;
    private final boolean hasEnderDragonFight;
    private final double coordinateScale;
    private final int minY;
    private final int height;
    private final int logicalHeight;
    private final net.minecraft.core.HolderSet<net.minecraft.world.level.block.Block> infiniburn;
    private final float ambientLight;
    private final net.minecraft.world.level.dimension.DimensionType$MonsterSettings monsterSettings;
    private final net.minecraft.world.level.dimension.DimensionType$Skybox skybox;
    private final net.minecraft.world.level.CardinalLighting$Type cardinalLightType;
    private final net.minecraft.world.attribute.EnvironmentAttributeMap attributes;
    private final net.minecraft.core.HolderSet<net.minecraft.world.timeline.Timeline> timelines;
    private final java.util.Optional<net.minecraft.core.Holder<net.minecraft.world.clock.WorldClock>> defaultClock;
    public static final int BITS_FOR_Y;
    public static final int MIN_HEIGHT;
    public static final int Y_SIZE;
    public static final int MAX_Y;
    public static final int MIN_Y;
    public static final int WAY_ABOVE_MAX_Y;
    public static final int WAY_BELOW_MIN_Y;
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.dimension.DimensionType> DIRECT_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.dimension.DimensionType> NETWORK_CODEC;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.core.Holder<net.minecraft.world.level.dimension.DimensionType>> STREAM_CODEC;
    public static final float[] MOON_BRIGHTNESS_PER_PHASE;
    public static final com.mojang.serialization.Codec<net.minecraft.core.Holder<net.minecraft.world.level.dimension.DimensionType>> CODEC;
    public net.minecraft.world.level.dimension.DimensionType(boolean, boolean, boolean, boolean, double, int, int, int, net.minecraft.core.HolderSet<net.minecraft.world.level.block.Block>, float, net.minecraft.world.level.dimension.DimensionType$MonsterSettings, net.minecraft.world.level.dimension.DimensionType$Skybox, net.minecraft.world.level.CardinalLighting$Type, net.minecraft.world.attribute.EnvironmentAttributeMap, net.minecraft.core.HolderSet<net.minecraft.world.timeline.Timeline>, java.util.Optional<net.minecraft.core.Holder<net.minecraft.world.clock.WorldClock>>);
    private static com.mojang.serialization.Codec<net.minecraft.world.level.dimension.DimensionType> createDirectCodec(com.mojang.serialization.Codec<net.minecraft.world.attribute.EnvironmentAttributeMap>);
    public static double getTeleportationScale(net.minecraft.world.level.dimension.DimensionType, net.minecraft.world.level.dimension.DimensionType);
    public static java.nio.file.Path getStorageFolder(net.minecraft.resources.ResourceKey<net.minecraft.world.level.Level>, java.nio.file.Path);
    public net.minecraft.util.valueproviders.IntProvider monsterSpawnLightTest();
    public int monsterSpawnBlockLightLimit();
    public boolean hasEndFlashes();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public boolean hasFixedTime();
    public boolean hasSkyLight();
    public boolean hasCeiling();
    public boolean hasEnderDragonFight();
    public double coordinateScale();
    public int minY();
    public int height();
    public int logicalHeight();
    public net.minecraft.core.HolderSet<net.minecraft.world.level.block.Block> infiniburn();
    public float ambientLight();
    public net.minecraft.world.level.dimension.DimensionType$MonsterSettings monsterSettings();
    public net.minecraft.world.level.dimension.DimensionType$Skybox skybox();
    public net.minecraft.world.level.CardinalLighting$Type cardinalLightType();
    public net.minecraft.world.attribute.EnvironmentAttributeMap attributes();
    public net.minecraft.core.HolderSet<net.minecraft.world.timeline.Timeline> timelines();
    public java.util.Optional<net.minecraft.core.Holder<net.minecraft.world.clock.WorldClock>> defaultClock();
    private static com.mojang.datafixers.kinds.App lambda$createDirectCodec$0(com.mojang.serialization.Codec, com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
