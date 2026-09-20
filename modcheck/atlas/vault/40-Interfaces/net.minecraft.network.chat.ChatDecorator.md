---
type: "interface"
fqcn: "net.minecraft.network.chat.ChatDecorator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.ChatDecorator

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `decorate` | `(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/network/chat/` | exact | invokeinterface@31 in `ServerMessageDecoratorEvent.lambda$static$1` | unknown | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| calls | `decorate` | `(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/network/chat/` | exact | invokeinterface@11 in `MinecraftServerMixin.lambda$onGetChatDecorator$0` | unknown | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| calls | `getClass` | `()Ljava/lang/Class;` | inherited_exact | invokeinterface@1 in `ServerMessageDecoratorEvent.handle` | unknown | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |

## Declared members (1 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final PLAIN : Lnet/minecraft/network/chat/ChatDecorator;
public abstract decorate(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$static$0(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
static <clinit>()V
```
