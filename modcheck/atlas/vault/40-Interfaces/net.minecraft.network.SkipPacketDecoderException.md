---
type: "interface"
fqcn: "net.minecraft.network.SkipPacketDecoderException"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.SkipPacketDecoderException

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`class` public; extends `io/netty/handler/codec/DecoderException`; implements `net/minecraft/network/codec/IdDispatchCodec$DontDecorateException`, `net/minecraft/network/SkipPacketException`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/String;)V` | exact | invokespecial@42 in `ClientboundRecipeSyncPayload$Entry.read` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (0 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>(Ljava/lang/String;)V
public <init>(Ljava/lang/Throwable;)V
```
