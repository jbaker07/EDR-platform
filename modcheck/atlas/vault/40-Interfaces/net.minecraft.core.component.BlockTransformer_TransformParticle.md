---
type: "interface"
fqcn: "net.minecraft.core.component.BlockTransformer$TransformParticle"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.BlockTransformer$TransformParticle

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`enum` public final; extends `java/lang/Enum`; implements `net/minecraft/util/StringRepresentable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `SCRAPE` | `Lnet/minecraft/core/component/BlockTransformer$TransformParticle;` | exact | getstatic@30 in `BlockTransformerHelperImpl.createOxidationScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `WAX_OFF` | `Lnet/minecraft/core/component/BlockTransformer$TransformParticle;` | exact | getstatic@30 in `BlockTransformerHelperImpl.createWaxScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (11 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NONE : Lnet/minecraft/core/component/BlockTransformer$TransformParticle;
public static final SCRAPE : Lnet/minecraft/core/component/BlockTransformer$TransformParticle;
public static final WAX_ON : Lnet/minecraft/core/component/BlockTransformer$TransformParticle;
public static final WAX_OFF : Lnet/minecraft/core/component/BlockTransformer$TransformParticle;
private final id : I
private final name : Ljava/lang/String;
private final levelEvent : I
public static final CODEC : Lcom/mojang/serialization/Codec;
private static final BY_ID : Ljava/util/function/IntFunction;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private static final synthetic $VALUES : [Lnet/minecraft/core/component/BlockTransformer$TransformParticle;
public static values()[Lnet/minecraft/core/component/BlockTransformer$TransformParticle;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/core/component/BlockTransformer$TransformParticle;
private <init>(Ljava/lang/String;IILjava/lang/String;I)V
public send(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/BlockPos;)V
public getSerializedName()Ljava/lang/String;
public getId()I
public getLevelEvent()I
private static synthetic $values()[Lnet/minecraft/core/component/BlockTransformer$TransformParticle;
static <clinit>()V
```
