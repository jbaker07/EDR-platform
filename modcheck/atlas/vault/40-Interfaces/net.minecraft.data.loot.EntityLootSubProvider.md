---
type: "interface"
fqcn: "net.minecraft.data.loot.EntityLootSubProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.loot.EntityLootSubProvider

System: [[20-Systems/net.minecraft.data.loot|net.minecraft.data.loot]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/dat` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `add(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/resou` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (26, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.data.loot.EntityLootSubProvider implements net.minecraft.data.loot.LootTableSubProvider {
    protected final net.minecraft.data.loot.LootTableSubProvider$Context output;
    protected final net.minecraft.core.HolderGetter<net.minecraft.world.item.Item> items;
    protected final net.minecraft.core.HolderGetter<net.minecraft.world.item.enchantment.Enchantment> enchantments;
    protected final net.minecraft.core.HolderGetter<net.minecraft.world.entity.EntityType<?>> entityTypes;
    protected final net.minecraft.core.HolderGetter<net.minecraft.world.entity.animal.frog.FrogVariant> frogVariants;
    protected final net.minecraft.core.HolderGetter<net.minecraft.world.damagesource.DamageType> damageTypes;
    protected final net.minecraft.core.HolderGetter<net.minecraft.world.level.storage.loot.LootTable> lootTables;
    private final net.minecraft.world.flag.FeatureFlagSet allowed;
    private final net.minecraft.world.flag.FeatureFlagSet required;
    private final java.util.Map<net.minecraft.world.entity.EntityType<?>, java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>, net.minecraft.world.level.storage.loot.LootTable$Builder>> map;
    protected net.minecraft.data.loot.EntityLootSubProvider(net.minecraft.world.flag.FeatureFlagSet, net.minecraft.data.loot.LootTableSubProvider$Context);
    protected net.minecraft.data.loot.EntityLootSubProvider(net.minecraft.world.flag.FeatureFlagSet, net.minecraft.world.flag.FeatureFlagSet, net.minecraft.data.loot.LootTableSubProvider$Context);
    protected net.minecraft.advancements.predicates.DamageSourcePredicate$Builder projectileDamage();
    protected final net.minecraft.world.level.storage.loot.predicates.AnyOfCondition$Builder shouldSmeltLoot();
    public static net.minecraft.world.level.storage.loot.LootPool$Builder createSheepDispatchPool(net.minecraft.world.level.block.ColorCollection<net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.LootTable>>);
    public abstract void generate();
    public void run();
    protected net.minecraft.world.level.storage.loot.predicates.LootItemCondition$Builder killedByFrog();
    protected net.minecraft.world.level.storage.loot.predicates.LootItemCondition$Builder killedByFrogVariant(net.minecraft.resources.ResourceKey<net.minecraft.world.entity.animal.frog.FrogVariant>);
    protected void add(net.minecraft.world.entity.EntityType<?>, net.minecraft.world.level.storage.loot.LootTable$Builder);
    protected void add(net.minecraft.world.entity.EntityType<?>, net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>, net.minecraft.world.level.storage.loot.LootTable$Builder);
    private static java.util.Map lambda$add$1(net.minecraft.world.entity.EntityType);
    private static java.lang.IllegalStateException lambda$add$0(net.minecraft.world.entity.EntityType);
    private void lambda$run$0(java.util.Set, net.minecraft.core.Holder$Reference);
    private static java.lang.String lambda$run$2(net.minecraft.resources.ResourceKey);
    private void lambda$run$1(java.util.Set, net.minecraft.core.Holder$Reference, net.minecraft.resources.ResourceKey, net.minecraft.world.level.storage.loot.LootTable$Builder);
}
```
