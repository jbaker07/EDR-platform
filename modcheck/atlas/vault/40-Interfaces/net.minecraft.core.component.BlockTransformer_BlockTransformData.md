---
type: "interface"
fqcn: "net.minecraft.core.component.BlockTransformer$BlockTransformData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.BlockTransformer$BlockTransformData

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `builder(Lnet/minecraft/world/level/levelgen/feature/stateproviders/` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (31, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.core.component.BlockTransformer$BlockTransformData extends java.lang.Record {
    private final net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider> blockStateProvider;
    private final net.minecraft.core.Holder<net.minecraft.sounds.SoundEvent> sound;
    private final net.minecraft.core.component.BlockTransformer$TransformParticle particle;
    private final java.util.List<net.minecraft.core.Direction> disallowedFaces;
    private final java.util.Optional<net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>> loot;
    private final net.minecraft.core.component.BlockTransformer$DropStrategy dropStrategy;
    private final boolean updateFromNeighbors;
    private final net.minecraft.core.component.BlockTransformer$TransformType transformType;
    private final boolean consumeOnUse;
    private final int itemDamagePerUse;
    public static final com.mojang.serialization.Codec<net.minecraft.core.component.BlockTransformer$BlockTransformData> CODEC;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.core.component.BlockTransformer$BlockTransformData> STREAM_CODEC;
    public net.minecraft.core.component.BlockTransformer$BlockTransformData(net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider>, net.minecraft.core.Holder<net.minecraft.sounds.SoundEvent>, net.minecraft.core.component.BlockTransformer$TransformParticle, java.util.List<net.minecraft.core.Direction>, java.util.Optional<net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>>, net.minecraft.core.component.BlockTransformer$DropStrategy, boolean, net.minecraft.core.component.BlockTransformer$TransformType, boolean, int);
    public static net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder builder(net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider>);
    public static net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder builder(net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider);
    public static net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder builder(net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate, net.minecraft.world.level.block.Block);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.core.Holder<net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider> blockStateProvider();
    public net.minecraft.core.Holder<net.minecraft.sounds.SoundEvent> sound();
    public net.minecraft.core.component.BlockTransformer$TransformParticle particle();
    public java.util.List<net.minecraft.core.Direction> disallowedFaces();
    public java.util.Optional<net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>> loot();
    public net.minecraft.core.component.BlockTransformer$DropStrategy dropStrategy();
    public boolean updateFromNeighbors();
    public net.minecraft.core.component.BlockTransformer$TransformType transformType();
    public boolean consumeOnUse();
    public int itemDamagePerUse();
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
