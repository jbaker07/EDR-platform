---
type: "interface"
fqcn: "net.minecraft.server.ReloadableServerResources"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.ReloadableServerResources

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| injects_into | `lambda$loadResources$2` | `@ModifyArg at INVOKE Lnet/minecraft/server/packs/resources/SimpleReloadInstance;` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `loadResources` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| injects_into | `updateComponentsAndStaticRegistryTags` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `updateComponentsAndStaticRegistryTags` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.ReloadableServerResources {
    private static final org.slf4j.Logger LOGGER;
    private static final java.util.concurrent.CompletableFuture<net.minecraft.util.Unit> DATA_RELOAD_INITIAL_TASK;
    private final net.minecraft.server.ReloadableServerRegistries$Holder fullRegistryHolder;
    private final net.minecraft.commands.Commands commands;
    private final net.minecraft.world.item.crafting.RecipeManager recipes;
    private final net.minecraft.server.ServerAdvancementManager advancements;
    private final net.minecraft.server.ServerFunctionLibrary functionLibrary;
    private final java.util.List<net.minecraft.core.Registry$PendingTags<?>> postponedTags;
    private final java.util.List<net.minecraft.core.component.DataComponentInitializers$PendingComponents<?>> newComponents;
    private net.minecraft.server.ReloadableServerResources(net.minecraft.server.ReloadableServerRegistries$LoadResult, net.minecraft.world.flag.FeatureFlagSet, net.minecraft.commands.Commands$CommandSelection, java.util.List<net.minecraft.core.Registry$PendingTags<?>>, net.minecraft.server.permissions.PermissionSet, java.util.List<net.minecraft.core.component.DataComponentInitializers$PendingComponents<?>>);
    public net.minecraft.server.ServerFunctionLibrary getFunctionLibrary();
    public net.minecraft.server.ReloadableServerRegistries$Holder fullRegistries();
    public net.minecraft.world.item.crafting.RecipeManager getRecipeManager();
    public net.minecraft.commands.Commands getCommands();
    public net.minecraft.server.ServerAdvancementManager getAdvancements();
    public java.util.List<net.minecraft.server.packs.resources.PreparableReloadListener> listeners();
    public static java.util.concurrent.CompletableFuture<net.minecraft.server.ReloadableServerResources> loadResources(net.minecraft.server.packs.resources.ResourceManager, net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>, java.util.List<net.minecraft.core.Registry$PendingTags<?>>, net.minecraft.world.flag.FeatureFlagSet, net.minecraft.commands.Commands$CommandSelection, net.minecraft.server.permissions.PermissionSet, java.util.concurrent.Executor, java.util.concurrent.Executor);
    public void updateComponentsAndStaticRegistryTags();
    private static java.util.concurrent.CompletionStage lambda$loadResources$0(java.util.concurrent.Executor, net.minecraft.world.flag.FeatureFlagSet, net.minecraft.commands.Commands$CommandSelection, java.util.List, net.minecraft.server.permissions.PermissionSet, net.minecraft.server.packs.resources.ResourceManager, java.util.concurrent.Executor, net.minecraft.server.ReloadableServerRegistries$LoadResult);
    private static java.util.concurrent.CompletionStage lambda$loadResources$2(net.minecraft.server.ReloadableServerRegistries$LoadResult, net.minecraft.world.flag.FeatureFlagSet, net.minecraft.commands.Commands$CommandSelection, java.util.List, net.minecraft.server.permissions.PermissionSet, net.minecraft.server.packs.resources.ResourceManager, java.util.concurrent.Executor, java.util.concurrent.Executor, java.util.List);
    private static net.minecraft.server.ReloadableServerResources lambda$loadResources$3(net.minecraft.server.ReloadableServerResources, java.lang.Object);
    private static java.util.List lambda$loadResources$1(net.minecraft.server.ReloadableServerRegistries$LoadResult);
    static {};
}
```
