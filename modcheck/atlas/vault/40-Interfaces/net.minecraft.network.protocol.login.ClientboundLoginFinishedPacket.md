---
type: "interface"
fqcn: "net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/network/protocol/Packet`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `gameProfile` | `()Lcom/mojang/authlib/GameProfile;` | exact | invokevirtual@11 in `ClientHandshakePacketListenerImplMixin.setGameProfileContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (3 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final gameProfile : Lcom/mojang/authlib/GameProfile;
private final sessionId : Ljava/util/UUID;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lcom/mojang/authlib/GameProfile;Ljava/util/UUID;)V
public type()Lnet/minecraft/network/protocol/PacketType;
public handle(Lnet/minecraft/network/protocol/login/ClientLoginPacketListener;)V
public isTerminal()Z
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public gameProfile()Lcom/mojang/authlib/GameProfile;
public sessionId()Ljava/util/UUID;
public synthetic handle(Lnet/minecraft/network/PacketListener;)V
static <clinit>()V
```
