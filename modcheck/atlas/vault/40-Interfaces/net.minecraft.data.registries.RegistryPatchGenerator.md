---
type: "interface"
fqcn: "net.minecraft.data.registries.RegistryPatchGenerator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.registries.RegistryPatchGenerator

System: [[20-Systems/net.minecraft.data.registries|net.minecraft.data.registries]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `lambda$createReloadableLookup$0` | `(Lnet/minecraft/core/RegistrySetBuilder;Lnet/minecraft/core/HolderLook` | name_only | @Redirect at ['FIELD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| wraps | `lambda$createWorldLookup$0` | `(Lnet/minecraft/core/RegistrySetBuilder;Lnet/minecraft/core/HolderLook` | name_only | @Redirect at ['FIELD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (0 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>()V
private static hasAnyPatchedElement(Lnet/minecraft/core/RegistrySetBuilder$PatchedRegistries;Lnet/minecraft/resources/ResourceKey;)Z
public static createWorldLookup(Ljava/util/concurrent/CompletableFuture;Lnet/minecraft/core/RegistrySetBuilder;)Ljava/util/concurrent/CompletableFuture;
public static createReloadableLookup(Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Lnet/minecraft/core/RegistrySetBuilder;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$createReloadableLookup$0(Lnet/minecraft/core/RegistrySetBuilder;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/RegistrySetBuilder$PatchedRegistries;
private static synthetic lambda$createReloadableLookup$2(Lnet/minecraft/core/RegistrySetBuilder$PatchedRegistries;Lnet/minecraft/world/level/storage/loot/LootDataType;)Z
private static synthetic lambda$createReloadableLookup$1(Lnet/minecraft/core/Cloner$Factory;Lnet/minecraft/resources/RegistryDataLoader$RegistryData;)V
private static synthetic lambda$createWorldLookup$0(Lnet/minecraft/core/RegistrySetBuilder;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/RegistrySetBuilder$PatchedRegistries;
private static synthetic lambda$createWorldLookup$1(Lnet/minecraft/core/Cloner$Factory;Lnet/minecraft/resources/RegistryDataLoader$RegistryData;)V
private static synthetic lambda$hasAnyPatchedElement$0(Lnet/minecraft/core/HolderLookup$RegistryLookup;)Ljava/util/Optional;
```
