---
type: "interface"
fqcn: "net.minecraft.world.item.enchantment.Enchantment"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.enchantment.Enchantment

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`record` public final; extends `java/lang/Record`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/network/chat/Component;Lnet/minecraft/world/item/encha` | exact | invokespecial@169 in `EnchantmentUtil.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `canEnchant` | `(Lnet/minecraft/world/item/ItemStack;)Z` | exact | invokevirtual@33 in `FabricItem.canBeEnchantedWith` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `definition` | `()Lnet/minecraft/world/item/enchantment/Enchantment$EnchantmentDefinit` | exact | invokevirtual@1 in `EnchantmentUtil.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `description` | `()Lnet/minecraft/network/chat/Component;` | exact | invokevirtual@142 in `EnchantmentUtil.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `effects` | `()Lnet/minecraft/core/component/DataComponentMap;` | exact | invokevirtual@41 in `EnchantmentUtil.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `effects` | `()Lnet/minecraft/core/component/DataComponentMap;` | exact | invokevirtual@49 in `EnchantmentUtil.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `enchantment` | `(Lnet/minecraft/world/item/enchantment/Enchantment$EnchantmentDefiniti` | exact | invokestatic@4 in `EnchantmentUtil.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `exclusiveSet` | `()Lnet/minecraft/core/HolderSet;` | exact | invokevirtual@26 in `EnchantmentUtil.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `isPrimaryItem` | `(Lnet/minecraft/world/item/ItemStack;)Z` | exact | invokevirtual@17 in `FabricItem.canBeEnchantedWith` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (8 fields, 78 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final description : Lnet/minecraft/network/chat/Component;
private final definition : Lnet/minecraft/world/item/enchantment/Enchantment$EnchantmentDefinition;
private final exclusiveSet : Lnet/minecraft/core/HolderSet;
private final effects : Lnet/minecraft/core/component/DataComponentMap;
public static final MAX_LEVEL : I
public static final DIRECT_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/network/chat/Component;Lnet/minecraft/world/item/enchantment/Enchantment$EnchantmentDefinition;Lnet/minecraft/core/HolderSet;Lnet/minecraft/core/component/DataComponentMap;)V
public static constantCost(I)Lnet/minecraft/world/item/enchantment/Enchantment$Cost;
public static dynamicCost(II)Lnet/minecraft/world/item/enchantment/Enchantment$Cost;
public static definition(Lnet/minecraft/core/HolderSet;Lnet/minecraft/core/HolderSet;IILnet/minecraft/world/item/enchantment/Enchantment$Cost;Lnet/minecraft/world/item/enchantment/Enchantment$Cost;I[Lnet/minecraft/world/entity/EquipmentSlotGroup;)Lnet/minecraft/world/item/enchantment/Enchantment$EnchantmentDefinition;
public static definition(Lnet/minecraft/core/HolderSet;IILnet/minecraft/world/item/enchantment/Enchantment$Cost;Lnet/minecraft/world/item/enchantment/Enchantment$Cost;I[Lnet/minecraft/world/entity/EquipmentSlotGroup;)Lnet/minecraft/world/item/enchantment/Enchantment$EnchantmentDefinition;
public getSlotItems(Lnet/minecraft/world/entity/LivingEntity;)Ljava/util/Map;
public getSupportedItems()Lnet/minecraft/core/HolderSet;
public matchingSlot(Lnet/minecraft/world/entity/EquipmentSlot;)Z
public isPrimaryItem(Lnet/minecraft/world/item/ItemStack;)Z
public isSupportedItem(Lnet/minecraft/world/item/ItemStack;)Z
public getWeight()I
public getAnvilCost()I
public getMinLevel()I
public getMaxLevel()I
public getMinCost(I)I
public getMaxCost(I)I
public toString()Ljava/lang/String;
public static areCompatible(Lnet/minecraft/core/Holder;Lnet/minecraft/core/Holder;)Z
public static getFullname(Lnet/minecraft/core/Holder;I)Lnet/minecraft/network/chat/Component;
public canEnchant(Lnet/minecraft/world/item/ItemStack;)Z
public getEffects(Lnet/minecraft/core/component/DataComponentType;)Ljava/util/List;
public isImmuneToDamage(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;)Z
public modifyDamageProtection(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyDurabilityChange(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyAmmoCount(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyPiercingCount(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyBlockExperience(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyMobExperience(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyDurabilityToRepairFromXp(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyTridentReturnToOwnerAcceleration(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyTridentSpinAttackStrength(Lnet/minecraft/util/RandomSource;ILorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyFishingTimeReduction(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyFishingLuckBonus(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyDamage(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyFallBasedDamage(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyKnockback(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyArmorEffectivness(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public doPostAttack(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/item/enchantment/EnchantmentTarget;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;)V
public static doPostAttack(Lnet/minecraft/world/item/enchantment/TargetedConditionalEffect;Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;)V
public doPostPiercingAttack(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/Entity;)V
public modifyProjectileCount(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyProjectileSpread(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyCrossbowChargeTime(Lnet/minecraft/util/RandomSource;ILorg/apache/commons/lang3/mutable/MutableFloat;)V
public modifyUnfilteredValue(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/util/RandomSource;ILorg/apache/commons/lang3/mutable/MutableFloat;)V
public tick(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/Entity;)V
public onProjectileSpawned(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/Entity;)V
public onHitBlock(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/level/block/state/BlockState;)V
public final modifyItemFilteredCount(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemInstance;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public final modifyEntityFilteredValue(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public final modifyDamageFilteredValue(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lorg/apache/commons/lang3/mutable/MutableFloat;)V
public static damageContext(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;)Lnet/minecraft/world/level/storage/loot/LootContext;
public static itemContext(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/ItemInstance;)Lnet/minecraft/world/level/storage/loot/LootContext;
public static locationContext(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/entity/Entity;Z)Lnet/minecraft/world/level/storage/loot/LootContext;
public static entityContext(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/level/storage/loot/LootContext;
public static blockHitContext(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/storage/loot/LootContext;
public static applyEffects(Ljava/util/List;Lnet/minecraft/world/level/storage/loot/LootContext;Lnet/minecraft/world/item/enchantment/Enchantment$GenericAction;)V
public static applyEffects(Ljava/util/List;Lnet/minecraft/world/level/storage/loot/LootContext;Lorg/apache/commons/lang3/mutable/MutableFloat;Lnet/minecraft/world/item/enchantment/Enchantment$FloatAction;)V
public runLocationChangedEffects(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/LivingEntity;)V
public stopLocationBasedEffects(ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/LivingEntity;)V
public static enchantment(Lnet/minecraft/world/item/enchantment/Enchantment$EnchantmentDefinition;)Lnet/minecraft/world/item/enchantment/Enchantment$Builder;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public description()Lnet/minecraft/network/chat/Component;
public definition()Lnet/minecraft/world/item/enchantment/Enchantment$EnchantmentDefinition;
public exclusiveSet()Lnet/minecraft/core/HolderSet;
public effects()Lnet/minecraft/core/component/DataComponentMap;
private static synthetic lambda$runLocationChangedEffects$0(Lnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/LivingEntity;ILnet/minecraft/world/item/enchantment/effects/EnchantmentLocationBasedEffect;)V
private static synthetic lambda$modifyDamageFilteredValue$0(ILnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/enchantment/effects/EnchantmentValueEffect;F)F
private static synthetic lambda$modifyEntityFilteredValue$0(ILnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/enchantment/effects/EnchantmentValueEffect;F)F
private static synthetic lambda$modifyItemFilteredCount$0(ILnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/enchantment/effects/EnchantmentValueEffect;F)F
private static synthetic lambda$onHitBlock$0(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/item/enchantment/effects/EnchantmentEntityEffect;)V
private static synthetic lambda$onProjectileSpawned$0(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/enchantment/effects/EnchantmentEntityEffect;)V
private static synthetic lambda$tick$0(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/enchantment/effects/EnchantmentEntityEffect;)V
private static synthetic lambda$doPostPiercingAttack$0(Lnet/minecraft/server/level/ServerLevel;ILnet/minecraft/world/item/enchantment/EnchantedItemInUse;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/enchantment/effects/EnchantmentEntityEffect;)V
private static synthetic lambda$modifyDamageProtection$0(ILnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/enchantment/effects/EnchantmentValueEffect;F)F
private static synthetic lambda$matchingSlot$0(Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/entity/EquipmentSlotGroup;)Z
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
