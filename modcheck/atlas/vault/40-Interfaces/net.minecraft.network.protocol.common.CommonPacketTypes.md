---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.CommonPacketTypes"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.CommonPacketTypes

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `CLIENTBOUND_CUSTOM_PAYLOAD` | `Lnet/minecraft/network/protocol/PacketType;` | exact | getstatic@4 in `IdDispatchCodecMixin.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `SERVERBOUND_CUSTOM_PAYLOAD` | `Lnet/minecraft/network/protocol/PacketType;` | exact | getstatic@14 in `IdDispatchCodecMixin.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (20 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CLIENTBOUND_CLEAR_DIALOG : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_CUSTOM_PAYLOAD : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_CUSTOM_REPORT_DETAILS : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_DISCONNECT : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_KEEP_ALIVE : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_PING : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_RESOURCE_PACK_POP : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_RESOURCE_PACK_PUSH : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_POST_EFFECTS : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_SERVER_LINKS : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_SHOW_DIALOG : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_STORE_COOKIE : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_TRANSFER : Lnet/minecraft/network/protocol/PacketType;
public static final CLIENTBOUND_UPDATE_TAGS : Lnet/minecraft/network/protocol/PacketType;
public static final SERVERBOUND_CLIENT_INFORMATION : Lnet/minecraft/network/protocol/PacketType;
public static final SERVERBOUND_CUSTOM_PAYLOAD : Lnet/minecraft/network/protocol/PacketType;
public static final SERVERBOUND_KEEP_ALIVE : Lnet/minecraft/network/protocol/PacketType;
public static final SERVERBOUND_PONG : Lnet/minecraft/network/protocol/PacketType;
public static final SERVERBOUND_RESOURCE_PACK : Lnet/minecraft/network/protocol/PacketType;
public static final SERVERBOUND_CUSTOM_CLICK_ACTION : Lnet/minecraft/network/protocol/PacketType;
public <init>()V
private static createClientbound(Ljava/lang/String;)Lnet/minecraft/network/protocol/PacketType;
private static createServerbound(Ljava/lang/String;)Lnet/minecraft/network/protocol/PacketType;
static <clinit>()V
```
