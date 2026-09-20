---
type: "system"
package: "net.minecraft.core"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core

103 classes (39 top-level) across 1 packages in the processed jar; 10 changed by Loom processing; 33 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.core.BlockMath|BlockMath]] -- calls:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.core.BlockPos|BlockPos]] -- calls:29, reads:6 -- by fabric-api-lookup-api-v1, fabric-block-api-v1, fabric-block-getter-api-v2, fabric-data-attachment-api-v1, fabric-events-interaction-v0, fabric-particles-v1, fabric-permission-api-v1, fabric-renderer-api-v1, fabric-renderer-indigo, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.BlockPos_MutableBlockPos|BlockPos$MutableBlockPos]] -- calls:27 -- by fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.core.DefaultedMappedRegistry|DefaultedMappedRegistry]] -- calls:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.core.DefaultedRegistry|DefaultedRegistry]] -- calls:30 -- by fabric-api-lookup-api-v1, fabric-biome-api-v1, fabric-content-registries-v0, fabric-data-generation-api-v1, fabric-item-api-v1, fabric-model-loading-api-v1, fabric-object-builder-api-v1, fabric-registry-sync-v0, fabric-rendering-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.Direction|Direction]] -- calls:48, reads:68 -- by fabric-block-api-v1, fabric-item-api-v1, fabric-renderer-api-v1, fabric-renderer-indigo, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.Direction_Axis|Direction$Axis]] -- calls:8, reads:6 -- by fabric-renderer-indigo, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.Direction_AxisDirection|Direction$AxisDirection]] -- reads:1 -- by fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.core.Holder|Holder]] -- calls:46 -- by fabric-biome-api-v1, fabric-content-registries-v0, fabric-data-generation-api-v1, fabric-dimensions-v1, fabric-entity-events-v1, fabric-item-api-v1, fabric-loot-api-v3, fabric-recipe-api-v1, fabric-tag-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.Holder_Reference|Holder$Reference]] -- calls:29 -- by fabric-biome-api-v1, fabric-convention-tags-v2, fabric-creative-tab-api-v1, fabric-data-generation-api-v1, fabric-item-api-v1, fabric-loot-api-v3, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.core.HolderGetter|HolderGetter]] -- calls:19 -- by fabric-advancement-api-v1, fabric-biome-api-v1, fabric-loot-api-v3, fabric-resource-conditions-api-v1
- [[40-Interfaces/net.minecraft.core.HolderLookup|HolderLookup]] -- calls:1 -- by fabric-loot-api-v3
- [[40-Interfaces/net.minecraft.core.HolderLookup_Provider|HolderLookup$Provider]] -- calls:21 -- by fabric-advancement-api-v1, fabric-biome-api-v1, fabric-data-generation-api-v1, fabric-loot-api-v3, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.core.HolderLookup_RegistryLookup|HolderLookup$RegistryLookup]] -- calls:14 -- by fabric-advancement-api-v1, fabric-data-generation-api-v1, fabric-loot-api-v3, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.core.HolderSet|HolderSet]] -- calls:14 -- by fabric-biome-api-v1, fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.core.HolderSet_Named|HolderSet$Named]] -- calls:3, reads:1, writes:1 -- by fabric-convention-tags-v2, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.core.IdMapper|IdMapper]] -- calls:1, reads:3 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.core.LayeredRegistryAccess|LayeredRegistryAccess]] -- calls:5 -- by fabric-client-gametest-api-v1, fabric-lifecycle-events-v1, fabric-registry-sync-v0, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]] -- calls:10, injects_into:10, reads:9 -- by fabric-biome-api-v1, fabric-registry-sync-v0, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.core.MappedRegistry_2|MappedRegistry$2]] -- calls:1 -- by fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.core.MappedRegistry_3|MappedRegistry$3]] -- injects_into:1, reads:1 -- by fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.core.MappedRegistry_TagSet|MappedRegistry$TagSet]] -- calls:2 -- by fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.core.NonNullList|NonNullList]] -- calls:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.RegistrationInfo|RegistrationInfo]] -- calls:8, reads:1 -- by fabric-biome-api-v1, fabric-dimensions-v1, fabric-item-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.core.Registry|Registry]] -- calls:96 -- by fabric-biome-api-v1, fabric-client-gametest-api-v1, fabric-command-api-v2, fabric-convention-tags-v2, fabric-creative-tab-api-v1, fabric-data-generation-api-v1, fabric-dimensions-v1, fabric-game-rule-api-v1, fabric-gametest-api-v1, fabric-loot-api-v3, fabric-menu-api-v1, fabric-object-builder-api-v1, fabric-particles-v1, fabric-recipe-api-v1, fabric-registry-sync-v0, fabric-tag-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.RegistryAccess|RegistryAccess]] -- calls:17 -- by fabric-biome-api-v1, fabric-convention-tags-v2, fabric-data-generation-api-v1, fabric-dimensions-v1, fabric-registry-sync-v0, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.core.RegistryAccess_Frozen|RegistryAccess$Frozen]] -- calls:5 -- by fabric-advancement-api-v1, fabric-client-gametest-api-v1, fabric-convention-tags-v2, fabric-loot-api-v3, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.core.RegistryAccess_RegistryEntry|RegistryAccess$RegistryEntry]] -- calls:4 -- by fabric-registry-sync-v0, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.core.RegistrySetBuilder|RegistrySetBuilder]] -- calls:5, reads:6 -- by fabric-data-generation-api-v1
- [[40-Interfaces/net.minecraft.core.RegistrySynchronization|RegistrySynchronization]] -- injects_into:2, reads:3, writes:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.core.SectionPos|SectionPos]] -- calls:13 -- by fabric-block-getter-api-v2
- [[40-Interfaces/net.minecraft.core.Vec3i|Vec3i]] -- calls:6 -- by fabric-networking-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.core.WritableRegistry|WritableRegistry]] -- calls:5 -- by fabric-gametest-api-v1, fabric-registry-sync-v0, fabric-tag-api-v1

