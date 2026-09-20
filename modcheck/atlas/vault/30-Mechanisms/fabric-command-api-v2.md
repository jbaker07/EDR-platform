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
- mixin classes: 8 found by annotation, 8 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback.EVENT|ClientCommandRegistrationCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback.EVENT|CommandRegistrationCallback.EVENT]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`<init>` | `(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lnet/minecraft/client/multiplayer/CommonListenerCookie;)V` | name_only | @Inject | RETURN | client | 1000 (default) | `ClientPacketListenerMixin.init` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleCommands` | `(Lnet/minecraft/network/protocol/game/ClientboundCommandsPacket;)V` | name_only | @Inject | RETURN | client | 1000 (default) | `ClientPacketListenerMixin.onOnCommandTree` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleCommands` | `(Lnet/minecraft/network/protocol/game/ClientboundCommandsPacket;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V` (exact) | client | 1000 (default) | `ClientPacketListenerMixin.setLastReceivedCommandsPacket` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleLogin` | `(Lnet/minecraft/network/protocol/game/ClientboundLoginPacket;)V` | name_only | @Inject | RETURN | client | 1000 (default) | `ClientPacketListenerMixin.onGameJoin` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`sendCommand` | `(Ljava/lang/String;)V` | name_only | @Inject | HEAD | client | 1000 (default) | `ClientPacketListenerMixin.onSendCommand` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`sendUnattendedCommand` | `(Ljava/lang/String;Lnet/minecraft/client/gui/screens/Screen;)V` | name_only | @Inject | HEAD | client | 1000 (default) | `ClientPacketListenerMixin.onSendCommand` |
| [[40-Interfaces/net.minecraft.commands.Commands|Commands]].`<init>` | `(Lnet/minecraft/commands/Commands$CommandSelection;Lnet/minecraft/commands/CommandBuildContext;)V` | name_only | @Inject | INVOKE `Lcom/mojang/brigadier/CommandDispatcher;setConsumer(Lcom/mojang/brigadier/ResultConsumer;)V` (exact) | both | 1000 (default) | `CommandsMixin.fabric_addCommands` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback|ClientCommandRegistrationCallback]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.command.v2.ClientCommands|ClientCommands]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.command.v2.FabricClientCommandSource|FabricClientCommandSource]] (interface, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.command.v2.ArgumentTypeRegistry|ArgumentTypeRegistry]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback|CommandRegistrationCallback]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.command.v2.EntitySelectorOptionRegistry|EntitySelectorOptionRegistry]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.command.v2.FabricEntitySelectorParser|FabricEntitySelectorParser]] (interface, 2 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
