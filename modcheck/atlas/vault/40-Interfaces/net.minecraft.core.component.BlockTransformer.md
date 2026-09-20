---
type: "interface"
fqcn: "net.minecraft.core.component.BlockTransformer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.BlockTransformer

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/List;)V` | exact | invokespecial@135 in `BlockTransformerHelperImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `transforms` | `()Ljava/util/List;` | exact | invokevirtual@5 in `BlockTransformerHelperImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `transforms` | `()Ljava/util/List;` | exact | invokevirtual@104 in `BlockTransformerHelperImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `transforms` | `()Ljava/util/List;` | exact | invokevirtual@4 in `BlockTransformerHelperImpl.registerAxe` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `transforms` | `()Ljava/util/List;` | exact | invokevirtual@4 in `BlockTransformerHelperImpl.registerHoe` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `transforms` | `()Ljava/util/List;` | exact | invokevirtual@4 in `BlockTransformerHelperImpl.registerShovel` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `<clinit>` | `()V` | exact | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (4 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final transforms : Ljava/util/List;
public static final DIRECT_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Ljava/util/List;)V
public transformBlock(Lnet/minecraft/world/item/context/UseOnContext;)Lnet/minecraft/world/InteractionResult;
private static playerHasBlockingItemUseIntent(Lnet/minecraft/world/item/context/UseOnContext;)Z
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public transforms()Ljava/util/List;
private static synthetic lambda$transformBlock$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/core/component/BlockTransformer$BlockTransformData;Lnet/minecraft/core/Direction;Lnet/minecraft/resources/ResourceKey;)V
private static synthetic lambda$transformBlock$1(Lnet/minecraft/core/component/BlockTransformer$BlockTransformData;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;)V
static <clinit>()V
```
