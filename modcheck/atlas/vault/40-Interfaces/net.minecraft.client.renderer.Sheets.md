---
type: "interface"
fqcn: "net.minecraft.client.renderer.Sheets"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.Sheets

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `cutoutBlockItemGlintSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutBlockItemGlintSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutBlockItemGlintSpecialSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutBlockItemGlintSpecialSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutBlockItemSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `cutoutBlockItemSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutBlockItemSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutItemGlintSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutItemGlintSpecialSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cutoutItemSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucentBlockItemGlintSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucentBlockItemGlintSpecialSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucentBlockItemSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucentBlockItemSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucentItemGlintSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucentItemGlintSpecialSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucentItemSheet()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (66, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.Sheets {
    public static final net.minecraft.resources.Identifier SHULKER_SHEET;
    public static final net.minecraft.resources.Identifier BANNER_SHEET;
    public static final net.minecraft.resources.Identifier SHIELD_SHEET;
    public static final net.minecraft.resources.Identifier CHEST_SHEET;
    public static final net.minecraft.resources.Identifier DECORATED_POT_SHEET;
    public static final net.minecraft.resources.Identifier GUI_SHEET;
    public static final net.minecraft.resources.Identifier MAP_DECORATIONS_SHEET;
    public static final net.minecraft.resources.Identifier PAINTINGS_SHEET;
    public static final net.minecraft.resources.Identifier CELESTIAL_SHEET;
    private static final net.minecraft.client.renderer.rendertype.RenderType CUTOUT_BLOCK_ITEM_SHEET;
    private static final net.minecraft.client.renderer.rendertype.RenderType CUTOUT_BLOCK_ITEM_GLINT_SHEET;
    private static final net.minecraft.client.renderer.rendertype.RenderType CUTOUT_BLOCK_ITEM_GLINT_SPECIAL_SHEET;
    private static final net.minecraft.client.renderer.rendertype.RenderType TRANSLUCENT_BLOCK_ITEM_SHEET;
    private static final net.minecraft.client.renderer.rendertype.RenderType TRANSLUCENT_BLOCK_ITEM_GLINT_SHEET;
    private static final net.minecraft.client.renderer.rendertype.RenderType TRANSLUCENT_BLOCK_ITEM_GLINT_SPECIAL_SHEET;
    private static final net.minecraft.client.renderer.rendertype.RenderType CUTOUT_ITEM_SHEET;
    private static final net.minecraft.client.renderer.rendertype.RenderType CUTOUT_ITEM_GLINT_SHEET;
    private static final net.minecraft.client.renderer.rendertype.RenderType CUTOUT_ITEM_GLINT_SPECIAL_SHEET;
    private static final net.minecraft.client.renderer.rendertype.RenderType TRANSLUCENT_ITEM_SHEET;
    private static final net.minecraft.client.renderer.rendertype.RenderType TRANSLUCENT_ITEM_GLINT_SHEET;
    private static final net.minecraft.client.renderer.rendertype.RenderType TRANSLUCENT_ITEM_GLINT_SPECIAL_SHEET;
    public static final net.minecraft.client.renderer.SpriteMapper ITEMS_MAPPER;
    public static final net.minecraft.client.renderer.SpriteMapper BLOCKS_MAPPER;
    public static final net.minecraft.client.renderer.SpriteMapper BLOCK_ENTITIES_MAPPER;
    public static final net.minecraft.client.renderer.SpriteMapper BANNER_MAPPER;
    public static final net.minecraft.client.renderer.SpriteMapper SHIELD_MAPPER;
    public static final net.minecraft.client.renderer.SpriteMapper CHEST_MAPPER;
    public static final net.minecraft.client.renderer.SpriteMapper DECORATED_POT_MAPPER;
    public static final net.minecraft.client.renderer.SpriteMapper SHULKER_MAPPER;
    public static final net.minecraft.client.resources.model.sprite.SpriteId DEFAULT_SHULKER_TEXTURE_LOCATION;
    public static final java.util.List<net.minecraft.client.resources.model.sprite.SpriteId> SHULKER_TEXTURE_LOCATION;
    public static final net.minecraft.client.resources.model.sprite.SpriteId BANNER_BASE;
    public static final net.minecraft.client.resources.model.sprite.SpriteId SHIELD_BASE;
    public static final net.minecraft.client.resources.model.sprite.SpriteId SHIELD_BASE_NO_PATTERN;
    public static final net.minecraft.client.resources.model.sprite.SpriteId BANNER_PATTERN_BASE;
    public static final net.minecraft.client.resources.model.sprite.SpriteId SHIELD_PATTERN_BASE;
    private static final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.resources.model.sprite.SpriteId> BANNER_SPRITES;
    private static final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.resources.model.sprite.SpriteId> SHIELD_SPRITES;
    public static final net.minecraft.client.resources.model.sprite.SpriteId DECORATED_POT_BASE;
    public static final net.minecraft.client.resources.model.sprite.SpriteId DECORATED_POT_SIDE;
    public static final net.minecraft.client.resources.model.sprite.SpriteId ENDER_CHEST_LOCATION;
    public static final net.minecraft.client.renderer.MultiblockChestResources<net.minecraft.client.resources.model.sprite.SpriteId> CHEST_REGULAR;
    public static final net.minecraft.client.renderer.MultiblockChestResources<net.minecraft.client.resources.model.sprite.SpriteId> CHEST_TRAPPED;
    public static final net.minecraft.client.renderer.MultiblockChestResources<net.minecraft.client.resources.model.sprite.SpriteId> CHEST_CHRISTMAS;
    public static final net.minecraft.world.level.block.WeatheringCopperCollection$ByState<net.minecraft.client.renderer.MultiblockChestResources<net.minecraft.client.resources.model.sprite.SpriteId>> CHEST_COPPER;
    public net.minecraft.client.renderer.Sheets();
    public static net.minecraft.client.renderer.rendertype.RenderType cutoutBlockItemSheet();
    public static net.minecraft.client.renderer.rendertype.RenderType cutoutBlockItemGlintSheet();
    public static net.minecraft.client.renderer.rendertype.RenderType cutoutBlockItemGlintSpecialSheet();
    public static net.minecraft.client.renderer.rendertype.RenderType cutoutItemSheet();
    public static net.minecraft.client.renderer.rendertype.RenderType cutoutItemGlintSheet();
    public static net.minecraft.client.renderer.rendertype.RenderType cutoutItemGlintSpecialSheet();
    public static net.minecraft.client.renderer.rendertype.RenderType translucentItemSheet();
    public static net.minecraft.client.renderer.rendertype.RenderType translucentItemGlintSheet();
    public static net.minecraft.client.renderer.rendertype.RenderType translucentItemGlintSpecialSheet();
    public static net.minecraft.client.renderer.rendertype.RenderType translucentBlockItemSheet();
    public static net.minecraft.client.renderer.rendertype.RenderType translucentBlockItemGlintSheet();
    public static net.minecraft.client.renderer.rendertype.RenderType translucentBlockItemGlintSpecialSheet();
    public static net.minecraft.client.resources.model.sprite.SpriteId getShulkerBoxSprite(net.minecraft.world.item.DyeColor);
    public static net.minecraft.resources.Identifier colorToShulkerSprite(net.minecraft.world.item.DyeColor);
    public static net.minecraft.client.resources.model.sprite.SpriteId createShulkerSprite(net.minecraft.world.item.DyeColor);
    public static net.minecraft.client.resources.model.sprite.SpriteId getBannerSprite(net.minecraft.core.Holder<net.minecraft.world.level.block.entity.BannerPattern>);
    public static net.minecraft.client.resources.model.sprite.SpriteId getShieldSprite(net.minecraft.core.Holder<net.minecraft.world.level.block.entity.BannerPattern>);
    public static net.minecraft.client.resources.model.sprite.SpriteId chooseSprite(net.minecraft.client.renderer.blockentity.state.ChestRenderState$ChestMaterialType, net.minecraft.world.level.block.state.properties.ChestType);
    private static net.minecraft.client.renderer.MultiblockChestResources lambda$static$0(net.minecraft.client.renderer.MultiblockChestResources);
    static {};
}
```
