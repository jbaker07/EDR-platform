---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.CommonPacketTypes"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.CommonPacketTypes

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `CLIENTBOUND_CUSTOM_PAYLOADLnet/minecraft/network/protocol/PacketType;` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `SERVERBOUND_CUSTOM_PAYLOADLnet/minecraft/network/protocol/PacketType;` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (24, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.protocol.common.CommonPacketTypes {
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundClearDialogPacket> CLIENTBOUND_CLEAR_DIALOG;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundCustomPayloadPacket> CLIENTBOUND_CUSTOM_PAYLOAD;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundCustomReportDetailsPacket> CLIENTBOUND_CUSTOM_REPORT_DETAILS;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundDisconnectPacket> CLIENTBOUND_DISCONNECT;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundKeepAlivePacket> CLIENTBOUND_KEEP_ALIVE;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundPingPacket> CLIENTBOUND_PING;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundResourcePackPopPacket> CLIENTBOUND_RESOURCE_PACK_POP;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundResourcePackPushPacket> CLIENTBOUND_RESOURCE_PACK_PUSH;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundPostEffectsPacket> CLIENTBOUND_POST_EFFECTS;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundServerLinksPacket> CLIENTBOUND_SERVER_LINKS;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundShowDialogPacket> CLIENTBOUND_SHOW_DIALOG;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundStoreCookiePacket> CLIENTBOUND_STORE_COOKIE;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundTransferPacket> CLIENTBOUND_TRANSFER;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ClientboundUpdateTagsPacket> CLIENTBOUND_UPDATE_TAGS;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ServerboundClientInformationPacket> SERVERBOUND_CLIENT_INFORMATION;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket> SERVERBOUND_CUSTOM_PAYLOAD;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ServerboundKeepAlivePacket> SERVERBOUND_KEEP_ALIVE;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ServerboundPongPacket> SERVERBOUND_PONG;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ServerboundResourcePackPacket> SERVERBOUND_RESOURCE_PACK;
    public static final net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ServerboundCustomClickActionPacket> SERVERBOUND_CUSTOM_CLICK_ACTION;
    public net.minecraft.network.protocol.common.CommonPacketTypes();
    private static <T extends net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.common.ClientCommonPacketListener>> net.minecraft.network.protocol.PacketType<T> createClientbound(java.lang.String);
    private static <T extends net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.common.ServerCommonPacketListener>> net.minecraft.network.protocol.PacketType<T> createServerbound(java.lang.String);
    static {};
}
```
