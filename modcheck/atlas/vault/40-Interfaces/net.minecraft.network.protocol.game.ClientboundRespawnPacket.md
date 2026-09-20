---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ClientboundRespawnPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ClientboundRespawnPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/network/protocol/Packet`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `shouldKeep` | `(B)Z` | exact | invokevirtual@6 in `ClientPacketListenerMixin.copyAttachmentsOnClientRespawn` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (6 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final commonPlayerSpawnInfo : Lnet/minecraft/network/protocol/game/CommonPlayerSpawnInfo;
private final dataToKeep : B
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final KEEP_ATTRIBUTE_MODIFIERS : B
public static final KEEP_ENTITY_DATA : B
public static final KEEP_ALL_DATA : B
public <init>(Lnet/minecraft/network/protocol/game/CommonPlayerSpawnInfo;B)V
public type()Lnet/minecraft/network/protocol/PacketType;
public handle(Lnet/minecraft/network/protocol/game/ClientGamePacketListener;)V
public shouldKeep(B)Z
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public commonPlayerSpawnInfo()Lnet/minecraft/network/protocol/game/CommonPlayerSpawnInfo;
public dataToKeep()B
public synthetic handle(Lnet/minecraft/network/PacketListener;)V
static <clinit>()V
```
