---
type: "interface"
fqcn: "net.minecraft.server.ReloadableServerResources"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.ReloadableServerResources

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<init>` | `(Lnet/minecraft/server/ReloadableServerRegistries$LoadResult;Lnet/mine` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/server/ReloadableServerRegistries$LoadResult;Lnet/mine` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| injects_into | `lambda$loadResources$2` | `(Lnet/minecraft/server/ReloadableServerRegistries$LoadResult;Lnet/mine` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `loadResources` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| injects_into | `updateComponentsAndStaticRegistryTags` | `()V` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `updateComponentsAndStaticRegistryTags` | `()V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (9 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final DATA_RELOAD_INITIAL_TASK : Ljava/util/concurrent/CompletableFuture;
private final fullRegistryHolder : Lnet/minecraft/server/ReloadableServerRegistries$Holder;
private final commands : Lnet/minecraft/commands/Commands;
private final recipes : Lnet/minecraft/world/item/crafting/RecipeManager;
private final advancements : Lnet/minecraft/server/ServerAdvancementManager;
private final functionLibrary : Lnet/minecraft/server/ServerFunctionLibrary;
private final postponedTags : Ljava/util/List;
private final newComponents : Ljava/util/List;
private <init>(Lnet/minecraft/server/ReloadableServerRegistries$LoadResult;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/commands/Commands$CommandSelection;Ljava/util/List;Lnet/minecraft/server/permissions/PermissionSet;Ljava/util/List;)V
public getFunctionLibrary()Lnet/minecraft/server/ServerFunctionLibrary;
public fullRegistries()Lnet/minecraft/server/ReloadableServerRegistries$Holder;
public getRecipeManager()Lnet/minecraft/world/item/crafting/RecipeManager;
public getCommands()Lnet/minecraft/commands/Commands;
public getAdvancements()Lnet/minecraft/server/ServerAdvancementManager;
public listeners()Ljava/util/List;
public static loadResources(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/core/LayeredRegistryAccess;Ljava/util/List;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/commands/Commands$CommandSelection;Lnet/minecraft/server/permissions/PermissionSet;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
public updateComponentsAndStaticRegistryTags()V
private static synthetic lambda$loadResources$0(Ljava/util/concurrent/Executor;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/commands/Commands$CommandSelection;Ljava/util/List;Lnet/minecraft/server/permissions/PermissionSet;Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/concurrent/Executor;Lnet/minecraft/server/ReloadableServerRegistries$LoadResult;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$loadResources$2(Lnet/minecraft/server/ReloadableServerRegistries$LoadResult;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/commands/Commands$CommandSelection;Ljava/util/List;Lnet/minecraft/server/permissions/PermissionSet;Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/List;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$loadResources$3(Lnet/minecraft/server/ReloadableServerResources;Ljava/lang/Object;)Lnet/minecraft/server/ReloadableServerResources;
private static synthetic lambda$loadResources$1(Lnet/minecraft/server/ReloadableServerRegistries$LoadResult;)Ljava/util/List;
static <clinit>()V
```
