---
type: "system"
package: "net.minecraft.world.item"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item

Analyst note: [[_authored/systems/net.minecraft.world.item|Items, item stacks and creative tabs]]

460 classes (323 top-level) across 14 packages in the processed jar; 21 changed by Loom processing; 42 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.world.item.BlockItem|BlockItem]] -- calls:2 -- by fabric-data-generation-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.item.BrushItem|BrushItem]] -- injects_into:1 -- by fabric-particles-v1
- [[40-Interfaces/net.minecraft.world.item.BucketItem|BucketItem]] -- injects_into:1, reads:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.CreativeModeTab|CreativeModeTab]] -- calls:13, injects_into:1, reads:2 -- by fabric-creative-tab-api-v1, fabric-data-generation-api-v1
- [[40-Interfaces/net.minecraft.world.item.CreativeModeTab_Builder|CreativeModeTab$Builder]] -- calls:3 -- by fabric-creative-tab-api-v1
- [[40-Interfaces/net.minecraft.world.item.CreativeModeTab_ItemDisplayParameters|CreativeModeTab$ItemDisplayParameters]] -- calls:4 -- by fabric-creative-tab-api-v1
- [[40-Interfaces/net.minecraft.world.item.CreativeModeTab_Row|CreativeModeTab$Row]] -- reads:3 -- by fabric-creative-tab-api-v1
- [[40-Interfaces/net.minecraft.world.item.CreativeModeTab_TabVisibility|CreativeModeTab$TabVisibility]] -- calls:12, reads:9 -- by fabric-creative-tab-api-v1
- [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] -- calls:7, injects_into:2, reads:21 -- by fabric-creative-tab-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.item.HoneycombItem|HoneycombItem]] -- injects_into:1, reads:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.item.Item|Item]] -- calls:17, injects_into:1, reads:1 -- by fabric-creative-tab-api-v1, fabric-data-generation-api-v1, fabric-item-api-v1, fabric-recipe-api-v1, fabric-registry-sync-v0, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.Item_Properties|Item$Properties]] -- reads:3, writes:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]] -- calls:106, injects_into:6, reads:7, wraps:3 -- by fabric-api-lookup-api-v1, fabric-content-registries-v0, fabric-creative-tab-api-v1, fabric-events-interaction-v0, fabric-item-api-v1, fabric-recipe-api-v1, fabric-rendering-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.ItemStackTemplate|ItemStackTemplate]] -- calls:9 -- by fabric-recipe-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.Items|Items]] -- reads:35 -- by fabric-data-generation-api-v1, fabric-item-api-v1, fabric-recipe-api-v1, fabric-registry-sync-v0, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.TooltipFlag|TooltipFlag]] -- calls:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.TooltipFlag_Default|TooltipFlag$Default]] -- reads:4 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.alchemy.PotionContents|PotionContents]] -- calls:3, reads:2 -- by fabric-item-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.alchemy.Potions|Potions]] -- reads:4 -- by fabric-data-generation-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.component.BlockTransformers|BlockTransformers]] -- reads:3 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.item.component.BundleContents|BundleContents]] -- calls:9, reads:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.component.BundleContents_Mutable|BundleContents$Mutable]] -- calls:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.component.Compostable|Compostable]] -- calls:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.component.CustomData|CustomData]] -- calls:2 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.component.ItemContainerContents|ItemContainerContents]] -- calls:2, reads:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.CraftingInput|CraftingInput]] -- calls:3 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.CraftingRecipe|CraftingRecipe]] -- wraps:2 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]] -- calls:28, injects_into:7, reads:12 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.Recipe|Recipe]] -- calls:5, reads:1 -- by fabric-data-generation-api-v1, fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.RecipeHolder|RecipeHolder]] -- calls:10 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.RecipeInput|RecipeInput]] -- calls:1 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.RecipeManager|RecipeManager]] -- injects_into:1, reads:1 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.RecipeMap|RecipeMap]] -- calls:5, injects_into:1 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.RecipeSerializer|RecipeSerializer]] -- calls:3 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.RecipeType|RecipeType]] -- calls:1 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.ShapelessRecipe|ShapelessRecipe]] -- injects_into:2, reads:1 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.display.SlotDisplay_Composite|SlotDisplay$Composite]] -- calls:4 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.display.SlotDisplay_ItemStackSlotDisplay|SlotDisplay$ItemStackSlotDisplay]] -- calls:2 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment|Enchantment]] -- calls:9 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment_Builder|Enchantment$Builder]] -- calls:1, injects_into:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.item.enchantment.EnchantmentHelper|EnchantmentHelper]] -- wraps:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.item.enchantment.ItemEnchantments|ItemEnchantments]] -- calls:1 -- by fabric-item-api-v1

