---
type: "interface"
fqcn: "net.minecraft.resources.RegistryOps$RegistryInfoLookup"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryOps$RegistryInfoLookup

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@5 in `AdvancementHolderProvider.lookup` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@30 in `AdvancementHolderProvider.lookup` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@8 in `AdvancementHolderProvider.get` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@8 in `AdvancementHolderProvider.getOrThrow` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@8 in `AdvancementHolderProvider.getOrThrow` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@8 in `AdvancementHolderProvider.get` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@5 in `LootTableHolderProvider.lookup` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@30 in `LootTableHolderProvider.lookup` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@8 in `LootTableHolderProvider.get` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@8 in `LootTableHolderProvider.getOrThrow` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@8 in `LootTableHolderProvider.getOrThrow` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@8 in `LootTableHolderProvider.get` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@24 in `ResourceConditionsImpl.tagsPopulated` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@24 in `ResourceConditionsImpl.registryContains` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (0 fields, 1 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract lookup(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
```
