---
type: "interface"
fqcn: "net.minecraft.network.chat.ComponentSerialization"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.ComponentSerialization

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@27 in `Networking$OpenScreenPayload.fromBuf` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| reads | `STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@18 in `Networking$OpenScreenPayload.write` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |

## Declared members (6 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final OPTIONAL_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final TRUSTED_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final TRUSTED_OPTIONAL_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final TRUSTED_CONTEXT_FREE_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>()V
public static flatRestrictedCodec(I)Lcom/mojang/serialization/Codec;
private static createFromList(Ljava/util/List;)Lnet/minecraft/network/chat/MutableComponent;
public static createLegacyComponentMatcher(Lnet/minecraft/util/ExtraCodecs$LateBoundIdMapper;Ljava/util/function/Function;Ljava/lang/String;)Lcom/mojang/serialization/MapCodec;
private static createCodec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
private static bootstrap(Lnet/minecraft/util/ExtraCodecs$LateBoundIdMapper;)V
private static synthetic lambda$createCodec$4(Lnet/minecraft/network/chat/Component;)Lcom/mojang/datafixers/util/Either;
private static synthetic lambda$createCodec$1(Lcom/mojang/datafixers/util/Either;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$createCodec$3(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$createCodec$2(Lcom/mojang/datafixers/util/Either;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$createCodec$0(Lcom/mojang/serialization/MapCodec;Lcom/mojang/serialization/Codec;Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$createLegacyComponentMatcher$0(Lcom/mojang/serialization/MapCodec;)Lcom/mojang/serialization/MapCodec;
static <clinit>()V
```
