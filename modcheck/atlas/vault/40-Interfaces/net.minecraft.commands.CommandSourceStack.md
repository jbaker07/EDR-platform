---
type: "interface"
fqcn: "net.minecraft.commands.CommandSourceStack"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.commands.CommandSourceStack

System: [[20-Systems/net.minecraft.commands|net.minecraft.commands]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/commands/SharedSuggestionProvider`, `net/minecraft/commands/ExecutionCommandSource`, `net/fabricmc/fabric/api/permission/v1/PermissionContextOwner`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getEntity` | `()Lnet/minecraft/world/entity/Entity;` | exact | invokevirtual@74 in `CommandPermissionContext.get` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getEntity` | `()Lnet/minecraft/world/entity/Entity;` | exact | invokevirtual@4 in `CommandPermissionContext.keys` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getLevel` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@9 in `EnumRuleCommand.executeAndSetEnum` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getLevel` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@59 in `CommandPermissionContext.get` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getPlayer` | `()Lnet/minecraft/server/level/ServerPlayer;` | exact | invokevirtual@44 in `PlayerListMixin.onSendCommandMessage` | unknown | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| calls | `getPosition` | `()Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@26 in `CommandPermissionContext.get` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getPosition` | `()Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@41 in `CommandPermissionContext.get` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getServer` | `()Lnet/minecraft/server/MinecraftServer;` | exact | invokevirtual@18 in `EnumRuleCommand.executeAndSetEnum` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getServer` | `()Lnet/minecraft/server/MinecraftServer;` | exact | invokevirtual@101 in `CommandPermissionContext.get` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getTextName` | `()Ljava/lang/String;` | exact | invokevirtual@11 in `CommandPermissionContext.get` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `permissions` | `()Lnet/minecraft/server/permissions/PermissionSet;` | exact | invokevirtual@12 in `CommandPermissionContext.permissionLevel` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `sendSuccess` | `(Ljava/util/function/Supplier;Z)V` | exact | invokevirtual@57 in `EnumRuleCommand.executeAndSetEnum` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| injects_into | `/^with/ desc=/CommandSourceStack;$/` | `?` | selector_unsupported | @ModifyReturnValue at ['RETURN'] | both | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/commands/CommandSource;Lnet/minecraft/world/phys/Vec3;` | exact | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |

## Declared members (15 fields, 56 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final ERROR_NOT_PLAYER : Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType;
public static final ERROR_NOT_ENTITY : Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType;
private final source : Lnet/minecraft/commands/CommandSource;
private final worldPosition : Lnet/minecraft/world/phys/Vec3;
private final level : Lnet/minecraft/server/level/ServerLevel;
private final permissions : Lnet/minecraft/server/permissions/PermissionSet;
private final namesProvider : Lnet/minecraft/commands/CommandSourceStack$NamesProvider;
private final server : Lnet/minecraft/server/MinecraftServer;
private final silent : Z
private final entity : Lnet/minecraft/world/entity/Entity;
private final resultCallback : Lnet/minecraft/commands/CommandResultCallback;
private final anchor : Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;
private final rotation : Lnet/minecraft/world/phys/Vec2;
private final signingContext : Lnet/minecraft/commands/CommandSigningContext;
private final chatMessageChainer : Lnet/minecraft/util/TaskChainer;
public <init>(Lnet/minecraft/commands/CommandSource;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec2;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/permissions/PermissionSet;Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/world/entity/Entity;)V
public <init>(Lnet/minecraft/commands/CommandSource;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec2;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/permissions/PermissionSet;Lnet/minecraft/network/chat/Component;Lnet/minecraft/server/MinecraftServer;)V
private <init>(Lnet/minecraft/commands/CommandSource;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec2;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/permissions/PermissionSet;Lnet/minecraft/commands/CommandSourceStack$NamesProvider;Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/world/entity/Entity;)V
private <init>(Lnet/minecraft/commands/CommandSource;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec2;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/permissions/PermissionSet;Lnet/minecraft/commands/CommandSourceStack$NamesProvider;Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/world/entity/Entity;ZLnet/minecraft/commands/CommandResultCallback;Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;Lnet/minecraft/commands/CommandSigningContext;Lnet/minecraft/util/TaskChainer;)V
public withSource(Lnet/minecraft/commands/CommandSource;)Lnet/minecraft/commands/CommandSourceStack;
public withEntity(Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/commands/CommandSourceStack;
public withPosition(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/commands/CommandSourceStack;
public withRotation(Lnet/minecraft/world/phys/Vec2;)Lnet/minecraft/commands/CommandSourceStack;
public withCallback(Lnet/minecraft/commands/CommandResultCallback;)Lnet/minecraft/commands/CommandSourceStack;
public withCallback(Lnet/minecraft/commands/CommandResultCallback;Ljava/util/function/BinaryOperator;)Lnet/minecraft/commands/CommandSourceStack;
public withSuppressedOutput()Lnet/minecraft/commands/CommandSourceStack;
public withPermission(Lnet/minecraft/server/permissions/PermissionSet;)Lnet/minecraft/commands/CommandSourceStack;
public withMaximumPermission(Lnet/minecraft/server/permissions/PermissionSet;)Lnet/minecraft/commands/CommandSourceStack;
public withAnchor(Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;)Lnet/minecraft/commands/CommandSourceStack;
public withLevel(Lnet/minecraft/server/level/ServerLevel;)Lnet/minecraft/commands/CommandSourceStack;
public facing(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;)Lnet/minecraft/commands/CommandSourceStack;
public facing(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/commands/CommandSourceStack;
public withSigningContext(Lnet/minecraft/commands/CommandSigningContext;Lnet/minecraft/util/TaskChainer;)Lnet/minecraft/commands/CommandSourceStack;
public getDisplayName()Lnet/minecraft/network/chat/Component;
public getTextName()Ljava/lang/String;
public permissions()Lnet/minecraft/server/permissions/PermissionSet;
public getPosition()Lnet/minecraft/world/phys/Vec3;
public getLevel()Lnet/minecraft/server/level/ServerLevel;
public getEntity()Lnet/minecraft/world/entity/Entity;
public getEntityOrException()Lnet/minecraft/world/entity/Entity;
public getPlayerOrException()Lnet/minecraft/server/level/ServerPlayer;
public getPlayer()Lnet/minecraft/server/level/ServerPlayer;
public isPlayer()Z
public getRotation()Lnet/minecraft/world/phys/Vec2;
public getServer()Lnet/minecraft/server/MinecraftServer;
public getAnchor()Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;
public getSigningContext()Lnet/minecraft/commands/CommandSigningContext;
public getChatMessageChainer()Lnet/minecraft/util/TaskChainer;
public shouldFilterMessageTo(Lnet/minecraft/server/level/ServerPlayer;)Z
public sendChatMessage(Lnet/minecraft/network/chat/OutgoingChatMessage;ZLnet/minecraft/network/chat/ChatType$Bound;)V
public sendSystemMessage(Lnet/minecraft/network/chat/Component;)V
public sendSuccess(Ljava/util/function/Supplier;Z)V
private broadcastToAdmins(Lnet/minecraft/network/chat/Component;)V
public sendFailure(Lnet/minecraft/network/chat/Component;)V
public callback()Lnet/minecraft/commands/CommandResultCallback;
public getOnlinePlayerNames()Ljava/util/Collection;
public getAllTeams()Ljava/util/Collection;
public getAvailableSounds()Ljava/util/stream/Stream;
public getAvailablePostEffects()Ljava/util/stream/Stream;
public customSuggestion(Lcom/mojang/brigadier/context/CommandContext;)Ljava/util/concurrent/CompletableFuture;
public suggestRegistryElements(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/commands/SharedSuggestionProvider$ElementSuggestionType;Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Lcom/mojang/brigadier/context/CommandContext;Ljava/util/function/Predicate;)Ljava/util/concurrent/CompletableFuture;
private getLookup(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public levels()Ljava/util/Set;
public registryAccess()Lnet/minecraft/core/RegistryAccess;
public enabledFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
public dispatcher()Lcom/mojang/brigadier/CommandDispatcher;
public handleError(Lcom/mojang/brigadier/exceptions/CommandExceptionType;Lcom/mojang/brigadier/Message;ZLnet/minecraft/commands/execution/TraceCallbacks;)V
public isSilent()Z
public synthetic withCallback(Lnet/minecraft/commands/CommandResultCallback;)Lnet/minecraft/commands/ExecutionCommandSource;
private synthetic lambda$suggestRegistryElements$0(Lnet/minecraft/commands/SharedSuggestionProvider$ElementSuggestionType;Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Ljava/util/function/Predicate;Lnet/minecraft/core/HolderLookup;)Ljava/util/concurrent/CompletableFuture;
static <clinit>()V
```
