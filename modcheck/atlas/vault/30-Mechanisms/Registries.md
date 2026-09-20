---
type: "mechanism"
artifact: "minecraft-merged"
sha256: "5918174887871ab0d484d4fa9462a4c5f91d90b3b93dd301e3bfa4c109bc4cde"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Registries

Read from `BuiltInRegistries` and `Registries` in the 26.3 jar (`5918174887871ab0`). Each built-in registry is a target for `Registry.register`; each key with a datapack type is also addable from data.

## Built-in registries (97)

| registry | element type |
|---|---|
| `DATA_COMPONENT_INITIALIZERS` | `net.minecraft.core.component.DataComponentInitializers` |
| `GAME_EVENT` | `net.minecraft.core.DefaultedRegistry<net.minecraft.world.level.gameevent.GameEvent>` |
| `SOUND_EVENT` | `net.minecraft.core.Registry<net.minecraft.sounds.SoundEvent>` |
| `FLUID` | `net.minecraft.core.DefaultedRegistry<net.minecraft.world.level.material.Fluid>` |
| `MOB_EFFECT` | `net.minecraft.core.Registry<net.minecraft.world.effect.MobEffect>` |
| `BLOCK` | `net.minecraft.core.DefaultedRegistry<net.minecraft.world.level.block.Block>` |
| `DEBUG_SUBSCRIPTION` | `net.minecraft.core.Registry<net.minecraft.util.debug.DebugSubscription<?>>` |
| `ENTITY_TYPE` | `net.minecraft.core.DefaultedRegistry<net.minecraft.world.entity.EntityType<?>>` |
| `ITEM` | `net.minecraft.core.DefaultedRegistry<net.minecraft.world.item.Item>` |
| `POTION` | `net.minecraft.core.Registry<net.minecraft.world.item.alchemy.Potion>` |
| `PARTICLE_TYPE` | `net.minecraft.core.Registry<net.minecraft.core.particles.ParticleType<?>>` |
| `BLOCK_ENTITY_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.level.block.entity.BlockEntityType<?>>` |
| `CUSTOM_STAT` | `net.minecraft.core.Registry<net.minecraft.resources.Identifier>` |
| `CHUNK_STATUS` | `net.minecraft.core.DefaultedRegistry<net.minecraft.world.level.chunk.status.ChunkStatus>` |
| `RULE_TEST` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.structure.templatesystem.RuleTestType<?>>` |
| `RULE_BLOCK_ENTITY_MODIFIER` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.structure.templatesystem.rule.blockentity.RuleBlockEntityModifierType<?>>` |
| `POS_RULE_TEST` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.structure.templatesystem.PosRuleTestType<?>>` |
| `MENU` | `net.minecraft.core.Registry<net.minecraft.world.inventory.MenuType<?>>` |
| `RECIPE_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.item.crafting.RecipeType<?>>` |
| `RECIPE_SERIALIZER` | `net.minecraft.core.Registry<net.minecraft.world.item.crafting.RecipeSerializer<?>>` |
| `ATTRIBUTE` | `net.minecraft.core.Registry<net.minecraft.world.entity.ai.attributes.Attribute>` |
| `POSITION_SOURCE_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.level.gameevent.PositionSourceType<?>>` |
| `COMMAND_ARGUMENT_TYPE` | `net.minecraft.core.Registry<net.minecraft.commands.synchronization.ArgumentTypeInfo<?, ?>>` |
| `STAT_TYPE` | `net.minecraft.core.Registry<net.minecraft.stats.StatType<?>>` |
| `VILLAGER_TYPE` | `net.minecraft.core.DefaultedRegistry<net.minecraft.world.entity.npc.villager.VillagerType>` |
| `VILLAGER_PROFESSION` | `net.minecraft.core.DefaultedRegistry<net.minecraft.world.entity.npc.villager.VillagerProfession>` |
| `POINT_OF_INTEREST_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.entity.ai.village.poi.PoiType>` |
| `MEMORY_MODULE_TYPE` | `net.minecraft.core.DefaultedRegistry<net.minecraft.world.entity.ai.memory.MemoryModuleType<?>>` |
| `SENSOR_TYPE` | `net.minecraft.core.DefaultedRegistry<net.minecraft.world.entity.ai.sensing.SensorType<?>>` |
| `ACTIVITY` | `net.minecraft.core.Registry<net.minecraft.world.entity.schedule.Activity>` |
| `LOOT_POOL_ENTRY_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.storage.loot.entries.LootPoolEntryContainer>>` |
| `LOOT_FUNCTION_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.storage.loot.functions.LootItemFunction>>` |
| `LOOT_CONDITION_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.storage.loot.predicates.LootItemCondition>>` |
| `CONTEXT_FLOAT_PROVIDER_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.storage.loot.providers.number.floats.ContextFloatProvider>>` |
| `CONTEXT_INT_PROVIDER_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.storage.loot.providers.number.ints.ContextIntProvider>>` |
| `LOOT_NBT_PROVIDER_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.storage.loot.providers.nbt.NbtProvider>>` |
| `LOOT_SCORE_PROVIDER_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.storage.loot.providers.score.ScoreboardNameProvider>>` |
| `FLOAT_PROVIDER_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.util.valueproviders.FloatProvider>>` |
| `INT_PROVIDER_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.util.valueproviders.IntProvider>>` |
| `HEIGHT_PROVIDER_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.heightproviders.HeightProviderType<?>>` |
| `BLOCK_PREDICATE_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.blockpredicates.BlockPredicateType<?>>` |
| `CARVER_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.levelgen.carver.WorldCarver>>` |
| `FEATURE_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.levelgen.feature.Feature>>` |
| `STRUCTURE_PLACEMENT` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.levelgen.structure.placement.StructurePlacement>>` |
| `STRUCTURE_PIECE` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.structure.pieces.StructurePieceType>` |
| `STRUCTURE_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.structure.StructureType<?>>` |
| `PLACEMENT_MODIFIER_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.levelgen.placement.PlacementModifier>>` |
| `BLOCK_STATE_PROVIDER_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider>>` |
| `FOLIAGE_PLACER_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.feature.foliageplacers.FoliagePlacerType<?>>` |
| `TRUNK_PLACER_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.feature.trunkplacers.TrunkPlacerType<?>>` |
| `ROOT_PLACER_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.feature.rootplacers.RootPlacerType<?>>` |
| `TREE_DECORATOR_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.feature.treedecorators.TreeDecoratorType<?>>` |
| `FEATURE_SIZE_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.feature.featuresize.FeatureSizeType<?>>` |
| `BIOME_SOURCE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.biome.BiomeSource>>` |
| `CHUNK_GENERATOR` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.chunk.ChunkGenerator>>` |
| `MATERIAL_CONDITION_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.levelgen.material.condition.MaterialCondition>>` |
| `MATERIAL_RULE_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.levelgen.material.rule.MaterialRule>>` |
| `DENSITY_FUNCTION_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.levelgen.densityfunction.DensityFunction>>` |
| `STRUCTURE_PROCESSOR` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.levelgen.structure.templatesystem.StructureProcessor>>` |
| `STRUCTURE_POOL_ELEMENT` | `net.minecraft.core.Registry<net.minecraft.world.level.levelgen.structure.pools.StructurePoolElementType<?>>` |
| `POOL_ALIAS_BINDING_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.levelgen.structure.pools.alias.PoolAliasBinding>>` |
| `CREATIVE_MODE_TAB` | `net.minecraft.core.Registry<net.minecraft.world.item.CreativeModeTab>` |
| `TRIGGER_TYPES` | `net.minecraft.core.Registry<net.minecraft.advancements.triggers.CriterionTrigger<?>>` |
| `NUMBER_FORMAT_TYPE` | `net.minecraft.core.Registry<net.minecraft.network.chat.numbers.NumberFormatType<?>>` |
| `DATA_COMPONENT_TYPE` | `net.minecraft.core.Registry<net.minecraft.core.component.DataComponentType<?>>` |
| `GAME_RULE` | `net.minecraft.core.Registry<net.minecraft.world.level.gamerules.GameRule<?>>` |
| `ENTITY_SUB_PREDICATE_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.Codec<? extends net.minecraft.advancements.predicates.entity.EntitySubPredicate>>` |
| `DATA_COMPONENT_PREDICATE_TYPE` | `net.minecraft.core.Registry<net.minecraft.core.component.predicates.DataComponentPredicate$Type<?>>` |
| `MAP_DECORATION_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.level.saveddata.maps.MapDecorationType>` |
| `ENCHANTMENT_EFFECT_COMPONENT_TYPE` | `net.minecraft.core.Registry<net.minecraft.core.component.DataComponentType<?>>` |
| `ENCHANTMENT_LEVEL_BASED_VALUE_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.item.enchantment.LevelBasedValue>>` |
| `ENCHANTMENT_ENTITY_EFFECT_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.item.enchantment.effects.EnchantmentEntityEffect>>` |
| `ENCHANTMENT_LOCATION_BASED_EFFECT_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.item.enchantment.effects.EnchantmentLocationBasedEffect>>` |
| `ENCHANTMENT_VALUE_EFFECT_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.item.enchantment.effects.EnchantmentValueEffect>>` |
| `ENCHANTMENT_PROVIDER_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.item.enchantment.providers.EnchantmentProvider>>` |
| `CONSUME_EFFECT_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.item.consume_effects.ConsumeEffect$Type<?>>` |
| `RECIPE_DISPLAY` | `net.minecraft.core.Registry<net.minecraft.world.item.crafting.display.RecipeDisplay$Type<?>>` |
| `SLOT_DISPLAY` | `net.minecraft.core.Registry<net.minecraft.world.item.crafting.display.SlotDisplay$Type<?>>` |
| `RECIPE_BOOK_CATEGORY` | `net.minecraft.core.Registry<net.minecraft.world.item.crafting.RecipeBookCategory>` |
| `TICKET_TYPE` | `net.minecraft.core.Registry<net.minecraft.server.level.TicketType>` |
| `INCOMING_RPC_METHOD` | `net.minecraft.core.Registry<net.minecraft.server.jsonrpc.IncomingRpcMethod<?, ?>>` |
| `OUTGOING_RPC_METHOD` | `net.minecraft.core.Registry<net.minecraft.server.jsonrpc.OutgoingRpcMethod<?, ?>>` |
| `TEST_ENVIRONMENT_DEFINITION_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.gametest.framework.TestEnvironmentDefinition<?>>>` |
| `TEST_INSTANCE_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.gametest.framework.GameTestInstance>>` |
| `SPAWN_CONDITION_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.entity.variant.SpawnCondition>>` |
| `DIALOG_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.server.dialog.Dialog>>` |
| `DIALOG_ACTION_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.server.dialog.action.Action>>` |
| `INPUT_CONTROL_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.server.dialog.input.InputControl>>` |
| `DIALOG_BODY_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.server.dialog.body.DialogBody>>` |
| `PERMISSION_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.server.permissions.Permission>>` |
| `PERMISSION_CHECK_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.server.permissions.PermissionCheck>>` |
| `ENVIRONMENT_ATTRIBUTE` | `net.minecraft.core.Registry<net.minecraft.world.attribute.EnvironmentAttribute<?>>` |
| `ATTRIBUTE_TYPE` | `net.minecraft.core.Registry<net.minecraft.world.attribute.AttributeType<?>>` |
| `SLOT_SOURCE_TYPE` | `net.minecraft.core.Registry<com.mojang.serialization.MapCodec<? extends net.minecraft.world.item.slot.SlotSource>>` |
| `CONTEXT_KEY_SET` | `net.minecraft.core.Registry<net.minecraft.util.context.ContextKeySet>` |
| `TEST_FUNCTION` | `net.minecraft.core.Registry<java.util.function.Consumer<net.minecraft.gametest.framework.GameTestHelper>>` |
| `REGISTRY` | `net.minecraft.core.Registry<? extends net.minecraft.core.Registry<?>>` |

## Registry keys (157)

`ROOT_REGISTRY_NAME`, `ACTIVITY`, `ATTRIBUTE`, `BIOME_SOURCE`, `BLOCK_ENTITY_TYPE`, `BLOCK_PREDICATE_TYPE`, `BLOCK_STATE_PROVIDER_TYPE`, `BLOCK`, `CARVER_TYPE`, `CHUNK_GENERATOR`, `CHUNK_STATUS`, `COMMAND_ARGUMENT_TYPE`, `CONSUME_EFFECT_TYPE`, `CREATIVE_MODE_TAB`, `CUSTOM_STAT`, `DATA_COMPONENT_PREDICATE_TYPE`, `DATA_COMPONENT_TYPE`, `GAME_RULE`, `DEBUG_SUBSCRIPTION`, `DENSITY_FUNCTION_TYPE`, `DIALOG_BODY_TYPE`, `DIALOG_TYPE`, `ENCHANTMENT_EFFECT_COMPONENT_TYPE`, `ENCHANTMENT_ENTITY_EFFECT_TYPE`, `ENCHANTMENT_LEVEL_BASED_VALUE_TYPE`, `ENCHANTMENT_LOCATION_BASED_EFFECT_TYPE`, `ENCHANTMENT_PROVIDER_TYPE`, `ENCHANTMENT_VALUE_EFFECT_TYPE`, `ENTITY_SUB_PREDICATE_TYPE`, `ENTITY_TYPE`, `ENVIRONMENT_ATTRIBUTE`, `ATTRIBUTE_TYPE`, `FEATURE_SIZE_TYPE`, `FEATURE_TYPE`, `FLOAT_PROVIDER_TYPE`, `FLUID`, `FOLIAGE_PLACER_TYPE`, `GAME_EVENT`, `HEIGHT_PROVIDER_TYPE`, `INPUT_CONTROL_TYPE`, `INT_PROVIDER_TYPE`, `ITEM`, `SLOT_SOURCE_TYPE`, `LOOT_CONDITION_TYPE`, `LOOT_FUNCTION_TYPE`, `LOOT_NBT_PROVIDER_TYPE`, `CONTEXT_FLOAT_PROVIDER_TYPE`, `CONTEXT_INT_PROVIDER_TYPE`, `LOOT_POOL_ENTRY_TYPE`, `LOOT_SCORE_PROVIDER_TYPE`, `MAP_DECORATION_TYPE`, `MATERIAL_CONDITION_TYPE`, `MATERIAL_RULE_TYPE`, `MEMORY_MODULE_TYPE`, `MENU`, `MOB_EFFECT`, `NUMBER_FORMAT_TYPE`, `PARTICLE_TYPE`, `PLACEMENT_MODIFIER_TYPE`, `POINT_OF_INTEREST_TYPE`, `POOL_ALIAS_BINDING`, `POSITION_SOURCE_TYPE`, `POS_RULE_TEST`, `POTION`, `RECIPE_BOOK_CATEGORY`, `RECIPE_DISPLAY`, `RECIPE_SERIALIZER`, `RECIPE_TYPE`, `ROOT_PLACER_TYPE`, `RULE_BLOCK_ENTITY_MODIFIER`, `RULE_TEST`, `SENSOR_TYPE`, `SLOT_DISPLAY`, `SOUND_EVENT`, `SPAWN_CONDITION_TYPE`, `STAT_TYPE`, `STRUCTURE_PIECE`, `STRUCTURE_PLACEMENT`, `STRUCTURE_POOL_ELEMENT`, `STRUCTURE_PROCESSOR`, `STRUCTURE_TYPE`, `DIALOG_ACTION_TYPE`, `TEST_ENVIRONMENT_DEFINITION_TYPE`, `TEST_FUNCTION`, `TEST_INSTANCE_TYPE`, `TICKET_TYPE`, `TREE_DECORATOR_TYPE`, `TRUNK_PLACER_TYPE`, `VILLAGER_PROFESSION`, `VILLAGER_TYPE`, `INCOMING_RPC_METHOD`, `OUTGOING_RPC_METHOD`, `PERMISSION_TYPE`, `PERMISSION_CHECK_TYPE`, `CONTEXT_KEY_SET`, `BANNER_PATTERN`, `BIOME`, `BLOCK_STATE_PROVIDER`, `CAT_SOUND_VARIANT`, `CAT_VARIANT`, `CARVER`, `CHAT_TYPE`, `CHICKEN_SOUND_VARIANT`, `CHICKEN_VARIANT`, `ZOMBIE_NAUTILUS_VARIANT`, `FEATURE`, `COW_SOUND_VARIANT`, `COW_VARIANT`, `DAMAGE_TYPE`, `DENSITY_FUNCTION`, `DIALOG`, `DIMENSION_TYPE`, `ENCHANTMENT_PROVIDER`, `ENCHANTMENT`, `FLAT_LEVEL_GENERATOR_PRESET`, `FROG_VARIANT`, `INSTRUMENT`, `JUKEBOX_SONG`, `MATERIAL_CONDITION`, `MATERIAL_RULE`, `MULTI_NOISE_BIOME_SOURCE_PARAMETER_LIST`, `NOISE_SETTINGS`, `NOISE`, `PAINTING_VARIANT`, `PIG_SOUND_VARIANT`, `PIG_VARIANT`, `PLACED_FEATURE`, `PROCESSOR_LIST`, `STRUCTURE_SET`, `STRUCTURE`, `SULFUR_CUBE_ARCHETYPE`, `TEMPLATE_POOL`, `TEST_ENVIRONMENT`, `TEST_INSTANCE`, `TIMELINE`, `TRADE_SET`, `TRIAL_SPAWNER_CONFIG`, `TRIGGER_TYPE`, `TRIM_MATERIAL`, `TRIM_PATTERN`, `VILLAGER_TRADE`, `WOLF_VARIANT`, `WOLF_SOUND_VARIANT`, `WORLD_CLOCK`, `WORLD_PRESET`, `DECORATED_POT_PATTERN`, `BLOCK_TRANSFORMER`, `DIMENSION`, `LEVEL_STEM`, `LOOT_TABLE`, `ITEM_MODIFIER`, `PREDICATE`, `SLOT_SOURCE`, `CONTEXT_FLOAT_PROVIDER`, `CONTEXT_INT_PROVIDER`, `ADVANCEMENT`, `RECIPE`
