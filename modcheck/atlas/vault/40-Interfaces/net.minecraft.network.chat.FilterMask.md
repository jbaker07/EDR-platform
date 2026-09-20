---
type: "interface"
fqcn: "net.minecraft.network.chat.FilterMask"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.FilterMask

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `applyWithFormatting` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;` | exact | invokevirtual@8 in `ChatListenerMixin.fabric_onFilteredSignedChatMessage` | unknown | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |

## Declared members (8 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final FULLY_FILTERED : Lnet/minecraft/network/chat/FilterMask;
public static final PASS_THROUGH : Lnet/minecraft/network/chat/FilterMask;
public static final FILTERED_STYLE : Lnet/minecraft/network/chat/Style;
private static final HASH : C
private final mask : Ljava/util/BitSet;
private final type : Lnet/minecraft/network/chat/FilterMask$Type;
private <init>(Ljava/util/BitSet;Lnet/minecraft/network/chat/FilterMask$Type;)V
private <init>(Ljava/util/BitSet;)V
public <init>(I)V
private type()Lnet/minecraft/network/chat/FilterMask$Type;
private mask()Ljava/util/BitSet;
public setFiltered(I)V
public apply(Ljava/lang/String;)Ljava/lang/String;
public applyWithFormatting(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
public isEmpty()Z
public isFullyFiltered()Z
public equals(Ljava/lang/Object;)Z
public hashCode()I
static <clinit>()V
```
