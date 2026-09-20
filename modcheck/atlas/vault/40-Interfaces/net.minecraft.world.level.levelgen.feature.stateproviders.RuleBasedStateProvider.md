---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `builder` | `()Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBased` | exact | invokestatic@0 in `BlockTransformerHelperImpl.createStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBased` | exact | invokestatic@0 in `BlockTransformerHelperImpl.createTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBased` | exact | invokestatic@0 in `BlockTransformerHelperImpl.createFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBased` | exact | invokestatic@0 in `BlockTransformerHelperImpl.createOxidationScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBased` | exact | invokestatic@0 in `BlockTransformerHelperImpl.createWaxScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (3 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final fallback : Lnet/minecraft/core/Holder;
private final rules : Ljava/util/List;
public static final CODEC : Lcom/mojang/serialization/MapCodec;
private <init>(Ljava/util/Optional;Ljava/util/List;)V
public <init>(Lnet/minecraft/core/Holder;Ljava/util/List;)V
public static ifTrueThenProvide(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBasedStateProvider;
public static ifTrueThenProvide(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider;)Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBasedStateProvider;
public codec()Lcom/mojang/serialization/MapCodec;
public getState(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/util/RandomSource;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public getOptionalState(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/util/RandomSource;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public static builder()Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBasedStateProvider$Builder;
public static builder(Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider;)Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBasedStateProvider$Builder;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public fallback()Lnet/minecraft/core/Holder;
public rules()Ljava/util/List;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBasedStateProvider;)Ljava/util/List;
private static synthetic lambda$static$1(Lnet/minecraft/world/level/levelgen/feature/stateproviders/RuleBasedStateProvider;)Ljava/util/Optional;
static <clinit>()V
```