## Declared inventory

### `net.minecraft.core` (39 top-level)

`AxisCycle`, `BlockBox`, [[40-Interfaces/net.minecraft.core.BlockMath|BlockMath]], [[40-Interfaces/net.minecraft.core.BlockPos|BlockPos]], `ClientAsset`, `Cloner`, `CompositeDirection`, `Cursor3D`, [[40-Interfaces/net.minecraft.core.DefaultedMappedRegistry|DefaultedMappedRegistry]], [[40-Interfaces/net.minecraft.core.DefaultedRegistry|DefaultedRegistry]], [[40-Interfaces/net.minecraft.core.Direction|Direction]], `Directional`, `FrontAndTop`, `GlobalPos`, [[40-Interfaces/net.minecraft.core.Holder|Holder]], [[40-Interfaces/net.minecraft.core.HolderGetter|HolderGetter]], [[40-Interfaces/net.minecraft.core.HolderLookup|HolderLookup]], `HolderOwner`, [[40-Interfaces/net.minecraft.core.HolderSet|HolderSet]], `IdMap`, [[40-Interfaces/net.minecraft.core.IdMapper|IdMapper]], [[40-Interfaces/net.minecraft.core.LayeredRegistryAccess|LayeredRegistryAccess]], [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]], [[40-Interfaces/net.minecraft.core.NonNullList|NonNullList]], `Position`, `PositionAndRotation`, `QuartPos`, [[40-Interfaces/net.minecraft.core.RegistrationInfo|RegistrationInfo]], [[40-Interfaces/net.minecraft.core.Registry|Registry]], [[40-Interfaces/net.minecraft.core.RegistryAccess|RegistryAccess]], [[40-Interfaces/net.minecraft.core.RegistrySetBuilder|RegistrySetBuilder]], [[40-Interfaces/net.minecraft.core.RegistrySynchronization|RegistrySynchronization]], `Rotations`, [[40-Interfaces/net.minecraft.core.SectionPos|SectionPos]], `TypedInstance`, `UUIDUtil`, [[40-Interfaces/net.minecraft.core.Vec3i|Vec3i]], [[40-Interfaces/net.minecraft.core.WritableRegistry|WritableRegistry]], `package-info`

