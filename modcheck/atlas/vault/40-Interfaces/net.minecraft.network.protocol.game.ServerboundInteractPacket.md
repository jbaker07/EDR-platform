---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ServerboundInteractPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ServerboundInteractPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/network/protocol/Packet`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(ILnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/Vec3;` | exact | invokespecial@98 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `hand` | `()Lnet/minecraft/world/InteractionHand;` | exact | invokevirtual@54 in `ServerGamePacketListenerImplMixin.handleInteract` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@15 in `ServerGamePacketListenerImplMixin.handleInteract` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (5 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final entityId : I
private final hand : Lnet/minecraft/world/InteractionHand;
private final location : Lnet/minecraft/world/phys/Vec3;
private final usingSecondaryAction : Z
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(ILnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/Vec3;Z)V
public type()Lnet/minecraft/network/protocol/PacketType;
public handle(Lnet/minecraft/network/protocol/game/ServerGamePacketListener;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public entityId()I
public hand()Lnet/minecraft/world/InteractionHand;
public location()Lnet/minecraft/world/phys/Vec3;
public usingSecondaryAction()Z
public synthetic handle(Lnet/minecraft/network/PacketListener;)V
static <clinit>()V
```
