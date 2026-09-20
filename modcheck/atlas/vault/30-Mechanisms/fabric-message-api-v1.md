---
type: "mechanism"
module: "fabric-message-api-v1"
version: "7.0.10+3434d6d95d"
sha256: "967f819dc19e4e50e3adb145dbcd23fdab1329cd772d781fd38addc246eea8e6"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-message-api-v1

**Version** `7.0.10+3434d6d95d` -- **artifact sha256** `967f819dc19e4e50e3adb145dbcd23fdab1329cd772d781fd38addc246eea8e6`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-message-api-v1.mixins.json", {"config": "fabric-message-api-v1.client.mixins.json", "environment": "client"}]`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents.ALLOW_CHAT|ClientReceiveMessageEvents.ALLOW_CHAT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents.ALLOW_GAME|ClientReceiveMessageEvents.ALLOW_GAME]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents.CHAT|ClientReceiveMessageEvents.CHAT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents.CHAT_CANCELED|ClientReceiveMessageEvents.CHAT_CANCELED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents.GAME|ClientReceiveMessageEvents.GAME]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents.GAME_CANCELED|ClientReceiveMessageEvents.GAME_CANCELED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents.MODIFY_GAME|ClientReceiveMessageEvents.MODIFY_GAME]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents.ALLOW_CHAT|ClientSendMessageEvents.ALLOW_CHAT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents.ALLOW_COMMAND|ClientSendMessageEvents.ALLOW_COMMAND]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents.CHAT|ClientSendMessageEvents.CHAT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents.CHAT_CANCELED|ClientSendMessageEvents.CHAT_CANCELED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents.COMMAND|ClientSendMessageEvents.COMMAND]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents.COMMAND_CANCELED|ClientSendMessageEvents.COMMAND_CANCELED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents.MODIFY_CHAT|ClientSendMessageEvents.MODIFY_CHAT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents.MODIFY_COMMAND|ClientSendMessageEvents.MODIFY_COMMAND]]
- [[50-Interactions/events/net.fabricmc.fabric.api.message.v1.ServerMessageDecoratorEvent.EVENT|ServerMessageDecoratorEvent.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.message.v1.ServerMessageEvents.ALLOW_CHAT_MESSAGE|ServerMessageEvents.ALLOW_CHAT_MESSAGE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.message.v1.ServerMessageEvents.ALLOW_COMMAND_MESSAGE|ServerMessageEvents.ALLOW_COMMAND_MESSAGE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.message.v1.ServerMessageEvents.ALLOW_GAME_MESSAGE|ServerMessageEvents.ALLOW_GAME_MESSAGE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.message.v1.ServerMessageEvents.CHAT_MESSAGE|ServerMessageEvents.CHAT_MESSAGE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.message.v1.ServerMessageEvents.COMMAND_MESSAGE|ServerMessageEvents.COMMAND_MESSAGE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.message.v1.ServerMessageEvents.GAME_MESSAGE|ServerMessageEvents.GAME_MESSAGE]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `sendChat` | injects_into `@Inject at HEAD` | client | `ClientPacketListenerMixin.fabric_allowSendChatMessage` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `sendCommand` | injects_into `@Inject at HEAD` | client | `ClientPacketListenerMixin.fabric_allowSendCommandMessage` |
| [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]] | `handleOverlay` | injects_into `@Inject at HEAD` | client | `ChatListenerMixin.fabric_allowOverlayMessage` |
| [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]] | `handleSystemMessage` | injects_into `@Inject at HEAD` | client | `ChatListenerMixin.fabric_allowSystemMessage` |
| [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]] | `lambda$handleDisguisedChatMessage$0` | injects_into `@Inject at HEAD` | client | `ChatListenerMixin.fabric_onProfilelessChatMessage` |
| [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]] | `showMessageToPlayer` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/Hud;getChat()Lnet/minecraft/client/gui/components/ChatComponent;` | client | `ChatListenerMixin.fabric_onSignedChatMessage` |
| [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]] | `showMessageToPlayer` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/Hud;getChat()Lnet/minecraft/client/gui/components/ChatComponent;` | client | `ChatListenerMixin.fabric_onFilteredSignedChatMessage` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `getChatDecorator` | injects_into `@Inject at RETURN` | both | `MinecraftServerMixin.onGetChatDecorator` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]] | `broadcastChatMessage(Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/commands/CommandSourceStack;Lnet/minecraft/network/chat/ChatType$Bound;)V` | injects_into `@Inject at HEAD` | both | `PlayerListMixin.onSendCommandMessage` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]] | `broadcastChatMessage(Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/network/chat/ChatType$Bound;)V` | injects_into `@Inject at HEAD` | both | `PlayerListMixin.onSendChatMessage` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]] | `broadcastSystemMessage(Lnet/minecraft/network/chat/Component;Ljava/util/function/Function;Z)V` | injects_into `@Inject at HEAD` | both | `PlayerListMixin.onSendGameMessage` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents|ClientReceiveMessageEvents]] (class, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents|ClientSendMessageEvents]] (class, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.message.v1.ServerMessageDecoratorEvent|ServerMessageDecoratorEvent]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.message.v1.ServerMessageEvents|ServerMessageEvents]] (class, 7 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
