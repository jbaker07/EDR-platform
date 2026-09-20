---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements `java/util/function/BiPredicate`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `allOf` | `(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Ln` | exact | invokestatic@13 in `BlockTransformerHelperImpl.createTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `allOf` | `(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Ln` | exact | invokestatic@13 in `BlockTransformerHelperImpl.createFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@49 in `OxidizableBlocksRegistryImpl.registerNextStage` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@41 in `OxidizableBlocksRegistryImpl.registerWaxable` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@8 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@8 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@8 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@1 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@1 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@1 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@8 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@8 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@8 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@1 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@1 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@1 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@8 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@8 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@8 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@1 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@1 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesBlocks` | `([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/l` | exact | invokestatic@1 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesTag` | `(Lnet/minecraft/core/Directional;Lnet/minecraft/tags/TagKey;)Lnet/mine` | exact | invokestatic@10 in `BlockTransformerHelperImpl.createTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesTag` | `(Lnet/minecraft/core/Directional;Lnet/minecraft/tags/TagKey;)Lnet/mine` | exact | invokestatic@10 in `BlockTransformerHelperImpl.createFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesTag` | `(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/level/levelgen/block` | exact | invokestatic@1 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesTag` | `(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/level/levelgen/block` | exact | invokestatic@1 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesTag` | `(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/level/levelgen/block` | exact | invokestatic@1 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesTag` | `(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/level/levelgen/block` | exact | invokestatic@1 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesTag` | `(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/level/levelgen/block` | exact | invokestatic@1 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesTag` | `(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/level/levelgen/block` | exact | invokestatic@1 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesTag` | `(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/level/levelgen/block` | exact | invokestatic@1 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesTag` | `(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/level/levelgen/block` | exact | invokestatic@1 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `matchesTag` | `(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/level/levelgen/block` | exact | invokestatic@1 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (3 fields, 30 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final ONLY_IN_AIR_PREDICATE : Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static final ONLY_IN_AIR_OR_WATER_PREDICATE : Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public abstract type()Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicateType;
public static allOf(Ljava/util/List;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static allOf([Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static allOf(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static anyOf(Ljava/util/List;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static anyOf([Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static anyOf(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static matchesBlocks(Lnet/minecraft/core/Vec3i;Ljava/util/List;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static matchesBlocks(Lnet/minecraft/core/Directional;[Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static matchesBlocks([Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static matchesTag(Lnet/minecraft/core/Vec3i;Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static matchesTag(Lnet/minecraft/core/Directional;Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static matchesTag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static matchesFluids(Lnet/minecraft/core/Directional;[Lnet/minecraft/world/level/material/Fluid;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static matchesFluids(Lnet/minecraft/core/Vec3i;Ljava/util/List;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static matchesFluids([Lnet/minecraft/world/level/material/Fluid;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static not(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static replaceable()Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static wouldSurvive(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static hasSturdyFace(Lnet/minecraft/core/Direction;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static hasSturdyFace(Lnet/minecraft/core/Directional;Lnet/minecraft/core/Direction;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static solid(Lnet/minecraft/core/Directional;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static solid()Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static noFluid()Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static insideWorld(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static alwaysTrue()Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static unobstructed()Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static heightRange(Lnet/minecraft/world/level/levelgen/VerticalAnchor;Lnet/minecraft/world/level/levelgen/VerticalAnchor;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
public static volumeMatch(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Vec3i;Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;)Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;
static <clinit>()V
```
