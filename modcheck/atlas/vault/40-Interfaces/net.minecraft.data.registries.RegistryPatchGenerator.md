---
type: "interface"
fqcn: "net.minecraft.data.registries.RegistryPatchGenerator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.registries.RegistryPatchGenerator

System: [[20-Systems/net.minecraft.data.registries|net.minecraft.data.registries]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `lambda$createReloadableLookup$0` | `@Redirect at FIELD Lnet/minecraft/resources/RegistryDataLoader;RELOADABLE_REGIST` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| wraps | `lambda$createWorldLookup$0` | `@Redirect at FIELD Lnet/minecraft/resources/RegistryDataLoader;WORLD_REGISTRIES:` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.registries.RegistryPatchGenerator {
    public net.minecraft.data.registries.RegistryPatchGenerator();
    private static boolean hasAnyPatchedElement(net.minecraft.core.RegistrySetBuilder$PatchedRegistries, net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>);
    public static java.util.concurrent.CompletableFuture<net.minecraft.core.RegistrySetBuilder$PatchedRegistries> createWorldLookup(java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider>, net.minecraft.core.RegistrySetBuilder);
    public static java.util.concurrent.CompletableFuture<net.minecraft.core.RegistrySetBuilder$PatchedRegistries> createReloadableLookup(java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider>, java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider>, net.minecraft.core.RegistrySetBuilder);
    private static net.minecraft.core.RegistrySetBuilder$PatchedRegistries lambda$createReloadableLookup$0(net.minecraft.core.RegistrySetBuilder, net.minecraft.core.HolderLookup$Provider, net.minecraft.core.HolderLookup$Provider);
    private static boolean lambda$createReloadableLookup$2(net.minecraft.core.RegistrySetBuilder$PatchedRegistries, net.minecraft.world.level.storage.loot.LootDataType);
    private static void lambda$createReloadableLookup$1(net.minecraft.core.Cloner$Factory, net.minecraft.resources.RegistryDataLoader$RegistryData);
    private static net.minecraft.core.RegistrySetBuilder$PatchedRegistries lambda$createWorldLookup$0(net.minecraft.core.RegistrySetBuilder, net.minecraft.core.HolderLookup$Provider);
    private static void lambda$createWorldLookup$1(net.minecraft.core.Cloner$Factory, net.minecraft.resources.RegistryDataLoader$RegistryData);
    private static java.util.Optional lambda$hasAnyPatchedElement$0(net.minecraft.core.HolderLookup$RegistryLookup);
}
```
