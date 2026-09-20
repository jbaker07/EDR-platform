---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@53 in `OxidizableBlocksRegistryImpl.registerNextStage` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@45 in `OxidizableBlocksRegistryImpl.registerWaxable` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@2 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@2 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@2 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@12 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@12 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@12 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@5 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@5 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@5 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@5 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@5 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/le` | exact | invokestatic@5 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@2 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@2 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@2 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@12 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@12 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@12 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@5 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@5 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@5 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@5 in `BlockTransformerHelper.registerStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@5 in `BlockTransformerHelper.registerTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@5 in `BlockTransformerHelper.registerFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (4 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final TYPED_CODEC : Lcom/mojang/serialization/Codec;
public static final STATE_OR_PROVIDER_CODEC : Lcom/mojang/serialization/Codec;
public static final DIRECT_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static of(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/levelgen/feature/stateproviders/SimpleStateProvider;
public static of(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/levelgen/feature/stateproviders/SimpleStateProvider;
public static holderOf(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/Holder;
public static holderOf(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/core/Holder;
public abstract codec()Lcom/mojang/serialization/MapCodec;
public abstract getState(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/util/RandomSource;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public getOptionalState(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/util/RandomSource;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
private static synthetic lambda$static$3(Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider;)Lcom/mojang/datafixers/util/Either;
private static synthetic lambda$static$1(Lcom/mojang/datafixers/util/Either;)Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider;)Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider;
private static synthetic lambda$static$0(Lcom/mojang/serialization/MapCodec;)Lcom/mojang/serialization/MapCodec;
static <clinit>()V
```
