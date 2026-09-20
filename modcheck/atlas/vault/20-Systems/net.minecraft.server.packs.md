---
type: "system"
package: "net.minecraft.server.packs"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs

Analyst note: [[_authored/systems/net.minecraft.server.packs|Packs, resources and reload]]

110 classes in the jar. Hooked types: 25

- [[40-Interfaces/net.minecraft.server.packs.OverlayedPackResources|OverlayedPackResources]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.PackLocationInfo|PackLocationInfo]] -- calls:4 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.PackMetadataResources|PackMetadataResources]] -- calls:1 -- by fabric-resource-conditions-api-v1
- [[40-Interfaces/net.minecraft.server.packs.PackResources|PackResources]] -- calls:4 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.PackSelectionConfig|PackSelectionConfig]] -- calls:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.PackType|PackType]] -- calls:3, reads:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.metadata.MetadataSectionType|MetadataSectionType]] -- calls:1 -- by fabric-resource-conditions-api-v1
- [[40-Interfaces/net.minecraft.server.packs.metadata.pack.PackMetadataSection|PackMetadataSection]] -- calls:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.BuiltInPackSource|BuiltInPackSource]] -- injects_into:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.FolderRepositorySource|FolderRepositorySource]] -- reads:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.KnownPack|KnownPack]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.Pack|Pack]] -- calls:10, injects_into:1 -- by fabric-resource-conditions-api-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.Pack_Metadata|Pack$Metadata]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.PackRepository|PackRepository]] -- calls:4, injects_into:4, reads:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.PackSource|PackSource]] -- reads:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.ServerPacksSource|ServerPacksSource]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.CloseableResourceManager|CloseableResourceManager]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.IoSupplier|IoSupplier]] -- calls:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.MultiPackResourceManager|MultiPackResourceManager]] -- injects_into:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.PreparableReloadListener|PreparableReloadListener]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.PreparableReloadListener_SharedState|PreparableReloadListener$SharedState]] -- calls:3 -- by fabric-resource-loader-v1, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.Resource|Resource]] -- calls:5 -- by fabric-advancement-api-v1, fabric-item-api-v1, fabric-loot-api-v3, fabric-resource-loader-v1, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.ResourceMetadata|ResourceMetadata]] -- calls:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener|SimpleJsonResourceReloadListener]] -- injects_into:1 -- by fabric-resource-conditions-api-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.SimpleReloadInstance|SimpleReloadInstance]] -- injects_into:3 -- by fabric-resource-loader-v1