## Declared inventory

### `net.minecraft.world.item` (98 top-level)

`AdventureModePredicate`, `AirItem`, `ArmorStandItem`, `ArrayItemProvider`, `ArrowItem`, `BannerItem`, [[40-Interfaces/net.minecraft.world.item.BlockItem|BlockItem]], `BoatItem`, `BoneMealItem`, `BottleItem`, `BowItem`, [[40-Interfaces/net.minecraft.world.item.BrushItem|BrushItem]], [[40-Interfaces/net.minecraft.world.item.BucketItem|BucketItem]], `BundleItem`, `CompassItem`, [[40-Interfaces/net.minecraft.world.item.CreativeModeTab|CreativeModeTab]], [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]], `CrossbowItem`, `CushionItem`, `DebugStickItem`, `DiscFragmentItem`, `DispensibleContainerItem`, `DoubleHighBlockItem`, `DyeColor`, `DyeItem`, `EggItem`, `EmptyMapItem`, `EndCrystalItem`, `EnderEyeItem`, `EnderpearlItem`, `ExperienceBottleItem`, `FireChargeItem`, `FireworkRocketItem`, `FishingRodItem`, `FlintAndSteelItem`, `FoodOnAStickItem`, `GameMasterBlockItem`, `GlowInkSacItem`, `HangingEntityItem`, `HangingSignItem`, [[40-Interfaces/net.minecraft.world.item.HoneycombItem|HoneycombItem]], `InkSacItem`, `Instrument`, `InstrumentItem`, `Instruments`, [[40-Interfaces/net.minecraft.world.item.Item|Item]], `ItemCooldowns`, `ItemDisplayContext`, `ItemFrameItem`, `ItemInstance`, `ItemProvider`, [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]], `ItemStackLinkedSet`, [[40-Interfaces/net.minecraft.world.item.ItemStackTemplate|ItemStackTemplate]], `ItemUseAnimation`, `ItemUtils`, [[40-Interfaces/net.minecraft.world.item.Items|Items]], `JukeboxPlayable`, `JukeboxSong`, `JukeboxSongPlayer`, `JukeboxSongs`, `KnowledgeBookItem`, `LeadItem`, `LingeringPotionItem`, `MaceItem`, `MapItem`, `MinecartItem`, `MobBucketItem`, `NameTagItem`, `PlaceOnWaterBlockItem`, `PlayerHeadItem`, `PotionItem`, `ProjectileItem`, `ProjectileWeaponItem`, `Rarity`, `ScaffoldingBlockItem`, `ServerItemCooldowns`, `ShearsItem`, `ShieldItem`, `SignApplicator`, `SmithingTemplateItem`, `SnowballItem`, `SolidBucketItem`, `SpawnEggItem`, `SpectralArrowItem`, `SplashPotionItem`, `SpyglassItem`, `StandingAndWallBlockItem`, `SwingAnimationType`, `ThrowablePotionItem`, `TippedArrowItem`, `ToolMaterial`, [[40-Interfaces/net.minecraft.world.item.TooltipFlag|TooltipFlag]], `TridentItem`, `WindChargeItem`, `WritableBookItem`, `WrittenBookItem`, `package-info`

### `net.minecraft.world.item.alchemy` (5 top-level)

`Potion`, [[40-Interfaces/net.minecraft.world.item.alchemy.PotionContents|PotionContents]], `PotionIds`, [[40-Interfaces/net.minecraft.world.item.alchemy.Potions|Potions]], `package-info`

