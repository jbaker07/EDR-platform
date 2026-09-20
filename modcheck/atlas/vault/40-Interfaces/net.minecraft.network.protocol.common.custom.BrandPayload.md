---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.custom.BrandPayload"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.custom.BrandPayload

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/network/protocol/common/custom/CustomPacketPayload`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `brand` | `()Ljava/lang/String;` | exact | invokevirtual@14 in `ServerConfigurationNetworkAddon.handle` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (3 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final brand : Ljava/lang/String;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final TYPE : Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Type;
private <init>(Lnet/minecraft/network/FriendlyByteBuf;)V
public <init>(Ljava/lang/String;)V
private write(Lnet/minecraft/network/FriendlyByteBuf;)V
public type()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Type;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public brand()Ljava/lang/String;
static <clinit>()V
```
