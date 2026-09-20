---
type: "interface"
fqcn: "net.minecraft.world.item.Item"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.Item

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/level/ItemLike`, `net/minecraft/world/flag/FeatureElement`, `net/fabricmc/fabric/api/item/v1/FabricItem`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `allowComponentsUpdateAnimation` | `(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/Intera` | inherited_exact | invokevirtual@35 in `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `allowComponentsUpdateAnimation` | `(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/Intera` | inherited_exact | invokevirtual@84 in `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `allowContinuingBlockBreaking` | `(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/I` | inherited_exact | invokevirtual@60 in `MultiPlayerGameModeMixin.fabricItemContinueBlockBreakingInject` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `builtInRegistryHolder` | `()Lnet/minecraft/core/Holder$Reference;` | exact | invokevirtual@69 in `DefaultItemComponentImpl$ModifyContextImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `builtInRegistryHolder` | `()Lnet/minecraft/core/Holder$Reference;` | exact | invokevirtual@10 in `CustomIngredientImpl.<init>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `builtInRegistryHolder` | `()Lnet/minecraft/core/Holder$Reference;` | exact | invokevirtual@6 in `ItemVariant.typeHolder` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `canBeEnchantedWith` | `(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Holder;Lnet/f` | inherited_exact | invokevirtual@13 in `FabricItemStack.lambda$canBeEnchantedWith$0` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `canFitInsideContainerItems` | `()Z` | exact | invokevirtual@48 in `ItemContainerContentsStorage$ContainerSlotWrapper.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `components` | `()Lnet/minecraft/core/component/DataComponentMap;` | exact | invokevirtual@45 in `DefaultItemComponentImpl$ModifyContextImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `components` | `()Lnet/minecraft/core/component/DataComponentMap;` | exact | invokevirtual@6 in `VariantCodecs.validateComponents` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getCraftingRemainder` | `()Lnet/minecraft/world/item/ItemStackTemplate;` | exact | invokevirtual@4 in `FabricItem.getCraftingRemainder` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getCraftingRemainder` | `(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemSt` | inherited_exact | invokevirtual@11 in `FabricItemStack.getCraftingRemainder` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getCreatorNamespace` | `(Lnet/minecraft/world/item/ItemStack;)Ljava/lang/String;` | inherited_exact | invokevirtual@11 in `FabricItemStack.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getDefaultInstance` | `()Lnet/minecraft/world/item/ItemStack;` | exact | invokevirtual@10 in `AllIngredient.lambda$items$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getDefaultMaxStackSize` | `()I` | exact | invokevirtual@15 in `ItemVariantImpl.getMaxStackSize` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getDescriptionId` | `()Ljava/lang/String;` | exact | invokevirtual@2 in `FabricLanguageProvider$TranslationBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `isEnabled` | `(Lnet/minecraft/world/flag/FeatureFlagSet;)Z` | inherited_exact | invokevirtual@8 in `FabricCreativeModeTabOutput.isEnabled` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/world/item/Item$Properties;)V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `BY_BLOCK` | `Ljava/util/Map;` | exact | getstatic@11 in `BlockItemTracker.onEntryAdded` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (15 fields, 51 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final CODEC_WITH_BOUND_COMPONENTS : Lcom/mojang/serialization/Codec;
private static final LOGGER : Lorg/slf4j/Logger;
public static final BY_BLOCK : Ljava/util/Map;
public static final BASE_ATTACK_DAMAGE_ID : Lnet/minecraft/resources/Identifier;
public static final BASE_ATTACK_SPEED_ID : Lnet/minecraft/resources/Identifier;
public static final DEFAULT_MAX_STACK_SIZE : I
public static final ABSOLUTE_MAX_STACK_SIZE : I
public static final MAX_BAR_WIDTH : I
protected static final APPROXIMATELY_INFINITE_USE_DURATION : I
private final builtInRegistryHolder : Lnet/minecraft/core/Holder$Reference;
private final craftingRemainingItem : Lnet/minecraft/world/item/ItemStackTemplate;
protected final descriptionId : Ljava/lang/String;
private final requiredFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
public static getId(Lnet/minecraft/world/item/Item;)I
public static byId(I)Lnet/minecraft/world/item/Item;
public static byBlock(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/item/Item;
public <init>(Lnet/minecraft/world/item/Item$Properties;)V
public builtInRegistryHolder()Lnet/minecraft/core/Holder$Reference;
public components()Lnet/minecraft/core/component/DataComponentMap;
public getDefaultMaxStackSize()I
public onUseTick(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/item/ItemStack;I)V
public onDestroyed(Lnet/minecraft/world/entity/item/ItemEntity;)V
public canDestroyBlock(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/LivingEntity;)Z
public asItem()Lnet/minecraft/world/item/Item;
public useOn(Lnet/minecraft/world/item/context/UseOnContext;)Lnet/minecraft/world/InteractionResult;
public getDestroySpeed(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/block/state/BlockState;)F
public use(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;
public finishUsingItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/world/item/ItemStack;
public isBarVisible(Lnet/minecraft/world/item/ItemStack;)Z
public getBarWidth(Lnet/minecraft/world/item/ItemStack;)I
public getBarColor(Lnet/minecraft/world/item/ItemStack;)I
public overrideStackedOnOther(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/inventory/Slot;Lnet/minecraft/world/inventory/ClickAction;Lnet/minecraft/world/entity/player/Player;)Z
public overrideOtherStackedOnMe(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/inventory/Slot;Lnet/minecraft/world/inventory/ClickAction;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/SlotAccess;)Z
public getAttackDamageBonus(Lnet/minecraft/world/entity/Entity;FLnet/minecraft/world/damagesource/DamageSource;)F
public getItemDamageSource(Lnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/world/damagesource/DamageSource;
public hurtEnemy(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/LivingEntity;)V
public postHurtEnemy(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/LivingEntity;)V
public mineBlock(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/LivingEntity;)Z
public isCorrectToolForDrops(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/block/state/BlockState;)Z
public interactLivingEntity(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;
public toString()Ljava/lang/String;
public final getCraftingRemainder()Lnet/minecraft/world/item/ItemStackTemplate;
public inventoryTick(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/EquipmentSlot;)V
public onCraftedBy(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/player/Player;)V
public onCraftedPostProcess(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;)V
public getUseAnimation(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemUseAnimation;
public getUseDuration(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/LivingEntity;)I
public releaseUsing(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/LivingEntity;I)Z
public appendHoverText(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/Item$TooltipContext;Lnet/minecraft/world/item/component/TooltipDisplay;Ljava/util/function/Consumer;Lnet/minecraft/world/item/TooltipFlag;)V
public getTooltipImage(Lnet/minecraft/world/item/ItemStack;)Ljava/util/Optional;
public final getDescriptionId()Ljava/lang/String;
public getName(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/network/chat/Component;
public isFoil(Lnet/minecraft/world/item/ItemStack;)Z
protected static getPlayerPOVHitResult(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/level/ClipContext$Fluid;)Lnet/minecraft/world/phys/BlockHitResult;
public useOnRelease(Lnet/minecraft/world/item/ItemStack;)Z
public getDefaultInstance()Lnet/minecraft/world/item/ItemStack;
public canFitInsideContainerItems()Z
public final requiredFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
public shouldPrintOpWarning(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/player/Player;)Z
private static synthetic lambda$static$2(Lnet/minecraft/core/Holder;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$3(Lnet/minecraft/core/Holder;)Ljava/lang/String;
private static synthetic lambda$static$0(Lnet/minecraft/core/Holder;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$1()Ljava/lang/String;
static <clinit>()V
```
