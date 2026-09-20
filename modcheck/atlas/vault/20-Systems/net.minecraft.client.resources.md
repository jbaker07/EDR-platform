---
type: "system"
package: "net.minecraft.client.resources"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources

205 classes (110 top-level) across 14 packages in the processed jar; 6 changed by Loom processing; 20 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.client.resources.language.ClientLanguage|ClientLanguage]] -- injects_into:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.resources.language.I18n|I18n]] -- calls:1 -- by fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.BlockStateModelLoader_LoadedModels|BlockStateModelLoader$LoadedModels]] -- calls:2 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.ModelBaker|ModelBaker]] -- calls:4 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.ModelBakery|ModelBakery]] -- injects_into:3, reads:4, wraps:2 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]] -- calls:1, injects_into:9, wraps:1 -- by fabric-model-loading-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.ResolvableModel_Resolver|ResolvableModel$Resolver]] -- calls:1 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.ResolvedModel|ResolvedModel]] -- calls:5 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.SimpleModelWrapper|SimpleModelWrapper]] -- calls:1, injects_into:1, reads:2, wraps:1 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.UnbakedModel|UnbakedModel]] -- calls:6 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.cuboid.CuboidModel|CuboidModel]] -- injects_into:1 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.geometry.BakedQuad|BakedQuad]] -- calls:16 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.resources.model.geometry.BakedQuad_MaterialInfo|BakedQuad$MaterialInfo]] -- calls:13 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.resources.model.geometry.ItemQuads|ItemQuads]] -- calls:5 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.geometry.QuadCollection|QuadCollection]] -- calls:2 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.sprite.AtlasManager|AtlasManager]] -- calls:1, injects_into:1 -- by fabric-particles-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.resources.model.sprite.AtlasManager_AtlasConfig|AtlasManager$AtlasConfig]] -- calls:8 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.resources.model.sprite.Material_Baked|Material$Baked]] -- calls:6 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.resources.model.sprite.MaterialBaker|MaterialBaker]] -- calls:2, reads:2 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.resources.sounds.SoundInstance|SoundInstance]] -- calls:1 -- by fabric-sound-api-v1

## Declared inventory

### `net.minecraft.client.resources` (13 top-level)

`ClientPackSource`, `DefaultPlayerSkin`, `DryFoliageColorReloadListener`, `FoliageColorReloadListener`, `GrassColorReloadListener`, `IndexedAssetSource`, `LegacyStuffWrapper`, `MapTextureManager`, `SkinManager`, `SplashManager`, `WaypointStyle`, `WaypointStyleManager`, `package-info`

### `net.minecraft.client.resources.language` (7 top-level)

[[40-Interfaces/net.minecraft.client.resources.language.ClientLanguage|ClientLanguage]], `EmptyTranslationsException`, `FormattedBidiReorder`, [[40-Interfaces/net.minecraft.client.resources.language.I18n|I18n]], `LanguageInfo`, `LanguageManager`, `package-info`

### `net.minecraft.client.resources.metadata` (1 top-level)

`package-info`

### `net.minecraft.client.resources.metadata.animation` (5 top-level)

`AnimationFrame`, `AnimationMetadataSection`, `FrameSize`, `VillagerMetadataSection`, `package-info`

### `net.minecraft.client.resources.metadata.gui` (3 top-level)

`GuiMetadataSection`, `GuiSpriteScaling`, `package-info`

### `net.minecraft.client.resources.metadata.language` (2 top-level)

`LanguageMetadataSection`, `package-info`

### `net.minecraft.client.resources.metadata.texture` (3 top-level)

`PaletteMetadataSection`, `TextureMetadataSection`, `package-info`

### `net.minecraft.client.resources.model` (16 top-level)

`BlockStateDefinitions`, `BlockStateModelLoader`, `ClientItemInfoLoader`, `EquipmentAssetManager`, `EquipmentClientInfo`, [[40-Interfaces/net.minecraft.client.resources.model.ModelBaker|ModelBaker]], [[40-Interfaces/net.minecraft.client.resources.model.ModelBakery|ModelBakery]], `ModelDebugName`, `ModelDiscovery`, `ModelGroupCollector`, [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]], `ResolvableModel`, [[40-Interfaces/net.minecraft.client.resources.model.ResolvedModel|ResolvedModel]], [[40-Interfaces/net.minecraft.client.resources.model.SimpleModelWrapper|SimpleModelWrapper]], [[40-Interfaces/net.minecraft.client.resources.model.UnbakedModel|UnbakedModel]], `package-info`

### `net.minecraft.client.resources.model.cuboid` (11 top-level)

`CuboidFace`, [[40-Interfaces/net.minecraft.client.resources.model.cuboid.CuboidModel|CuboidModel]], `CuboidModelElement`, `CuboidRotation`, `FaceBakery`, `ItemModelGenerator`, `ItemTransform`, `ItemTransforms`, `MissingCuboidModel`, `UnbakedCuboidGeometry`, `package-info`

### `net.minecraft.client.resources.model.geometry` (5 top-level)

[[40-Interfaces/net.minecraft.client.resources.model.geometry.BakedQuad|BakedQuad]], [[40-Interfaces/net.minecraft.client.resources.model.geometry.ItemQuads|ItemQuads]], [[40-Interfaces/net.minecraft.client.resources.model.geometry.QuadCollection|QuadCollection]], `UnbakedGeometry`, `package-info`

### `net.minecraft.client.resources.model.sprite` (7 top-level)

[[40-Interfaces/net.minecraft.client.resources.model.sprite.AtlasManager|AtlasManager]], `Material`, [[40-Interfaces/net.minecraft.client.resources.model.sprite.MaterialBaker|MaterialBaker]], `SpriteGetter`, `SpriteId`, `TextureSlots`, `package-info`

### `net.minecraft.client.resources.palette` (5 top-level)

`Palette`, `PaletteMapping`, `PaletteMappingCache`, `PalettedTextureManager`, `package-info`

### `net.minecraft.client.resources.server` (6 top-level)

`DownloadedPackSource`, `PackDownloader`, `PackLoadFeedback`, `PackReloadConfig`, `ServerPackManager`, `package-info`

### `net.minecraft.client.resources.sounds` (26 top-level)

`AbstractSoundInstance`, `AbstractTickableSoundInstance`, `AmbientSoundHandler`, `BeeAggressiveSoundInstance`, `BeeFlyingSoundInstance`, `BeeSoundInstance`, `BiomeAmbientSoundsHandler`, `BubbleColumnAmbientSoundHandler`, `DirectionalSoundInstance`, `ElytraOnPlayerSoundInstance`, `EntityBoundSoundInstance`, `GuardianAttackSoundInstance`, `MinecartSoundInstance`, `RidingEntitySoundInstance`, `RidingMinecartSoundInstance`, `SimpleSoundInstance`, `SnifferSoundInstance`, `Sound`, `SoundEventRegistration`, `SoundEventRegistrationSerializer`, [[40-Interfaces/net.minecraft.client.resources.sounds.SoundInstance|SoundInstance]], `TickableSoundInstance`, `UnderLiquidAmbientSoundInstance`, `UnderLiquidSubSound`, `UnderwaterAmbientSoundHandler`, `package-info`

