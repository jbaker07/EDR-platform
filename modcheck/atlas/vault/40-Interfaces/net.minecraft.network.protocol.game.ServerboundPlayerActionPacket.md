---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ServerboundPlayerActionPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ServerboundPlayerActionPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/network/protocol/Packet`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Ac` | exact | invokespecial@10 in `MultiPlayerGameModeMixin.lambda$fabric_fireAttackBlockCallback$0` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (5 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private final pos : Lnet/minecraft/core/BlockPos;
private final direction : Lnet/minecraft/core/Direction;
private final action : Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
private final sequence : I
public <init>(Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;I)V
public <init>(Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)V
private <init>(Lnet/minecraft/network/FriendlyByteBuf;)V
private write(Lnet/minecraft/network/FriendlyByteBuf;)V
public type()Lnet/minecraft/network/protocol/PacketType;
public handle(Lnet/minecraft/network/protocol/game/ServerGamePacketListener;)V
public getPos()Lnet/minecraft/core/BlockPos;
public getDirection()Lnet/minecraft/core/Direction;
public getAction()Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
public getSequence()I
public synthetic handle(Lnet/minecraft/network/PacketListener;)V
static <clinit>()V
```
