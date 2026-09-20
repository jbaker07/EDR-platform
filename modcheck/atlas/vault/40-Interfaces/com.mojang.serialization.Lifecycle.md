---
type: "interface"
fqcn: "com.mojang.serialization.Lifecycle"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.serialization.Lifecycle

Package `com.mojang.serialization`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `deprecated` | `(I)Lcom/mojang/serialization/Lifecycle;` | exact | invokestatic@37 in `WorldOpenFlowsMixin.injectHereForCustomScreen` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `stable` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokestatic@17 in `AdvancementLookup.registryLifecycle` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `stable` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokestatic@31 in `FabricRecipeProvider$FabricBootstrapContext.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `stable` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokestatic@7 in `FailSoftMapCodec.decode` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `stable` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokestatic@17 in `LootTableLookup.registryLifecycle` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `stable` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokestatic@5 in `FabricRegistryBuilder.create` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `stable` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokestatic@9 in `FabricRegistryBuilder.createDefaulted` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
