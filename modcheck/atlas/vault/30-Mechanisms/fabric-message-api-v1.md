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
- mixin classes: 4 found by annotation, 4 declared in configs; extraction failures: 0

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

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`sendChat` | `(Ljava/lang/String;)V` | name_only | @Inject | HEAD | client | 800 | `ClientPacketListenerMixin.fabric_allowSendChatMessage` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`sendCommand` | `(Ljava/lang/String;)V` | name_only | @Inject | HEAD | client | 800 | `ClientPacketListenerMixin.fabric_allowSendCommandMessage` |
| [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]].`handleOverlay` | `(Lnet/minecraft/network/chat/Component;)V` | name_only | @Inject | HEAD | client | 1000 (default) | `ChatListenerMixin.fabric_allowOverlayMessage` |
| [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]].`handleSystemMessage` | `(Lnet/minecraft/network/chat/Component;Z)V` | name_only | @Inject | HEAD | client | 1000 (default) | `ChatListenerMixin.fabric_allowSystemMessage` |
| [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]].`lambda$handleDisguisedChatMessage$0` | `(Lnet/minecraft/network/chat/ChatType$Bound;Lnet/minecraft/network/chat/Component;Ljava/time/Instant;)Z` | name_only | @Inject | HEAD | client | 1000 (default) | `ChatListenerMixin.fabric_onProfilelessChatMessage` |
| [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]].`showMessageToPlayer` | `(Lnet/minecraft/network/chat/ChatType$Bound;Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/network/chat/Component;Lcom/mojang/authlib/GameProfile;ZLjava/time/Instant;)Z` | name_only | @Inject | INVOKE `Lnet/minecraft/client/gui/Hud;getChat()Lnet/minecraft/client/gui/components/ChatComponent;` (exact) | client | 1000 (default) | `ChatListenerMixin.fabric_onSignedChatMessage` |
| [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]].`showMessageToPlayer` | `(Lnet/minecraft/network/chat/ChatType$Bound;Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/network/chat/Component;Lcom/mojang/authlib/GameProfile;ZLjava/time/Instant;)Z` | name_only | @Inject | INVOKE `Lnet/minecraft/client/gui/Hud;getChat()Lnet/minecraft/client/gui/components/ChatComponent;` (exact) | client | 1000 (default) | `ChatListenerMixin.fabric_onFilteredSignedChatMessage` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`getChatDecorator` | `()Lnet/minecraft/network/chat/ChatDecorator;` | name_only | @Inject | RETURN | both | 1000 (default) | `MinecraftServerMixin.onGetChatDecorator` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`broadcastChatMessage` | `(Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/network/chat/ChatType$Bound;)V` | exact | @Inject | HEAD | both | 1000 (default) | `PlayerListMixin.onSendChatMessage` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`broadcastChatMessage` | `(Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/commands/CommandSourceStack;Lnet/minecraft/network/chat/ChatType$Bound;)V` | exact | @Inject | HEAD | both | 1000 (default) | `PlayerListMixin.onSendCommandMessage` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`broadcastSystemMessage` | `(Lnet/minecraft/network/chat/Component;Ljava/util/function/Function;Z)V` | exact | @Inject | HEAD | both | 1000 (default) | `PlayerListMixin.onSendGameMessage` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents|ClientReceiveMessageEvents]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents|ClientSendMessageEvents]] (class, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.message.v1.ServerMessageDecoratorEvent|ServerMessageDecoratorEvent]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.message.v1.ServerMessageEvents|ServerMessageEvents]] (class, 6 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
