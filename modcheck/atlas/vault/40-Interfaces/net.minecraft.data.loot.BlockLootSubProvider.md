---
type: "interface"
fqcn: "net.minecraft.data.loot.BlockLootSubProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.loot.BlockLootSubProvider

System: [[20-Systems/net.minecraft.data.loot|net.minecraft.data.loot]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/data/loot/LootTableSubProvider`, `net/fabricmc/fabric/api/datagen/v1/loot/FabricBlockLootSubProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/Set;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraf` | exact | invokespecial@24 in `FabricBlockLootSubProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/util/Set;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraf` | exact | invokespecial@19 in `ConditionBlockLootSubProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `add` | `(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/sto` | exact | invokevirtual@14 in `ConditionBlockLootSubProvider.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `run` | `()V` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (10 fields, 70 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
protected final output : Lnet/minecraft/data/loot/LootTableSubProvider$Context;
protected final enchantments : Lnet/minecraft/core/HolderGetter;
protected final items : Lnet/minecraft/core/HolderGetter;
protected final blocks : Lnet/minecraft/core/HolderGetter;
protected final predicates : Lnet/minecraft/core/HolderGetter;
private final explosionResistant : Ljava/util/Set;
private final enabledFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
private final map : Ljava/util/Map;
protected static final NORMAL_LEAVES_SAPLING_CHANCES : [F
private static final NORMAL_LEAVES_STICK_CHANCES : [F
protected <init>(Ljava/util/Set;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/data/loot/LootTableSubProvider$Context;)V
public hasSilkTouch()Lnet/minecraft/core/Holder;
public doesNotHaveSilkTouch()Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;
public hasShears()Lnet/minecraft/core/Holder;
public final hasShearsOrSilkTouch()Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;
public final doesNotHaveShearsOrSilkTouch()Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;
public applyExplosionDecay(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/storage/loot/functions/FunctionUserBuilder;)Lnet/minecraft/world/level/storage/loot/functions/FunctionUserBuilder;
public applyExplosionCondition(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/storage/loot/predicates/ConditionUserBuilder;)Lnet/minecraft/world/level/storage/loot/predicates/ConditionUserBuilder;
public createSingleItemTable(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public static createSelfDropDispatchTable(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/core/Holder;Lnet/minecraft/world/level/storage/loot/entries/LootPoolEntryContainer$Builder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createSilkTouchDispatchTable(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/storage/loot/entries/LootPoolEntryContainer$Builder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createShearsDispatchTable(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/storage/loot/entries/LootPoolEntryContainer$Builder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createSilkTouchOrShearsDispatchTable(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/storage/loot/entries/LootPoolEntryContainer$Builder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createSingleItemTableWithSilkTouch(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createSingleItemTable(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createSingleItemTableWithSilkTouch(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public final createSilkTouchOnlyTable(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public final createPotFlowerItemTable(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createSlabItemTable(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createSinglePropConditionTable(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang/Comparable;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createNameableBlockEntityTable(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createShulkerBoxDrop(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createCopperOreDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createLapisOreDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createRedstoneOreDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createBannerDrop(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createBeeNestDrop(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createBeeHiveDrop(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createCaveVinesDrop(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createCopperGolemStatueBlock(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createOreDrop(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/item/Item;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createMushroomBlockDrop(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createGrassDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createStemDrops(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/item/Item;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createAttachedStemDrops(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/item/Item;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createShearsOnlyDrop(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createShearsOrSilkTouchOnlyDrop(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createMultifaceBlockDrops(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createMultifaceBlockDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createMossyCarpetBlockDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createLeavesDrops(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;[F)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createOakLeavesDrops(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;[F)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createMangroveLeavesDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createCropDrops(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createDoublePlantShearsDrop(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createDoublePlantWithSeedDrops(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createCandleDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public createSegmentedBlockDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public static createCandleCakeDrops(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public static noDrop()Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public abstract generate()V
public run()V
public addNetherVinesDropTable(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V
public createDoorTable(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public dropPottedContents(Lnet/minecraft/world/level/block/Block;)V
public otherWhenSilkTouch(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V
public dropOther(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/ItemLike;)V
public dropWhenSilkTouch(Lnet/minecraft/world/level/block/Block;)V
public dropSelf(Lnet/minecraft/world/level/block/Block;)V
public add(Lnet/minecraft/world/level/block/Block;Ljava/util/function/Function;)V
public add(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/storage/loot/LootTable$Builder;)V
private static synthetic lambda$add$0(Lnet/minecraft/world/level/block/Block;)Ljava/lang/IllegalStateException;
private synthetic lambda$dropPottedContents$0(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
private synthetic lambda$run$0(Ljava/util/Set;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/resources/ResourceKey;)V
private synthetic lambda$createSegmentedBlockDrops$0(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/SegmentableBlock;Ljava/lang/Integer;)Lnet/minecraft/world/level/storage/loot/functions/LootItemFunction$Builder;
private synthetic lambda$createCandleDrops$0(Lnet/minecraft/world/level/block/Block;Ljava/lang/Integer;)Lnet/minecraft/world/level/storage/loot/functions/LootItemFunction$Builder;
private synthetic lambda$createMultifaceBlockDrops$1(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/core/Direction;)Lnet/minecraft/world/level/storage/loot/functions/LootItemFunction$Builder;
private synthetic lambda$createMultifaceBlockDrops$0(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/core/Direction;)Lnet/minecraft/world/level/storage/loot/functions/LootItemFunction$Builder;
private synthetic lambda$createStemDrops$0(Lnet/minecraft/world/level/block/Block;Ljava/lang/Integer;)Lnet/minecraft/world/level/storage/loot/functions/LootItemFunction$Builder;
static <clinit>()V
```
