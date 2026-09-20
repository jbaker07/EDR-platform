---
type: "scope"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Processed jar versus cache jar

What Loom did to Mojang's merged jar before the reference project compiled against it. `from`/`to` are JVM access flags.

## Hierarchy changes (interfaces injected)

| class | interfaces added |
|---|---|
| [[40-Interfaces/com.mojang.renderpearl.api.pipeline.RenderPipeline_Builder|RenderPipeline$Builder]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderPipeline$Builder` |
| [[40-Interfaces/com.mojang.renderpearl.api.pipeline.RenderPipeline_Snippet|RenderPipeline$Snippet]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderPipeline$Snippet` |
| [[40-Interfaces/com.mojang.renderpearl.api.pipeline.RenderPipeline|RenderPipeline]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderPipeline` |
| [[40-Interfaces/net.minecraft.advancements.Advancement_Builder|Advancement$Builder]] | `net/fabricmc/fabric/api/advancement/v1/FabricAdvancementBuilder`, `net/fabricmc/fabric/api/datagen/v1/advancement/FabricAdvancementBuilder` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]] | `net/fabricmc/fabric/api/client/creativetab/v1/FabricCreativeModeInventoryScreen` |
| [[40-Interfaces/net.minecraft.client.model.Model|Model]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricModel` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl|ClientCommonPacketListenerImpl]] | `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl|ClientHandshakePacketListenerImpl]] | `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider` |
| [[40-Interfaces/net.minecraft.client.player.LocalPlayer|LocalPlayer]] | `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider` |
| [[40-Interfaces/net.minecraft.client.renderer.OrderedSubmitNodeCollector|OrderedSubmitNodeCollector]] | `net/fabricmc/fabric/api/client/renderer/v1/render/FabricOrderedSubmitNodeCollector`, `net/fabricmc/fabric/api/client/rendering/v1/FabricOrderedSubmitNodeCollector` |
| [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelRenderState|BlockModelRenderState]] | `net/fabricmc/fabric/api/client/renderer/v1/render/FabricBlockModelRenderState`, `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.block.BlockStateModelSet|BlockStateModelSet]] | `net/fabricmc/fabric/api/client/renderer/v1/model/FabricBlockStateModelSet` |
| [[40-Interfaces/net.minecraft.client.renderer.block.MovingBlockRenderState|MovingBlockRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModel|BlockStateModel]] | `net/fabricmc/fabric/api/client/renderer/v1/model/FabricBlockStateModel` |
| [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModelPart|BlockStateModelPart]] | `net/fabricmc/fabric/api/client/renderer/v1/model/FabricBlockStateModelPart` |
| `net.minecraft.client.renderer.blockentity.state.BlockEntityRenderState` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| `net.minecraft.client.renderer.entity.state.EntityRenderState$LeashState` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| `net.minecraft.client.renderer.entity.state.EntityRenderState` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| `net.minecraft.client.renderer.fog.FogData` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState_LayerRenderState|ItemStackRenderState$LayerRenderState]] | `net/fabricmc/fabric/api/client/renderer/v1/render/FabricLayerRenderState`, `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState|ItemStackRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| `net.minecraft.client.renderer.state.GameRenderState` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| `net.minecraft.client.renderer.state.LightmapRenderState` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| `net.minecraft.client.renderer.state.MapRenderState$MapDecorationRenderState` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| `net.minecraft.client.renderer.state.MapRenderState` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.state.OptionsRenderState|OptionsRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| `net.minecraft.client.renderer.state.WindowRenderState` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.state.gui.GuiRenderState|GuiRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| `net.minecraft.client.renderer.state.gui.PanoramaRenderState` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.BlockBreakingRenderState|BlockBreakingRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| `net.minecraft.client.renderer.state.level.BlockOutlineRenderState` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| `net.minecraft.client.renderer.state.level.CameraEntityRenderState` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.CameraRenderState|CameraRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState|FirstPersonHandsAndItemsRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.LevelRenderState|LevelRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.ParticlesRenderState|ParticlesRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| `net.minecraft.client.renderer.state.level.PlayerRenderState$ItemActivationRenderState` | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.PlayerRenderState|PlayerRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.SkyRenderState|SkyRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.WeatherRenderState|WeatherRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.WorldBorderRenderState|WorldBorderRenderState]] | `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState` |
| [[40-Interfaces/net.minecraft.client.renderer.texture.SpriteLoader_Preparations|SpriteLoader$Preparations]] | `net/fabricmc/fabric/api/client/renderer/v1/sprite/FabricPreparations` |
| [[40-Interfaces/net.minecraft.client.renderer.texture.TextureAtlas|TextureAtlas]] | `net/fabricmc/fabric/api/client/renderer/v1/sprite/FabricTextureAtlas` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]] | `net/fabricmc/fabric/api/client/model/loading/v1/FabricModelManager` |
| [[40-Interfaces/net.minecraft.client.resources.model.sprite.MaterialBaker|MaterialBaker]] | `net/fabricmc/fabric/api/client/renderer/v1/sprite/FabricMaterialBaker` |
| [[40-Interfaces/net.minecraft.client.resources.sounds.SoundInstance|SoundInstance]] | `net/fabricmc/fabric/api/client/sound/v1/FabricSoundInstance` |
| [[40-Interfaces/net.minecraft.commands.CommandSourceStack|CommandSourceStack]] | `net/fabricmc/fabric/api/permission/v1/PermissionContextOwner` |
| [[40-Interfaces/net.minecraft.commands.arguments.selector.EntitySelectorParser|EntitySelectorParser]] | `net/fabricmc/fabric/api/command/v2/FabricEntitySelectorParser` |
| [[40-Interfaces/net.minecraft.core.Registry|Registry]] | `net/fabricmc/fabric/api/event/registry/FabricRegistry` |
| [[40-Interfaces/net.minecraft.core.component.DataComponentMap_Builder|DataComponentMap$Builder]] | `net/fabricmc/fabric/api/item/v1/FabricComponentMapBuilder` |
| [[40-Interfaces/net.minecraft.core.particles.BlockParticleOption|BlockParticleOption]] | `net/fabricmc/fabric/api/particle/v1/FabricBlockParticleOption` |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `net/fabricmc/fabric/api/datagen/v1/loot/FabricBlockLootSubProvider` |
| [[40-Interfaces/net.minecraft.data.loot.EntityLootSubProvider|EntityLootSubProvider]] | `net/fabricmc/fabric/api/datagen/v1/loot/FabricEntityLootSubProvider` |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeOutput|RecipeOutput]] | `net/fabricmc/fabric/api/datagen/v1/recipe/FabricRecipeOutput` |
| [[40-Interfaces/net.minecraft.data.tags.TagAppender|TagAppender]] | `net/fabricmc/fabric/api/datagen/v1/provider/FabricTagAppender` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]] | `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `net/fabricmc/fabric/api/attachment/v1/GlobalAttachmentsProvider`, `net/fabricmc/fabric/api/resource/v1/DataResourceStore` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]] | `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider` |
| [[40-Interfaces/net.minecraft.server.network.ServerCommonPacketListenerImpl|ServerCommonPacketListenerImpl]] | `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider` |
| [[40-Interfaces/net.minecraft.server.network.ServerConfigurationPacketListenerImpl|ServerConfigurationPacketListenerImpl]] | `net/fabricmc/fabric/api/networking/v1/FabricServerConfigurationPacketListenerImpl` |
| [[40-Interfaces/net.minecraft.server.network.ServerHandshakePacketListenerImpl|ServerHandshakePacketListenerImpl]] | `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider` |
| [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]] | `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider` |
| [[40-Interfaces/net.minecraft.server.packs.resources.Resource|Resource]] | `net/fabricmc/fabric/api/resource/v1/FabricResource` |
| [[40-Interfaces/net.minecraft.tags.TagFile|TagFile]] | `net/fabricmc/fabric/api/tag/v1/FabricTagFile` |
| [[40-Interfaces/net.minecraft.tags.TagKey|TagKey]] | `net/fabricmc/fabric/api/tag/FabricTagKey` |
| [[40-Interfaces/net.minecraft.world.MenuProvider|MenuProvider]] | `net/fabricmc/fabric/api/menu/v1/FabricMenuProvider` |
| [[40-Interfaces/net.minecraft.world.effect.MobEffect|MobEffect]] | `net/fabricmc/fabric/api/entity/event/v1/effect/FabricMobEffect` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]] | `net/fabricmc/fabric/api/attachment/v1/AttachmentTarget`, `net/fabricmc/fabric/api/event/lifecycle/v1/EntityLoadData`, `net/fabricmc/fabric/api/permission/v1/PermissionContextOwner` |
| [[40-Interfaces/net.minecraft.world.entity.EntityType_Builder|EntityType$Builder]] | `net/fabricmc/fabric/api/object/builder/v1/entity/FabricEntityType$Builder` |
| [[40-Interfaces/net.minecraft.world.item.Item_Properties|Item$Properties]] | `net/fabricmc/fabric/api/item/v1/FabricItem$Properties` |
| [[40-Interfaces/net.minecraft.world.item.Item|Item]] | `net/fabricmc/fabric/api/item/v1/FabricItem` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]] | `net/fabricmc/fabric/api/item/v1/FabricItemStack` |
| [[40-Interfaces/net.minecraft.world.item.TooltipFlag|TooltipFlag]] | `net/fabricmc/fabric/api/item/v1/FabricTooltipFlag` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]] | `net/fabricmc/fabric/api/recipe/v1/ingredient/FabricIngredient` |
| `net.minecraft.world.item.crafting.RecipeAccess` | `net/fabricmc/fabric/api/recipe/v1/FabricRecipeAccess` |
| [[40-Interfaces/net.minecraft.world.item.crafting.RecipeManager|RecipeManager]] | `net/fabricmc/fabric/api/recipe/v1/FabricRecipeManager` |
| [[40-Interfaces/net.minecraft.world.level.BlockGetter|BlockGetter]] | `net/fabricmc/fabric/api/blockgetter/v2/FabricBlockGetter` |
| [[40-Interfaces/net.minecraft.world.level.Level|Level]] | `net/fabricmc/fabric/api/attachment/v1/AttachmentTarget`, `net/fabricmc/fabric/api/attachment/v1/GlobalAttachmentsProvider` |
| [[40-Interfaces/net.minecraft.world.level.block.Block|Block]] | `net/fabricmc/fabric/api/block/v1/FabricBlock` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntity|BlockEntity]] | `net/fabricmc/fabric/api/attachment/v1/AttachmentTarget`, `net/fabricmc/fabric/api/blockgetter/v2/RenderDataBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntityType|BlockEntityType]] | `net/fabricmc/fabric/api/object/builder/v1/block/entity/FabricBlockEntityType` |
| [[40-Interfaces/net.minecraft.world.level.block.state.BlockBehaviour_Properties|BlockBehaviour$Properties]] | `net/fabricmc/fabric/api/block/v1/FabricBlock$FabricProperties` |
| [[40-Interfaces/net.minecraft.world.level.block.state.BlockState|BlockState]] | `net/fabricmc/fabric/api/block/v1/FabricBlockState` |
| [[40-Interfaces/net.minecraft.world.level.chunk.ChunkAccess|ChunkAccess]] | `net/fabricmc/fabric/api/attachment/v1/AttachmentTarget` |
| [[40-Interfaces/net.minecraft.world.level.storage.ValueInput|ValueInput]] | `net/fabricmc/fabric/api/serialization/v1/value/FabricValueInput` |
| [[40-Interfaces/net.minecraft.world.level.storage.ValueOutput|ValueOutput]] | `net/fabricmc/fabric/api/serialization/v1/value/FabricValueOutput` |
| [[40-Interfaces/net.minecraft.world.level.storage.loot.LootPool_Builder|LootPool$Builder]] | `net/fabricmc/fabric/api/loot/v3/FabricLootPoolBuilder` |
| [[40-Interfaces/net.minecraft.world.level.storage.loot.LootTable_Builder|LootTable$Builder]] | `net/fabricmc/fabric/api/loot/v3/FabricLootTableBuilder` |

## Class access changes

| class | from | to |
|---|---|---|
| `net.minecraft.client.data.models.BlockModelGenerators$BlockFamilyProvider` |  | public |
| `net.minecraft.client.data.models.BlockModelGenerators$PlantType` | final | public final |
| `net.minecraft.client.data.models.BlockModelGenerators$WoodProvider` |  | public |
| `net.minecraft.client.gui.GuiGraphicsExtractor$ScissorStack` |  | public |
| [[40-Interfaces/net.minecraft.client.gui.screens.MenuScreens_ScreenConstructor|MenuScreens$ScreenConstructor]] | abstract | public abstract |
| `net.minecraft.client.renderer.block.BuiltInBlockModels$Builder` |  | public |
| `net.minecraft.client.renderer.block.BuiltInBlockModels$ModelFactory` | abstract | public abstract |
| `net.minecraft.client.renderer.block.BuiltInBlockModels$SpecialModelFactory` | abstract | public abstract |
| `net.minecraft.world.inventory.MenuType$MenuSupplier` | abstract | public abstract |
| `net.minecraft.world.item.enchantment.Enchantment$FloatAction` | abstract | public abstract |
| `net.minecraft.world.item.enchantment.Enchantment$GenericAction` | abstract | public abstract |
| `net.minecraft.world.item.enchantment.EnchantmentHelper$EnchantmentInSlotVisitor` | abstract | public abstract |
| `net.minecraft.world.item.enchantment.EnchantmentHelper$EnchantmentVisitor` | abstract | public abstract |

## Member access changes (771)

| class | member | from | to |
|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `missTime` | protected | public |
| `net.minecraft.client.color.item.ItemTintSources` | `ID_MAPPER` | private static final | public static final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `blockStateOutput` | private final | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `itemModelOutput` | private final | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `modelOutput` | private final | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `ROTATION_FACING` | private static final | public static final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `ROTATIONS_COLUMN_WITH_FACING` | private static final | public static final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `ROTATION_TORCH` | private static final | public static final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `ROTATION_HORIZONTAL_FACING_ALT` | private static final | public static final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `ROTATION_HORIZONTAL_FACING` | private static final | public static final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `plainModel(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/block/dispatch/Variant;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `variant(Lnet/minecraft/client/renderer/block/dispatch/Variant;)Lnet/minecraft/client/data/models/MultiVariant;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `variants([Lnet/minecraft/client/renderer/block/dispatch/Variant;)Lnet/minecraft/client/data/models/MultiVariant;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `plainVariant(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/data/models/MultiVariant;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `condition()Lnet/minecraft/client/data/models/blockstates/ConditionBuilder;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `condition(Lnet/minecraft/world/level/block/state/properties/EnumProperty;Ljava/lang/Enum;[Ljava/lang/Enum;)Lnet/minecraft/client/data/models/blockstates/ConditionBuilder;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `condition(Lnet/minecraft/world/level/block/state/properties/BooleanProperty;Z)Lnet/minecraft/client/data/models/blockstates/ConditionBuilder;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `or([Lnet/minecraft/client/data/models/blockstates/ConditionBuilder;)Lnet/minecraft/client/renderer/block/dispatch/multipart/Condition;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `and([Lnet/minecraft/client/data/models/blockstates/ConditionBuilder;)Lnet/minecraft/client/renderer/block/dispatch/multipart/Condition;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createMirroredCubeGenerator(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/renderer/block/dispatch/Variant;Lnet/minecraft/client/data/models/model/TextureMapping;Ljava/util/function/BiConsumer;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createNorthWestMirroredCubeGenerator(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/renderer/block/dispatch/Variant;Lnet/minecraft/client/data/models/model/TextureMapping;Ljava/util/function/BiConsumer;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createMirroredColumnGenerator(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/renderer/block/dispatch/Variant;Lnet/minecraft/client/data/models/model/TextureMapping;Ljava/util/function/BiConsumer;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `registerSimpleItemModel(Lnet/minecraft/world/item/Item;Lnet/minecraft/resources/Identifier;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `registerSimpleItemModel(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/resources/Identifier;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `registerSimpleTintedItemModel(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/color/item/ItemTintSource;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createFlatItemModel(Lnet/minecraft/world/item/Item;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createFlatItemModelWithBlockTexture(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createFlatItemModelWithBlockTexture(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/level/block/Block;Ljava/lang/String;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createFlatItemModel(Lnet/minecraft/world/item/Item;Lnet/minecraft/client/resources/model/sprite/Material;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createTwoLayeredItemModel(Lnet/minecraft/world/item/Item;Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/resources/model/sprite/Material;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `registerSimpleFlatItemModel(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `registerSimpleFlatItemModel(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `registerSimpleFlatItemModel(Lnet/minecraft/world/level/block/Block;Ljava/lang/String;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createRotatedVariants(Lnet/minecraft/client/renderer/block/dispatch/Variant;)Lnet/minecraft/client/data/models/MultiVariant;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createRotatedVariants(Lnet/minecraft/client/renderer/block/dispatch/Variant;Lnet/minecraft/client/renderer/block/dispatch/Variant;)Lnet/minecraft/client/data/models/MultiVariant;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createBooleanModelDispatch(Lnet/minecraft/world/level/block/state/properties/BooleanProperty;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/PropertyDispatch;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createRotatedMirroredVariantBlock(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createRotatedVariantBlock(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createRotatedVariantBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/model/TexturedModel$Provider;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createRotatedVariantBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/resources/Identifier;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createBrushableBlock(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createButton(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createDoor(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCustomFence(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createFence(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createWall(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createFenceGate(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Z)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createStairs(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createOrientableTrapdoor(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createTrapdoor(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createBed(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createStrawBed(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createSign(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createHangingSign(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createSimpleBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/MultiVariantGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createRotatedPillar()Lnet/minecraft/client/data/models/blockstates/PropertyDispatch;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createPillarBlockUVLocked(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/model/TextureMapping;Ljava/util/function/BiConsumer;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createAxisAlignedPillarBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createAxisAlignedPillarBlockCustomModel(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createAxisAlignedPillarBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/model/TexturedModel$Provider;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createHorizontallyRotatedBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/model/TexturedModel$Provider;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createRotatedPillarWithHorizontalVariant(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createRotatedPillarWithHorizontalVariant(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/model/TexturedModel$Provider;Lnet/minecraft/client/data/models/model/TexturedModel$Provider;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCreakingHeart(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCreakingHeartModel(Lnet/minecraft/client/data/models/model/TexturedModel$Provider;Lnet/minecraft/world/level/block/Block;Ljava/lang/String;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createSuffixedVariant(Lnet/minecraft/world/level/block/Block;Ljava/lang/String;Lnet/minecraft/client/data/models/model/ModelTemplate;Ljava/util/function/Function;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createPressurePlate(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createSlab(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createTrivialCube(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createTrivialBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/model/TexturedModel$Provider;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createItemWithGrassTint(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `family(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/client/data/models/BlockModelGenerators$BlockFamilyProvider;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createDoor(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `copyDoorModel(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createOrientableTrapdoor(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createTrapdoor(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `copyTrapdoorModel(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `woodProvider(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/client/data/models/BlockModelGenerators$WoodProvider;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createNonTemplateModelBlock(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createNonTemplateModelBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCrossBlockWithDefaultItem(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/BlockModelGenerators$PlantType;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCrossBlockWithDefaultItem(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/BlockModelGenerators$PlantType;Lnet/minecraft/client/data/models/model/TextureMapping;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCrossBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/BlockModelGenerators$PlantType;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCrossBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/BlockModelGenerators$PlantType;Lnet/minecraft/client/data/models/model/TextureMapping;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCrossBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/BlockModelGenerators$PlantType;Lnet/minecraft/world/level/block/state/properties/Property;[I)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createPlantWithDefaultItem(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/BlockModelGenerators$PlantType;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createPlant(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/BlockModelGenerators$PlantType;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCoralFans(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createStems(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCoral(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createDoublePlant(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/BlockModelGenerators$PlantType;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createDoublePlantWithDefaultItem(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/BlockModelGenerators$PlantType;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createTintedDoublePlant(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createDoubleBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createPassiveRail(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createActiveRail(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createAirLikeBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createAirLikeBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/resources/model/sprite/Material;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createParticleOnlyBlockModel(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/client/data/models/MultiVariant;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createParticleOnlyBlock(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createFullAndCarpetBlocks(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createLeafLitter(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createFlowerBed(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createSegmentedBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Ljava/util/function/Function;Lnet/minecraft/client/data/models/MultiVariant;Ljava/util/function/Function;Lnet/minecraft/client/data/models/MultiVariant;Ljava/util/function/Function;Lnet/minecraft/client/data/models/MultiVariant;Ljava/util/function/Function;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createColoredBlockWithRandomRotations(Lnet/minecraft/client/data/models/model/TexturedModel$Provider;Ljava/util/List;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createColoredBlockWithStateRotations(Lnet/minecraft/client/data/models/model/TexturedModel$Provider;Ljava/util/List;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createGlassBlocks(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCommandBlock(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createAnvil(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createBambooModels(I)Lnet/minecraft/client/data/models/MultiVariant;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createEmptyOrFullDispatch(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang/Comparable;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/PropertyDispatch;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createBeeNest(Lnet/minecraft/world/level/block/Block;Ljava/util/function/Function;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCropBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/state/properties/Property;[I)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createFurnace(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/model/TexturedModel$Provider;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCampfires([Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createAzalea(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createPottedAzalea(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createMushroomBlock(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCraftingTableLike(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Ljava/util/function/BiFunction;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createPumpkinVariant(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/model/TextureMapping;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createDispenserBlock(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCopperBulb(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCopperBulb(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `copyCopperBulbModel(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createAmethystCluster(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createSpeleothem(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createSpeleothemVariant(Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/block/state/properties/SpeleothemThickness;Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/client/data/models/MultiVariant;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createNyliumBlock(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createRotatableColumn(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createLightningRod(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createFarmland(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/data/models/model/ModelTemplate;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createFloorFireModels(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/client/data/models/MultiVariant;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createSideFireModels(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/client/data/models/MultiVariant;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createTopFireModels(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/client/data/models/MultiVariant;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createLantern(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCopperLantern(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCopperChain(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createGrassLikeBlock(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/MultiVariant;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createWeightedPressurePlate(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `copyModel(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createBarsAndItem(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createBarsAndItem(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createBars(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createNonTemplateHorizontalBlock(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createPistonVariant(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/MultiVariant;Lnet/minecraft/client/data/models/model/TextureMapping;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createNormalTorch(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createTurtleEggModel(ILjava/lang/String;Lnet/minecraft/client/data/models/model/TextureMapping;)Lnet/minecraft/client/renderer/block/dispatch/Variant;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createTurtleEggModel(II)Lnet/minecraft/client/renderer/block/dispatch/Variant;` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createMultiface(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createMultiface(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `selectMultifaceProperties(Lnet/minecraft/world/level/block/state/StateHolder;Ljava/util/function/Function;)Ljava/util/Map;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createMultifaceBlockStates(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createMossyCarpet(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createHangingMoss(Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createShelf(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `addShelfPart(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/model/TextureMapping;Lnet/minecraft/client/data/models/blockstates/MultiPartGenerator;Lnet/minecraft/client/data/models/model/ModelTemplate;Ljava/lang/Boolean;Lnet/minecraft/world/level/block/state/properties/SideChainPart;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `forEachHorizontalDirection(Ljava/util/function/BiConsumer;)V` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `shelfCondition(Lnet/minecraft/core/Direction;Ljava/lang/Boolean;Lnet/minecraft/world/level/block/state/properties/SideChainPart;)Lnet/minecraft/client/renderer/block/dispatch/multipart/Condition;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `addSlotStateAndRotationVariants(Lnet/minecraft/client/data/models/blockstates/MultiPartGenerator;Lnet/minecraft/client/renderer/block/dispatch/multipart/Condition;Lnet/minecraft/client/renderer/block/dispatch/VariantMutator;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `addBookSlotModel(Lnet/minecraft/client/data/models/blockstates/MultiPartGenerator;Lnet/minecraft/client/renderer/block/dispatch/multipart/Condition;Lnet/minecraft/client/renderer/block/dispatch/VariantMutator;Lnet/minecraft/world/level/block/state/properties/BooleanProperty;Lnet/minecraft/client/data/models/model/ModelTemplate;Z)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createShulkerBox(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/item/DyeColor;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createGrowingPlant(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/data/models/BlockModelGenerators$PlantType;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createNetherRoots(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `applyRotation(Lnet/minecraft/core/FrontAndTop;)Lnet/minecraft/client/renderer/block/dispatch/VariantMutator;` | private static | public static |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createHead(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/SkullBlock$Type;Lnet/minecraft/resources/Identifier;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCopperGolemStatue(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createBanner(Lnet/minecraft/world/item/DyeColor;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createChest(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/resources/Identifier;Z)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createChest(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/client/renderer/MultiblockChestResources;Z)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createBed(Lnet/minecraft/world/item/DyeColor;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `generateSimpleSpecialItemModel(Lnet/minecraft/world/level/block/Block;Ljava/util/Optional;Lnet/minecraft/client/renderer/special/SpecialModelRenderer$Unbaked;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCopperChainItem(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.BlockModelGenerators` | `createCandleAndCandleCake(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `itemModelOutput` | private final | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `modelOutput` | private final | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `prefixForSlotTrim(Ljava/lang/String;)Lnet/minecraft/resources/Identifier;` | private static | public static |
| `net.minecraft.client.data.models.ItemModelGenerators` | `declareCustomModelItem(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `createFlatItemModel(Lnet/minecraft/world/item/Item;Lnet/minecraft/client/data/models/model/ModelTemplate;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateFlatItem(Lnet/minecraft/world/item/Item;Lnet/minecraft/client/data/models/model/ModelTemplate;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `createFlatItemModel(Lnet/minecraft/world/item/Item;Ljava/lang/String;Lnet/minecraft/client/data/models/model/ModelTemplate;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `createFlatItemModel(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;Lnet/minecraft/client/data/models/model/ModelTemplate;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateFlatItem(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;Lnet/minecraft/client/data/models/model/ModelTemplate;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateFlatItem(Lnet/minecraft/world/item/Item;Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/data/models/model/ModelTemplate;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateItemWithTintedOverlay(Lnet/minecraft/world/item/Item;Lnet/minecraft/client/color/item/ItemTintSource;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateItemWithTintedOverlay(Lnet/minecraft/world/item/Item;Ljava/lang/String;Lnet/minecraft/client/color/item/ItemTintSource;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateItemWithTintedBaseLayer(Lnet/minecraft/world/item/Item;I)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `createCompassModels(Lnet/minecraft/world/item/Item;)Ljava/util/List;` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateStandardCompassItem(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateRecoveryCompassItem(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateClockItem(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateLayeredItem(Lnet/minecraft/world/item/Item;Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/resources/model/sprite/Material;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateLayeredItem(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/resources/model/sprite/Material;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateLayeredItem(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/resources/model/sprite/Material;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateTrimmableArmorSet(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;ZLjava/util/Map;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateTrimmableItem(Lnet/minecraft/world/item/Item;Lnet/minecraft/resources/Identifier;ZLjava/util/Map;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateBundleModels(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateBundleCoverModel(Lnet/minecraft/world/item/Item;Lnet/minecraft/client/data/models/model/ModelTemplate;Ljava/lang/String;)Lnet/minecraft/resources/Identifier;` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateBow(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateCrossbow(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateBooleanDispatch(Lnet/minecraft/world/item/Item;Lnet/minecraft/client/renderer/item/properties/conditional/ConditionalItemModelProperty;Lnet/minecraft/client/renderer/item/ItemModel$Unbaked;Lnet/minecraft/client/renderer/item/ItemModel$Unbaked;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateElytra(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateBrush(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateFishingRod(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateGoatHorn(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateShield(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `createFlatModelDispatch(Lnet/minecraft/client/renderer/item/ItemModel$Unbaked;Lnet/minecraft/client/renderer/item/ItemModel$Unbaked;)Lnet/minecraft/client/renderer/item/ItemModel$Unbaked;` | private static | public static |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateSpyglass(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateTrident(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateSpear(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `addPotionTint(Lnet/minecraft/world/item/Item;Lnet/minecraft/resources/Identifier;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generatePotion(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateTippedArrow(Lnet/minecraft/world/item/Item;)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateDyedItem(Lnet/minecraft/world/item/Item;I)V` | private | public final |
| `net.minecraft.client.data.models.ItemModelGenerators` | `generateTwoLayerDyedItem(Lnet/minecraft/world/item/Item;)V` | private | public final |
| [[40-Interfaces/net.minecraft.client.data.models.ModelProvider|ModelProvider]] | `getName()Ljava/lang/String;` | public final | public |
| `net.minecraft.client.data.models.model.TextureSlot` | `create(Ljava/lang/String;)Lnet/minecraft/client/data/models/model/TextureSlot;` | private static | public static |
| `net.minecraft.client.data.models.model.TextureSlot` | `create(Ljava/lang/String;Lnet/minecraft/client/data/models/model/TextureSlot;)Lnet/minecraft/client/data/models/model/TextureSlot;` | private static | public static |
| `net.minecraft.client.data.models.model.TexturedModel` | `createDefault(Ljava/util/function/Function;Lnet/minecraft/client/data/models/model/ModelTemplate;)Lnet/minecraft/client/data/models/model/TexturedModel$Provider;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.gui.GuiGraphicsExtractor|GuiGraphicsExtractor]] | `scissorStack` | private final | public final |
| [[40-Interfaces/net.minecraft.client.gui.GuiGraphicsExtractor|GuiGraphicsExtractor]] | `guiRenderState` | private final | public final |
| [[40-Interfaces/net.minecraft.client.gui.components.debug.DebugScreenEntries|DebugScreenEntries]] | `register(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/gui/components/debug/DebugScreenEntry;)Lnet/minecraft/resources/Identifier;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.gui.screens.MenuScreens|MenuScreens]] | `register(Lnet/minecraft/world/inventory/MenuType;Lnet/minecraft/client/gui/screens/MenuScreens$ScreenConstructor;)V` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `GLOBALS_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `MATRICES_FOG_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `MATRICES_FOG_LIGHT_DIR_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `OIT_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `GENERIC_BLOCKS_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `LIT_BLOCKS_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `TERRAIN_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `MULTIDRAW_TERRAIN_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `BLOCK_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `WATER_MASK_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `LIGHTNING_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `DRAGON_RAYS_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `ENTITY_NO_LIGHTMAP_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `ENTITY_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `OIT_ENTITY_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `ENTITY_EMISSIVE_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `EYES_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `BEACON_BEAM_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `ITEM_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `OIT_ITEM_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `TEXT_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `WORLD_TEXT_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `END_PORTAL_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `CLOUDS_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `OIT_CLOUDS_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `LINES_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `OIT_LINES_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `DEBUG_FILLED_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `OIT_DEBUG_FILLED_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `DEBUG_POINTS_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `PARTICLE_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `OIT_PARTICLE_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `WEATHER_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `CRUMBLING_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `GUI_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `GUI_TEXTURED_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `GUI_TEXT_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `OUTLINE_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `ENERGY_SWIRL_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `ENTITY_SHADOW_SNIPPET` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `register(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `register(Lnet/minecraft/client/renderer/oit/OitPipelineSet;)Lnet/minecraft/client/renderer/oit/OitPipelineSet;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] | `registerOptional(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;` | private static | public static |
| `net.minecraft.client.renderer.block.BuiltInBlockModels$Builder` | `<init>(Lnet/minecraft/client/color/block/BlockColors;)V` | private | public |
| `net.minecraft.client.renderer.block.BuiltInBlockModels$Builder` | `put(Lnet/minecraft/client/renderer/block/BuiltInBlockModels$ModelFactory;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.renderer.block.BuiltInBlockModels$Builder` | `put(Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| `net.minecraft.client.renderer.block.BuiltInBlockModels$Builder` | `put(Lnet/minecraft/client/renderer/block/BuiltInBlockModels$ModelFactory;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `addDefaults(Lnet/minecraft/client/renderer/block/BuiltInBlockModels$Builder;)V` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createAir(Lnet/minecraft/client/renderer/block/BuiltInBlockModels$Builder;Lnet/minecraft/world/level/block/Block;)V` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `special(Lnet/minecraft/client/renderer/special/SpecialModelRenderer$Unbaked;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `special(Lnet/minecraft/client/renderer/special/SpecialModelRenderer$Unbaked;Lcom/mojang/math/Transformation;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createMobHead(Lnet/minecraft/world/level/block/SkullBlock$Types;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createMobWallHead(Lnet/minecraft/world/level/block/SkullBlock$Types;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createMobHeads(Lnet/minecraft/client/renderer/block/BuiltInBlockModels$Builder;Lnet/minecraft/world/level/block/SkullBlock$Types;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createPlayerHead()Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createPlayerWallHead()Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createBanner(Lnet/minecraft/world/item/DyeColor;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createWallBanner(Lnet/minecraft/world/item/DyeColor;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createShulkerBox()Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createDyedShulkerBox(Lnet/minecraft/world/item/DyeColor;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createChest(Lnet/minecraft/resources/Identifier;Lnet/minecraft/world/level/block/state/properties/ChestType;Lnet/minecraft/core/Direction;)Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createSingletonChest(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createChest(Lnet/minecraft/client/renderer/MultiblockChestResources;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createXmasChest(Lnet/minecraft/client/renderer/MultiblockChestResources;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createCopperGolem(Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createDecoratedPot()Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createBlockStateModelWrapper(Lnet/minecraft/client/color/block/BlockColors;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/renderer/block/model/BlockStateModelWrapper$Unbaked;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `combineSpecialAndBlockModels(Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;Lnet/minecraft/client/color/block/BlockColors;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/renderer/block/model/CompositeBlockModel$Unbaked;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createFlowerBedModel(Lnet/minecraft/client/color/block/BlockColors;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/renderer/block/SelectBlockModel$Unbaked;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createEnchantingTable()Lnet/minecraft/client/renderer/block/model/BlockModel$Unbaked;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `specialModelWithPropertyDispatch(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/util/function/Function;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `specialModelWithPropertyDispatch(Lnet/minecraft/world/level/block/state/properties/Property;Lnet/minecraft/world/level/block/state/properties/Property;Ljava/util/function/BiFunction;)Lnet/minecraft/client/renderer/block/BuiltInBlockModels$SpecialModelFactory;` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.block.FluidRenderer|FluidRenderer]] | `fluidModels` | private final | public final |
| [[40-Interfaces/net.minecraft.client.renderer.blockentity.BlockEntityRenderers|BlockEntityRenderers]] | `register(Lnet/minecraft/world/level/block/entity/BlockEntityType;Lnet/minecraft/client/renderer/blockentity/BlockEntityRendererProvider;)V` | private static | public static |
| [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRenderers|EntityRenderers]] | `register(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/client/renderer/entity/EntityRendererProvider;)V` | private static | public static |
| `net.minecraft.client.renderer.item.ItemModels` | `ID_MAPPER` | private static final | public static final |
| `net.minecraft.client.renderer.item.properties.conditional.ConditionalItemModelProperties` | `ID_MAPPER` | private static final | public static final |
| `net.minecraft.client.renderer.item.properties.numeric.RangeSelectItemModelProperties` | `ID_MAPPER` | private static final | public static final |
| `net.minecraft.client.renderer.item.properties.select.SelectItemModelProperties` | `ID_MAPPER` | private static final | public static final |
| [[40-Interfaces/net.minecraft.client.renderer.rendertype.RenderType|RenderType]] | `create(Ljava/lang/String;Lnet/minecraft/client/renderer/rendertype/RenderSetup;)Lnet/minecraft/client/renderer/rendertype/RenderType;` | static | public static |
| `net.minecraft.client.renderer.special.SpecialModelRenderers` | `ID_MAPPER` | private static final | public static final |
| `net.minecraft.commands.Commands$CommandSelection` | `includeIntegrated` | private final | public final |
| `net.minecraft.commands.Commands$CommandSelection` | `includeDedicated` | private final | public final |
| `net.minecraft.data.BlockFamilies` | `familyBuilder(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/data/BlockFamily$Builder;` | private static | public static |
| `net.minecraft.data.info.BiomeParametersDumpReport` | `getName()Ljava/lang/String;` | public final | public |
| `net.minecraft.data.info.BlockListReport` | `getName()Ljava/lang/String;` | public final | public |
| `net.minecraft.data.info.CommandsReport` | `getName()Ljava/lang/String;` | public final | public |
| `net.minecraft.data.info.RegistryComponentsReport` | `getName()Ljava/lang/String;` | public final | public |
| `net.minecraft.data.info.RegistryDumpReport` | `getName()Ljava/lang/String;` | public final | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `hasSilkTouch()Lnet/minecraft/core/Holder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `doesNotHaveSilkTouch()Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `hasShears()Lnet/minecraft/core/Holder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `hasShearsOrSilkTouch()Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `doesNotHaveShearsOrSilkTouch()Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `applyExplosionDecay(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/storage/loot/functions/FunctionUserBuilder;)Lnet/minecraft/world/level/storage/loot/functions/FunctionUserBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `applyExplosionCondition(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/storage/loot/predicates/ConditionUserBuilder;)Lnet/minecraft/world/level/storage/loot/predicates/ConditionUserBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createSelfDropDispatchTable(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/core/Holder;Lnet/minecraft/world/level/storage/loot/entries/LootPoolEntryContainer$Builder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | private static | public static |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createSilkTouchDispatchTable(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/storage/loot/entries/LootPoolEntryContainer$Builder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createShearsDispatchTable(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/storage/loot/entries/LootPoolEntryContainer$Builder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createSilkTouchOrShearsDispatchTable(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/storage/loot/entries/LootPoolEntryContainer$Builder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createSingleItemTableWithSilkTouch(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createSingleItemTable(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createSingleItemTableWithSilkTouch(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createSilkTouchOnlyTable(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createPotFlowerItemTable(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createSlabItemTable(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createSinglePropConditionTable(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang/Comparable;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createNameableBlockEntityTable(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createShulkerBoxDrop(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createCopperOreDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createLapisOreDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createRedstoneOreDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createBannerDrop(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createBeeNestDrop(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createBeeHiveDrop(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createCaveVinesDrop(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createCopperGolemStatueBlock(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createOreDrop(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/item/Item;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createMushroomBlockDrop(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createGrassDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createShearsOnlyDrop(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createShearsOrSilkTouchOnlyDrop(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createMultifaceBlockDrops(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createMultifaceBlockDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createMossyCarpetBlockDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createLeavesDrops(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;[F)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createOakLeavesDrops(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;[F)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createMangroveLeavesDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createCropDrops(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createDoublePlantShearsDrop(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createDoublePlantWithSeedDrops(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createCandleDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createCandleCakeDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected static | public static |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `generate()V` | protected abstract | public abstract |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `addNetherVinesDropTable(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `createDoorTable(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `dropPottedContents(Lnet/minecraft/world/level/block/Block;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `otherWhenSilkTouch(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `dropOther(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `dropWhenSilkTouch(Lnet/minecraft/world/level/block/Block;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `dropSelf(Lnet/minecraft/world/level/block/Block;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `add(Lnet/minecraft/world/level/block/Block;Ljava/util/function/Function;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]] | `add(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/storage/loot/LootTable$Builder;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.EntityLootSubProvider|EntityLootSubProvider]] | `add(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/storage/loot/LootTable$Builder;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.loot.EntityLootSubProvider|EntityLootSubProvider]] | `add(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/storage/loot/LootTable$Builder;)V` | protected | public |
| `net.minecraft.data.metadata.PackMetadataGenerator` | `getName()Ljava/lang/String;` | public final | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `buildRecipes()V` | protected abstract | public abstract |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `generateForEnabledBlockFamilies(Lnet/minecraft/world/flag/FeatureFlagSet;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `oneToOneConversionRecipe(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;Ljava/lang/String;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `oneToOneConversionRecipe(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;Ljava/lang/String;I)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `oreSmelting(Ljava/util/List;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/CookingBookCategory;Lnet/minecraft/world/level/ItemLike;FILjava/lang/String;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `oreBlasting(Ljava/util/List;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/CookingBookCategory;Lnet/minecraft/world/level/ItemLike;FILjava/lang/String;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `oreCooking(Lnet/minecraft/world/item/crafting/AbstractCookingRecipe$Factory;Ljava/util/List;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/CookingBookCategory;Lnet/minecraft/world/level/ItemLike;FILjava/lang/String;Ljava/lang/String;)V` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `netheriteSmithing(Lnet/minecraft/world/item/Item;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/Item;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `trimSmithing(Lnet/minecraft/world/item/Item;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/ResourceKey;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `twoByTwoPacker(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `threeByThreePacker(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;Ljava/lang/String;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `threeByThreePacker(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `planksFromLog(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/tags/TagKey;I)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `planksFromLogs(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/tags/TagKey;I)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `woodFromLogs(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `woodenBoat(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `chestBoat(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `buttonBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `doorBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `fenceBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `fenceGateBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `pressurePlate(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `pressurePlateBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `slab(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `shelf(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `slabBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `stairBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `trapdoorBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `signBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `hangingSignBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `colorItemWithDye(Ljava/util/List;Ljava/util/List;Ljava/lang/String;Lnet/minecraft/data/recipes/RecipeCategory;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `colorWithDye(Ljava/util/List;Ljava/util/List;Lnet/minecraft/world/item/Item;Ljava/lang/String;Lnet/minecraft/data/recipes/RecipeCategory;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `carpet(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `bedFromPlanksAndWool(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `banner(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `stainedGlassFromGlassAndDye(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `dryGhast(Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `harness(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `stainedGlassPaneFromStainedGlass(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `stainedGlassPaneFromGlassPaneAndDye(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `coloredTerracottaFromTerracottaAndDye(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `concretePowder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `candle(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `wall(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `wallBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `bricksBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `tilesBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `pillarBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `polished(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `polishedBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `cut(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `cutBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/ShapedRecipeBuilder;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `chiseled(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `mosaicBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `chiseledBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/ShapedRecipeBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `carpetBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/ShapedRecipeBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `stonecutterResultFromBase(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `stonecutterResultFromBase(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;I)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `smeltingResultFromBase(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `nineBlockStorageRecipes(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `nineBlockStorageRecipesWithCustomPacking(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Ljava/lang/String;Ljava/lang/String;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `nineBlockStorageRecipesRecipesWithCustomUnpacking(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Ljava/lang/String;Ljava/lang/String;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `nineBlockStorageRecipes(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `copySmithingTemplate(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `copySmithingTemplate(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `cookRecipes(Ljava/lang/String;Lnet/minecraft/world/item/crafting/AbstractCookingRecipe$Factory;I)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `simpleCookingRecipe(Ljava/lang/String;Lnet/minecraft/world/item/crafting/AbstractCookingRecipe$Factory;ILnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;F)V` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `waxRecipes(Lnet/minecraft/world/flag/FeatureFlagSet;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `grate(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `copperBulb(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `waxedChiseled(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `suspiciousStew(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/level/block/SuspiciousEffectHolder;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `dyedItem(Lnet/minecraft/world/item/Item;Ljava/lang/String;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `dyedShulkerBoxRecipe(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `dyedBundleRecipe(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `cushionRecipe(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `generateRecipes(Lnet/minecraft/data/BlockFamily;Lnet/minecraft/world/flag/FeatureFlagSet;)V` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `generateCraftingRecipe(Lnet/minecraft/data/BlockFamily;Lnet/minecraft/data/BlockFamily$Variant;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/ItemLike;)V` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `generateSmeltingRecipe(Lnet/minecraft/data/BlockFamily$Variant;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/ItemLike;)V` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `generateStonecutterRecipe(Lnet/minecraft/data/BlockFamily;Lnet/minecraft/data/BlockFamily$Variant;Lnet/minecraft/world/level/block/Block;)V` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `getBaseBlockForCrafting(Lnet/minecraft/data/BlockFamily;Lnet/minecraft/data/BlockFamily$Variant;)Lnet/minecraft/world/level/block/Block;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `getCraftingCriterionName(Lnet/minecraft/data/BlockFamily;Lnet/minecraft/data/BlockFamily$Variant;Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `insideOf(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/advancements/triggers/Criterion;` | private static | public static |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `bredAnimal()Lnet/minecraft/advancements/triggers/Criterion;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `has(Lnet/minecraft/advancements/predicates/MinMaxBounds$Ints;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/advancements/triggers/Criterion;` | private | public final |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `has(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/advancements/triggers/Criterion;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `has(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/advancements/triggers/Criterion;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `inventoryTrigger([Lnet/minecraft/advancements/predicates/ItemPredicate$Builder;)Lnet/minecraft/advancements/triggers/Criterion;` | private static | public static |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `inventoryTrigger([Lnet/minecraft/advancements/predicates/ItemPredicate;)Lnet/minecraft/advancements/triggers/Criterion;` | private static | public static |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `getHasName(Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;` | protected static | public static |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `getItemName(Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;` | protected static | public static |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `getSimpleRecipeName(Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;` | protected static | public static |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `getConversionRecipeName(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;` | protected static | public static |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `getSmeltingRecipeName(Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;` | protected static | public static |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `getBlastingRecipeName(Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;` | protected static | public static |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `tag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/item/crafting/Ingredient;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `shaped(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/ShapedRecipeBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `shaped(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;I)Lnet/minecraft/data/recipes/ShapedRecipeBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `shapeless(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/ItemStackTemplate;)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `shapeless(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]] | `shapeless(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;I)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;` | protected | public |
| [[40-Interfaces/net.minecraft.data.registries.RegistriesDatapackGenerator|RegistriesDatapackGenerator]] | `getName()Ljava/lang/String;` | public final | public |
| `net.minecraft.data.structures.NbtToSnbt` | `getName()Ljava/lang/String;` | public final | public |
| `net.minecraft.data.structures.SnbtToNbt` | `getName()Ljava/lang/String;` | public final | public |
| [[40-Interfaces/net.minecraft.data.tags.TagsProvider|TagsProvider]] | `getName()Ljava/lang/String;` | public final | public |
| `net.minecraft.data.worldgen.ProcessorLists` | `EMPTY` | private static final | public static final |
| `net.minecraft.data.worldgen.biome.EndBiomes` | `baseEndBiome(Lnet/minecraft/world/level/biome/BiomeGenerationSettings$Builder;)Lnet/minecraft/world/level/biome/Biome;` | private static | public static |
| `net.minecraft.data.worldgen.biome.NetherBiomes` | `baseBiome()Lnet/minecraft/world/level/biome/Biome$BiomeBuilder;` | private static | public static |
| `net.minecraft.data.worldgen.biome.OverworldBiomes` | `baseBiome(FF)Lnet/minecraft/world/level/biome/Biome$BiomeBuilder;` | private static | public static |
| `net.minecraft.data.worldgen.biome.OverworldBiomes` | `globalOverworldGeneration(Lnet/minecraft/world/level/biome/BiomeGenerationSettings$Builder;)V` | private static | public static |
| `net.minecraft.data.worldgen.biome.OverworldBiomes` | `baseJungle(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/core/HolderGetter;FZZZ)Lnet/minecraft/world/level/biome/Biome$BiomeBuilder;` | private static | public static |
| `net.minecraft.data.worldgen.biome.OverworldBiomes` | `baseOcean()Lnet/minecraft/world/level/biome/Biome$BiomeBuilder;` | private static | public static |
| `net.minecraft.data.worldgen.biome.OverworldBiomes` | `baseOceanGeneration(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/biome/BiomeGenerationSettings$Builder;` | private static | public static |
| [[40-Interfaces/net.minecraft.network.chat.TextColor|TextColor]] | `formatValue()Ljava/lang/String;` | private | public final |
| [[40-Interfaces/net.minecraft.network.chat.contents.TranslatableContents|TranslatableContents]] | `getArgument(I)Lnet/minecraft/network/chat/FormattedText;` | private | public final |
| [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]] | `sendParticles(Lnet/minecraft/server/level/ServerPlayer;ZDDDLnet/minecraft/network/protocol/Packet;)Z` | private | public final |
| [[40-Interfaces/net.minecraft.world.SimpleContainer|SimpleContainer]] | `items` | private final | public final |
| `net.minecraft.world.damagesource.DamageSources` | `damageTypes` | private final | public final |
| `net.minecraft.world.damagesource.DamageSources` | `source(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/damagesource/DamageSource;` | private | public final |
| `net.minecraft.world.damagesource.DamageSources` | `source(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/damagesource/DamageSource;` | private | public final |
| `net.minecraft.world.damagesource.DamageSources` | `source(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/damagesource/DamageSource;` | private | public final |
| `net.minecraft.world.entity.Display$BlockDisplay` | `getBlockState()Lnet/minecraft/world/level/block/state/BlockState;` | private | public final |
| `net.minecraft.world.entity.Display$BlockDisplay` | `setBlockState(Lnet/minecraft/world/level/block/state/BlockState;)V` | private | public final |
| `net.minecraft.world.entity.Display$ItemDisplay` | `getItemStack()Lnet/minecraft/world/item/ItemStack;` | private | public final |
| `net.minecraft.world.entity.Display$ItemDisplay` | `setItemStack(Lnet/minecraft/world/item/ItemStack;)V` | private | public final |
| `net.minecraft.world.entity.Display$ItemDisplay` | `setItemTransform(Lnet/minecraft/world/item/ItemDisplayContext;)V` | private | public final |
| `net.minecraft.world.entity.Display$ItemDisplay` | `getItemTransform()Lnet/minecraft/world/item/ItemDisplayContext;` | private | public final |
| `net.minecraft.world.entity.Display$TextDisplay` | `getText()Lnet/minecraft/network/chat/Component;` | private | public final |
| `net.minecraft.world.entity.Display$TextDisplay` | `setText(Lnet/minecraft/network/chat/Component;)V` | private | public final |
| `net.minecraft.world.entity.Display$TextDisplay` | `getLineWidth()I` | private | public final |
| `net.minecraft.world.entity.Display$TextDisplay` | `setLineWidth(I)V` | private | public final |
| `net.minecraft.world.entity.Display$TextDisplay` | `getTextOpacity()B` | private | public final |
| `net.minecraft.world.entity.Display$TextDisplay` | `setTextOpacity(B)V` | private | public final |
| `net.minecraft.world.entity.Display$TextDisplay` | `getBackgroundColor()I` | private | public final |
| `net.minecraft.world.entity.Display$TextDisplay` | `setBackgroundColor(I)V` | private | public final |
| `net.minecraft.world.entity.Display$TextDisplay` | `getFlags()B` | private | public final |
| `net.minecraft.world.entity.Display$TextDisplay` | `setFlags(B)V` | private | public final |
| `net.minecraft.world.entity.Display` | `setTransformation(Lcom/mojang/math/Transformation;)V` | private | public final |
| `net.minecraft.world.entity.Display` | `setTransformationInterpolationDuration(I)V` | private | public final |
| `net.minecraft.world.entity.Display` | `getTransformationInterpolationDuration()I` | private | public final |
| `net.minecraft.world.entity.Display` | `setTransformationInterpolationDelay(I)V` | private | public final |
| `net.minecraft.world.entity.Display` | `getTransformationInterpolationDelay()I` | private | public final |
| `net.minecraft.world.entity.Display` | `setPosRotInterpolationDuration(I)V` | private | public final |
| `net.minecraft.world.entity.Display` | `getPosRotInterpolationDuration()I` | private | public final |
| `net.minecraft.world.entity.Display` | `setBillboardConstraints(Lnet/minecraft/world/entity/Display$BillboardConstraints;)V` | private | public final |
| `net.minecraft.world.entity.Display` | `getBillboardConstraints()Lnet/minecraft/world/entity/Display$BillboardConstraints;` | private | public final |
| `net.minecraft.world.entity.Display` | `setBrightnessOverride(Lnet/minecraft/util/Brightness;)V` | private | public final |
| `net.minecraft.world.entity.Display` | `getBrightnessOverride()Lnet/minecraft/util/Brightness;` | private | public final |
| `net.minecraft.world.entity.Display` | `getPackedBrightnessOverride()I` | private | public final |
| `net.minecraft.world.entity.Display` | `setViewRange(F)V` | private | public final |
| `net.minecraft.world.entity.Display` | `getViewRange()F` | private | public final |
| `net.minecraft.world.entity.Display` | `setShadowRadius(F)V` | private | public final |
| `net.minecraft.world.entity.Display` | `getShadowRadius()F` | private | public final |
| `net.minecraft.world.entity.Display` | `setShadowStrength(F)V` | private | public final |
| `net.minecraft.world.entity.Display` | `getShadowStrength()F` | private | public final |
| `net.minecraft.world.entity.Display` | `setWidth(F)V` | private | public final |
| `net.minecraft.world.entity.Display` | `getWidth()F` | private | public final |
| `net.minecraft.world.entity.Display` | `setHeight(F)V` | private | public final |
| `net.minecraft.world.entity.Display` | `getGlowColorOverride()I` | private | public final |
| `net.minecraft.world.entity.Display` | `setGlowColorOverride(I)V` | private | public final |
| `net.minecraft.world.entity.Display` | `getHeight()F` | private | public final |
| `net.minecraft.world.entity.Interaction` | `setWidth(F)V` | private | public final |
| `net.minecraft.world.entity.Interaction` | `getWidth()F` | private | public final |
| `net.minecraft.world.entity.Interaction` | `setHeight(F)V` | private | public final |
| `net.minecraft.world.entity.Interaction` | `getHeight()F` | private | public final |
| `net.minecraft.world.entity.Interaction` | `setResponse(Z)V` | private | public final |
| `net.minecraft.world.entity.Interaction` | `getResponse()Z` | private | public final |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `hurtArmor(Lnet/minecraft/world/damagesource/DamageSource;F)V` | protected | public |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `hurtHelmet(Lnet/minecraft/world/damagesource/DamageSource;F)V` | protected | public |
| [[40-Interfaces/net.minecraft.world.entity.SpawnPlacements|SpawnPlacements]] | `register(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/entity/SpawnPlacementType;Lnet/minecraft/world/level/levelgen/Heightmap$Types;Lnet/minecraft/world/entity/SpawnPlacements$SpawnPredicate;)V` | private static | public static |
| `net.minecraft.world.entity.ai.sensing.SensorType` | `<init>(Ljava/util/function/Supplier;)V` | private | public |
| `net.minecraft.world.entity.npc.villager.VillagerType` | `BY_BIOME` | private static final | public static final |
| `net.minecraft.world.entity.projectile.FishingHook` | `<init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;II)V` | private | public |
| `net.minecraft.world.entity.projectile.Projectile` | `<init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V` | protected | public |
| `net.minecraft.world.entity.schedule.Activity` | `<init>(Ljava/lang/String;)V` | private | public |
| [[40-Interfaces/net.minecraft.world.inventory.MenuType|MenuType]] | `<init>(Lnet/minecraft/world/inventory/MenuType$MenuSupplier;Lnet/minecraft/world/flag/FeatureFlagSet;)V` | private | public |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `BUILDING_BLOCKS` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `COLORED_BLOCKS` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `NATURAL_BLOCKS` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `FUNCTIONAL_BLOCKS` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `REDSTONE_BLOCKS` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `HOTBAR` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `SEARCH` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `TOOLS_AND_UTILITIES` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `COMBAT` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `FOOD_AND_DRINKS` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `INGREDIENTS` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `SPAWN_EGGS` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `OP_BLOCKS` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `INVENTORY` | private static final | public static final |
| [[40-Interfaces/net.minecraft.world.item.Item_Properties|Item$Properties]] | `itemIdOrThrow()Lnet/minecraft/resources/ResourceKey;` | private | public final |
| `net.minecraft.world.item.context.BlockPlaceContext` | `<init>(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/phys/BlockHitResult;)V` | protected | public |
| `net.minecraft.world.item.context.UseOnContext` | `<init>(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/phys/BlockHitResult;)V` | protected | public |
| [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment|Enchantment]] | `modifyItemFilteredCount(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemInstance;Lorg/apache/commons/lang3/mutable/MutableFloat;)V` | private | public final |
| [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment|Enchantment]] | `modifyEntityFilteredValue(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;)V` | private | public final |
| [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment|Enchantment]] | `modifyDamageFilteredValue(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;)V` | private | public final |
| [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment|Enchantment]] | `itemContext(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemInstance;)Lnet/minecraft/world/level/storage/loot/LootContext;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment|Enchantment]] | `locationContext(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/entity/Entity;Z)Lnet/minecraft/world/level/storage/loot/LootContext;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment|Enchantment]] | `entityContext(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/level/storage/loot/LootContext;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment|Enchantment]] | `blockHitContext(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/storage/loot/LootContext;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment|Enchantment]] | `applyEffects(Ljava/util/List;Lnet/minecraft/world/level/storage/loot/LootContext;Lnet/minecraft/world/item/enchantment/Enchantment$GenericAction;)V` | private static | public static |
| [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment|Enchantment]] | `applyEffects(Ljava/util/List;Lnet/minecraft/world/level/storage/loot/LootContext;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/world/item/enchantment/Enchantment$FloatAction;)V` | private static | public static |
| [[40-Interfaces/net.minecraft.world.item.enchantment.EnchantmentHelper|EnchantmentHelper]] | `getComponentType(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/core/component/DataComponentType;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.item.enchantment.EnchantmentHelper|EnchantmentHelper]] | `runIterationOnItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/enchantment/EnchantmentHelper$EnchantmentVisitor;)V` | private static | public static |
| [[40-Interfaces/net.minecraft.world.item.enchantment.EnchantmentHelper|EnchantmentHelper]] | `runIterationOnItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/item/enchantment/EnchantmentHelper$EnchantmentInSlotVisitor;)V` | private static | public static |
| [[40-Interfaces/net.minecraft.world.item.enchantment.EnchantmentHelper|EnchantmentHelper]] | `runIterationOnEquipment(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/item/enchantment/EnchantmentHelper$EnchantmentInSlotVisitor;)V` | private static | public static |
| `net.minecraft.world.item.equipment.ArmorMaterials` | `makeDefense(IIIII)Ljava/util/Map;` | private static | public static |
| `net.minecraft.world.level.block.AttachedStemBlock` | `<init>(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/tags/TagKey;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.AzaleaBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.BarrierBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.BaseCoralFanBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.BaseCoralPlantBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.BaseCoralWallFanBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.BigDripleafBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.BigDripleafStemBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.BlastFurnaceBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `litBlockEmission(I)Ljava/util/function/ToIntFunction;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `never(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/EntityType;)Ljava/lang/Boolean;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `always(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/EntityType;)Ljava/lang/Boolean;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `ocelotOrParrot(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/EntityType;)Ljava/lang/Boolean;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `logProperties(Lnet/minecraft/world/level/material/MapColor;Lnet/minecraft/world/level/material/MapColor;Lnet/minecraft/world/level/block/SoundType;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `netherStemProperties(Lnet/minecraft/world/level/material/MapColor;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `always(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Z` | private static | public static |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `never(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Z` | private static | public static |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `leavesProperties(Lnet/minecraft/world/level/block/SoundType;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `buttonProperties()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `flowerPotProperties()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `register(Lnet/minecraft/resources/ResourceKey;Ljava/util/function/Function;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)Lnet/minecraft/world/level/block/Block;` | private static | public static |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `register(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)Lnet/minecraft/world/level/block/Block;` | private static | public static |
| `net.minecraft.world.level.block.BushBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.BushBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;I)V` | protected | public |
| `net.minecraft.world.level.block.ButtonBlock` | `<init>(Lnet/minecraft/world/level/block/state/properties/BlockSetType;ILnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.CactusBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.CakeBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.CandleCakeBlock` | `<init>(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.CartographyTableBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.CarvedPumpkinBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| [[40-Interfaces/net.minecraft.world.level.block.ChestBlock|ChestBlock]] | `<init>(Ljava/util/function/Supplier;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.ChorusFlowerBlock` | `<init>(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.ChorusPlantBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.CoralFanBlock` | `<init>(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.CoralPlantBlock` | `<init>(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.CoralWallFanBlock` | `<init>(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.CraftingTableBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.CreakingHeartBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.CropBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.DecoratedPotBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| [[40-Interfaces/net.minecraft.world.level.block.DispenserBlock|DispenserBlock]] | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.DoorBlock` | `<init>(Lnet/minecraft/world/level/block/state/properties/BlockSetType;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.DryVegetationBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.EnchantingTableBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.EndGatewayBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.EndPortalBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.EndRodBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.EnderChestBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.FarmlandBlock` | `<init>(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.FlowerBedBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;I)V` | protected | public |
| `net.minecraft.world.level.block.FurnaceBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.GrindstoneBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.HalfTransparentBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.HangingRootsBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.IronBarsBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.JigsawBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.JukeboxBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.KelpBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.KelpPlantBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| [[40-Interfaces/net.minecraft.world.level.block.LadderBlock|LadderBlock]] | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.LecternBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.LeverBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.LilyPadBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| [[40-Interfaces/net.minecraft.world.level.block.LiquidBlock|LiquidBlock]] | `<init>(Lnet/minecraft/world/level/material/FlowingFluid;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.LoomBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.MangroveRootsBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.NetherFungusBlock` | `<init>(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/tags/TagKey;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.NetherRootsBlock` | `<init>(Lnet/minecraft/tags/TagKey;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.NetherWartBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.NyliumBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.PathBlock` | `<init>(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.PlayerHeadBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.PlayerWallHeadBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.PoweredRailBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.PressurePlateBlock` | `<init>(Lnet/minecraft/world/level/block/state/properties/BlockSetType;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.PumpkinBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.RailBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.RedstoneTorchBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.RedstoneWallTorchBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.RepeaterBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.SaplingBlock` | `<init>(Lnet/minecraft/world/level/block/grower/TreeGrower;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.ScaffoldingBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.SeaPickleBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.SeagrassBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.ShortDryGrassBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.SkullBlock` | `<init>(Lnet/minecraft/world/level/block/SkullBlock$Type;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.SmithingTableBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.SmokerBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.SnowLayerBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.SnowyBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.SpawnerBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.SpongeBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.StairBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.StemBlock` | `<init>(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/tags/TagKey;Lnet/minecraft/tags/TagKey;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.StructureBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.StructureVoidBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.SugarCaneBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.TallDryGrassBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.TallGrassBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.TorchBlock` | `<init>(Lnet/minecraft/core/particles/SimpleParticleType;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.TransparentBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| [[40-Interfaces/net.minecraft.world.level.block.TrapDoorBlock|TrapDoorBlock]] | `<init>(Lnet/minecraft/world/level/block/state/properties/BlockSetType;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WallSkullBlock` | `<init>(Lnet/minecraft/world/level/block/SkullBlock$Type;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WallTorchBlock` | `<init>(Lnet/minecraft/core/particles/SimpleParticleType;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WaterloggedTransparentBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WeatheringCopperBarsBlock` | `<init>(Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WeatheringCopperChainBlock` | `<init>(Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WeatheringCopperDoorBlock` | `<init>(Lnet/minecraft/world/level/block/state/properties/BlockSetType;Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WeatheringCopperGrateBlock` | `<init>(Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WeatheringCopperTrapDoorBlock` | `<init>(Lnet/minecraft/world/level/block/state/properties/BlockSetType;Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WeatheringLanternBlock` | `<init>(Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WeatheringLightningRodBlock` | `<init>(Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WeightedPressurePlateBlock` | `<init>(ILnet/minecraft/world/level/block/state/properties/BlockSetType;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WetSpongeBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WitherSkullBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WitherWallSkullBlock` | `<init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.block.WoolCarpetBlock` | `<init>(Lnet/minecraft/world/item/DyeColor;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | protected | public |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `ORE_THICKNESS` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `VEININESS_FREQUENCY` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `NOODLE_SPACING_AND_STRAIGHTNESS` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `SURFACE_DENSITY_THRESHOLD` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `CHEESE_NOISE_TARGET` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `DENSITY_Y_ANCHOR_BOTTOM` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `DENSITY_Y_ANCHOR_TOP` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `DENSITY_Y_BOTTOM` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `DENSITY_Y_TOP` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `OVERWORLD_BOTTOM_SLIDE_HEIGHT` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `BASE_DENSITY_MULTIPLIER` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `BLENDING_FACTOR` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `BLENDING_JAGGEDNESS` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `ZERO` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `Y` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `SHIFT_X` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `SHIFT_Z` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `BASE_3D_NOISE_OVERWORLD` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `BASE_3D_NOISE_NETHER` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `BASE_3D_NOISE_END` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `END_ISLANDS` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `SLOPED_CHEESE_END` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `SPAGHETTI_ROUGHNESS_FUNCTION` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `ENTRANCES` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `NOODLE` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `PILLARS` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `SPAGHETTI_2D_THICKNESS_MODULATOR` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `SPAGHETTI_2D` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `ORE_VEIN_MASK` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `ORE_VEIN_TOGGLE` | private static final | public static final |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `createEndIslands()Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `registerTerrainNoises(Lnet/minecraft/data/worldgen/BootstrapContext;Lnet/minecraft/core/HolderGetter;Lnet/minecraft/core/HolderGetter;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;Lnet/minecraft/world/level/levelgen/OverworldFunctionSet;Z)V` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `offsetToDepth(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `registerAndWrap(Lnet/minecraft/data/worldgen/BootstrapContext;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `spaghettiRoughnessFunction(Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `entrances(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `noodle(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `pillars(Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `spaghetti2D(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `underground(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/core/HolderGetter;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `postProcess(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;II)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `overworld(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/world/level/levelgen/OverworldFunctionSet;)Lnet/minecraft/world/level/levelgen/NoiseRouter;` | protected static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `overworldAquifers(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/core/HolderGetter;Lnet/minecraft/world/level/levelgen/OverworldFunctionSet;)Lnet/minecraft/world/level/levelgen/Aquifer$Config;` | protected static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `registerOreVeins(Lnet/minecraft/data/worldgen/BootstrapContext;)V` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `createBaseOreVeinMask(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;II)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `createOreVeinDensity(Lnet/minecraft/world/level/levelgen/material/rule/OreVeinRule$VeinType;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;Z)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `slideOverworld(ZLnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `slideNetherLike(Lnet/minecraft/core/HolderGetter;II)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `slideEndLike(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;II)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `nether(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/levelgen/NoiseRouter;` | protected static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `caves(Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/levelgen/NoiseRouter;` | protected static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `floatingIslands(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/levelgen/NoiseRouter;` | protected static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `slideEnd(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `end(Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/levelgen/NoiseRouter;` | protected static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `simpleRouter(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;)Lnet/minecraft/world/level/levelgen/NoiseRouter;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `splineWithBlending(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `noiseGradientDensity(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `preliminarySurfaceLevel(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;Z)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `yLimitedInterpolatable(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;IIIII)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.NoiseRouterData` | `slide(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;IIIIFIIF)Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;` | private static | public static |
| `net.minecraft.world.level.levelgen.feature.foliageplacers.FoliagePlacerType` | `<init>(Lcom/mojang/serialization/MapCodec;)V` | private | public |
| `net.minecraft.world.level.levelgen.feature.rootplacers.RootPlacerType` | `<init>(Lcom/mojang/serialization/MapCodec;)V` | private | public |
| `net.minecraft.world.level.levelgen.feature.treedecorators.TreeDecoratorType` | `<init>(Lcom/mojang/serialization/MapCodec;)V` | private | public |
| `net.minecraft.world.level.levelgen.feature.trunkplacers.TrunkPlacerType` | `<init>(Lcom/mojang/serialization/MapCodec;)V` | private | public |
| [[40-Interfaces/net.minecraft.world.level.storage.loot.LootPool|LootPool]] | `entries` | private final | public final |
| [[40-Interfaces/net.minecraft.world.level.storage.loot.LootPool|LootPool]] | `condition` | private final | public final |
| [[40-Interfaces/net.minecraft.world.level.storage.loot.LootPool|LootPool]] | `modifier` | private final | public final |
| [[40-Interfaces/net.minecraft.world.level.storage.loot.LootPool|LootPool]] | `rolls` | private final | public final |
| [[40-Interfaces/net.minecraft.world.level.storage.loot.LootPool|LootPool]] | `bonusRolls` | private final | public final |
| `net.minecraft.world.level.storage.loot.functions.ModifyContainerContents` | `<init>(Ljava/util/Optional;Lnet/minecraft/world/level/storage/loot/ContainerComponentManipulator;Lnet/minecraft/core/Holder;)V` | private | public |
| `net.minecraft.world.level.storage.loot.functions.SetComponentsFunction` | `<init>(Ljava/util/Optional;Lnet/minecraft/core/component/DataComponentPatch;)V` | private | public |
| `net.minecraft.world.level.storage.loot.functions.SetFireworksFunction` | `<init>(Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;)V` | protected | public |
| `net.minecraft.world.level.storage.loot.functions.SetItemFunction` | `<init>(Ljava/util/Optional;Lnet/minecraft/core/Holder;)V` | private | public |
| `net.minecraft.world.level.storage.loot.functions.SetWrittenBookPagesFunction` | `<init>(Ljava/util/Optional;Ljava/util/List;Lnet/minecraft/world/level/storage/loot/functions/ListOperation;)V` | protected | public |
| `net.minecraft.world.level.storage.loot.functions.ToggleTooltips` | `<init>(Ljava/util/Optional;Ljava/util/Map;)V` | private | public |
| `net.minecraft.world.level.storage.loot.providers.number.ints.ContextIntProviders` | `compostable(Lnet/minecraft/core/HolderGetter;I)Lnet/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProvider;` | private static | public static |
| `net.minecraft.world.level.storage.loot.providers.number.ints.ContextIntProviders` | `cooking(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/core/Holder$Reference;Lnet/minecraft/core/Holder$Reference;I)Lnet/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProvider;` | private static | public static |
| `net.minecraft.world.scores.criteria.ObjectiveCriteria` | `registerCustom(Ljava/lang/String;ZLnet/minecraft/world/scores/criteria/ObjectiveCriteria$RenderType;)Lnet/minecraft/world/scores/criteria/ObjectiveCriteria;` | private static | public static |
| `net.minecraft.world.scores.criteria.ObjectiveCriteria` | `registerCustom(Ljava/lang/String;)Lnet/minecraft/world/scores/criteria/ObjectiveCriteria;` | private static | public static |
