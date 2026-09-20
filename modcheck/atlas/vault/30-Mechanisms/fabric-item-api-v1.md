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
- mixin classes: 23 found by annotation, 23 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.item.v1.ItemTooltipCallback.EVENT|ItemTooltipCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.BlockTransformerEvents.MODIFY|BlockTransformerEvents.MODIFY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.DefaultItemComponentEvents.MODIFY|DefaultItemComponentEvents.MODIFY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.EnchantmentEvents.ALLOW_ENCHANTING|EnchantmentEvents.ALLOW_ENCHANTING]]
- [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.EnchantmentEvents.MODIFY|EnchantmentEvents.MODIFY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.EnchantmentEvents.MODIFY_WITH_LOOKUP|EnchantmentEvents.MODIFY_WITH_LOOKUP]]
- [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.ItemClickBehaviorCallback.EVENT|ItemClickBehaviorCallback.EVENT]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]].`sameDestroyTarget` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @Redirect | INVOKE `Lnet/minecraft/world/item/ItemStack;isSameItemSameComponents(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)Z` (exact) | client | 1000 (default) | `MultiPlayerGameModeMixin.fabricItemContinueBlockBreakingInject` |
| [[40-Interfaces/net.minecraft.client.player.FirstPersonHandsAndItems|FirstPersonHandsAndItems]].`tick` | `(Lnet/minecraft/client/player/LocalPlayer;)V` | name_only | @Inject | HEAD | client | 1000 (default) | `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` |
| [[40-Interfaces/net.minecraft.core.component.BlockTransformer|BlockTransformer]].`<clinit>` | `()V` | exact | @ModifyArg | INVOKE `Lcom/mojang/serialization/Codec;listOf(II)Lcom/mojang/serialization/Codec;` (exact) | both | 1000 (default) | `BlockTransformerMixin.removeBlockTransformerSizeLimit` |
| [[40-Interfaces/net.minecraft.core.component.DataComponentInitializers|DataComponentInitializers]].`createInitializerForRegistry` | `(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/component/DataComponentInitializers$PendingComponentBuilders;)Lnet/minecraft/core/component/DataComponentInitializers$PendingComponents;` | name_only | @WrapMethod | - | both | 1000 (default) | `DataComponentInitializersMixin.captureLookup` |
| [[40-Interfaces/net.minecraft.core.component.DataComponentInitializers_1|DataComponentInitializers$1]].`<init>` | `(Lnet/minecraft/resources/ResourceKey;Ljava/util/List;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `DataComponentInitializersPendingComponentsMixin.store` |
| [[40-Interfaces/net.minecraft.core.component.DataComponentInitializers_1|DataComponentInitializers$1]].`apply` | `()V` | name_only | @Inject | RETURN | both | 1000 (default) | `DataComponentInitializersPendingComponentsMixin.apply` |
| [[40-Interfaces/net.minecraft.core.registries.BuiltInRegistries|BuiltInRegistries]].`freeze` | `()V` | name_only | @Inject | HEAD | both | 1000 (default) | `BuiltInRegistriesMixin.modifyDefaultItemComponents` |
| [[40-Interfaces/net.minecraft.resources.ResourceManagerRegistryLoadTask|ResourceManagerRegistryLoadTask]].`lambda$load$2` | `(Lnet/minecraft/resources/FileToIdConverter;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/resources/Identifier;Lnet/minecraft/server/packs/resources/Resource;)Lnet/minecraft/resources/RegistryLoadTask$PendingRegistration;` | name_only | @WrapOperation | NEW `net/minecraft/resources/RegistryLoadTask$PendingRegistration` (exact) | both | 1000 (default) | `ResourceManagerRegistryLoadTaskMixin.modify` |
| [[40-Interfaces/net.minecraft.resources.ResourceManagerRegistryLoadTask|ResourceManagerRegistryLoadTask]].`load` | `(Lnet/minecraft/resources/RegistryOps$RegistryInfoLookup;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @Inject | HEAD | both | 1000 (default) | `ResourceManagerRegistryLoadTaskMixin.captureRegistries` |
| [[40-Interfaces/net.minecraft.server.commands.EnchantCommand|EnchantCommand]].`enchant` | `(Lnet/minecraft/commands/CommandSourceStack;Ljava/util/Collection;Lnet/minecraft/core/Holder;I)I` | name_only | @Redirect | INVOKE `Lnet/minecraft/world/item/enchantment/Enchantment;canEnchant(Lnet/minecraft/world/item/ItemStack;)Z` (exact) | both | 1000 (default) | `EnchantCommandMixin.callAllowEnchantingEvent` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`getEquipmentSlotForItem` | `(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/entity/EquipmentSlot;` | name_only | @Inject | HEAD | both | 1000 (default) | `LivingEntityMixin.onGetPreferredEquipmentSlot` |
| [[40-Interfaces/net.minecraft.world.inventory.AbstractContainerMenu|AbstractContainerMenu]].`tryItemClickBehaviourOverride` | `(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/inventory/ClickAction;Lnet/minecraft/world/inventory/Slot;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)Z` | name_only | @Inject | HEAD | both | 1000 (default) | `AbstractContainerMenuMixin.overrideContainerMenuItemClickBehaviour` |
| [[40-Interfaces/net.minecraft.world.inventory.AnvilMenu|AnvilMenu]].`createResult` | `()V` | name_only | @Redirect | INVOKE `Lnet/minecraft/world/item/enchantment/Enchantment;canEnchant(Lnet/minecraft/world/item/ItemStack;)Z` (exact) | both | 1000 (default) | `AnvilMenuMixin.callAllowEnchantingEvent` |
| [[40-Interfaces/net.minecraft.world.item.Item|Item]].`<init>` | `(Lnet/minecraft/world/item/Item$Properties;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `ItemMixin.onConstruct` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]].`addDetailsToTooltip` | `(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/item/component/TooltipDisplay;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/TooltipFlag;Ljava/util/function/Consumer;)V` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/world/item/ItemStack;addToTooltip(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/item/component/TooltipDisplay;Ljava/util/function/Consumer;Lnet/minecraft/world/item/TooltipFlag;)V` (exact) | both | 1000 (default) | `ItemStackMixin.preAppendComponentTooltip` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]].`addDetailsToTooltip` | `(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/item/component/TooltipDisplay;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/TooltipFlag;Ljava/util/function/Consumer;)V` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/world/item/component/TooltipDisplay;shows(Lnet/minecraft/core/component/DataComponentType;)Z` (exact) | both | 1000 (default) | `ItemStackMixin.preShouldDisplay` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]].`addDetailsToTooltip` | `(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/item/component/TooltipDisplay;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/TooltipFlag;Ljava/util/function/Consumer;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/world/item/ItemStack;addAttributeTooltips(Ljava/util/function/Consumer;Lnet/minecraft/world/item/component/TooltipDisplay;Lnet/minecraft/world/entity/player/Player;)V` (exact) | both | 1000 (default) | `ItemStackMixin.preAttributeModifiers` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]].`addDetailsToTooltip` | `(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/item/component/TooltipDisplay;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/TooltipFlag;Ljava/util/function/Consumer;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/core/DefaultedRegistry;getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` (exact) | both | 1000 (default) | `ItemStackMixin.postTooltipsAdvanced` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]].`addDetailsToTooltip` | `(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/item/component/TooltipDisplay;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/TooltipFlag;Ljava/util/function/Consumer;)V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/world/item/TooltipFlag;isAdvanced()Z` (exact) | both | 1000 (default) | `ItemStackMixin.postTooltipsNonAdvanced` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]].`getTooltipLines` | `(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/TooltipFlag;)Ljava/util/List;` | name_only | @Inject | RETURN | client | 1000 (default) | `ItemStackMixin.getTooltip` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]].`hurtAndBreak` | `(ILnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/EquipmentSlot;)V` | exact | @WrapOperation | INVOKE `Lnet/minecraft/world/item/ItemStack;hurtAndBreak(ILnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/level/ServerPlayer;Ljava/util/function/Consumer;)V` (exact) | both | 1000 (default) | `ItemStackMixin.hookDamage` |
| [[40-Interfaces/net.minecraft.world.item.crafting.CraftingRecipe|CraftingRecipe]].`defaultCraftingReminder` | `(Lnet/minecraft/world/item/crafting/CraftingInput;)Lnet/minecraft/core/NonNullList;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/item/ItemStack;getItem()Lnet/minecraft/world/item/Item;` (exact) | both | 1000 (default) | `CraftingRecipeMixin.captureStack` |
| [[40-Interfaces/net.minecraft.world.item.crafting.CraftingRecipe|CraftingRecipe]].`defaultCraftingReminder` | `(Lnet/minecraft/world/item/crafting/CraftingInput;)Lnet/minecraft/core/NonNullList;` | name_only | @Redirect | INVOKE `Lnet/minecraft/world/item/Item;getCraftingRemainder()Lnet/minecraft/world/item/ItemStackTemplate;` (exact) | both | 1000 (default) | `CraftingRecipeMixin.getStackRemainder` |
| [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment_Builder|Enchantment$Builder]].`*` | `?` | selector_unsupported | @Inject | RETURN | both | 1000 (default) | `EnchantmentBuilderMixin.markModified` |
| [[40-Interfaces/net.minecraft.world.item.enchantment.EnchantmentHelper|EnchantmentHelper]].`lambda$getAvailableEnchantmentResults$0` | `(Lnet/minecraft/world/item/ItemStack;ZLnet/minecraft/core/Holder;)Z` | name_only | @Redirect | INVOKE `Lnet/minecraft/world/item/enchantment/Enchantment;isPrimaryItem(Lnet/minecraft/world/item/ItemStack;)Z` (exact) | both | 1000 (default) | `EnchantmentHelperMixin.useCustomEnchantingChecks` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity|AbstractFurnaceBlockEntity]].`consumeFuel` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/NonNullList;Lnet/minecraft/world/item/ItemStack;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `AbstractFurnaceBlockEntityMixin.copyStack` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity|AbstractFurnaceBlockEntity]].`consumeFuel` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/NonNullList;Lnet/minecraft/world/item/ItemStack;)V` | name_only | @Redirect | INVOKE `Lnet/minecraft/world/item/Item;getCraftingRemainder()Lnet/minecraft/world/item/ItemStackTemplate;` (exact) | both | 1000 (default) | `AbstractFurnaceBlockEntityMixin.getCraftingRemainder` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.BrewingStandBlockEntity|BrewingStandBlockEntity]].`doBrew` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/BrewingStandBlockEntity;)V` | name_only | @Redirect | INVOKE `Lnet/minecraft/world/item/Item;getCraftingRemainder()Lnet/minecraft/world/item/ItemStackTemplate;` (exact) | both | 1000 (default) | `BrewingStandBlockEntityMixin.getCraftingRemainder` |
| [[40-Interfaces/net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction|EnchantRandomlyFunction]].`lambda$run$1` | `(ZLnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Holder;)Z` | name_only | @Redirect | INVOKE `Lnet/minecraft/world/item/enchantment/Enchantment;canEnchant(Lnet/minecraft/world/item/ItemStack;)Z` (exact) | both | 1000 (default) | `EnchantRandomlyFunctionMixin.callAllowEnchantingEvent` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.item.v1.ItemTooltipCallback|ItemTooltipCallback]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.BlockTransformerEvents|BlockTransformerEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.BlockTransformerHelper|BlockTransformerHelper]] (class, 42 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.CustomDamageHandler|CustomDamageHandler]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.DefaultItemComponentEvents|DefaultItemComponentEvents]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.EnchantingContext|EnchantingContext]] (enum, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.EnchantmentEvents|EnchantmentEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.EnchantmentSource|EnchantmentSource]] (enum, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.EquipmentSlotProvider|EquipmentSlotProvider]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.FabricComponentMapBuilder|FabricComponentMapBuilder]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.FabricItem|FabricItem]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.FabricItemStack|FabricItemStack]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.FabricTooltipFlag|FabricTooltipFlag]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.ItemClickBehaviorCallback|ItemClickBehaviorCallback]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.ItemComponentTooltipProviderRegistry|ItemComponentTooltipProviderRegistry]] (interface, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.item.v1.ResourceSource|ResourceSource]] (enum, 6 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
