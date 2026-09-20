---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `START_DESTROY_BLOCKLnet/minecraft/network/protocol/game/ServerboundPlayerAction` | `` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `START_DESTROY_BLOCKLnet/minecraft/network/protocol/game/ServerboundPlayerAction` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action extends java.lang.Enum<net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action> {
    public static final net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action START_DESTROY_BLOCK;
    public static final net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action CHANGE_DESTROY_DIRECTION;
    public static final net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action ABORT_DESTROY_BLOCK;
    public static final net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action STOP_DESTROY_BLOCK;
    public static final net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action DROP_ALL_ITEMS;
    public static final net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action DROP_ITEM;
    public static final net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action RELEASE_USE_ITEM;
    public static final net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action SWAP_ITEM_WITH_OFFHAND;
    public static final net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action STAB;
    private static final net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action[] $VALUES;
    public static net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action[] values();
    public static net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action valueOf(java.lang.String);
    private net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action();
    private static net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action[] $values();
    static {};
}
```
