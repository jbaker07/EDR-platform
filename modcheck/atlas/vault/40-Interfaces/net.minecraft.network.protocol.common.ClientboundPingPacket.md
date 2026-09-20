---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.ClientboundPingPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.ClientboundPingPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/network/protocol/Packet`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(I)V` | exact | invokespecial@21 in `ServerConfigurationNetworkAddon.startConfiguration` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (2 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private final id : I
public <init>(I)V
private <init>(Lnet/minecraft/network/FriendlyByteBuf;)V
private write(Lnet/minecraft/network/FriendlyByteBuf;)V
public type()Lnet/minecraft/network/protocol/PacketType;
public handle(Lnet/minecraft/network/protocol/common/ClientCommonPacketListener;)V
public getId()I
public synthetic handle(Lnet/minecraft/network/PacketListener;)V
static <clinit>()V
```
