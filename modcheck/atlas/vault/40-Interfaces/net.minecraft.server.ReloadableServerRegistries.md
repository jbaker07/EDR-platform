---
type: "interface"
fqcn: "net.minecraft.server.ReloadableServerRegistries"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.ReloadableServerRegistries

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `reload` | `@Redirect at FIELD Lnet/minecraft/resources/RegistryDataLoader;RELOADABLE_REGIST` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.ReloadableServerRegistries {
    private static final org.slf4j.Logger LOGGER;
    private static final net.minecraft.core.RegistrationInfo DEFAULT_REGISTRATION_INFO;
    public net.minecraft.server.ReloadableServerRegistries();
    public static java.util.concurrent.CompletableFuture<net.minecraft.server.ReloadableServerRegistries$LoadResult> reload(net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>, java.util.List<net.minecraft.core.Registry$PendingTags<?>>, net.minecraft.server.packs.resources.ResourceManager, java.util.concurrent.Executor);
    private static net.minecraft.server.ReloadableServerRegistries$LoadResult createAndValidateFullContext(net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>, net.minecraft.core.HolderLookup$Provider, net.minecraft.core.RegistryAccess$Frozen);
    private static net.minecraft.core.HolderLookup$Provider concatenateLookups(net.minecraft.core.HolderLookup$Provider, net.minecraft.core.HolderLookup$Provider);
    private static void validateLootRegistries(net.minecraft.core.HolderLookup$Provider);
    private static void lambda$validateLootRegistries$1(java.lang.String, net.minecraft.util.ProblemReporter$Problem);
    private static void lambda$validateLootRegistries$0(net.minecraft.world.level.storage.loot.ValidationContextSource, net.minecraft.core.HolderLookup$Provider, net.minecraft.world.level.storage.loot.LootDataType);
    private static net.minecraft.server.ReloadableServerRegistries$LoadResult lambda$reload$0(net.minecraft.core.LayeredRegistryAccess, net.minecraft.core.HolderLookup$Provider, net.minecraft.core.RegistryAccess$Frozen);
    static {};
}
```
