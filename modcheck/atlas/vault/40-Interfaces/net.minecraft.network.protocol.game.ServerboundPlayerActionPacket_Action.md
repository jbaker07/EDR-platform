---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `START_DESTROY_BLOCK` | `Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Act` | exact | getstatic@1 in `ServerPlayerGameModeMixin.startBlockBreak` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `START_DESTROY_BLOCK` | `Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Act` | exact | getstatic@4 in `MultiPlayerGameModeMixin.lambda$fabric_fireAttackBlockCallback$0` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (10 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final START_DESTROY_BLOCK : Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
public static final CHANGE_DESTROY_DIRECTION : Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
public static final ABORT_DESTROY_BLOCK : Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
public static final STOP_DESTROY_BLOCK : Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
public static final DROP_ALL_ITEMS : Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
public static final DROP_ITEM : Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
public static final RELEASE_USE_ITEM : Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
public static final SWAP_ITEM_WITH_OFFHAND : Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
public static final STAB : Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
private static final synthetic $VALUES : [Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
public static values()[Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
private <init>(Ljava/lang/String;I)V
private static synthetic $values()[Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;
static <clinit>()V
```
