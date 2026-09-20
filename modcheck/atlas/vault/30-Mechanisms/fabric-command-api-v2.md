---
type: "mechanism"
module: "fabric-command-api-v2"
version: "3.1.2+fcdff87f5d"
sha256: "71e0ce2931b3467422b17fd49b181698a7ed79ca8cb92d112fc50ad2fced105e"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-command-api-v2

**Version** `3.1.2+fcdff87f5d` -- **artifact sha256** `71e0ce2931b3467422b17fd49b181698a7ed79ca8cb92d112fc50ad2fced105e`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "minecraft": ">1.19-alpha.22.11.a"}`
- entrypoints: `null`
- mixin configs: `["fabric-command-api-v2.mixins.json", {"config": "fabric-command-api-v2.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-command-api-v2.classtweaker`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback.EVENT|ClientCommandRegistrationCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback.EVENT|CommandRegistrationCallback.EVENT]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `<init>` | injects_into `@Inject at RETURN` | client | `ClientPacketListenerMixin.init` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `handleCommands` | injects_into `@Inject at RETURN` | client | `ClientPacketListenerMixin.onOnCommandTree` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `handleCommands` | injects_into `@Inject at INVOKE Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V` | client | `ClientPacketListenerMixin.setLastReceivedCommandsPacket` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `handleLogin` | injects_into `@Inject at RETURN` | client | `ClientPacketListenerMixin.onGameJoin` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `sendCommand` | injects_into `@Inject at HEAD` | client | `ClientPacketListenerMixin.onSendCommand` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `sendUnattendedCommand` | injects_into `@Inject at HEAD` | client | `ClientPacketListenerMixin.onSendCommand` |
| [[40-Interfaces/net.minecraft.commands.Commands|Commands]] | `<init>` | injects_into `@Inject at INVOKE Lcom/mojang/brigadier/CommandDispatcher;setConsumer(Lcom/mojang/brigadier/ResultConsumer;)V` | both | `CommandsMixin.fabric_addCommands` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback|ClientCommandRegistrationCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.command.v2.ClientCommands|ClientCommands]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.command.v2.FabricClientCommandSource|FabricClientCommandSource]] (interface, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.command.v2.ArgumentTypeRegistry|ArgumentTypeRegistry]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback|CommandRegistrationCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.command.v2.EntitySelectorOptionRegistry|EntitySelectorOptionRegistry]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.command.v2.FabricEntitySelectorParser|FabricEntitySelectorParser]] (interface, 2 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
