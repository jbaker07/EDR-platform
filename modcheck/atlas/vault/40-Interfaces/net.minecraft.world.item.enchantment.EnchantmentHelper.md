---
type: "interface"
fqcn: "net.minecraft.world.item.enchantment.EnchantmentHelper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.enchantment.EnchantmentHelper

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `lambda$getAvailableEnchantmentResults$0` | `(Lnet/minecraft/world/item/ItemStack;ZLnet/minecraft/core/Holder;)Z` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (0 fields, 105 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>()V
public static getItemEnchantmentLevel(Lnet/minecraft/core/Holder;Lnet/minecraft/world/item/ItemInstance;)I
public static updateEnchantments(Lnet/minecraft/world/item/ItemStack;Ljava/util/function/Consumer;)Lnet/minecraft/world/item/enchantment/ItemEnchantments;
public static canStoreEnchantments(Lnet/minecraft/world/item/ItemStack;)Z
public static setEnchantments(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/enchantment/ItemEnchantments;)V
public static getEnchantmentsForCrafting(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/enchantment/ItemEnchantments;
public static getComponentType(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/core/component/DataComponentType;
public static hasAnyEnchantments(Lnet/minecraft/world/item/ItemStack;)Z
public static processDurabilityChange(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;I)I
public static processAmmoUse(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;I)I
public static processBlockExperience(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;I)I
public static processMobExperience(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;I)I
public static createBook(Lnet/minecraft/world/item/enchantment/EnchantmentInstance;)Lnet/minecraft/world/item/ItemStack;
public static runIterationOnItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/enchantment/EnchantmentHelper$EnchantmentVisitor;)V
public static runIterationOnItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/item/enchantment/EnchantmentHelper$EnchantmentInSlotVisitor;)V
public static runIterationOnEquipment(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/item/enchantment/EnchantmentHelper$EnchantmentInSlotVisitor;)V
public static isImmuneToDamage(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;)Z
public static getDamageProtection(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;)F
public static modifyDamage(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;F)F
public static modifyFallBasedDamage(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;F)F
public static modifyArmorEffectiveness(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;F)F
public static modifyKnockback(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;F)F
public static doPostAttackEffects(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;)V
public static doPostPiercingAttackEffects(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;)V
public static doPostAttackEffectsWithItemSource(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lnet/minecraft/world/item/ItemStack;)V
public static doPostAttackEffectsWithItemSourceOnBreak(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lnet/minecraft/world/item/ItemStack;Ljava/util/function/Consumer;)V
public static runLocationChangedEffects(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;)V
public static runLocationChangedEffects(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/EquipmentSlot;)V
public static stopLocationBasedEffects(Lnet/minecraft/world/entity/LivingEntity;)V
public static stopLocationBasedEffects(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/EquipmentSlot;)V
public static tickEffects(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;)V
public static getEnchantmentLevel(Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/LivingEntity;)I
public static processProjectileCount(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;I)I
public static processProjectileSpread(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;F)F
public static getPiercingCount(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)I
public static onProjectileSpawned(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/projectile/Projectile;Ljava/util/function/Consumer;)V
public static onHitBlock(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/level/block/state/BlockState;Ljava/util/function/Consumer;)V
public static modifyDurabilityToRepairFromXp(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;I)I
public static processEquipmentDropChance(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;F)F
public static forEachModifier(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlotGroup;Ljava/util/function/BiConsumer;)V
public static forEachModifier(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;Ljava/util/function/BiConsumer;)V
public static getFishingLuckBonus(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;)I
public static getFishingTimeReduction(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;)F
public static getTridentReturnToOwnerAcceleration(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;)I
public static modifyCrossbowChargingTime(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/LivingEntity;F)F
public static getTridentSpinAttackStrength(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/LivingEntity;)F
public static hasTag(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/tags/TagKey;)Z
public static has(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/component/DataComponentType;)Z
public static pickHighestLevel(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/component/DataComponentType;)Ljava/util/Optional;
public static getHighestLevel(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/component/DataComponentType;)Lcom/mojang/datafixers/util/Pair;
public static getRandomItemWith(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/world/entity/LivingEntity;Ljava/util/function/Predicate;)Ljava/util/Optional;
public static getEnchantmentCost(Lnet/minecraft/util/RandomSource;IILnet/minecraft/world/item/ItemStack;)I
public static enchantItem(Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/item/ItemStack;ILnet/minecraft/core/RegistryAccess;Ljava/util/Optional;)Lnet/minecraft/world/item/ItemStack;
public static enchantItem(Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/item/ItemStack;ILjava/util/stream/Stream;)Lnet/minecraft/world/item/ItemStack;
public static selectEnchantment(Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/item/ItemStack;ILjava/util/stream/Stream;)Ljava/util/List;
public static filterCompatibleEnchantments(Ljava/util/List;Lnet/minecraft/world/item/enchantment/EnchantmentInstance;)V
public static isEnchantmentCompatible(Ljava/util/Collection;Lnet/minecraft/core/Holder;)Z
public static getAvailableEnchantmentResults(ILnet/minecraft/world/item/ItemStack;Ljava/util/stream/Stream;)Ljava/util/List;
public static enchantItemFromProvider(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/RegistryAccess;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/DifficultyInstance;Lnet/minecraft/util/RandomSource;)V
private static synthetic lambda$enchantItemFromProvider$0(Lnet/minecraft/world/item/enchantment/providers/EnchantmentProvider;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/DifficultyInstance;Lnet/minecraft/world/item/enchantment/ItemEnchantments$Mutable;)V
private static synthetic lambda$getAvailableEnchantmentResults$1(ILjava/util/List;Lnet/minecraft/core/Holder;)V
private static synthetic lambda$getAvailableEnchantmentResults$0(Lnet/minecraft/world/item/ItemStack;ZLnet/minecraft/core/Holder;)Z
private static synthetic lambda$filterCompatibleEnchantments$0(Lnet/minecraft/world/item/enchantment/EnchantmentInstance;Lnet/minecraft/world/item/enchantment/EnchantmentInstance;)Z
private static synthetic lambda$enchantItem$0(Lnet/minecraft/core/RegistryAccess;)Ljava/util/stream/Stream;
private static synthetic lambda$enchantItem$1(Lnet/minecraft/core/Holder$Reference;)Lnet/minecraft/core/Holder;
private static synthetic lambda$getHighestLevel$0(Lorg/apache/commons/lang3/mutable/MutableObject;Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$has$0(Lnet/minecraft/core/component/DataComponentType;Lorg/apache/commons/lang3/mutable/MutableBoolean;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$getTridentSpinAttackStrength$0(Lnet/minecraft/world/entity/LivingEntity;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$modifyCrossbowChargingTime$0(Lnet/minecraft/world/entity/LivingEntity;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$getTridentReturnToOwnerAcceleration$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$getFishingTimeReduction$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$getFishingLuckBonus$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$forEachModifier$2(Lnet/minecraft/world/entity/EquipmentSlot;Ljava/util/function/BiConsumer;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$forEachModifier$3(Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/EquipmentSlot;Ljava/util/function/BiConsumer;ILnet/minecraft/world/item/enchantment/effects/EnchantmentAttributeEffect;)V
private static synthetic lambda$forEachModifier$0(Lnet/minecraft/world/entity/EquipmentSlotGroup;Ljava/util/function/BiConsumer;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$forEachModifier$1(Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/EquipmentSlotGroup;Ljava/util/function/BiConsumer;ILnet/minecraft/world/item/enchantment/effects/EnchantmentAttributeEffect;)V
private static synthetic lambda$processEquipmentDropChance$2(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/util/RandomSource;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$processEquipmentDropChance$3(Lnet/minecraft/world/level/storage/loot/LootContext;Lorg/apache/commons/lang3/mutable/MutableFloat;ILnet/minecraft/util/RandomSource;Lnet/minecraft/world/item/enchantment/TargetedConditionalEffect;)V
private static synthetic lambda$processEquipmentDropChance$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/util/RandomSource;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$processEquipmentDropChance$1(Lnet/minecraft/world/level/storage/loot/LootContext;Lorg/apache/commons/lang3/mutable/MutableFloat;ILnet/minecraft/util/RandomSource;Lnet/minecraft/world/item/enchantment/TargetedConditionalEffect;)V
private static synthetic lambda$modifyDurabilityToRepairFromXp$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$onHitBlock$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$onProjectileSpawned$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/projectile/Projectile;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$getPiercingCount$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$processProjectileSpread$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$processProjectileCount$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$tickEffects$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$stopLocationBasedEffects$1(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$stopLocationBasedEffects$0(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$runLocationChangedEffects$1(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$runLocationChangedEffects$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$doPostAttackEffectsWithItemSourceOnBreak$2(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$doPostAttackEffectsWithItemSourceOnBreak$1(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$doPostAttackEffectsWithItemSourceOnBreak$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$doPostPiercingAttackEffects$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$modifyKnockback$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$modifyArmorEffectiveness$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$modifyFallBasedDamage$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$modifyDamage$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$getDamageProtection$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$isImmuneToDamage$0(Lorg/apache/commons/lang3/mutable/MutableBoolean;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$processMobExperience$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;)V
private static synthetic lambda$processBlockExperience$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$processAmmoUse$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
private static synthetic lambda$processDurabilityChange$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/core/Holder;I)V
```
