---
type: "interface"
fqcn: "net.minecraft.client.renderer.Sheets"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.Sheets

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `cutoutBlockItemGlintSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@119 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `cutoutBlockItemGlintSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@147 in `AoCalculator.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutBlockItemGlintSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@39 in `ItemGlintRenderType.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutBlockItemGlintSpecialSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@137 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `cutoutBlockItemGlintSpecialSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@150 in `AoCalculator.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutBlockItemGlintSpecialSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@39 in `ItemGlintSpecialRenderType.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutBlockItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@101 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `cutoutBlockItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@10 in `ChunkSectionLayerHelper.getRenderType` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `cutoutBlockItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@19 in `BlockModelRenderStateMixin.setupMesh` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `cutoutBlockItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@144 in `AoCalculator.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutBlockItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@39 in `ItemRenderType.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutItemGlintSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@176 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `cutoutItemGlintSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@7 in `ItemGlintRenderType.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutItemGlintSpecialSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@194 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `cutoutItemGlintSpecialSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@7 in `ItemGlintSpecialRenderType.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@158 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `cutoutItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@7 in `ItemRenderType.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucentBlockItemGlintSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@113 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucentBlockItemGlintSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@55 in `ItemGlintRenderType.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucentBlockItemGlintSpecialSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@131 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucentBlockItemGlintSpecialSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@55 in `ItemGlintSpecialRenderType.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucentBlockItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@95 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucentBlockItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@4 in `ChunkSectionLayerHelper.getRenderType` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucentBlockItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@13 in `BlockModelRenderStateMixin.setupMesh` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucentBlockItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@55 in `ItemRenderType.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucentItemGlintSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@170 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucentItemGlintSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@23 in `ItemGlintRenderType.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucentItemGlintSpecialSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@188 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucentItemGlintSpecialSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@23 in `ItemGlintSpecialRenderType.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucentItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@152 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucentItemSheet` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@23 in `ItemRenderType.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (45 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SHULKER_SHEET : Lnet/minecraft/resources/Identifier;
public static final BANNER_SHEET : Lnet/minecraft/resources/Identifier;
public static final SHIELD_SHEET : Lnet/minecraft/resources/Identifier;
public static final CHEST_SHEET : Lnet/minecraft/resources/Identifier;
public static final DECORATED_POT_SHEET : Lnet/minecraft/resources/Identifier;
public static final GUI_SHEET : Lnet/minecraft/resources/Identifier;
public static final MAP_DECORATIONS_SHEET : Lnet/minecraft/resources/Identifier;
public static final PAINTINGS_SHEET : Lnet/minecraft/resources/Identifier;
public static final CELESTIAL_SHEET : Lnet/minecraft/resources/Identifier;
private static final CUTOUT_BLOCK_ITEM_SHEET : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final CUTOUT_BLOCK_ITEM_GLINT_SHEET : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final CUTOUT_BLOCK_ITEM_GLINT_SPECIAL_SHEET : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final TRANSLUCENT_BLOCK_ITEM_SHEET : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final TRANSLUCENT_BLOCK_ITEM_GLINT_SHEET : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final TRANSLUCENT_BLOCK_ITEM_GLINT_SPECIAL_SHEET : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final CUTOUT_ITEM_SHEET : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final CUTOUT_ITEM_GLINT_SHEET : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final CUTOUT_ITEM_GLINT_SPECIAL_SHEET : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final TRANSLUCENT_ITEM_SHEET : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final TRANSLUCENT_ITEM_GLINT_SHEET : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final TRANSLUCENT_ITEM_GLINT_SPECIAL_SHEET : Lnet/minecraft/client/renderer/rendertype/RenderType;
public static final ITEMS_MAPPER : Lnet/minecraft/client/renderer/SpriteMapper;
public static final BLOCKS_MAPPER : Lnet/minecraft/client/renderer/SpriteMapper;
public static final BLOCK_ENTITIES_MAPPER : Lnet/minecraft/client/renderer/SpriteMapper;
public static final BANNER_MAPPER : Lnet/minecraft/client/renderer/SpriteMapper;
public static final SHIELD_MAPPER : Lnet/minecraft/client/renderer/SpriteMapper;
public static final CHEST_MAPPER : Lnet/minecraft/client/renderer/SpriteMapper;
public static final DECORATED_POT_MAPPER : Lnet/minecraft/client/renderer/SpriteMapper;
public static final SHULKER_MAPPER : Lnet/minecraft/client/renderer/SpriteMapper;
public static final DEFAULT_SHULKER_TEXTURE_LOCATION : Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static final SHULKER_TEXTURE_LOCATION : Ljava/util/List;
public static final BANNER_BASE : Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static final SHIELD_BASE : Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static final SHIELD_BASE_NO_PATTERN : Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static final BANNER_PATTERN_BASE : Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static final SHIELD_PATTERN_BASE : Lnet/minecraft/client/resources/model/sprite/SpriteId;
private static final BANNER_SPRITES : Ljava/util/Map;
private static final SHIELD_SPRITES : Ljava/util/Map;
public static final DECORATED_POT_BASE : Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static final DECORATED_POT_SIDE : Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static final ENDER_CHEST_LOCATION : Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static final CHEST_REGULAR : Lnet/minecraft/client/renderer/MultiblockChestResources;
public static final CHEST_TRAPPED : Lnet/minecraft/client/renderer/MultiblockChestResources;
public static final CHEST_CHRISTMAS : Lnet/minecraft/client/renderer/MultiblockChestResources;
public static final CHEST_COPPER : Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
public <init>()V
public static cutoutBlockItemSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static cutoutBlockItemGlintSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static cutoutBlockItemGlintSpecialSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static cutoutItemSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static cutoutItemGlintSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static cutoutItemGlintSpecialSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static translucentItemSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static translucentItemGlintSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static translucentItemGlintSpecialSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static translucentBlockItemSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static translucentBlockItemGlintSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static translucentBlockItemGlintSpecialSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static getShulkerBoxSprite(Lnet/minecraft/world/item/DyeColor;)Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static colorToShulkerSprite(Lnet/minecraft/world/item/DyeColor;)Lnet/minecraft/resources/Identifier;
public static createShulkerSprite(Lnet/minecraft/world/item/DyeColor;)Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static getBannerSprite(Lnet/minecraft/core/Holder;)Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static getShieldSprite(Lnet/minecraft/core/Holder;)Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static chooseSprite(Lnet/minecraft/client/renderer/blockentity/state/ChestRenderState$ChestMaterialType;Lnet/minecraft/world/level/block/state/properties/ChestType;)Lnet/minecraft/client/resources/model/sprite/SpriteId;
private static synthetic lambda$static$0(Lnet/minecraft/client/renderer/MultiblockChestResources;)Lnet/minecraft/client/renderer/MultiblockChestResources;
static <clinit>()V
```
