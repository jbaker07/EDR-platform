---
type: "interface"
fqcn: "net.minecraft.network.protocol.PacketFlow"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.PacketFlow

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `CLIENTBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@1 in `FakePlayerPacketListener$FakeConnection.<init>` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `CLIENTBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@49 in `PayloadTypeRegistryImpl.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CLIENTBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@46 in `PayloadTypeRegistryImpl.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CLIENTBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@70 in `PayloadTypeRegistryImpl.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CLIENTBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@23 in `PayloadTypeRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CLIENTBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@55 in `PayloadTypeRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CLIENTBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@46 in `VanillaPacketTypes.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CLIENTBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@70 in `VanillaPacketTypes.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CLIENTBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@4 in `ClientNetworkingImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CLIENTBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@21 in `ClientNetworkingImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CLIENTBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@40 in `ClientNetworkingImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `SERVERBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@7 in `PayloadTypeRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `SERVERBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@39 in `PayloadTypeRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `SERVERBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@4 in `ServerNetworkingImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `SERVERBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@21 in `ServerNetworkingImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `SERVERBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@40 in `ServerNetworkingImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `SERVERBOUND` | `Lnet/minecraft/network/protocol/PacketFlow;` | exact | getstatic@4 in `PassthroughPacket.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (4 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SERVERBOUND : Lnet/minecraft/network/protocol/PacketFlow;
public static final CLIENTBOUND : Lnet/minecraft/network/protocol/PacketFlow;
private final id : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/network/protocol/PacketFlow;
public static values()[Lnet/minecraft/network/protocol/PacketFlow;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/network/protocol/PacketFlow;
private <init>(Ljava/lang/String;ILjava/lang/String;)V
public getOpposite()Lnet/minecraft/network/protocol/PacketFlow;
public id()Ljava/lang/String;
private static synthetic $values()[Lnet/minecraft/network/protocol/PacketFlow;
static <clinit>()V
```
