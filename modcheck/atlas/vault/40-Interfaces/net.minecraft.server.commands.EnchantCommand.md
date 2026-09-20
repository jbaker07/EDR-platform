---
type: "interface"
fqcn: "net.minecraft.server.commands.EnchantCommand"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.commands.EnchantCommand

System: [[20-Systems/net.minecraft.server.commands|net.minecraft.server.commands]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `enchant` | `(Lnet/minecraft/commands/CommandSourceStack;Ljava/util/Collection;Lnet` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (6 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final ERROR_NOT_LIVING_ENTITY : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
private static final ERROR_NO_ITEM : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
private static final ERROR_INCOMPATIBLE : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
private static final ERROR_LEVEL_TOO_HIGH : Lcom/mojang/brigadier/exceptions/Dynamic2CommandExceptionType;
private static final ERROR_NOTHING_HAPPENED : Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType;
private static final RESPONSE_ENCHANT : Lnet/minecraft/server/commands/CommandResponseTracker$MessagesWithArgs;
public <init>()V
public static register(Lcom/mojang/brigadier/CommandDispatcher;Lnet/minecraft/commands/CommandBuildContext;)V
private static enchant(Lnet/minecraft/commands/CommandSourceStack;Ljava/util/Collection;Lnet/minecraft/core/Holder;I)I
private static synthetic lambda$register$1(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$0(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$static$5(IILnet/minecraft/core/Holder;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$static$4(Lnet/minecraft/world/entity/Entity;ILnet/minecraft/core/Holder;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$static$3(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$2(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$1(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$0(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
static <clinit>()V
```
