---
type: "interface"
fqcn: "net.minecraft.core.HolderSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.HolderSet

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`interface` public abstract; extends `java/lang/Object`; implements `java/lang/Iterable`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `contains` | `(Lnet/minecraft/core/Holder;)Z` | exact | invokeinterface@64 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.addFeatu | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `contains` | `(Lnet/minecraft/core/Holder;)Z` | exact | invokeinterface@36 in `BiomeSelectionContextImpl.validForStructure` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `direct` | `(Ljava/util/List;)Lnet/minecraft/core/HolderSet$Direct;` | exact | invokestatic@101 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.removeFeat | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `direct` | `(Ljava/util/List;)Lnet/minecraft/core/HolderSet$Direct;` | exact | invokestatic@29 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.addFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `direct` | `(Ljava/util/List;)Lnet/minecraft/core/HolderSet$Direct;` | exact | invokestatic@60 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.removeCarve | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `direct` | `(Ljava/util/List;)Lnet/minecraft/core/HolderSet$Direct;` | exact | invokestatic@44 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.plus` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `direct` | `(Ljava/util/List;)Lnet/minecraft/core/HolderSet$Direct;` | exact | invokestatic@17 in `IngredientMixin.onGetEntries` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `direct` | `([Lnet/minecraft/core/Holder;)Lnet/minecraft/core/HolderSet$Direct;` | exact | invokestatic@12 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.plus` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `direct` | `([Lnet/minecraft/core/Holder;)Lnet/minecraft/core/HolderSet$Direct;` | exact | invokestatic@14 in `CustomIngredientImpl.<init>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `iterator` | `()Ljava/util/Iterator;` | inherited_exact | invokeinterface@42 in `BiomeSelectionContext.hasFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `iterator` | `()Ljava/util/Iterator;` | inherited_exact | invokeinterface@42 in `BiomeSelectionContext.hasPlacedFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `stream` | `()Ljava/util/stream/Stream;` | exact | invokeinterface@64 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.removeFe | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `stream` | `()Ljava/util/stream/Stream;` | exact | invokeinterface@26 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.removeCa | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `stream` | `()Ljava/util/stream/Stream;` | exact | invokeinterface@21 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.plus` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (0 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract stream()Ljava/util/stream/Stream;
public abstract size()I
public abstract isBound()Z
public abstract unwrap()Lcom/mojang/datafixers/util/Either;
public abstract getRandomElement(Lnet/minecraft/util/RandomSource;)Ljava/util/Optional;
public abstract get(I)Lnet/minecraft/core/Holder;
public abstract contains(Lnet/minecraft/core/Holder;)Z
public abstract canSerializeIn(Lnet/minecraft/core/HolderOwner;)Z
public abstract unwrapKey()Ljava/util/Optional;
public static emptyNamed(Lnet/minecraft/core/HolderOwner;Lnet/minecraft/tags/TagKey;)Lnet/minecraft/core/HolderSet$Named;
public static empty()Lnet/minecraft/core/HolderSet;
public static direct([Lnet/minecraft/core/Holder;)Lnet/minecraft/core/HolderSet$Direct;
public static direct(Ljava/util/List;)Lnet/minecraft/core/HolderSet$Direct;
public static direct(Ljava/util/function/Function;[Ljava/lang/Object;)Lnet/minecraft/core/HolderSet$Direct;
public static direct(Ljava/util/function/Function;Ljava/util/Collection;)Lnet/minecraft/core/HolderSet$Direct;
```
