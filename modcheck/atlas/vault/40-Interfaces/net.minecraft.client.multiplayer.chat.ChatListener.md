---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.chat.ChatListener"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.chat.ChatListener

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `handleOverlay` | `(Lnet/minecraft/network/chat/Component;)V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `handleSystemMessage` | `(Lnet/minecraft/network/chat/Component;Z)V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `lambda$handleDisguisedChatMessage$0` | `(Lnet/minecraft/network/chat/ChatType$Bound;Lnet/minecraft/network/cha` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `showMessageToPlayer` | `(Lnet/minecraft/network/chat/ChatType$Bound;Lnet/minecraft/network/cha` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `showMessageToPlayer` | `(Lnet/minecraft/network/chat/ChatType$Bound;Lnet/minecraft/network/cha` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |

## Declared members (5 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final CHAT_VALIDATION_ERROR : Lnet/minecraft/network/chat/Component;
private final minecraft : Lnet/minecraft/client/Minecraft;
private final delayedMessageQueue : Ljava/util/Deque;
private messageDelay : J
private previousMessageTime : J
public <init>(Lnet/minecraft/client/Minecraft;)V
public tick()V
public setMessageDelay(D)V
public acceptNextDelayedMessage()V
public queueSize()J
public flushQueue()V
public removeFromDelayedMessageQueue(Lnet/minecraft/network/chat/MessageSignature;)Z
private handleMessage(Lnet/minecraft/network/chat/MessageSignature;Ljava/util/function/BooleanSupplier;)V
public handlePlayerChatMessage(Lnet/minecraft/network/chat/PlayerChatMessage;Lcom/mojang/authlib/GameProfile;Lnet/minecraft/network/chat/ChatType$Bound;)V
public handleChatMessageError(Ljava/util/UUID;Lnet/minecraft/network/chat/MessageSignature;Lnet/minecraft/network/chat/ChatType$Bound;)V
public handleDisguisedChatMessage(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/ChatType$Bound;)V
private showMessageToPlayer(Lnet/minecraft/network/chat/ChatType$Bound;Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/network/chat/Component;Lcom/mojang/authlib/GameProfile;ZLjava/time/Instant;)Z
private narrateChatMessage(Lnet/minecraft/network/chat/ChatType$Bound;Lnet/minecraft/network/chat/Component;)V
private evaluateTrustLevel(Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/network/chat/Component;Ljava/time/Instant;)Lnet/minecraft/client/multiplayer/chat/ChatTrustLevel;
private logPlayerMessage(Lnet/minecraft/network/chat/PlayerChatMessage;Lcom/mojang/authlib/GameProfile;Lnet/minecraft/client/multiplayer/chat/ChatTrustLevel;)V
private logSystemMessage(Lnet/minecraft/network/chat/Component;Ljava/time/Instant;)V
public handleSystemMessage(Lnet/minecraft/network/chat/Component;Z)V
public handleOverlay(Lnet/minecraft/network/chat/Component;)V
private guessChatUUID(Lnet/minecraft/network/chat/Component;)Ljava/util/UUID;
private isSenderLocalPlayer(Ljava/util/UUID;)Z
private synthetic lambda$handleDisguisedChatMessage$0(Lnet/minecraft/network/chat/ChatType$Bound;Lnet/minecraft/network/chat/Component;Ljava/time/Instant;)Z
private synthetic lambda$handleChatMessageError$0(Lnet/minecraft/network/chat/MessageSignature;Ljava/util/UUID;Lnet/minecraft/network/chat/ChatType$Bound;)Z
private synthetic lambda$handlePlayerChatMessage$0(Lnet/minecraft/network/chat/ChatType$Bound;Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/network/chat/Component;Lcom/mojang/authlib/GameProfile;ZLjava/time/Instant;)Z
private static synthetic lambda$removeFromDelayedMessageQueue$0(Lnet/minecraft/network/chat/MessageSignature;Lnet/minecraft/client/multiplayer/chat/ChatListener$Message;)Z
static <clinit>()V
```
