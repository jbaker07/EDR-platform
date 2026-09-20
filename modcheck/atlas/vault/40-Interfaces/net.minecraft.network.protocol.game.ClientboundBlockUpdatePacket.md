---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/network/protocol/Packet`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)` | exact | invokespecial@59 in `InteractionEventsRouter.lambda$onInitialize$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)` | exact | invokespecial@61 in `ServerPlayerGameModeMixin.startBlockBreak` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (3 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private final pos : Lnet/minecraft/core/BlockPos;
private final blockState : Lnet/minecraft/world/level/block/state/BlockState;
public <init>(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public <init>(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)V
public type()Lnet/minecraft/network/protocol/PacketType;
public handle(Lnet/minecraft/network/protocol/game/ClientGamePacketListener;)V
public getBlockState()Lnet/minecraft/world/level/block/state/BlockState;
public getPos()Lnet/minecraft/core/BlockPos;
public synthetic handle(Lnet/minecraft/network/PacketListener;)V
static <clinit>()V
```
