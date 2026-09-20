---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `allOf(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPr` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/wor` | `` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `matchesTag(Lnet/minecraft/core/Directional;Lnet/minecraft/tags/TagKey;` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (33, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate extends java.util.function.BiPredicate<net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos> {
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate> CODEC;
    public static final net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate ONLY_IN_AIR_PREDICATE;
    public static final net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate ONLY_IN_AIR_OR_WATER_PREDICATE;
    public abstract net.minecraft.world.level.levelgen.blockpredicates.BlockPredicateType<?> type();
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate allOf(java.util.List<net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate>);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate allOf(net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate...);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate allOf(net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate, net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate anyOf(java.util.List<net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate>);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate anyOf(net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate...);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate anyOf(net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate, net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate matchesBlocks(net.minecraft.core.Vec3i, java.util.List<net.minecraft.world.level.block.Block>);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate matchesBlocks(net.minecraft.core.Directional, net.minecraft.world.level.block.Block...);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate matchesBlocks(net.minecraft.world.level.block.Block...);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate matchesTag(net.minecraft.core.Vec3i, net.minecraft.tags.TagKey<net.minecraft.world.level.block.Block>);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate matchesTag(net.minecraft.core.Directional, net.minecraft.tags.TagKey<net.minecraft.world.level.block.Block>);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate matchesTag(net.minecraft.tags.TagKey<net.minecraft.world.level.block.Block>);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate matchesFluids(net.minecraft.core.Directional, net.minecraft.world.level.material.Fluid...);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate matchesFluids(net.minecraft.core.Vec3i, java.util.List<net.minecraft.world.level.material.Fluid>);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate matchesFluids(net.minecraft.world.level.material.Fluid...);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate not(net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate replaceable();
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate wouldSurvive(net.minecraft.world.level.block.Block);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate hasSturdyFace(net.minecraft.core.Direction);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate hasSturdyFace(net.minecraft.core.Directional, net.minecraft.core.Direction);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate solid(net.minecraft.core.Directional);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate solid();
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate noFluid();
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate insideWorld(net.minecraft.core.Vec3i);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate alwaysTrue();
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate unobstructed();
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate heightRange(net.minecraft.world.level.levelgen.VerticalAnchor, net.minecraft.world.level.levelgen.VerticalAnchor);
    public static net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate volumeMatch(net.minecraft.core.Vec3i, net.minecraft.core.Vec3i, net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate);
    static {};
}
```
