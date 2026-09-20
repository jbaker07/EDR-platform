---
type: "interface"
fqcn: "net.minecraft.world.item.ItemStack"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.ItemStack

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public final; extends `java/lang/Object`; implements `net/minecraft/core/component/DataComponentHolder`, `net/minecraft/world/item/ItemInstance`, `net/fabricmc/fabric/api/item/v1/FabricItemStack`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/DataCompone` | exact | invokespecial@30 in `ItemVariant.toStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/world/level/ItemLike;)V` | exact | invokespecial@6 in `FabricCreativeModeTabOutput.prepend` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `addToTooltip` | `(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/world/` | exact | invokevirtual@46 in `ItemComponentTooltipProviderRegistryImpl$TooltipPair.appendCustomComp | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `applyComponents` | `(Lnet/minecraft/core/component/DataComponentMap;)V` | exact | invokevirtual@128 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `canBeEnchantedWith` | `(Lnet/minecraft/core/Holder;Lnet/fabricmc/fabric/api/item/v1/Enchantin` | inherited_exact | invokevirtual@5 in `AnvilMenuMixin.callAllowEnchantingEvent` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `canBeEnchantedWith` | `(Lnet/minecraft/core/Holder;Lnet/fabricmc/fabric/api/item/v1/Enchantin` | inherited_exact | invokevirtual@6 in `EnchantCommandMixin.callAllowEnchantingEvent` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `canBeEnchantedWith` | `(Lnet/minecraft/core/Holder;Lnet/fabricmc/fabric/api/item/v1/Enchantin` | inherited_exact | invokevirtual@6 in `EnchantRandomlyFunctionMixin.callAllowEnchantingEvent` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `canBeEnchantedWith` | `(Lnet/minecraft/core/Holder;Lnet/fabricmc/fabric/api/item/v1/Enchantin` | inherited_exact | invokevirtual@6 in `EnchantmentHelperMixin.useCustomEnchantingChecks` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `copy` | `()Lnet/minecraft/world/item/ItemStack;` | exact | invokevirtual@3 in `AbstractFurnaceBlockEntityMixin.copyStack` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `copy` | `()Lnet/minecraft/world/item/ItemStack;` | exact | invokevirtual@7 in `SingleStackStorage.createSnapshot` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `copy` | `()Lnet/minecraft/world/item/ItemStack;` | exact | invokevirtual@81 in `ItemContainerContentsStorage$ContainerSlotWrapper.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `copy` | `()Lnet/minecraft/world/item/ItemStack;` | exact | invokevirtual@56 in `ItemContainerContentsStorage$ContainerSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;` | inherited_exact | invokevirtual@33 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;` | inherited_exact | invokevirtual@85 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;` | inherited_exact | invokevirtual@62 in `ComponentsIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;` | inherited_exact | invokevirtual@17 in `CustomDataIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getComponents` | `()Lnet/minecraft/core/component/DataComponentMap;` | exact | invokevirtual@78 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getComponents` | `()Lnet/minecraft/core/component/DataComponentMap;` | exact | invokevirtual@125 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getComponents` | `()Lnet/minecraft/core/component/DataComponentMap;` | exact | invokevirtual@4 in `ItemVariantImpl.getComponents` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getComponentsPatch` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@22 in `DefaultCustomIngredients.components` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getComponentsPatch` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@29 in `FluidStorage.lambda$static$4` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getComponentsPatch` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@5 in `ItemVariant.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getComponentsPatch` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@14 in `ItemVariant.matches` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getComponentsPatch` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@30 in `WaterPotionStorage.mapToGlassBottle` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getComponentsPatch` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@64 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getComponentsPatch` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@68 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCount` | `()I` | exact | invokevirtual@18 in `FabricCreativeModeTabOutput.checkStack` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getCount` | `()I` | exact | invokevirtual@9 in `ContainerItemContext.forCreativeInteraction` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCount` | `()I` | exact | invokevirtual@5 in `ContainerItemContext.withConstant` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCount` | `()I` | exact | invokevirtual@4 in `SingleStackStorage.getAmount` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCount` | `()I` | exact | invokevirtual@46 in `SingleStackStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCount` | `()I` | exact | invokevirtual@32 in `SingleStackStorage.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCount` | `()I` | exact | invokevirtual@4 in `BundleContentsStorage$BundleSlotWrapper.getAmount` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCount` | `()I` | exact | invokevirtual@133 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCount` | `()I` | exact | invokevirtual@61 in `ItemContainerContentsStorage$ContainerSlotWrapper.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCount` | `()I` | exact | invokevirtual@36 in `ItemContainerContentsStorage$ContainerSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCount` | `()I` | exact | invokevirtual@4 in `ItemContainerContentsStorage$ContainerSlotWrapper.getAmount` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCount` | `()I` | exact | invokevirtual@59 in `CrafterBlockMixin.transferOrSpawnStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCraftingRemainder` | `()Lnet/minecraft/world/item/ItemStackTemplate;` | inherited_exact | invokevirtual@9 in `AbstractFurnaceBlockEntityMixin.getCraftingRemainder` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getCraftingRemainder` | `()Lnet/minecraft/world/item/ItemStackTemplate;` | inherited_exact | invokevirtual@1 in `BrewingStandBlockEntityMixin.getCraftingRemainder` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getCraftingRemainder` | `()Lnet/minecraft/world/item/ItemStackTemplate;` | inherited_exact | invokevirtual@9 in `CraftingRecipeMixin.getStackRemainder` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@12 in `ItemApiLookupImpl.find` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@1 in `ItemApiLookupImpl.lambda$registerSelf$0` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@4 in `VillagerMixin.useGatherableItemsSet` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@1 in `FabricCreativeModeTabOutput.isEnabled` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@4 in `FabricItemStack.getCraftingRemainder` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@4 in `FabricItemStack.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@4 in `FabricItemStack.lambda$canBeEnchantedWith$0` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@1 in `LivingEntityMixin.onGetPreferredEquipmentSlot` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@9 in `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@13 in `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@23 in `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@56 in `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@61 in `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@71 in `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@35 in `MultiPlayerGameModeMixin.fabricItemContinueBlockBreakingInject` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@46 in `MultiPlayerGameModeMixin.fabricItemContinueBlockBreakingInject` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@14 in `DefaultCustomIngredients.components` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@1 in `HumanoidArmorLayerMixin.renderArmor` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@28 in `HumanoidMobRendererMixin.permitArmorWithCustomRenderers` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@30 in `LivingEntityRendererMixin.toggleDefaultHeadItem` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@28 in `FluidStorageUtil.interactWithFluidStorage` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@1 in `ItemVariant.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@2 in `ItemVariant.matches` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@53 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@57 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getMaxStackSize` | `()I` | inherited_exact | invokevirtual@4 in `ItemContainerContentsStorage$ContainerSlotWrapper.getCapacity` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `grow` | `(I)V` | exact | invokevirtual@99 in `SingleStackStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `grow` | `(I)V` | exact | invokevirtual@111 in `ItemContainerContentsStorage$ContainerSlotWrapper.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `has` | `(Lnet/minecraft/core/component/DataComponentType;)Z` | inherited_exact | invokevirtual@23 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `has` | `(Lnet/minecraft/core/component/DataComponentType;)Z` | inherited_exact | invokevirtual@75 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `has` | `(Lnet/minecraft/core/component/DataComponentType;)Z` | inherited_exact | invokevirtual@109 in `ComponentsIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/lang/Object;)Z` | inherited_exact | invokevirtual@37 in `FabricCreativeModeTabOutput.insertBefore` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/lang/Object;)Z` | inherited_exact | invokevirtual@38 in `FabricCreativeModeTabOutput.insertAfter` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/lang/Object;)Z` | inherited_exact | invokevirtual@65 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/lang/Object;)Z` | inherited_exact | invokevirtual@38 in `MultiPlayerGameModeMixin.fabricItemContinueBlockBreakingInject` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@1 in `FabricCreativeModeTabOutput.checkStack` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@69 in `ServerGamePacketListenerImplMixin.onPickItemFromBlock` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@50 in `ServerGamePacketListenerImplMixin.onPickItemFromEntity` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@42 in `ShapelessRecipeMixin.customIngredientMatch` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@1 in `GuiGraphicsExtractorMixin.drawStackOverlay` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@4 in `SingleStackStorage.isResourceBlank` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@24 in `SingleStackStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@76 in `SingleStackStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@4 in `BundleContentsStorage$BundleSlotWrapper.isResourceBlank` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@46 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@36 in `ItemContainerContentsStorage$ContainerSlotWrapper.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@88 in `ItemContainerContentsStorage$ContainerSlotWrapper.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@4 in `ItemContainerContentsStorage$ContainerSlotWrapper.isResourceBlank` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@12 in `AbstractFurnaceBlockEntityMixin.fabric_onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@8 in `CrafterBlockMixin.transferOrSpawnStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isSameItemSameComponents` | `(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemSta` | exact | invokestatic@27 in `FabricCreativeModeTabOutput.insertBefore` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `isSameItemSameComponents` | `(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemSta` | exact | invokestatic@28 in `FabricCreativeModeTabOutput.insertAfter` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `isSameItemSameComponents` | `(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemSta` | exact | invokestatic@5 in `MultiPlayerGameModeMixin.fabricItemContinueBlockBreakingInject` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `isSameItemSameComponents` | `(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemSta` | exact | invokestatic@22 in `AbstractFurnaceBlockEntityMixin.fabric_onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lj` | exact | invokevirtual@21 in `FluidStorage.lambda$static$4` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lj` | exact | invokevirtual@22 in `WaterPotionStorage.mapToGlassBottle` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lj` | exact | invokevirtual@116 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setCount` | `(I)V` | exact | invokevirtual@136 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setCount` | `(I)V` | exact | invokevirtual@149 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `shrink` | `(I)V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |
| calls | `shrink` | `(I)V` | exact | invokevirtual@64 in `SingleStackStorage.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `shrink` | `(I)V` | exact | invokevirtual@65 in `ItemContainerContentsStorage$ContainerSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `shrink` | `(I)V` | exact | invokevirtual@84 in `CrafterBlockMixin.transferOrSpawnStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `typeHolder` | `()Lnet/minecraft/core/Holder;` | exact | invokevirtual@1 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `addDetailsToTooltip` | `(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/it` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `addDetailsToTooltip` | `(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/it` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `addDetailsToTooltip` | `(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/it` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `addDetailsToTooltip` | `(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/it` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `addDetailsToTooltip` | `(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/it` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `getTooltipLines` | `(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/en` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/item/ItemStack;` | exact | getstatic@81 in `ServerGamePacketListenerImplMixin.onPickItemFromBlock` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/item/ItemStack;` | exact | getstatic@62 in `ServerGamePacketListenerImplMixin.onPickItemFromEntity` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/item/ItemStack;` | exact | getstatic@9 in `ItemVariant.toStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/item/ItemStack;` | exact | getstatic@17 in `BundleContentsStorage$BundleSlotWrapper.getStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/item/ItemStack;` | exact | getstatic@26 in `ItemContainerContentsStorage$ContainerSlotWrapper.getStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/item/ItemStack;` | exact | getstatic@51 in `ItemContainerContentsStorage$ContainerSlotWrapper.getStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/item/ItemStack;` | exact | getstatic@37 in `ItemContainerContentsStorage$ContainerSlotWrapper.setStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| wraps | `hurtAndBreak` | `(ILnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity` | exact | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| wraps | `use` | `(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/P` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| wraps | `useOn` | `(Lnet/minecraft/world/item/context/UseOnContext;)Lnet/minecraft/world/` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (17 fields, 146 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final OP_NBT_WARNING : Ljava/util/List;
private static final UNBREAKABLE_TOOLTIP : Lnet/minecraft/network/chat/Component;
private static final INTANGIBLE_TOOLTIP : Lnet/minecraft/network/chat/Component;
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final OPTIONAL_CODEC : Lcom/mojang/serialization/Codec;
public static final OPTIONAL_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final OPTIONAL_UNTRUSTED_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final OPTIONAL_LIST_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private static final LOGGER : Lorg/slf4j/Logger;
public static final EMPTY : Lnet/minecraft/world/item/ItemStack;
private static final DISABLED_ITEM_TOOLTIP : Lnet/minecraft/network/chat/Component;
private count : I
private popTime : I
private final item : Lnet/minecraft/core/Holder;
private final components : Lnet/minecraft/core/component/PatchedDataComponentMap;
public static validateStrict(Lnet/minecraft/world/item/ItemStack;)Lcom/mojang/serialization/DataResult;
private static createOptionalStreamCodec(Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
public static validatedStreamCodec(Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
public getTooltipImage()Ljava/util/Optional;
public getComponents()Lnet/minecraft/core/component/DataComponentMap;
public getPrototype()Lnet/minecraft/core/component/DataComponentMap;
public getComponentsPatch()Lnet/minecraft/core/component/DataComponentPatch;
public immutableComponents()Lnet/minecraft/core/component/DataComponentMap;
public hasNonDefault(Lnet/minecraft/core/component/DataComponentType;)Z
public <init>(Lnet/minecraft/world/level/ItemLike;I)V
public <init>(Lnet/minecraft/world/level/ItemLike;)V
public <init>(Lnet/minecraft/core/Holder;I)V
public <init>(Lnet/minecraft/core/Holder;)V
public <init>(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/DataComponentPatch;)V
private <init>(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/PatchedDataComponentMap;)V
private <init>(Ljava/lang/Void;)V
private static validateComponents(Lnet/minecraft/core/component/DataComponentMap;)Lcom/mojang/serialization/DataResult;
private static validateContainedItemSizes(Ljava/lang/Iterable;)Lcom/mojang/serialization/DataResult;
public isEmpty()Z
public isItemEnabled(Lnet/minecraft/world/flag/FeatureFlagSet;)Z
public split(I)Lnet/minecraft/world/item/ItemStack;
public copyAndClear()Lnet/minecraft/world/item/ItemStack;
public getItem()Lnet/minecraft/world/item/Item;
public typeHolder()Lnet/minecraft/core/Holder;
public is(Ljava/util/function/Predicate;)Z
public useOn(Lnet/minecraft/world/item/context/UseOnContext;)Lnet/minecraft/world/InteractionResult;
public getDestroySpeed(Lnet/minecraft/world/level/block/state/BlockState;)F
public use(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;
public finishUsingItem(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/world/item/ItemStack;
private applyAfterUseComponentSideEffects(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemStack;
public isStackable()Z
public isDamageableItem()Z
public isDamaged()Z
public getDamageValue()I
public setDamageValue(I)V
public getMaxDamage()I
public isBroken()Z
public nextDamageWillBreak()Z
public hurtAndBreak(ILnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/level/ServerPlayer;Ljava/util/function/Consumer;)V
private processDurabilityChange(ILnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/level/ServerPlayer;)I
private applyDamage(ILnet/minecraft/server/level/ServerPlayer;Ljava/util/function/Consumer;)V
public hurtWithoutBreaking(ILnet/minecraft/world/entity/player/Player;)V
public hurtAndBreak(ILnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/InteractionHand;)V
public hurtAndBreak(ILnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/EquipmentSlot;)V
public hurtAndConvertOnBreak(ILnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/EquipmentSlot;)Lnet/minecraft/world/item/ItemStack;
public isBarVisible()Z
public getBarWidth()I
public getBarColor()I
public overrideStackedOnOther(Lnet/minecraft/world/inventory/Slot;Lnet/minecraft/world/inventory/ClickAction;Lnet/minecraft/world/entity/player/Player;)Z
public overrideOtherStackedOnMe(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/inventory/Slot;Lnet/minecraft/world/inventory/ClickAction;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/SlotAccess;)Z
public hurtEnemy(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/LivingEntity;)Z
public postHurtEnemy(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/LivingEntity;)V
public mineBlock(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/player/Player;)V
public isCorrectToolForDrops(Lnet/minecraft/world/level/block/state/BlockState;)Z
public interactLivingEntity(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;
public copy()Lnet/minecraft/world/item/ItemStack;
public copyWithCount(I)Lnet/minecraft/world/item/ItemStack;
public transmuteCopy(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/item/ItemStack;
public transmuteCopy(Lnet/minecraft/world/level/ItemLike;I)Lnet/minecraft/world/item/ItemStack;
private transmuteCopyIgnoreEmpty(Lnet/minecraft/world/level/ItemLike;I)Lnet/minecraft/world/item/ItemStack;
public static matches(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)Z
public static listMatches(Ljava/util/List;Ljava/util/List;)Z
public static isSameItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)Z
public static isSameItemSameComponents(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)Z
public static matchesIgnoringComponents(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;Ljava/util/function/Predicate;)Z
public static lenientOptionalFieldOf(Ljava/lang/String;)Lcom/mojang/serialization/MapCodec;
public static hashItemAndComponents(Lnet/minecraft/world/item/ItemStack;)I
public static hashStackList(Ljava/util/List;)I
public toString()Ljava/lang/String;
public inventoryTick(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/EquipmentSlot;)V
public onCraftedBy(Lnet/minecraft/world/entity/player/Player;I)V
public onCraftedBySystem(Lnet/minecraft/world/level/Level;)V
public getUseDuration(Lnet/minecraft/world/entity/LivingEntity;)I
public getUseAnimation()Lnet/minecraft/world/item/ItemUseAnimation;
public releaseUsing(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/LivingEntity;I)V
public causeUseVibration(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder$Reference;)V
public useOnRelease()Z
public set(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ljava/lang/Object;
public set(Lnet/minecraft/core/component/TypedDataComponent;)Ljava/lang/Object;
public copyFrom(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/core/component/DataComponentGetter;)V
public update(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
public update(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;Ljava/util/function/UnaryOperator;)Ljava/lang/Object;
public remove(Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;
public applyComponentsAndValidate(Lnet/minecraft/core/component/DataComponentPatch;)V
public applyComponents(Lnet/minecraft/core/component/DataComponentPatch;)V
public applyComponents(Lnet/minecraft/core/component/DataComponentMap;)V
public getHoverName()Lnet/minecraft/network/chat/Component;
public getCustomName()Lnet/minecraft/network/chat/Component;
public getItemName()Lnet/minecraft/network/chat/Component;
public getStyledHoverName()Lnet/minecraft/network/chat/Component;
public addToTooltip(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/world/item/component/TooltipProvider$Getter;Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/item/component/TooltipDisplay;Ljava/util/function/Consumer;Lnet/minecraft/world/item/TooltipFlag;)V
public addToTooltip(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/item/component/TooltipDisplay;Ljava/util/function/Consumer;Lnet/minecraft/world/item/TooltipFlag;)V
public getTooltipLines(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/TooltipFlag;)Ljava/util/List;
public addDetailsToTooltip(Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/item/component/TooltipDisplay;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/TooltipFlag;Ljava/util/function/Consumer;)V
private addUnitComponentToTooltip(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/network/chat/Component;Lnet/minecraft/world/item/component/TooltipDisplay;Ljava/util/function/Consumer;)V
private addAttributeTooltips(Ljava/util/function/Consumer;Lnet/minecraft/world/item/component/TooltipDisplay;Lnet/minecraft/world/entity/player/Player;)V
public hasFoil()Z
public getRarity()Lnet/minecraft/world/item/Rarity;
public isEnchantable()Z
public enchant(Lnet/minecraft/core/Holder;I)V
public isEnchanted()Z
public getEnchantments()Lnet/minecraft/world/item/enchantment/ItemEnchantments;
public forEachModifier(Lnet/minecraft/world/entity/EquipmentSlotGroup;Lorg/apache/commons/lang3/function/TriConsumer;)V
public forEachModifier(Lnet/minecraft/world/entity/EquipmentSlot;Ljava/util/function/BiConsumer;)V
public getDisplayName()Lnet/minecraft/network/chat/Component;
public getAttackAnimation()Lnet/minecraft/world/item/component/SwingAnimation;
public getInteractAnimation()Lnet/minecraft/world/item/component/SwingAnimation;
public canPlaceOnBlockInAdventureMode(Lnet/minecraft/world/level/block/state/pattern/BlockInWorld;)Z
public canBreakBlockInAdventureMode(Lnet/minecraft/world/level/block/state/pattern/BlockInWorld;)Z
public getPopTime()I
public setPopTime(I)V
public getCount()I
public count()I
public setCount(I)V
public limitSize(I)V
public grow(I)V
public shrink(I)V
public consume(ILnet/minecraft/world/entity/LivingEntity;)V
public consumeAndReturn(ILnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/world/item/ItemStack;
public onUseTick(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/LivingEntity;I)V
public onDestroyed(Lnet/minecraft/world/entity/item/ItemEntity;)V
public canBeHurtBy(Lnet/minecraft/world/damagesource/DamageSource;)Z
public isValidRepairItem(Lnet/minecraft/world/item/ItemStack;)Z
public canDestroyBlock(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/player/Player;)Z
public getDamageSource(Lnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/world/damagesource/DamageSource;
private synthetic lambda$getDamageSource$1(Lnet/minecraft/world/entity/LivingEntity;)Ljava/util/Optional;
private static synthetic lambda$getDamageSource$0(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/core/Holder;)Lnet/minecraft/world/damagesource/DamageSource;
private synthetic lambda$getDisplayName$0(Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
private static synthetic lambda$forEachModifier$0(Lorg/apache/commons/lang3/function/TriConsumer;Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/ai/attributes/AttributeModifier;)V
private static synthetic lambda$enchant$0(Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/ItemEnchantments$Mutable;)V
private static synthetic lambda$addAttributeTooltips$0(Lorg/apache/commons/lang3/mutable/MutableBoolean;Ljava/util/function/Consumer;Lnet/minecraft/world/entity/EquipmentSlotGroup;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/ai/attributes/AttributeModifier;Lnet/minecraft/world/item/component/ItemAttributeModifiers$Display;)V
private static synthetic lambda$addToTooltip$0(Lnet/minecraft/world/item/component/TooltipProvider;)Lnet/minecraft/world/item/component/TooltipProvider;
private static synthetic lambda$lenientOptionalFieldOf$1(Lnet/minecraft/world/item/ItemStack;)Ljava/util/Optional;
private static synthetic lambda$lenientOptionalFieldOf$0(Ljava/util/Optional;)Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$hurtAndBreak$0(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/item/ItemStack;)V
private static synthetic lambda$hurtWithoutBreaking$0(Lnet/minecraft/world/item/ItemStack;)V
private static synthetic lambda$validateContainedItemSizes$0(II)Ljava/lang/String;
private static synthetic lambda$validateComponents$0()Ljava/lang/String;
private static synthetic lambda$static$4(Lnet/minecraft/world/item/ItemStack;)Ljava/util/Optional;
private static synthetic lambda$static$3(Ljava/util/Optional;)Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$validateStrict$1(Lnet/minecraft/world/item/ItemStack;)Ljava/lang/String;
private static synthetic lambda$validateStrict$0(Lnet/minecraft/world/item/ItemStack;Ljava/lang/Object;)Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$static$0(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/MapCodec;
private static synthetic lambda$static$1(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$2(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/core/component/DataComponentPatch;
static <clinit>()V
```