### `net.minecraft.world.item.component` (53 top-level)

`AttackRange`, `Bees`, `BlockItemStateProperties`, [[40-Interfaces/net.minecraft.world.item.component.BlockTransformers|BlockTransformers]], `BlocksAttacks`, `BookContent`, `BrewingFuel`, [[40-Interfaces/net.minecraft.world.item.component.BundleContents|BundleContents]], `ChargedProjectiles`, [[40-Interfaces/net.minecraft.world.item.component.Compostable|Compostable]], `Consumable`, `ConsumableListener`, `Consumables`, `ContainerComponent`, `CookingFuel`, [[40-Interfaces/net.minecraft.world.item.component.CustomData|CustomData]], `CustomModelData`, `DamageResistant`, `DeathProtection`, `DebugStickState`, `DyedItemColor`, `FireworkExplosion`, `Fireworks`, `GrowableMutableContainer`, `InstrumentComponent`, `ItemAttributeModifiers`, [[40-Interfaces/net.minecraft.world.item.component.ItemContainerContents|ItemContainerContents]], `ItemLore`, `KineticWeapon`, `LodestoneTracker`, `MapDecorations`, `MapPostProcessing`, `MobVisibility`, `OminousBottleAmplifier`, `PiercingWeapon`, `ProvidesTrimMaterial`, `ResolvableProfile`, `SeededContainerLoot`, `SimpleMutableContainer`, `SulfurCubeContent`, `SuspiciousStewEffects`, `SwingAnimation`, `Tool`, `TooltipDisplay`, `TooltipProvider`, `TypedEntityData`, `UseCooldown`, `UseEffects`, `UseRemainder`, `Weapon`, `WritableBookContent`, `WrittenBookContent`, `package-info`

### `net.minecraft.world.item.consume_effects` (7 top-level)

`ApplyStatusEffectsConsumeEffect`, `ClearAllStatusEffectsConsumeEffect`, `ConsumeEffect`, `PlaySoundConsumeEffect`, `RemoveStatusEffectsConsumeEffect`, `TeleportRandomlyConsumeEffect`, `package-info`

### `net.minecraft.world.item.context` (4 top-level)

`BlockPlaceContext`, `DirectionalPlaceContext`, `UseOnContext`, `package-info`

### `net.minecraft.world.item.crafting` (56 top-level)

`AbstractCookingRecipe`, `BannerDuplicateRecipe`, `BlastingRecipe`, `BookCloningRecipe`, `BrewingInput`, `BrewingRecipe`, `CampfireCookingRecipe`, `CookingBookCategory`, `CraftingBookCategory`, [[40-Interfaces/net.minecraft.world.item.crafting.CraftingInput|CraftingInput]], [[40-Interfaces/net.minecraft.world.item.crafting.CraftingRecipe|CraftingRecipe]], `CustomRecipe`, `DecoratedPotRecipe`, `DyeRecipe`, `ExtendedRecipeBookCategory`, `FireworkRocketRecipe`, `FireworkStarFadeRecipe`, `FireworkStarRecipe`, `ImbueRecipe`, [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]], `MapExtendingRecipe`, `NormalCraftingRecipe`, `PlacementInfo`, `PotionIngredient`, [[40-Interfaces/net.minecraft.world.item.crafting.Recipe|Recipe]], `RecipeAccess`, `RecipeBookCategories`, `RecipeBookCategory`, `RecipeCache`, [[40-Interfaces/net.minecraft.world.item.crafting.RecipeHolder|RecipeHolder]], [[40-Interfaces/net.minecraft.world.item.crafting.RecipeInput|RecipeInput]], [[40-Interfaces/net.minecraft.world.item.crafting.RecipeManager|RecipeManager]], [[40-Interfaces/net.minecraft.world.item.crafting.RecipeMap|RecipeMap]], `RecipePropertySet`, [[40-Interfaces/net.minecraft.world.item.crafting.RecipeSerializer|RecipeSerializer]], `RecipeSerializers`, [[40-Interfaces/net.minecraft.world.item.crafting.RecipeType|RecipeType]], `RepairItemRecipe`, `SelectableRecipe`, `ShapedRecipe`, `ShapedRecipePattern`, [[40-Interfaces/net.minecraft.world.item.crafting.ShapelessRecipe|ShapelessRecipe]], `ShieldDecorationRecipe`, `SimpleSmithingRecipe`, `SingleItemRecipe`, `SingleRecipeInput`, `SmeltingRecipe`, `SmithingRecipe`, `SmithingRecipeInput`, `SmithingTransformRecipe`, `SmithingTrimRecipe`, `SmokingRecipe`, `StonecutterRecipe`, `TransmuteRecipe`, `TransmuteResult`, `package-info`

