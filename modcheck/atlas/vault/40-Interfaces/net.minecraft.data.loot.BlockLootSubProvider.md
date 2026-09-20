---
type: "interface"
fqcn: "net.minecraft.data.loot.BlockLootSubProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.loot.BlockLootSubProvider

System: [[20-Systems/net.minecraft.data.loot|net.minecraft.data.loot]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/util/Set;Lnet/minecraft/world/flag/FeatureFlagSet;Lne` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `add(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (80, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.data.loot.BlockLootSubProvider implements net.minecraft.data.loot.LootTableSubProvider {
    protected final net.minecraft.data.loot.LootTableSubProvider$Context output;
    protected final net.minecraft.core.HolderGetter<net.minecraft.world.item.enchantment.Enchantment> enchantments;
    protected final net.minecraft.core.HolderGetter<net.minecraft.world.item.Item> items;
    protected final net.minecraft.core.HolderGetter<net.minecraft.world.level.block.Block> blocks;
    protected final net.minecraft.core.HolderGetter<net.minecraft.world.level.storage.loot.predicates.LootItemCondition> predicates;
    private final java.util.Set<net.minecraft.world.item.Item> explosionResistant;
    private final net.minecraft.world.flag.FeatureFlagSet enabledFeatures;
    private final java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>, net.minecraft.world.level.storage.loot.LootTable$Builder> map;
    protected static final float[] NORMAL_LEAVES_SAPLING_CHANCES;
    private static final float[] NORMAL_LEAVES_STICK_CHANCES;
    protected net.minecraft.data.loot.BlockLootSubProvider(java.util.Set<net.minecraft.world.item.Item>, net.minecraft.world.flag.FeatureFlagSet, net.minecraft.data.loot.LootTableSubProvider$Context);
    protected net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.predicates.LootItemCondition> hasSilkTouch();
    protected net.minecraft.world.level.storage.loot.predicates.LootItemCondition$Builder doesNotHaveSilkTouch();
    protected net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.predicates.LootItemCondition> hasShears();
    private net.minecraft.world.level.storage.loot.predicates.LootItemCondition$Builder hasShearsOrSilkTouch();
    private net.minecraft.world.level.storage.loot.predicates.LootItemCondition$Builder doesNotHaveShearsOrSilkTouch();
    protected <T extends net.minecraft.world.level.storage.loot.functions.FunctionUserBuilder<T>> T applyExplosionDecay(net.minecraft.world.level.ItemLike, net.minecraft.world.level.storage.loot.functions.FunctionUserBuilder<T>);
    protected <T extends net.minecraft.world.level.storage.loot.predicates.ConditionUserBuilder<T>> T applyExplosionCondition(net.minecraft.world.level.ItemLike, net.minecraft.world.level.storage.loot.predicates.ConditionUserBuilder<T>);
    public net.minecraft.world.level.storage.loot.LootTable$Builder createSingleItemTable(net.minecraft.world.level.ItemLike);
    private static net.minecraft.world.level.storage.loot.LootTable$Builder createSelfDropDispatchTable(net.minecraft.world.level.block.Block, net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.predicates.LootItemCondition>, net.minecraft.world.level.storage.loot.entries.LootPoolEntryContainer$Builder<?>);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createSilkTouchDispatchTable(net.minecraft.world.level.block.Block, net.minecraft.world.level.storage.loot.entries.LootPoolEntryContainer$Builder<?>);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createShearsDispatchTable(net.minecraft.world.level.block.Block, net.minecraft.world.level.storage.loot.entries.LootPoolEntryContainer$Builder<?>);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createSilkTouchOrShearsDispatchTable(net.minecraft.world.level.block.Block, net.minecraft.world.level.storage.loot.entries.LootPoolEntryContainer$Builder<?>);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createSingleItemTableWithSilkTouch(net.minecraft.world.level.block.Block, net.minecraft.world.level.ItemLike);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createSingleItemTable(net.minecraft.world.level.ItemLike, net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.providers.number.ints.ContextIntProvider>);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createSingleItemTableWithSilkTouch(net.minecraft.world.level.block.Block, net.minecraft.world.level.ItemLike, net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.providers.number.ints.ContextIntProvider>);
    private net.minecraft.world.level.storage.loot.LootTable$Builder createSilkTouchOnlyTable(net.minecraft.world.level.ItemLike);
    private net.minecraft.world.level.storage.loot.LootTable$Builder createPotFlowerItemTable(net.minecraft.world.level.ItemLike);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createSlabItemTable(net.minecraft.world.level.block.Block);
    protected <T extends java.lang.Comparable<T> & net.minecraft.util.StringRepresentable> net.minecraft.world.level.storage.loot.LootTable$Builder createSinglePropConditionTable(net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.properties.Property<T>, T);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createNameableBlockEntityTable(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createShulkerBoxDrop(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createCopperOreDrops(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createLapisOreDrops(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createRedstoneOreDrops(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createBannerDrop(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createBeeNestDrop(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createBeeHiveDrop(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createCaveVinesDrop(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createCopperGolemStatueBlock(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createOreDrop(net.minecraft.world.level.block.Block, net.minecraft.world.item.Item);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createMushroomBlockDrop(net.minecraft.world.level.block.Block, net.minecraft.world.level.ItemLike);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createGrassDrops(net.minecraft.world.level.block.Block);
    public net.minecraft.world.level.storage.loot.LootTable$Builder createStemDrops(net.minecraft.world.level.block.Block, net.minecraft.world.item.Item);
    public net.minecraft.world.level.storage.loot.LootTable$Builder createAttachedStemDrops(net.minecraft.world.level.block.Block, net.minecraft.world.item.Item);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createShearsOnlyDrop(net.minecraft.world.level.ItemLike);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createShearsOrSilkTouchOnlyDrop(net.minecraft.world.level.ItemLike);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createMultifaceBlockDrops(net.minecraft.world.level.block.Block, net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.predicates.LootItemCondition>);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createMultifaceBlockDrops(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createMossyCarpetBlockDrops(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createLeavesDrops(net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block, float...);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createOakLeavesDrops(net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block, float...);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createMangroveLeavesDrops(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createCropDrops(net.minecraft.world.level.block.Block, net.minecraft.world.item.Item, net.minecraft.world.item.Item, net.minecraft.world.level.storage.loot.predicates.LootItemCondition$Builder);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createDoublePlantShearsDrop(net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createDoublePlantWithSeedDrops(net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createCandleDrops(net.minecraft.world.level.block.Block);
    public net.minecraft.world.level.storage.loot.LootTable$Builder createSegmentedBlockDrops(net.minecraft.world.level.block.Block);
    protected static net.minecraft.world.level.storage.loot.LootTable$Builder createCandleCakeDrops(net.minecraft.world.level.block.Block);
    public static net.minecraft.world.level.storage.loot.LootTable$Builder noDrop();
    protected abstract void generate();
    public void run();
    protected void addNetherVinesDropTable(net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block);
    protected net.minecraft.world.level.storage.loot.LootTable$Builder createDoorTable(net.minecraft.world.level.block.Block);
    protected void dropPottedContents(net.minecraft.world.level.block.Block);
    protected void otherWhenSilkTouch(net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block);
    protected void dropOther(net.minecraft.world.level.block.Block, net.minecraft.world.level.ItemLike);
    protected void dropWhenSilkTouch(net.minecraft.world.level.block.Block);
    protected void dropSelf(net.minecraft.world.level.block.Block);
    protected void add(net.minecraft.world.level.block.Block, java.util.function.Function<net.minecraft.world.level.block.Block, net.minecraft.world.level.storage.loot.LootTable$Builder>);
    protected void add(net.minecraft.world.level.block.Block, net.minecraft.world.level.storage.loot.LootTable$Builder);
    private static java.lang.IllegalStateException lambda$add$0(net.minecraft.world.level.block.Block);
    private net.minecraft.world.level.storage.loot.LootTable$Builder lambda$dropPottedContents$0(net.minecraft.world.level.block.Block);
    private void lambda$run$0(java.util.Set, net.minecraft.world.level.block.Block, net.minecraft.resources.ResourceKey);
    private net.minecraft.world.level.storage.loot.functions.LootItemFunction$Builder lambda$createSegmentedBlockDrops$0(net.minecraft.world.level.block.Block, net.minecraft.world.level.block.SegmentableBlock, java.lang.Integer);
    private net.minecraft.world.level.storage.loot.functions.LootItemFunction$Builder lambda$createCandleDrops$0(net.minecraft.world.level.block.Block, java.lang.Integer);
    private net.minecraft.world.level.storage.loot.functions.LootItemFunction$Builder lambda$createMultifaceBlockDrops$1(net.minecraft.world.level.block.Block, net.minecraft.core.Direction);
    private net.minecraft.world.level.storage.loot.functions.LootItemFunction$Builder lambda$createMultifaceBlockDrops$0(net.minecraft.world.level.block.Block, net.minecraft.core.Direction);
    private net.minecraft.world.level.storage.loot.functions.LootItemFunction$Builder lambda$createStemDrops$0(net.minecraft.world.level.block.Block, java.lang.Integer);
    static {};
}
```
