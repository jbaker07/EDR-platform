---
type: "interface"
fqcn: "net.minecraft.core.component.BlockTransformer$BlockTransformData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.BlockTransformer$BlockTransformData

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `builder` | `(Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockState` | exact | invokestatic@21 in `BlockTransformerHelperImpl.createStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `builder` | `(Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockState` | exact | invokestatic@23 in `BlockTransformerHelperImpl.createTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `builder` | `(Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockState` | exact | invokestatic@23 in `BlockTransformerHelperImpl.createFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `builder` | `(Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockState` | exact | invokestatic@21 in `BlockTransformerHelperImpl.createOxidationScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `builder` | `(Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockState` | exact | invokestatic@21 in `BlockTransformerHelperImpl.createWaxScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (12 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final blockStateProvider : Lnet/minecraft/core/Holder;
private final sound : Lnet/minecraft/core/Holder;
private final particle : Lnet/minecraft/core/component/BlockTransformer$TransformParticle;
private final disallowedFaces : Ljava/util/List;
private final loot : Ljava/util/Optional;
private final dropStrategy : Lnet/minecraft/core/component/BlockTransformer$DropStrategy;
private final updateFromNeighbors : Z
private final transformType : Lnet/minecraft/core/component/BlockTransformer$TransformType;
private final consumeOnUse : Z
private final itemDamagePerUse : I
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/core/Holder;Lnet/minecraft/core/Holder;Lnet/minecraft/core/component/BlockTransformer$TransformParticle;Ljava/util/List;Ljava/util/Optional;Lnet/minecraft/core/component/BlockTransformer$DropStrategy;ZLnet/minecraft/core/component/BlockTransformer$TransformType;ZI)V
public static builder(Lnet/minecraft/core/Holder;)Lnet/minecraft/core/component/BlockTransformer$BlockTransformData$Builder;
public static builder(Lnet/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider;)Lnet/minecraft/core/component/BlockTransformer$BlockTransformData$Builder;
public static builder(Lnet/minecraft/world/level/levelgen/blockpredicates/BlockPredicate;Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/core/component/BlockTransformer$BlockTransformData$Builder;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public blockStateProvider()Lnet/minecraft/core/Holder;
public sound()Lnet/minecraft/core/Holder;
public particle()Lnet/minecraft/core/component/BlockTransformer$TransformParticle;
public disallowedFaces()Ljava/util/List;
public loot()Ljava/util/Optional;
public dropStrategy()Lnet/minecraft/core/component/BlockTransformer$DropStrategy;
public updateFromNeighbors()Z
public transformType()Lnet/minecraft/core/component/BlockTransformer$TransformType;
public consumeOnUse()Z
public itemDamagePerUse()I
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
