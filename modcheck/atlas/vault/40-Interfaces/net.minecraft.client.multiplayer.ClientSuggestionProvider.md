---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientSuggestionProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientSuggestionProvider

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/commands/SharedSuggestionProvider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `minecraft` | `Lnet/minecraft/client/Minecraft;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | declared |

## Declared members (6 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final connection : Lnet/minecraft/client/multiplayer/ClientPacketListener;
private final minecraft : Lnet/minecraft/client/Minecraft;
private pendingSuggestionsId : I
private pendingSuggestionsFuture : Ljava/util/concurrent/CompletableFuture;
private final customCompletionSuggestions : Ljava/util/Set;
private final permissions : Lnet/minecraft/server/permissions/PermissionSet;
public <init>(Lnet/minecraft/client/multiplayer/ClientPacketListener;Lnet/minecraft/client/Minecraft;Lnet/minecraft/server/permissions/PermissionSet;)V
public getOnlinePlayerNames()Ljava/util/Collection;
public getCustomTabSuggestions()Ljava/util/Collection;
public getSelectedEntities()Ljava/util/Collection;
public getAllTeams()Ljava/util/Collection;
public getAvailableSounds()Ljava/util/stream/Stream;
public getAvailablePostEffects()Ljava/util/stream/Stream;
public permissions()Lnet/minecraft/server/permissions/PermissionSet;
public suggestRegistryElements(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/commands/SharedSuggestionProvider$ElementSuggestionType;Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Lcom/mojang/brigadier/context/CommandContext;Ljava/util/function/Predicate;)Ljava/util/concurrent/CompletableFuture;
public customSuggestion(Lcom/mojang/brigadier/context/CommandContext;)Ljava/util/concurrent/CompletableFuture;
private static prettyPrint(D)Ljava/lang/String;
private static prettyPrint(I)Ljava/lang/String;
public getRelevantCoordinates()Ljava/util/Collection;
public getAbsoluteCoordinates()Ljava/util/Collection;
public levels()Ljava/util/Set;
public registryAccess()Lnet/minecraft/core/RegistryAccess;
public enabledFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
public completeCustomSuggestions(ILcom/mojang/brigadier/suggestion/Suggestions;)V
public modifyCustomCompletions(Lnet/minecraft/network/protocol/game/ClientboundCustomChatCompletionsPacket$Action;Ljava/util/List;)V
private synthetic lambda$suggestRegistryElements$1(Lcom/mojang/brigadier/context/CommandContext;)Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$suggestRegistryElements$0(Lnet/minecraft/commands/SharedSuggestionProvider$ElementSuggestionType;Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Ljava/util/function/Predicate;Lnet/minecraft/core/Registry;)Ljava/util/concurrent/CompletableFuture;
```