### `net.minecraft.world.item.crafting.display` (14 top-level)

`DisplayContentsFactory`, `FurnaceRecipeDisplay`, `RecipeDisplay`, `RecipeDisplayEntry`, `RecipeDisplayId`, `RecipeDisplays`, `ShapedCraftingRecipeDisplay`, `ShapelessCraftingRecipeDisplay`, `SlotDisplay`, `SlotDisplayContext`, `SlotDisplays`, `SmithingRecipeDisplay`, `StonecutterRecipeDisplay`, `package-info`

### `net.minecraft.world.item.enchantment` (14 top-level)

`ConditionalEffect`, `Enchantable`, `EnchantedItemInUse`, [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment|Enchantment]], `EnchantmentEffectComponents`, [[40-Interfaces/net.minecraft.world.item.enchantment.EnchantmentHelper|EnchantmentHelper]], `EnchantmentInstance`, `EnchantmentTarget`, `Enchantments`, [[40-Interfaces/net.minecraft.world.item.enchantment.ItemEnchantments|ItemEnchantments]], `LevelBasedValue`, `Repairable`, `TargetedConditionalEffect`, `package-info`

### `net.minecraft.world.item.enchantment.effects` (26 top-level)

`AddValue`, `AllOf`, `ApplyEntityImpulse`, `ApplyExhaustion`, `ApplyMobEffect`, `ChangeItemDamage`, `DamageEntity`, `DamageImmunity`, `EnchantmentAttributeEffect`, `EnchantmentEntityEffect`, `EnchantmentLocationBasedEffect`, `EnchantmentValueEffect`, `ExplodeEffect`, `Ignite`, `MultiplyValue`, `PlaySoundEffect`, `RemoveBinomial`, `ReplaceBlock`, `ReplaceDisk`, `RunFunction`, `ScaleExponentially`, `SetBlockProperties`, `SetValue`, `SpawnParticlesEffect`, `SummonEntityEffect`, `package-info`

### `net.minecraft.world.item.enchantment.providers` (7 top-level)

`EnchantmentProvider`, `EnchantmentProviderTypes`, `EnchantmentsByCost`, `EnchantmentsByCostWithDifficulty`, `SingleEnchantment`, `VanillaEnchantmentProviders`, `package-info`

### `net.minecraft.world.item.equipment` (8 top-level)

`AllowedEntitiesProvider`, `ArmorMaterial`, `ArmorMaterials`, `ArmorType`, `EquipmentAsset`, `EquipmentAssets`, `Equippable`, `package-info`

### `net.minecraft.world.item.equipment.trim` (6 top-level)

`ArmorTrim`, `TrimMaterial`, `TrimMaterials`, `TrimPattern`, `TrimPatterns`, `package-info`

### `net.minecraft.world.item.slot` (14 top-level)

`CompositeSlotSource`, `ContentsSlotSource`, `CountingModifier`, `EmptySlotSource`, `FilteredSlotSource`, `GroupSlotSource`, `LimitSlotSource`, `RangeSlotSource`, `SlotCollection`, `SlotSelector`, `SlotSource`, `SlotSources`, `TransformedSlotSource`, `package-info`

### `net.minecraft.world.item.trading` (11 top-level)

`ItemCost`, `Merchant`, `MerchantOffer`, `MerchantOffers`, `TradeCost`, `TradeRebalanceVillagerTrades`, `TradeSet`, `TradeSets`, `VillagerTrade`, `VillagerTrades`, `package-info`

