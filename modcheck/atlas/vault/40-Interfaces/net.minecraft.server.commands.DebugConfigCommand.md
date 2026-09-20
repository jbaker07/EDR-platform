---
type: "interface"
fqcn: "net.minecraft.server.commands.DebugConfigCommand"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.commands.DebugConfigCommand

System: [[20-Systems/net.minecraft.server.commands|net.minecraft.server.commands]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `register` | `(Lcom/mojang/brigadier/CommandDispatcher;Lnet/minecraft/commands/Comma` | exact | invokestatic@24 in `CommandsMixin.init` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| wraps | `unconfig` | `(Lnet/minecraft/commands/CommandSourceStack;Ljava/util/UUID;)I` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (0 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>()V
public static register(Lcom/mojang/brigadier/CommandDispatcher;Lnet/minecraft/commands/CommandBuildContext;)V
private static getUuidsInConfig(Lnet/minecraft/server/MinecraftServer;)Ljava/lang/Iterable;
private static config(Lnet/minecraft/commands/CommandSourceStack;Lnet/minecraft/server/level/ServerPlayer;)I
private static findConfigPlayer(Lnet/minecraft/server/MinecraftServer;Ljava/util/UUID;)Lnet/minecraft/server/network/ServerConfigurationPacketListenerImpl;
private static unconfig(Lnet/minecraft/commands/CommandSourceStack;Ljava/util/UUID;)I
private static showDialog(Lnet/minecraft/commands/CommandSourceStack;Ljava/util/UUID;Lnet/minecraft/core/Holder;)I
private static synthetic lambda$config$0(Lcom/mojang/authlib/GameProfile;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$register$4(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$3(Lcom/mojang/brigadier/context/CommandContext;Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$register$2(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$1(Lcom/mojang/brigadier/context/CommandContext;Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$register$0(Lcom/mojang/brigadier/context/CommandContext;)I
```
