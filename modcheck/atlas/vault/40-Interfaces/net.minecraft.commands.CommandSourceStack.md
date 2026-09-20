---
type: "interface"
fqcn: "net.minecraft.commands.CommandSourceStack"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.commands.CommandSourceStack

System: [[20-Systems/net.minecraft.commands|net.minecraft.commands]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getEntity()Lnet/minecraft/world/entity/Entity;` | `` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getLevel()Lnet/minecraft/server/level/ServerLevel;` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getLevel()Lnet/minecraft/server/level/ServerLevel;` | `` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getPlayer()Lnet/minecraft/server/level/ServerPlayer;` | `` | both | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| calls | `getPosition()Lnet/minecraft/world/phys/Vec3;` | `` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getServer()Lnet/minecraft/server/MinecraftServer;` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getServer()Lnet/minecraft/server/MinecraftServer;` | `` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getTextName()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `permissions()Lnet/minecraft/server/permissions/PermissionSet;` | `` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `sendSuccess(Ljava/util/function/Supplier;Z)V` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| injects_into | `<init>(Lnet/minecraft/commands/CommandSource;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec2;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/permissions/PermissionSet;Lnet/minecraft/commands/CommandSourceStack$NamesProvider;Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/world/entity/Entity;)V` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |

## Declared members (71, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.commands.CommandSourceStack implements net.minecraft.commands.SharedSuggestionProvider, net.minecraft.commands.ExecutionCommandSource<net.minecraft.commands.CommandSourceStack> {
    public static final com.mojang.brigadier.exceptions.SimpleCommandExceptionType ERROR_NOT_PLAYER;
    public static final com.mojang.brigadier.exceptions.SimpleCommandExceptionType ERROR_NOT_ENTITY;
    private final net.minecraft.commands.CommandSource source;
    private final net.minecraft.world.phys.Vec3 worldPosition;
    private final net.minecraft.server.level.ServerLevel level;
    private final net.minecraft.server.permissions.PermissionSet permissions;
    private final net.minecraft.commands.CommandSourceStack$NamesProvider namesProvider;
    private final net.minecraft.server.MinecraftServer server;
    private final boolean silent;
    private final net.minecraft.world.entity.Entity entity;
    private final net.minecraft.commands.CommandResultCallback resultCallback;
    private final net.minecraft.commands.arguments.EntityAnchorArgument$Anchor anchor;
    private final net.minecraft.world.phys.Vec2 rotation;
    private final net.minecraft.commands.CommandSigningContext signingContext;
    private final net.minecraft.util.TaskChainer chatMessageChainer;
    public net.minecraft.commands.CommandSourceStack(net.minecraft.commands.CommandSource, net.minecraft.world.phys.Vec3, net.minecraft.world.phys.Vec2, net.minecraft.server.level.ServerLevel, net.minecraft.server.permissions.PermissionSet, net.minecraft.server.MinecraftServer, net.minecraft.world.entity.Entity);
    public net.minecraft.commands.CommandSourceStack(net.minecraft.commands.CommandSource, net.minecraft.world.phys.Vec3, net.minecraft.world.phys.Vec2, net.minecraft.server.level.ServerLevel, net.minecraft.server.permissions.PermissionSet, net.minecraft.network.chat.Component, net.minecraft.server.MinecraftServer);
    private net.minecraft.commands.CommandSourceStack(net.minecraft.commands.CommandSource, net.minecraft.world.phys.Vec3, net.minecraft.world.phys.Vec2, net.minecraft.server.level.ServerLevel, net.minecraft.server.permissions.PermissionSet, net.minecraft.commands.CommandSourceStack$NamesProvider, net.minecraft.server.MinecraftServer, net.minecraft.world.entity.Entity);
    private net.minecraft.commands.CommandSourceStack(net.minecraft.commands.CommandSource, net.minecraft.world.phys.Vec3, net.minecraft.world.phys.Vec2, net.minecraft.server.level.ServerLevel, net.minecraft.server.permissions.PermissionSet, net.minecraft.commands.CommandSourceStack$NamesProvider, net.minecraft.server.MinecraftServer, net.minecraft.world.entity.Entity, boolean, net.minecraft.commands.CommandResultCallback, net.minecraft.commands.arguments.EntityAnchorArgument$Anchor, net.minecraft.commands.CommandSigningContext, net.minecraft.util.TaskChainer);
    public net.minecraft.commands.CommandSourceStack withSource(net.minecraft.commands.CommandSource);
    public net.minecraft.commands.CommandSourceStack withEntity(net.minecraft.world.entity.Entity);
    public net.minecraft.commands.CommandSourceStack withPosition(net.minecraft.world.phys.Vec3);
    public net.minecraft.commands.CommandSourceStack withRotation(net.minecraft.world.phys.Vec2);
    public net.minecraft.commands.CommandSourceStack withCallback(net.minecraft.commands.CommandResultCallback);
    public net.minecraft.commands.CommandSourceStack withCallback(net.minecraft.commands.CommandResultCallback, java.util.function.BinaryOperator<net.minecraft.commands.CommandResultCallback>);
    public net.minecraft.commands.CommandSourceStack withSuppressedOutput();
    public net.minecraft.commands.CommandSourceStack withPermission(net.minecraft.server.permissions.PermissionSet);
    public net.minecraft.commands.CommandSourceStack withMaximumPermission(net.minecraft.server.permissions.PermissionSet);
    public net.minecraft.commands.CommandSourceStack withAnchor(net.minecraft.commands.arguments.EntityAnchorArgument$Anchor);
    public net.minecraft.commands.CommandSourceStack withLevel(net.minecraft.server.level.ServerLevel);
    public net.minecraft.commands.CommandSourceStack facing(net.minecraft.world.entity.Entity, net.minecraft.commands.arguments.EntityAnchorArgument$Anchor);
    public net.minecraft.commands.CommandSourceStack facing(net.minecraft.world.phys.Vec3);
    public net.minecraft.commands.CommandSourceStack withSigningContext(net.minecraft.commands.CommandSigningContext, net.minecraft.util.TaskChainer);
    public net.minecraft.network.chat.Component getDisplayName();
    public java.lang.String getTextName();
    public net.minecraft.server.permissions.PermissionSet permissions();
    public net.minecraft.world.phys.Vec3 getPosition();
    public net.minecraft.server.level.ServerLevel getLevel();
    public net.minecraft.world.entity.Entity getEntity();
    public net.minecraft.world.entity.Entity getEntityOrException() throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    public net.minecraft.server.level.ServerPlayer getPlayerOrException() throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    public net.minecraft.server.level.ServerPlayer getPlayer();
    public boolean isPlayer();
    public net.minecraft.world.phys.Vec2 getRotation();
    public net.minecraft.server.MinecraftServer getServer();
    public net.minecraft.commands.arguments.EntityAnchorArgument$Anchor getAnchor();
    public net.minecraft.commands.CommandSigningContext getSigningContext();
    public net.minecraft.util.TaskChainer getChatMessageChainer();
    public boolean shouldFilterMessageTo(net.minecraft.server.level.ServerPlayer);
    public void sendChatMessage(net.minecraft.network.chat.OutgoingChatMessage, boolean, net.minecraft.network.chat.ChatType$Bound);
    public void sendSystemMessage(net.minecraft.network.chat.Component);
    public void sendSuccess(java.util.function.Supplier<net.minecraft.network.chat.Component>, boolean);
    private void broadcastToAdmins(net.minecraft.network.chat.Component);
    public void sendFailure(net.minecraft.network.chat.Component);
    public net.minecraft.commands.CommandResultCallback callback();
    public java.util.Collection<java.lang.String> getOnlinePlayerNames();
    public java.util.Collection<java.lang.String> getAllTeams();
    public java.util.stream.Stream<net.minecraft.resources.Identifier> getAvailableSounds();
    public java.util.stream.Stream<net.minecraft.resources.Identifier> getAvailablePostEffects();
    public java.util.concurrent.CompletableFuture<com.mojang.brigadier.suggestion.Suggestions> customSuggestion(com.mojang.brigadier.context.CommandContext<?>);
    public <E> java.util.concurrent.CompletableFuture<com.mojang.brigadier.suggestion.Suggestions> suggestRegistryElements(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<E>>, net.minecraft.commands.SharedSuggestionProvider$ElementSuggestionType, com.mojang.brigadier.suggestion.SuggestionsBuilder, com.mojang.brigadier.context.CommandContext<?>, java.util.function.Predicate<E>);
    private <E> java.util.Optional<? extends net.minecraft.core.HolderLookup<E>> getLookup(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<E>>);
    public java.util.Set<net.minecraft.resources.ResourceKey<net.minecraft.world.level.Level>> levels();
    public net.minecraft.core.RegistryAccess registryAccess();
    public net.minecraft.world.flag.FeatureFlagSet enabledFeatures();
    public com.mojang.brigadier.CommandDispatcher<net.minecraft.commands.CommandSourceStack> dispatcher();
    public void handleError(com.mojang.brigadier.exceptions.CommandExceptionType, com.mojang.brigadier.Message, boolean, net.minecraft.commands.execution.TraceCallbacks);
    public boolean isSilent();
    public net.minecraft.commands.ExecutionCommandSource withCallback(net.minecraft.commands.CommandResultCallback);
    private java.util.concurrent.CompletableFuture lambda$suggestRegistryElements$0(net.minecraft.commands.SharedSuggestionProvider$ElementSuggestionType, com.mojang.brigadier.suggestion.SuggestionsBuilder, java.util.function.Predicate, net.minecraft.core.HolderLookup);
    static {};
}
```
