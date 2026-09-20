---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ServerboundUseItemOnPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ServerboundUseItemOnPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/network/protocol/Packet`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/BlockH` | exact | invokespecial@7 in `MultiPlayerGameModeMixin.lambda$interactBlock$0` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (4 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final hand : Lnet/minecraft/world/InteractionHand;
private final hitResult : Lnet/minecraft/world/phys/BlockHitResult;
private final sequence : I
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/BlockHitResult;I)V
public type()Lnet/minecraft/network/protocol/PacketType;
public handle(Lnet/minecraft/network/protocol/game/ServerGamePacketListener;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public hand()Lnet/minecraft/world/InteractionHand;
public hitResult()Lnet/minecraft/world/phys/BlockHitResult;
public sequence()I
public synthetic handle(Lnet/minecraft/network/PacketListener;)V
static <clinit>()V
```
