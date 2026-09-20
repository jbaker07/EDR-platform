---
type: "system"
package: "net.minecraft.server.packs"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs

Analyst note: [[_authored/systems/net.minecraft.server.packs|Packs, resources and reload]]

110 classes (57 top-level) across 6 packages in the processed jar; 4 changed by Loom processing; 29 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.server.packs.OverlayedPackResources|OverlayedPackResources]] -- calls:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.PackLocationInfo|PackLocationInfo]] -- calls:4 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.PackMetadataResources|PackMetadataResources]] -- calls:1 -- by fabric-resource-conditions-api-v1
- [[40-Interfaces/net.minecraft.server.packs.PackResources|PackResources]] -- calls:7 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.PackResources_ResourceOutput|PackResources$ResourceOutput]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.PackSelectionConfig|PackSelectionConfig]] -- calls:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.PackType|PackType]] -- calls:7, reads:20 -- by fabric-resource-loader-v1, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.server.packs.metadata.MetadataSectionType|MetadataSectionType]] -- calls:1 -- by fabric-resource-conditions-api-v1
- [[40-Interfaces/net.minecraft.server.packs.metadata.pack.PackMetadataSection|PackMetadataSection]] -- calls:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.BuiltInPackSource|BuiltInPackSource]] -- injects_into:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.FolderRepositorySource|FolderRepositorySource]] -- reads:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.KnownPack|KnownPack]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.Pack|Pack]] -- calls:18, injects_into:2 -- by fabric-resource-conditions-api-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.Pack_Metadata|Pack$Metadata]] -- calls:6 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.Pack_Position|Pack$Position]] -- reads:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.PackRepository|PackRepository]] -- calls:4, injects_into:4, reads:4 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.PackSource|PackSource]] -- reads:7 -- by fabric-advancement-api-v1, fabric-item-api-v1, fabric-loot-api-v3, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.repository.ServerPacksSource|ServerPacksSource]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.CloseableResourceManager|CloseableResourceManager]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.IoSupplier|IoSupplier]] -- calls:3 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.MultiPackResourceManager|MultiPackResourceManager]] -- injects_into:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.PreparableReloadListener|PreparableReloadListener]] -- calls:2 -- by fabric-resource-loader-v0, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.PreparableReloadListener_SharedState|PreparableReloadListener$SharedState]] -- calls:9 -- by fabric-resource-loader-v0, fabric-resource-loader-v1, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.PreparableReloadListener_StateKey|PreparableReloadListener$StateKey]] -- calls:3 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.Resource|Resource]] -- calls:6 -- by fabric-advancement-api-v1, fabric-gametest-api-v1, fabric-item-api-v1, fabric-loot-api-v3, fabric-resource-loader-v1, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.ResourceManager|ResourceManager]] -- calls:1 -- by fabric-gametest-api-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.ResourceMetadata|ResourceMetadata]] -- calls:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener|SimpleJsonResourceReloadListener]] -- injects_into:1, reads:1, wraps:1 -- by fabric-resource-conditions-api-v1
- [[40-Interfaces/net.minecraft.server.packs.resources.SimpleReloadInstance|SimpleReloadInstance]] -- injects_into:3 -- by fabric-resource-loader-v1

## Declared inventory

### `net.minecraft.server.packs` (17 top-level)

`AbstractPackMetadataResources`, `DownloadCacheCleaner`, `DownloadQueue`, `FeatureFlagsMetadataSection`, `FilePackResources`, `FixedPathPackResources`, `OverlayMetadataSection`, [[40-Interfaces/net.minecraft.server.packs.OverlayedPackResources|OverlayedPackResources]], [[40-Interfaces/net.minecraft.server.packs.PackLocationInfo|PackLocationInfo]], [[40-Interfaces/net.minecraft.server.packs.PackMetadataResources|PackMetadataResources]], [[40-Interfaces/net.minecraft.server.packs.PackResources|PackResources]], [[40-Interfaces/net.minecraft.server.packs.PackSelectionConfig|PackSelectionConfig]], [[40-Interfaces/net.minecraft.server.packs.PackType|PackType]], `PathPackResources`, `VanillaPackResources`, `VanillaPackResourcesBuilder`, `package-info`

### `net.minecraft.server.packs.linkfs` (6 top-level)

`LinkFSFileStore`, `LinkFSPath`, `LinkFSProvider`, `LinkFileSystem`, `PathContents`, `package-info`

### `net.minecraft.server.packs.metadata` (2 top-level)

[[40-Interfaces/net.minecraft.server.packs.metadata.MetadataSectionType|MetadataSectionType]], `package-info`

### `net.minecraft.server.packs.metadata.pack` (3 top-level)

`PackFormat`, [[40-Interfaces/net.minecraft.server.packs.metadata.pack.PackMetadataSection|PackMetadataSection]], `package-info`

### `net.minecraft.server.packs.repository` (11 top-level)

[[40-Interfaces/net.minecraft.server.packs.repository.BuiltInPackSource|BuiltInPackSource]], [[40-Interfaces/net.minecraft.server.packs.repository.FolderRepositorySource|FolderRepositorySource]], [[40-Interfaces/net.minecraft.server.packs.repository.KnownPack|KnownPack]], [[40-Interfaces/net.minecraft.server.packs.repository.Pack|Pack]], `PackCompatibility`, `PackDetector`, [[40-Interfaces/net.minecraft.server.packs.repository.PackRepository|PackRepository]], [[40-Interfaces/net.minecraft.server.packs.repository.PackSource|PackSource]], `RepositorySource`, [[40-Interfaces/net.minecraft.server.packs.repository.ServerPacksSource|ServerPacksSource]], `package-info`

### `net.minecraft.server.packs.resources` (18 top-level)

[[40-Interfaces/net.minecraft.server.packs.resources.CloseableResourceManager|CloseableResourceManager]], `FallbackResourceManager`, [[40-Interfaces/net.minecraft.server.packs.resources.IoSupplier|IoSupplier]], [[40-Interfaces/net.minecraft.server.packs.resources.MultiPackResourceManager|MultiPackResourceManager]], [[40-Interfaces/net.minecraft.server.packs.resources.PreparableReloadListener|PreparableReloadListener]], `ProfiledReloadInstance`, `ReloadInstance`, `ReloadableResourceManager`, [[40-Interfaces/net.minecraft.server.packs.resources.Resource|Resource]], `ResourceFilterSection`, [[40-Interfaces/net.minecraft.server.packs.resources.ResourceManager|ResourceManager]], `ResourceManagerReloadListener`, [[40-Interfaces/net.minecraft.server.packs.resources.ResourceMetadata|ResourceMetadata]], `ResourceProvider`, [[40-Interfaces/net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener|SimpleJsonResourceReloadListener]], `SimplePreparableReloadListener`, [[40-Interfaces/net.minecraft.server.packs.resources.SimpleReloadInstance|SimpleReloadInstance]], `package-info`

