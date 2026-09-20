---
type: "interface"
fqcn: "net.minecraft.network.codec.IdDispatchCodec"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.codec.IdDispatchCodec

System: [[20-Systems/net.minecraft.network.codec|net.minecraft.network.codec]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/network/codec/StreamCodec`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `decode` | `(Lio/netty/buffer/ByteBuf;)Ljava/lang/Object;` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `decode` | `(Lio/netty/buffer/ByteBuf;)Ljava/lang/Object;` | exact | @Inject at ['NEW'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `encode` | `(Lio/netty/buffer/ByteBuf;Ljava/lang/Object;)V` | exact | @Inject at ['NEW'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (4 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final UNKNOWN_TYPE : I
private final typeGetter : Ljava/util/function/Function;
private final byId : Ljava/util/List;
private final toId : Lit/unimi/dsi/fastutil/objects/Object2IntMap;
private <init>(Ljava/util/function/Function;Ljava/util/List;Lit/unimi/dsi/fastutil/objects/Object2IntMap;)V
public decode(Lio/netty/buffer/ByteBuf;)Ljava/lang/Object;
public encode(Lio/netty/buffer/ByteBuf;Ljava/lang/Object;)V
public static builder(Ljava/util/function/Function;)Lnet/minecraft/network/codec/IdDispatchCodec$Builder;
public synthetic encode(Ljava/lang/Object;Ljava/lang/Object;)V
public synthetic decode(Ljava/lang/Object;)Ljava/lang/Object;
```
