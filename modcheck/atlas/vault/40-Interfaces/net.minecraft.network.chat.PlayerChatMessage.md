---
type: "interface"
fqcn: "net.minecraft.network.chat.PlayerChatMessage"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.PlayerChatMessage

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `filterMask` | `()Lnet/minecraft/network/chat/FilterMask;` | exact | invokevirtual@1 in `ChatListenerMixin.fabric_onFilteredSignedChatMessage` | unknown | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| calls | `signedContent` | `()Ljava/lang/String;` | exact | invokevirtual@5 in `ChatListenerMixin.fabric_onFilteredSignedChatMessage` | unknown | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |

## Declared members (9 fields, 38 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final link : Lnet/minecraft/network/chat/SignedMessageLink;
private final signature : Lnet/minecraft/network/chat/MessageSignature;
private final signedBody : Lnet/minecraft/network/chat/SignedMessageBody;
private final unsignedContent : Lnet/minecraft/network/chat/Component;
private final filterMask : Lnet/minecraft/network/chat/FilterMask;
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
private static final SYSTEM_SENDER : Ljava/util/UUID;
public static final MESSAGE_EXPIRES_AFTER_SERVER : Ljava/time/Duration;
public static final MESSAGE_EXPIRES_AFTER_CLIENT : Ljava/time/Duration;
public <init>(Lnet/minecraft/network/chat/SignedMessageLink;Lnet/minecraft/network/chat/MessageSignature;Lnet/minecraft/network/chat/SignedMessageBody;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/FilterMask;)V
public static system(Ljava/lang/String;)Lnet/minecraft/network/chat/PlayerChatMessage;
public static unsigned(Ljava/util/UUID;Ljava/lang/String;)Lnet/minecraft/network/chat/PlayerChatMessage;
public withUnsignedContent(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/PlayerChatMessage;
public removeUnsignedContent()Lnet/minecraft/network/chat/PlayerChatMessage;
public filter(Lnet/minecraft/network/chat/FilterMask;)Lnet/minecraft/network/chat/PlayerChatMessage;
public filter(Z)Lnet/minecraft/network/chat/PlayerChatMessage;
public removeSignature()Lnet/minecraft/network/chat/PlayerChatMessage;
public static updateSignature(Lnet/minecraft/util/SignatureUpdater$Output;Lnet/minecraft/network/chat/SignedMessageLink;Lnet/minecraft/network/chat/SignedMessageBody;)V
public verify(Lnet/minecraft/util/SignatureValidator;)Z
public signedContent()Ljava/lang/String;
public decoratedContent()Lnet/minecraft/network/chat/Component;
public timeStamp()Ljava/time/Instant;
public salt()J
public hasExpiredServer(Ljava/time/Instant;)Z
public hasExpiredClient(Ljava/time/Instant;)Z
public sender()Ljava/util/UUID;
public isSystem()Z
public hasSignature()Z
public hasSignatureFrom(Ljava/util/UUID;)Z
public isFullyFiltered()Z
public static describeSigned(Lnet/minecraft/network/chat/PlayerChatMessage;)Ljava/lang/String;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public link()Lnet/minecraft/network/chat/SignedMessageLink;
public signature()Lnet/minecraft/network/chat/MessageSignature;
public signedBody()Lnet/minecraft/network/chat/SignedMessageBody;
public unsignedContent()Lnet/minecraft/network/chat/Component;
public filterMask()Lnet/minecraft/network/chat/FilterMask;
private static synthetic lambda$describeSigned$0(Lnet/minecraft/network/chat/MessageSignature;)Ljava/lang/String;
private synthetic lambda$decoratedContent$0()Lnet/minecraft/network/chat/Component;
private synthetic lambda$verify$0(Lnet/minecraft/util/SignatureUpdater$Output;)V
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$3(Lnet/minecraft/network/chat/SignedMessageLink;Ljava/util/Optional;Lnet/minecraft/network/chat/SignedMessageBody;Ljava/util/Optional;Lnet/minecraft/network/chat/FilterMask;)Lnet/minecraft/network/chat/PlayerChatMessage;
private static synthetic lambda$static$2(Lnet/minecraft/network/chat/PlayerChatMessage;)Ljava/util/Optional;
private static synthetic lambda$static$1(Lnet/minecraft/network/chat/PlayerChatMessage;)Ljava/util/Optional;
static <clinit>()V
```
