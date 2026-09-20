---
type: "interface"
fqcn: "net.minecraft.server.commands.DataPackCommand"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.commands.DataPackCommand

System: [[20-Systems/net.minecraft.server.commands|net.minecraft.server.commands]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `getPack` | `(Lcom/mojang/brigadier/context/CommandContext;Ljava/lang/String;Z)Lnet` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| wraps | `lambda$static$10` | `(Lcom/mojang/brigadier/context/CommandContext;Lcom/mojang/brigadier/su` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| wraps | `lambda$static$11` | `(Lcom/mojang/brigadier/context/CommandContext;Lcom/mojang/brigadier/su` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (13 fields, 48 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final ERROR_UNKNOWN_PACK : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
private static final ERROR_PACK_ALREADY_ENABLED : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
private static final ERROR_PACK_ALREADY_DISABLED : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
private static final ERROR_CANNOT_DISABLE_FEATURE : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
private static final ERROR_PACK_FEATURES_NOT_ENABLED : Lcom/mojang/brigadier/exceptions/Dynamic2CommandExceptionType;
private static final ERROR_PACK_INVALID_NAME : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
private static final ERROR_PACK_INVALID_FULL_NAME : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
private static final ERROR_PACK_ALREADY_EXISTS : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
private static final ERROR_PACK_METADATA_ENCODE_FAILURE : Lcom/mojang/brigadier/exceptions/Dynamic2CommandExceptionType;
private static final ERROR_PACK_IO_FAILURE : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
private static final SELECTED_PACKS : Lcom/mojang/brigadier/suggestion/SuggestionProvider;
private static final UNSELECTED_PACKS : Lcom/mojang/brigadier/suggestion/SuggestionProvider;
public <init>()V
public static register(Lcom/mojang/brigadier/CommandDispatcher;Lnet/minecraft/commands/CommandBuildContext;)V
private static createPack(Lnet/minecraft/commands/CommandSourceStack;Ljava/lang/String;Lnet/minecraft/network/chat/Component;)I
private static enablePack(Lnet/minecraft/commands/CommandSourceStack;Lnet/minecraft/server/packs/repository/Pack;Lnet/minecraft/server/commands/DataPackCommand$Inserter;)I
private static disablePack(Lnet/minecraft/commands/CommandSourceStack;Lnet/minecraft/server/packs/repository/Pack;)I
private static listPacks(Lnet/minecraft/commands/CommandSourceStack;)I
private static listAvailablePacks(Lnet/minecraft/commands/CommandSourceStack;)I
private static listEnabledPacks(Lnet/minecraft/commands/CommandSourceStack;)I
private static getPack(Lcom/mojang/brigadier/context/CommandContext;Ljava/lang/String;Z)Lnet/minecraft/server/packs/repository/Pack;
private static synthetic lambda$listEnabledPacks$1(Ljava/util/Collection;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$listEnabledPacks$2(Lnet/minecraft/server/packs/repository/Pack;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$listEnabledPacks$0()Lnet/minecraft/network/chat/Component;
private static synthetic lambda$listAvailablePacks$2(Ljava/util/List;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$listAvailablePacks$3(Lnet/minecraft/server/packs/repository/Pack;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$listAvailablePacks$1()Lnet/minecraft/network/chat/Component;
private static synthetic lambda$listAvailablePacks$0(Ljava/util/Collection;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/server/packs/repository/Pack;)Z
private static synthetic lambda$disablePack$0(Lnet/minecraft/server/packs/repository/Pack;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$enablePack$0(Lnet/minecraft/server/packs/repository/Pack;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$createPack$0(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$register$13(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$12(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$11(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$10(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$9(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$7(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$8(Ljava/util/List;Lnet/minecraft/server/packs/repository/Pack;)V
private static synthetic lambda$register$6(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$4(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$5(Lcom/mojang/brigadier/context/CommandContext;Ljava/util/List;Lnet/minecraft/server/packs/repository/Pack;)V
private static synthetic lambda$register$2(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$3(Lcom/mojang/brigadier/context/CommandContext;Ljava/util/List;Lnet/minecraft/server/packs/repository/Pack;)V
private static synthetic lambda$register$0(Lcom/mojang/brigadier/context/CommandContext;)I
private static synthetic lambda$register$1(Ljava/util/List;Lnet/minecraft/server/packs/repository/Pack;)V
private static synthetic lambda$static$11(Lcom/mojang/brigadier/context/CommandContext;Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$static$13(Ljava/util/Collection;Ljava/lang/String;)Z
private static synthetic lambda$static$12(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/server/packs/repository/Pack;)Z
private static synthetic lambda$static$10(Lcom/mojang/brigadier/context/CommandContext;Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$static$9(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$8(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$7(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$6(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$5(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$4(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$3(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$2(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$1(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$0(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
static <clinit>()V
```
