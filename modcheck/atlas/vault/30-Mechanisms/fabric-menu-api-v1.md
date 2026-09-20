---
type: "mechanism"
module: "fabric-menu-api-v1"
version: "2.0.27+ed66f0a35d"
sha256: "2d406ae5995d2249b2f76faf3046e68683449a96d0214aabe7f8b16b35ec50e6"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-menu-api-v1

**Version** `2.0.27+ed66f0a35d` -- **artifact sha256** `2d406ae5995d2249b2f76faf3046e68683449a96d0214aabe7f8b16b35ec50e6`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-networking-api-v1": "*"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.menu.Networking"], "client": ["net.fabricmc.fabric.impl.menu.client.ClientNetworking"]}`
- mixin configs: `["fabric-menu-api-v1.mixins.json"]`
- access widener: `fabric-menu-api-v1.classtweaker`

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]] | `openMenu(Lnet/minecraft/world/MenuProvider;)Ljava/util/OptionalInt;` | wraps `@Redirect at INVOKE Lnet/minecraft/server/level/ServerPlayer;closeContainer()V` | both | `ServerPlayerMixin.fabric_closeContainerScreenIfAllowed` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]] | `openMenu(Lnet/minecraft/world/MenuProvider;)Ljava/util/OptionalInt;` | injects_into `@Inject at INVOKE Lnet/minecraft/server/network/ServerGamePacketListenerImpl;send(Lnet/minecraft/network/protocol/Packet;)V` | both | `ServerPlayerMixin.fabric_storeOpenedMenu` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]] | `openMenu(Lnet/minecraft/world/MenuProvider;)Ljava/util/OptionalInt;` | wraps `@Redirect at INVOKE Lnet/minecraft/server/network/ServerGamePacketListenerImpl;send(Lnet/minecraft/network/protocol/Packet;)V` | both | `ServerPlayerMixin.fabric_replaceVanillaScreenPacket` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.menu.v1.ExtendedMenuProvider|ExtendedMenuProvider]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.menu.v1.ExtendedMenuType|ExtendedMenuType]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.menu.v1.FabricMenuProvider|FabricMenuProvider]] (interface, 1 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
