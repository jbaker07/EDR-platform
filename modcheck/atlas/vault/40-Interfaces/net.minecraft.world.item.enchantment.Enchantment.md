---
type: "interface"
fqcn: "net.minecraft.world.item.enchantment.Enchantment"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.enchantment.Enchantment

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/network/chat/Component;Lnet/minecraft/world/` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `definition()Lnet/minecraft/world/item/enchantment/Enchantment$Enchantm` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `description()Lnet/minecraft/network/chat/Component;` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `effects()Lnet/minecraft/core/component/DataComponentMap;` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `enchantment(Lnet/minecraft/world/item/enchantment/Enchantment$Enchantme` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `exclusiveSet()Lnet/minecraft/core/HolderSet;` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (86, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.item.enchantment.Enchantment extends java.lang.Record {
    private final net.minecraft.network.chat.Component description;
    private final net.minecraft.world.item.enchantment.Enchantment$EnchantmentDefinition definition;
    private final net.minecraft.core.HolderSet<net.minecraft.world.item.enchantment.Enchantment> exclusiveSet;
    private final net.minecraft.core.component.DataComponentMap effects;
    public static final int MAX_LEVEL;
    public static final com.mojang.serialization.Codec<net.minecraft.world.item.enchantment.Enchantment> DIRECT_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.core.Holder<net.minecraft.world.item.enchantment.Enchantment>> CODEC;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.core.Holder<net.minecraft.world.item.enchantment.Enchantment>> STREAM_CODEC;
    public net.minecraft.world.item.enchantment.Enchantment(net.minecraft.network.chat.Component, net.minecraft.world.item.enchantment.Enchantment$EnchantmentDefinition, net.minecraft.core.HolderSet<net.minecraft.world.item.enchantment.Enchantment>, net.minecraft.core.component.DataComponentMap);
    public static net.minecraft.world.item.enchantment.Enchantment$Cost constantCost(int);
    public static net.minecraft.world.item.enchantment.Enchantment$Cost dynamicCost(int, int);
    public static net.minecraft.world.item.enchantment.Enchantment$EnchantmentDefinition definition(net.minecraft.core.HolderSet<net.minecraft.world.item.Item>, net.minecraft.core.HolderSet<net.minecraft.world.item.Item>, int, int, net.minecraft.world.item.enchantment.Enchantment$Cost, net.minecraft.world.item.enchantment.Enchantment$Cost, int, net.minecraft.world.entity.EquipmentSlotGroup...);
    public static net.minecraft.world.item.enchantment.Enchantment$EnchantmentDefinition definition(net.minecraft.core.HolderSet<net.minecraft.world.item.Item>, int, int, net.minecraft.world.item.enchantment.Enchantment$Cost, net.minecraft.world.item.enchantment.Enchantment$Cost, int, net.minecraft.world.entity.EquipmentSlotGroup...);
    public java.util.Map<net.minecraft.world.entity.EquipmentSlot, net.minecraft.world.item.ItemStack> getSlotItems(net.minecraft.world.entity.LivingEntity);
    public net.minecraft.core.HolderSet<net.minecraft.world.item.Item> getSupportedItems();
    public boolean matchingSlot(net.minecraft.world.entity.EquipmentSlot);
    public boolean isPrimaryItem(net.minecraft.world.item.ItemStack);
    public boolean isSupportedItem(net.minecraft.world.item.ItemStack);
    public int getWeight();
    public int getAnvilCost();
    public int getMinLevel();
    public int getMaxLevel();
    public int getMinCost(int);
    public int getMaxCost(int);
    public java.lang.String toString();
    public static boolean areCompatible(net.minecraft.core.Holder<net.minecraft.world.item.enchantment.Enchantment>, net.minecraft.core.Holder<net.minecraft.world.item.enchantment.Enchantment>);
    public static net.minecraft.network.chat.Component getFullname(net.minecraft.core.Holder<net.minecraft.world.item.enchantment.Enchantment>, int);
    public boolean canEnchant(net.minecraft.world.item.ItemStack);
    public <T> java.util.List<T> getEffects(net.minecraft.core.component.DataComponentType<java.util.List<T>>);
    public boolean isImmuneToDamage(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.entity.Entity, net.minecraft.world.damagesource.DamageSource);
    public void modifyDamageProtection(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, net.minecraft.world.damagesource.DamageSource, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyDurabilityChange(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyAmmoCount(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyPiercingCount(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyBlockExperience(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyMobExperience(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyDurabilityToRepairFromXp(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyTridentReturnToOwnerAcceleration(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyTridentSpinAttackStrength(net.minecraft.util.RandomSource, int, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyFishingTimeReduction(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyFishingLuckBonus(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyDamage(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, net.minecraft.world.damagesource.DamageSource, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyFallBasedDamage(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, net.minecraft.world.damagesource.DamageSource, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyKnockback(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, net.minecraft.world.damagesource.DamageSource, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyArmorEffectivness(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, net.minecraft.world.damagesource.DamageSource, org.apache.commons.lang3.mutable.MutableFloat);
    public void doPostAttack(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.item.enchantment.EnchantmentTarget, net.minecraft.world.entity.Entity, net.minecraft.world.damagesource.DamageSource);
    public static void doPostAttack(net.minecraft.world.item.enchantment.TargetedConditionalEffect<net.minecraft.world.item.enchantment.effects.EnchantmentEntityEffect>, net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.entity.Entity, net.minecraft.world.damagesource.DamageSource);
    public void doPostPiercingAttack(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.entity.Entity);
    public void modifyProjectileCount(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyProjectileSpread(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyCrossbowChargeTime(net.minecraft.util.RandomSource, int, org.apache.commons.lang3.mutable.MutableFloat);
    public void modifyUnfilteredValue(net.minecraft.core.component.DataComponentType<net.minecraft.world.item.enchantment.effects.EnchantmentValueEffect>, net.minecraft.util.RandomSource, int, org.apache.commons.lang3.mutable.MutableFloat);
    public void tick(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.entity.Entity);
    public void onProjectileSpawned(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.entity.Entity);
    public void onHitBlock(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.entity.Entity, net.minecraft.world.phys.Vec3, net.minecraft.world.level.block.state.BlockState);
    private void modifyItemFilteredCount(net.minecraft.core.component.DataComponentType<java.util.List<net.minecraft.world.item.enchantment.ConditionalEffect<net.minecraft.world.item.enchantment.effects.EnchantmentValueEffect>>>, net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemInstance, org.apache.commons.lang3.mutable.MutableFloat);
    private void modifyEntityFilteredValue(net.minecraft.core.component.DataComponentType<java.util.List<net.minecraft.world.item.enchantment.ConditionalEffect<net.minecraft.world.item.enchantment.effects.EnchantmentValueEffect>>>, net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, org.apache.commons.lang3.mutable.MutableFloat);
    private void modifyDamageFilteredValue(net.minecraft.core.component.DataComponentType<java.util.List<net.minecraft.world.item.enchantment.ConditionalEffect<net.minecraft.world.item.enchantment.effects.EnchantmentValueEffect>>>, net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.Entity, net.minecraft.world.damagesource.DamageSource, org.apache.commons.lang3.mutable.MutableFloat);
    public static net.minecraft.world.level.storage.loot.LootContext damageContext(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.entity.Entity, net.minecraft.world.damagesource.DamageSource);
    private static net.minecraft.world.level.storage.loot.LootContext itemContext(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.ItemInstance);
    private static net.minecraft.world.level.storage.loot.LootContext locationContext(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.entity.Entity, boolean);
    private static net.minecraft.world.level.storage.loot.LootContext entityContext(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.entity.Entity, net.minecraft.world.phys.Vec3);
    private static net.minecraft.world.level.storage.loot.LootContext blockHitContext(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.entity.Entity, net.minecraft.world.phys.Vec3, net.minecraft.world.level.block.state.BlockState);
    private static <T> void applyEffects(java.util.List<net.minecraft.world.item.enchantment.ConditionalEffect<T>>, net.minecraft.world.level.storage.loot.LootContext, net.minecraft.world.item.enchantment.Enchantment$GenericAction<T>);
    private static <T> void applyEffects(java.util.List<net.minecraft.world.item.enchantment.ConditionalEffect<T>>, net.minecraft.world.level.storage.loot.LootContext, org.apache.commons.lang3.mutable.MutableFloat, net.minecraft.world.item.enchantment.Enchantment$FloatAction<T>);
    public void runLocationChangedEffects(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.entity.LivingEntity);
    public void stopLocationBasedEffects(int, net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.entity.LivingEntity);
    public static net.minecraft.world.item.enchantment.Enchantment$Builder enchantment(net.minecraft.world.item.enchantment.Enchantment$EnchantmentDefinition);
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.network.chat.Component description();
    public net.minecraft.world.item.enchantment.Enchantment$EnchantmentDefinition definition();
    public net.minecraft.core.HolderSet<net.minecraft.world.item.enchantment.Enchantment> exclusiveSet();
    public net.minecraft.core.component.DataComponentMap effects();
    private static void lambda$runLocationChangedEffects$0(net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.entity.LivingEntity, int, net.minecraft.world.item.enchantment.effects.EnchantmentLocationBasedEffect);
    private static float lambda$modifyDamageFilteredValue$0(int, net.minecraft.world.entity.Entity, net.minecraft.world.item.enchantment.effects.EnchantmentValueEffect, float);
    private static float lambda$modifyEntityFilteredValue$0(int, net.minecraft.world.entity.Entity, net.minecraft.world.item.enchantment.effects.EnchantmentValueEffect, float);
    private static float lambda$modifyItemFilteredCount$0(int, net.minecraft.server.level.ServerLevel, net.minecraft.world.item.enchantment.effects.EnchantmentValueEffect, float);
    private static void lambda$onHitBlock$0(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.entity.Entity, net.minecraft.world.phys.Vec3, net.minecraft.world.item.enchantment.effects.EnchantmentEntityEffect);
    private static void lambda$onProjectileSpawned$0(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.entity.Entity, net.minecraft.world.item.enchantment.effects.EnchantmentEntityEffect);
    private static void lambda$tick$0(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.entity.Entity, net.minecraft.world.item.enchantment.effects.EnchantmentEntityEffect);
    private static void lambda$doPostPiercingAttack$0(net.minecraft.server.level.ServerLevel, int, net.minecraft.world.item.enchantment.EnchantedItemInUse, net.minecraft.world.entity.Entity, net.minecraft.world.item.enchantment.effects.EnchantmentEntityEffect);
    private static float lambda$modifyDamageProtection$0(int, net.minecraft.world.entity.Entity, net.minecraft.world.item.enchantment.effects.EnchantmentValueEffect, float);
    private static boolean lambda$matchingSlot$0(net.minecraft.world.entity.EquipmentSlot, net.minecraft.world.entity.EquipmentSlotGroup);
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
