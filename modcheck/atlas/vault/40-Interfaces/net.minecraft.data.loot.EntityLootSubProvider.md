---
type: "interface"
fqcn: "net.minecraft.data.loot.EntityLootSubProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.loot.EntityLootSubProvider

System: [[20-Systems/net.minecraft.data.loot|net.minecraft.data.loot]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/data/loot/LootTableSubProvider`, `net/fabricmc/fabric/api/datagen/v1/loot/FabricEntityLootSubProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/data/loot/Loo` | exact | invokespecial@21 in `FabricEntityLootSubProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/data/loot/Loo` | exact | invokespecial@16 in `ConditionEntityLootSubProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `add` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/resources/Resou` | exact | invokevirtual@15 in `ConditionEntityLootSubProvider.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `run` | `()V` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (10 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
protected final output : Lnet/minecraft/data/loot/LootTableSubProvider$Context;
protected final items : Lnet/minecraft/core/HolderGetter;
protected final enchantments : Lnet/minecraft/core/HolderGetter;
protected final entityTypes : Lnet/minecraft/core/HolderGetter;
protected final frogVariants : Lnet/minecraft/core/HolderGetter;
protected final damageTypes : Lnet/minecraft/core/HolderGetter;
protected final lootTables : Lnet/minecraft/core/HolderGetter;
private final allowed : Lnet/minecraft/world/flag/FeatureFlagSet;
private final required : Lnet/minecraft/world/flag/FeatureFlagSet;
private final map : Ljava/util/Map;
protected <init>(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/data/loot/LootTableSubProvider$Context;)V
protected <init>(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/data/loot/LootTableSubProvider$Context;)V
protected projectileDamage()Lnet/minecraft/advancements/predicates/DamageSourcePredicate$Builder;
protected final shouldSmeltLoot()Lnet/minecraft/world/level/storage/loot/predicates/AnyOfCondition$Builder;
public static createSheepDispatchPool(Lnet/minecraft/world/level/block/ColorCollection;)Lnet/minecraft/world/level/storage/loot/LootPool$Builder;
public abstract generate()V
public run()V
protected killedByFrog()Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;
protected killedByFrogVariant(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;
public add(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/storage/loot/LootTable$Builder;)V
public add(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/storage/loot/LootTable$Builder;)V
private static synthetic lambda$add$1(Lnet/minecraft/world/entity/EntityType;)Ljava/util/Map;
private static synthetic lambda$add$0(Lnet/minecraft/world/entity/EntityType;)Ljava/lang/IllegalStateException;
private synthetic lambda$run$0(Ljava/util/Set;Lnet/minecraft/core/Holder$Reference;)V
private static synthetic lambda$run$2(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
private synthetic lambda$run$1(Ljava/util/Set;Lnet/minecraft/core/Holder$Reference;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/storage/loot/LootTable$Builder;)V
```
