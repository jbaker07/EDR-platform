---
type: "interface"
fqcn: "net.minecraft.core.RegistrySetBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.RegistrySetBuilder

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@4 in `FabricDataGenHelper.createWorldLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@4 in `FabricDataGenHelper.createReloadableLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `add` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/registries/S` | exact | invokevirtual@7 in `FabricDataGenHelper.addEmptyRegistry` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `build` | `(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/Holder` | exact | invokevirtual@169 in `FabricDataGenHelper.createWorldLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `build` | `(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/Holder` | exact | invokevirtual@168 in `FabricDataGenHelper.createReloadableLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `entries` | `Ljava/util/List;` | exact | getfield@19 in `FabricDataGenHelper.createWorldLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `entries` | `Ljava/util/List;` | exact | getfield@107 in `FabricDataGenHelper.createWorldLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `entries` | `Ljava/util/List;` | exact | getfield@113 in `FabricDataGenHelper.createWorldLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `entries` | `Ljava/util/List;` | exact | getfield@19 in `FabricDataGenHelper.createReloadableLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `entries` | `Ljava/util/List;` | exact | getfield@110 in `FabricDataGenHelper.createReloadableLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `entries` | `Ljava/util/List;` | exact | getfield@116 in `FabricDataGenHelper.createReloadableLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (1 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final entries : Ljava/util/List;
public <init>()V
private static placeholderStub(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/RegistrySetBuilder$RegistryStub;
public add(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/registries/SingleRegistryBootstrap;)Lnet/minecraft/core/RegistrySetBuilder;
public add(Lnet/minecraft/core/registries/MultiRegistryBootstrap;)Lnet/minecraft/core/RegistrySetBuilder;
private static buildProviderWithContext(Lnet/minecraft/core/HolderLookup$Provider;Ljava/util/stream/Stream;)Lnet/minecraft/core/HolderLookup$Provider;
public build(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/HolderLookup$Provider;
private static findRegistriesMissingFromPatch(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/HolderLookup$Provider;Ljava/util/List;)Ljava/util/Set;
public buildPatch(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/Cloner$Factory;)Lnet/minecraft/core/RegistrySetBuilder$PatchedRegistries;
private static eyerollCast(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/resources/ResourceKey;
private static newRegistryKeys(Ljava/util/stream/Stream;)Ljava/util/stream/Stream;
private static synthetic lambda$findRegistriesMissingFromPatch$0(Ljava/util/Set;Lnet/minecraft/resources/ResourceKey;)Z
private static synthetic lambda$buildProviderWithContext$1(Ljava/util/Map;Lnet/minecraft/core/HolderLookup$RegistryLookup;)V
private static synthetic lambda$buildProviderWithContext$0(Ljava/util/Map;Lnet/minecraft/core/HolderLookup$RegistryLookup;)V
```
