---
type: "interface"
fqcn: "net.minecraft.network.chat.ChatType$Bound"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.ChatType$Bound

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `decorate` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Co` | exact | invokevirtual@22 in `ChatListenerMixin.fabric_onFilteredSignedChatMessage` | unknown | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| calls | `decorate` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Co` | exact | invokevirtual@3 in `ChatListenerMixin.fabric_onProfilelessChatMessage` | unknown | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |

## Declared members (4 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final chatType : Lnet/minecraft/core/Holder;
private final name : Lnet/minecraft/network/chat/Component;
private final targetName : Ljava/util/Optional;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private <init>(Lnet/minecraft/core/Holder;Lnet/minecraft/network/chat/Component;)V
public <init>(Lnet/minecraft/core/Holder;Lnet/minecraft/network/chat/Component;Ljava/util/Optional;)V
public decorate(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
public decorateNarration(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
public withTargetName(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/ChatType$Bound;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public chatType()Lnet/minecraft/core/Holder;
public name()Lnet/minecraft/network/chat/Component;
public targetName()Ljava/util/Optional;
static <clinit>()V
```
