---
type: "interface"
fqcn: "net.minecraft.network.chat.PlayerChatMessage"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.PlayerChatMessage

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `filterMask()Lnet/minecraft/network/chat/FilterMask;` | `` | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| calls | `signedContent()Ljava/lang/String;` | `` | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |

## Declared members (47, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.chat.PlayerChatMessage extends java.lang.Record {
    private final net.minecraft.network.chat.SignedMessageLink link;
    private final net.minecraft.network.chat.MessageSignature signature;
    private final net.minecraft.network.chat.SignedMessageBody signedBody;
    private final net.minecraft.network.chat.Component unsignedContent;
    private final net.minecraft.network.chat.FilterMask filterMask;
    public static final com.mojang.serialization.MapCodec<net.minecraft.network.chat.PlayerChatMessage> MAP_CODEC;
    private static final java.util.UUID SYSTEM_SENDER;
    public static final java.time.Duration MESSAGE_EXPIRES_AFTER_SERVER;
    public static final java.time.Duration MESSAGE_EXPIRES_AFTER_CLIENT;
    public net.minecraft.network.chat.PlayerChatMessage(net.minecraft.network.chat.SignedMessageLink, net.minecraft.network.chat.MessageSignature, net.minecraft.network.chat.SignedMessageBody, net.minecraft.network.chat.Component, net.minecraft.network.chat.FilterMask);
    public static net.minecraft.network.chat.PlayerChatMessage system(java.lang.String);
    public static net.minecraft.network.chat.PlayerChatMessage unsigned(java.util.UUID, java.lang.String);
    public net.minecraft.network.chat.PlayerChatMessage withUnsignedContent(net.minecraft.network.chat.Component);
    public net.minecraft.network.chat.PlayerChatMessage removeUnsignedContent();
    public net.minecraft.network.chat.PlayerChatMessage filter(net.minecraft.network.chat.FilterMask);
    public net.minecraft.network.chat.PlayerChatMessage filter(boolean);
    public net.minecraft.network.chat.PlayerChatMessage removeSignature();
    public static void updateSignature(net.minecraft.util.SignatureUpdater$Output, net.minecraft.network.chat.SignedMessageLink, net.minecraft.network.chat.SignedMessageBody) throws java.security.SignatureException;
    public boolean verify(net.minecraft.util.SignatureValidator);
    public java.lang.String signedContent();
    public net.minecraft.network.chat.Component decoratedContent();
    public java.time.Instant timeStamp();
    public long salt();
    public boolean hasExpiredServer(java.time.Instant);
    public boolean hasExpiredClient(java.time.Instant);
    public java.util.UUID sender();
    public boolean isSystem();
    public boolean hasSignature();
    public boolean hasSignatureFrom(java.util.UUID);
    public boolean isFullyFiltered();
    public static java.lang.String describeSigned(net.minecraft.network.chat.PlayerChatMessage);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.network.chat.SignedMessageLink link();
    public net.minecraft.network.chat.MessageSignature signature();
    public net.minecraft.network.chat.SignedMessageBody signedBody();
    public net.minecraft.network.chat.Component unsignedContent();
    public net.minecraft.network.chat.FilterMask filterMask();
    private static java.lang.String lambda$describeSigned$0(net.minecraft.network.chat.MessageSignature);
    private net.minecraft.network.chat.Component lambda$decoratedContent$0();
    private void lambda$verify$0(net.minecraft.util.SignatureUpdater$Output) throws java.security.SignatureException;
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static net.minecraft.network.chat.PlayerChatMessage lambda$static$3(net.minecraft.network.chat.SignedMessageLink, java.util.Optional, net.minecraft.network.chat.SignedMessageBody, java.util.Optional, net.minecraft.network.chat.FilterMask);
    private static java.util.Optional lambda$static$2(net.minecraft.network.chat.PlayerChatMessage);
    private static java.util.Optional lambda$static$1(net.minecraft.network.chat.PlayerChatMessage);
    static {};
}
```
