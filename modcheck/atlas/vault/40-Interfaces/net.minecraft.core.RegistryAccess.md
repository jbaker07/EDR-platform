---
type: "interface"
fqcn: "net.minecraft.core.RegistryAccess"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.RegistryAccess

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/core/HolderLookup$Provider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `fromRegistryOfRegistries` | `(Lnet/minecraft/core/Registry;)Lnet/minecraft/core/RegistryAccess$Froz` | exact | invokestatic@163 in `FabricDataGenHelper.createWorldLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `listRegistries` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@9 in `RegistryCustomContentState.construct` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@19 in `TagUtil.isIn` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@68 in `RegistryCustomContentState.validate` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@45 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | exact | invokeinterface@25 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | exact | invokeinterface@44 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | exact | invokeinterface@19 in `BiomeModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | exact | invokeinterface@270 in `BiomeModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | exact | invokeinterface@24 in `BiomeSelectionContextImpl.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | exact | invokeinterface@7 in `BiomeSelectionContextImpl.getFeatureKey` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | exact | invokeinterface@7 in `BiomeSelectionContextImpl.getPlacedFeatureKey` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | exact | invokeinterface@7 in `BiomeSelectionContextImpl.validForStructure` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | exact | invokeinterface@7 in `BiomeSelectionContextImpl.getStructureKey` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | exact | invokeinterface@7 in `BiomeSelectionContextImpl.canGenerateIn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | exact | invokeinterface@7 in `BiomeSelectionContextImpl.hasTag` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | exact | invokeinterface@15 in `DimensionModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |

## Declared members (2 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final LOGGER : Lorg/slf4j/Logger;
public static final EMPTY : Lnet/minecraft/core/RegistryAccess$Frozen;
public abstract lookup(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;
public abstract registries()Ljava/util/stream/Stream;
public listRegistryKeys()Ljava/util/stream/Stream;
public static fromRegistryOfRegistries(Lnet/minecraft/core/Registry;)Lnet/minecraft/core/RegistryAccess$Frozen;
public freeze()Lnet/minecraft/core/RegistryAccess$Frozen;
public synthetic lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderLookup$RegistryLookup;
public synthetic lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderGetter;
private static synthetic lambda$listRegistryKeys$0(Lnet/minecraft/core/RegistryAccess$RegistryEntry;)Lnet/minecraft/resources/ResourceKey;
private static synthetic lambda$lookupOrThrow$0(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/IllegalStateException;
static <clinit>()V
```
