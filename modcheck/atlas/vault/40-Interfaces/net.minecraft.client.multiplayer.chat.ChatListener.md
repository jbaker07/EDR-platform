---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.chat.ChatListener"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.chat.ChatListener

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `handleOverlay` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `handleSystemMessage` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `lambda$handleDisguisedChatMessage$0` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `showMessageToPlayer` | `@Inject at INVOKE Lnet/minecraft/client/gui/Hud;getChat()Lnet/minecraft/client/g` | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `showMessageToPlayer` | `@Inject at INVOKE Lnet/minecraft/client/gui/Hud;getChat()Lnet/minecraft/client/g` | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |

## Declared members (30, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.multiplayer.chat.ChatListener {
    private static final net.minecraft.network.chat.Component CHAT_VALIDATION_ERROR;
    private final net.minecraft.client.Minecraft minecraft;
    private final java.util.Deque<net.minecraft.client.multiplayer.chat.ChatListener$Message> delayedMessageQueue;
    private long messageDelay;
    private long previousMessageTime;
    public net.minecraft.client.multiplayer.chat.ChatListener(net.minecraft.client.Minecraft);
    public void tick();
    public void setMessageDelay(double);
    public void acceptNextDelayedMessage();
    public long queueSize();
    public void flushQueue();
    public boolean removeFromDelayedMessageQueue(net.minecraft.network.chat.MessageSignature);
    private void handleMessage(net.minecraft.network.chat.MessageSignature, java.util.function.BooleanSupplier);
    public void handlePlayerChatMessage(net.minecraft.network.chat.PlayerChatMessage, com.mojang.authlib.GameProfile, net.minecraft.network.chat.ChatType$Bound);
    public void handleChatMessageError(java.util.UUID, net.minecraft.network.chat.MessageSignature, net.minecraft.network.chat.ChatType$Bound);
    public void handleDisguisedChatMessage(net.minecraft.network.chat.Component, net.minecraft.network.chat.ChatType$Bound);
    private boolean showMessageToPlayer(net.minecraft.network.chat.ChatType$Bound, net.minecraft.network.chat.PlayerChatMessage, net.minecraft.network.chat.Component, com.mojang.authlib.GameProfile, boolean, java.time.Instant);
    private void narrateChatMessage(net.minecraft.network.chat.ChatType$Bound, net.minecraft.network.chat.Component);
    private net.minecraft.client.multiplayer.chat.ChatTrustLevel evaluateTrustLevel(net.minecraft.network.chat.PlayerChatMessage, net.minecraft.network.chat.Component, java.time.Instant);
    private void logPlayerMessage(net.minecraft.network.chat.PlayerChatMessage, com.mojang.authlib.GameProfile, net.minecraft.client.multiplayer.chat.ChatTrustLevel);
    private void logSystemMessage(net.minecraft.network.chat.Component, java.time.Instant);
    public void handleSystemMessage(net.minecraft.network.chat.Component, boolean);
    public void handleOverlay(net.minecraft.network.chat.Component);
    private java.util.UUID guessChatUUID(net.minecraft.network.chat.Component);
    private boolean isSenderLocalPlayer(java.util.UUID);
    private boolean lambda$handleDisguisedChatMessage$0(net.minecraft.network.chat.ChatType$Bound, net.minecraft.network.chat.Component, java.time.Instant);
    private boolean lambda$handleChatMessageError$0(net.minecraft.network.chat.MessageSignature, java.util.UUID, net.minecraft.network.chat.ChatType$Bound);
    private boolean lambda$handlePlayerChatMessage$0(net.minecraft.network.chat.ChatType$Bound, net.minecraft.network.chat.PlayerChatMessage, net.minecraft.network.chat.Component, com.mojang.authlib.GameProfile, boolean, java.time.Instant);
    private static boolean lambda$removeFromDelayedMessageQueue$0(net.minecraft.network.chat.MessageSignature, net.minecraft.client.multiplayer.chat.ChatListener$Message);
    static {};
}
```
