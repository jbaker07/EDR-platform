---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.GameProtocols"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.GameProtocols

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `CLIENTBOUND_TEMPLATE` | `Lnet/minecraft/network/protocol/SimpleUnboundProtocol;` | exact | getstatic@0 in `VanillaPacketTypes.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `SERVERBOUND_TEMPLATE` | `Lnet/minecraft/network/protocol/UnboundProtocol;` | exact | getstatic@9 in `VanillaPacketTypes.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (3 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final HAS_INFINITE_MATERIALS : Lnet/minecraft/network/protocol/CodecModifier;
public static final SERVERBOUND_TEMPLATE : Lnet/minecraft/network/protocol/UnboundProtocol;
public static final CLIENTBOUND_TEMPLATE : Lnet/minecraft/network/protocol/SimpleUnboundProtocol;
public <init>()V
private static dispatchCodec(Ljava/util/function/Function;)Lnet/minecraft/network/protocol/CodecModifier;
private static synthetic lambda$static$3(Lnet/minecraft/network/protocol/ProtocolInfoBuilder;)V
private static synthetic lambda$static$1(Lnet/minecraft/network/protocol/ProtocolInfoBuilder;)V
private static synthetic lambda$static$2(Lnet/minecraft/network/protocol/game/GameProtocols$Context;)Lnet/minecraft/network/codec/StreamCodec;
private static synthetic lambda$dispatchCodec$0(Ljava/util/function/Function;Lnet/minecraft/network/codec/StreamCodec;Lnet/minecraft/network/protocol/game/GameProtocols$Context;)Lnet/minecraft/network/codec/StreamCodec;
private static synthetic lambda$static$0(Lnet/minecraft/network/codec/StreamCodec;Lnet/minecraft/network/protocol/game/GameProtocols$Context;)Lnet/minecraft/network/codec/StreamCodec;
static <clinit>()V
```
