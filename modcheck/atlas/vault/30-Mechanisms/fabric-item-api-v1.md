---
type: "mechanism"
module: "fabric-item-api-v1"
version: "14.7.0+cf6bc2db5d"
sha256: "aff8cffc3d6da5475f21060e45cd28307676974674c4e10504c0fcc89c7746c4"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-item-api-v1

**Version** `14.7.0+cf6bc2db5d` -- **artifact sha256** `aff8cffc3d6da5475f21060e45cd28307676974674c4e10504c0fcc89c7746c4`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-resource-loader-v1": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-item-api-v1.mixins.json", {"config": "fabric-item-api-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-item-api-v1.classtweaker`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.item.v1.ItemTooltipCallback.EVENT|ItemTooltipCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.BlockTransformerEvents.MODIFY|BlockTransformerEvents.MODIFY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.DefaultItemComponentEvents.MODIFY|DefaultItemComponentEvents.MODIFY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.EnchantmentEvents.ALLOW_ENCHANTING|EnchantmentEvents.ALLOW_ENCHANTING]]
- [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.EnchantmentEvents.MODIFY|EnchantmentEvents.MODIFY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.EnchantmentEvents.MODIFY_WITH_LOOKUP|EnchantmentEvents.MODIFY_WITH_LOOKUP]]
- [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.ItemClickBehaviorCallback.EVENT|ItemClickBehaviorCallback.EVENT]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]] | `sameDestroyTarget` | wraps `@Redirect at INVOKE Lnet/minecraft/world/item/ItemStack;isSameItemSameComponents(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)Z` | client | `MultiPlayerGameModeMixin.fabricItemContinueBlockBreakingInject` |
| [[40-Interfaces/net.minecraft.client.player.FirstPersonHandsAndItems|FirstPersonHandsAndItems]] | `tick` | injects_into `@Inject at HEAD` | client | `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` |
| [[40-Interfaces/net.minecraft.core.component.BlockTransformer|BlockTransformer]] | `<clinit>` | injects_into `@ModifyArg at INVOKE Lcom/mojang/serialization/Codec;listOf(II)Lcom/mojang/serialization/Codec;` | both | `BlockTransformerMixin.removeBlockTransformerSizeLimit` |
| [[40-Interfaces/net.minecraft.core.component.DataComponentInitializers_1|DataComponentInitializers$1]] | `<init>` | injects_into `@Inject at TAIL` | both | `DataComponentInitializersPendingComponentsMixin.store` |
| [[40-Interfaces/net.minecraft.core.component.DataComponentInitializers_1|DataComponentInitializers$1]] | `apply` | injects_into `@Inject at RETURN` | both | `DataComponentInitializersPendingComponentsMixin.apply` |
| [[40-Interfaces/net.minecraft.core.registries.BuiltInRegistries|BuiltInRegistries]] | `freeze` | injects_into `@Inject at HEAD` | both | `BuiltInRegistriesMixin.modifyDefaultItemComponents` |
| [[40-Interfaces/net.minecraft.resources.ResourceManagerRegistryLoadTask|ResourceManagerRegistryLoadTask]] | `load` | injects_into `@Inject at HEAD` | both | `ResourceManagerRegistryLoadTaskMixin.captureRegistries` |
| [[40-Interfaces/net.minecraft.server.commands.EnchantCommand|EnchantCommand]] | `enchant` | wraps `@Redirect at INVOKE Lnet/minecraft/world/item/enchantment/Enchantment;canEnchant(Lnet/minecraft/world/item/ItemStack;)Z` | both | `EnchantCommandMixin.callAllowEnchantingEvent` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `getEquipmentSlotForItem` | injects_into `@Inject at HEAD` | both | `LivingEntityMixin.onGetPreferredEquipmentSlot` |
| [[40-Interfaces/net.minecraft.world.inventory.AbstractContainerMenu|AbstractContainerMenu]] | `tryItemClickBehaviourOverride` | injects_into `@Inject at HEAD` | both | `AbstractContainerMenuMixin.overrideContainerMenuItemClickBehaviour` |
| [[40-Interfaces/net.minecraft.world.inventory.AnvilMenu|AnvilMenu]] | `createResult` | wraps `@Redirect at INVOKE Lnet/minecraft/world/item/enchantment/Enchantment;canEnchant(Lnet/minecraft/world/item/ItemStack;)Z` | both | `AnvilMenuMixin.callAllowEnchantingEvent` |
| [[40-Interfaces/net.minecraft.world.item.Item|Item]] | `<init>` | injects_into `@Inject at RETURN` | both | `ItemMixin.onConstruct` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]] | `addDetailsToTooltip` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/world/item/ItemStack;addToTooltip(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/item/component/TooltipDisplay;Ljava/util/function/Consumer;Lnet/minecraft/world/item/TooltipFlag;)V` | both | `ItemStackMixin.preAppendComponentTooltip` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]] | `addDetailsToTooltip` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/world/item/component/TooltipDisplay;shows(Lnet/minecraft/core/component/DataComponentType;)Z` | both | `ItemStackMixin.preShouldDisplay` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]] | `addDetailsToTooltip` | injects_into `@Inject at INVOKE Lnet/minecraft/world/item/ItemStack;addAttributeTooltips(Ljava/util/function/Consumer;Lnet/minecraft/world/item/component/TooltipDisplay;Lnet/minecraft/world/entity/player/Player;)V` | both | `ItemStackMixin.preAttributeModifiers` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]] | `addDetailsToTooltip` | injects_into `@Inject at INVOKE Lnet/minecraft/core/DefaultedRegistry;getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | both | `ItemStackMixin.postTooltipsAdvanced` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]] | `getTooltipLines` | injects_into `@Inject at RETURN` | client | `ItemStackMixin.getTooltip` |
| [[40-Interfaces/net.minecraft.world.item.crafting.CraftingRecipe|CraftingRecipe]] | `defaultCraftingReminder` | wraps `@Redirect at INVOKE Lnet/minecraft/world/item/Item;getCraftingRemainder()Lnet/minecraft/world/item/ItemStackTemplate;` | both | `CraftingRecipeMixin.getStackRemainder` |
| [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment_Builder|Enchantment$Builder]] | `*` | injects_into `@Inject at RETURN` | both | `EnchantmentBuilderMixin.markModified` |
| [[40-Interfaces/net.minecraft.world.item.enchantment.EnchantmentHelper|EnchantmentHelper]] | `lambda$getAvailableEnchantmentResults$0` | wraps `@Redirect at INVOKE Lnet/minecraft/world/item/enchantment/Enchantment;isPrimaryItem(Lnet/minecraft/world/item/ItemStack;)Z` | both | `EnchantmentHelperMixin.useCustomEnchantingChecks` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity|AbstractFurnaceBlockEntity]] | `consumeFuel` | injects_into `@Inject at HEAD` | both | `AbstractFurnaceBlockEntityMixin.copyStack` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity|AbstractFurnaceBlockEntity]] | `consumeFuel` | wraps `@Redirect at INVOKE Lnet/minecraft/world/item/Item;getCraftingRemainder()Lnet/minecraft/world/item/ItemStackTemplate;` | both | `AbstractFurnaceBlockEntityMixin.getCraftingRemainder` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.BrewingStandBlockEntity|BrewingStandBlockEntity]] | `doBrew` | wraps `@Redirect at INVOKE Lnet/minecraft/world/item/Item;getCraftingRemainder()Lnet/minecraft/world/item/ItemStackTemplate;` | both | `BrewingStandBlockEntityMixin.getCraftingRemainder` |
| [[40-Interfaces/net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction|EnchantRandomlyFunction]] | `lambda$run$1` | wraps `@Redirect at INVOKE Lnet/minecraft/world/item/enchantment/Enchantment;canEnchant(Lnet/minecraft/world/item/ItemStack;)Z` | both | `EnchantRandomlyFunctionMixin.callAllowEnchantingEvent` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.item.v1.ItemTooltipCallback|ItemTooltipCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.BlockTransformerEvents|BlockTransformerEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.BlockTransformerHelper|BlockTransformerHelper]] (class, 42 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.CustomDamageHandler|CustomDamageHandler]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.DefaultItemComponentEvents|DefaultItemComponentEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.EnchantingContext|EnchantingContext]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.EnchantmentEvents|EnchantmentEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.EnchantmentSource|EnchantmentSource]] (class, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.EquipmentSlotProvider|EquipmentSlotProvider]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.FabricComponentMapBuilder|FabricComponentMapBuilder]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.FabricItem|FabricItem]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.FabricItemStack|FabricItemStack]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.FabricTooltipFlag|FabricTooltipFlag]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.ItemClickBehaviorCallback|ItemClickBehaviorCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.ItemComponentTooltipProviderRegistry|ItemComponentTooltipProviderRegistry]] (interface, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.ResourceSource|ResourceSource]] (class, 7 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
