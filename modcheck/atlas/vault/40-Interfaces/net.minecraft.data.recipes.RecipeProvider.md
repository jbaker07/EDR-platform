---
type: "interface"
fqcn: "net.minecraft.data.recipes.RecipeProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.RecipeProvider

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `lambda$waxRecipes$0` | `@ModifyArg at INVOKE Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;save(Lne` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `lambda$waxRecipes$0` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `stonecutterResultFromBase(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;I)V` | `@ModifyArg at INVOKE Lnet/minecraft/data/recipes/SingleItemRecipeBuilder;save(Ln` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (150, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.data.recipes.RecipeProvider {
    private final net.minecraft.core.HolderGetter<net.minecraft.world.item.Item> items;
    private final net.minecraft.core.HolderGetter<net.minecraft.world.item.equipment.trim.TrimPattern> trimPatterns;
    protected final net.minecraft.data.recipes.RecipeOutput output;
    protected final net.minecraft.data.worldgen.BootstrapContext<net.minecraft.advancements.Advancement> advancementOutput;
    private static final java.util.Map<net.minecraft.data.BlockFamily$Variant, net.minecraft.data.recipes.RecipeProvider$FamilyCraftingRecipeProvider> SHAPE_BUILDERS;
    private static final java.util.Map<net.minecraft.data.BlockFamily$Variant, net.minecraft.data.recipes.RecipeProvider$FamilyStonecutterRecipeProvider> STONECUTTER_RECIPE_BUILDERS;
    protected net.minecraft.data.recipes.RecipeProvider(net.minecraft.data.worldgen.BootstrapContext<net.minecraft.world.item.crafting.Recipe<?>>, net.minecraft.data.worldgen.BootstrapContext<net.minecraft.advancements.Advancement>);
    protected abstract void buildRecipes();
    protected void generateForEnabledBlockFamilies(net.minecraft.world.flag.FeatureFlagSet);
    protected void oneToOneConversionRecipe(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike, java.lang.String);
    protected void oneToOneConversionRecipe(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike, java.lang.String, int);
    protected void oreSmelting(java.util.List<net.minecraft.world.level.ItemLike>, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.CookingBookCategory, net.minecraft.world.level.ItemLike, float, int, java.lang.String);
    protected void oreBlasting(java.util.List<net.minecraft.world.level.ItemLike>, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.CookingBookCategory, net.minecraft.world.level.ItemLike, float, int, java.lang.String);
    private <T extends net.minecraft.world.item.crafting.AbstractCookingRecipe> void oreCooking(net.minecraft.world.item.crafting.AbstractCookingRecipe$Factory<T>, java.util.List<net.minecraft.world.level.ItemLike>, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.CookingBookCategory, net.minecraft.world.level.ItemLike, float, int, java.lang.String, java.lang.String);
    protected void netheriteSmithing(net.minecraft.world.item.Item, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.Item);
    protected void trimSmithing(net.minecraft.world.item.Item, net.minecraft.resources.ResourceKey<net.minecraft.world.item.equipment.trim.TrimPattern>, net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>);
    protected void twoByTwoPacker(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void threeByThreePacker(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike, java.lang.String);
    protected void threeByThreePacker(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void planksFromLog(net.minecraft.world.level.ItemLike, net.minecraft.tags.TagKey<net.minecraft.world.item.Item>, int);
    protected void planksFromLogs(net.minecraft.world.level.ItemLike, net.minecraft.tags.TagKey<net.minecraft.world.item.Item>, int);
    protected void woodFromLogs(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void woodenBoat(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void chestBoat(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private net.minecraft.data.recipes.RecipeBuilder buttonBuilder(net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected net.minecraft.data.recipes.RecipeBuilder doorBuilder(net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    private net.minecraft.data.recipes.RecipeBuilder fenceBuilder(net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    private net.minecraft.data.recipes.RecipeBuilder fenceGateBuilder(net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected void pressurePlate(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private net.minecraft.data.recipes.RecipeBuilder pressurePlateBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected void slab(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void shelf(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected net.minecraft.data.recipes.RecipeBuilder slabBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected net.minecraft.data.recipes.RecipeBuilder stairBuilder(net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected net.minecraft.data.recipes.RecipeBuilder trapdoorBuilder(net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    private net.minecraft.data.recipes.RecipeBuilder signBuilder(net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected net.minecraft.data.recipes.RecipeBuilder hangingSignBuilder(net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected void colorItemWithDye(java.util.List<net.minecraft.world.item.Item>, java.util.List<net.minecraft.world.item.Item>, java.lang.String, net.minecraft.data.recipes.RecipeCategory);
    protected void colorWithDye(java.util.List<net.minecraft.world.item.Item>, java.util.List<net.minecraft.world.item.Item>, net.minecraft.world.item.Item, java.lang.String, net.minecraft.data.recipes.RecipeCategory);
    protected void carpet(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void bedFromPlanksAndWool(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void banner(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void stainedGlassFromGlassAndDye(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void dryGhast(net.minecraft.world.level.ItemLike);
    protected void harness(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void stainedGlassPaneFromStainedGlass(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void stainedGlassPaneFromGlassPaneAndDye(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void coloredTerracottaFromTerracottaAndDye(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void concretePowder(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void candle(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void wall(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private net.minecraft.data.recipes.RecipeBuilder wallBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    private net.minecraft.data.recipes.RecipeBuilder bricksBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    private net.minecraft.data.recipes.RecipeBuilder tilesBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    private net.minecraft.data.recipes.RecipeBuilder pillarBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected void polished(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private net.minecraft.data.recipes.RecipeBuilder polishedBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected void cut(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private net.minecraft.data.recipes.ShapedRecipeBuilder cutBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected void chiseled(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void mosaicBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected net.minecraft.data.recipes.ShapedRecipeBuilder chiseledBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected net.minecraft.data.recipes.ShapedRecipeBuilder carpetBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected void stonecutterResultFromBase(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void stonecutterResultFromBase(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike, int);
    private void smeltingResultFromBase(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void nineBlockStorageRecipes(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike);
    protected void nineBlockStorageRecipesWithCustomPacking(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, java.lang.String, java.lang.String);
    protected void nineBlockStorageRecipesRecipesWithCustomUnpacking(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, java.lang.String, java.lang.String);
    private void nineBlockStorageRecipes(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, java.lang.String, java.lang.String, java.lang.String, java.lang.String);
    protected void copySmithingTemplate(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected void copySmithingTemplate(net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient);
    protected <T extends net.minecraft.world.item.crafting.AbstractCookingRecipe> void cookRecipes(java.lang.String, net.minecraft.world.item.crafting.AbstractCookingRecipe$Factory<T>, int);
    private <T extends net.minecraft.world.item.crafting.AbstractCookingRecipe> void simpleCookingRecipe(java.lang.String, net.minecraft.world.item.crafting.AbstractCookingRecipe$Factory<T>, int, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike, float);
    protected void waxRecipes(net.minecraft.world.flag.FeatureFlagSet);
    protected void grate(net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block);
    protected void copperBulb(net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block);
    protected void waxedChiseled(net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block);
    protected void suspiciousStew(net.minecraft.world.item.Item, net.minecraft.world.level.block.SuspiciousEffectHolder);
    protected void dyedItem(net.minecraft.world.item.Item, java.lang.String);
    protected void dyedShulkerBoxRecipe(net.minecraft.world.item.Item, net.minecraft.world.item.Item);
    protected void dyedBundleRecipe(net.minecraft.world.item.Item, net.minecraft.world.item.Item);
    protected void cushionRecipe(net.minecraft.world.item.Item, net.minecraft.world.item.Item);
    protected void generateRecipes(net.minecraft.data.BlockFamily, net.minecraft.world.flag.FeatureFlagSet);
    private void generateCraftingRecipe(net.minecraft.data.BlockFamily, net.minecraft.data.BlockFamily$Variant, net.minecraft.world.level.block.Block, net.minecraft.world.level.ItemLike);
    private void generateSmeltingRecipe(net.minecraft.data.BlockFamily$Variant, net.minecraft.world.level.block.Block, net.minecraft.world.level.ItemLike);
    private void generateStonecutterRecipe(net.minecraft.data.BlockFamily, net.minecraft.data.BlockFamily$Variant, net.minecraft.world.level.block.Block);
    private net.minecraft.world.level.block.Block getBaseBlockForCrafting(net.minecraft.data.BlockFamily, net.minecraft.data.BlockFamily$Variant);
    private java.lang.String getCraftingCriterionName(net.minecraft.data.BlockFamily, net.minecraft.data.BlockFamily$Variant, net.minecraft.world.level.ItemLike);
    private static net.minecraft.advancements.triggers.Criterion<net.minecraft.advancements.triggers.EnterBlockTrigger$TriggerInstance> insideOf(net.minecraft.world.level.block.Block);
    protected net.minecraft.advancements.triggers.Criterion<net.minecraft.advancements.triggers.BredAnimalsTrigger$TriggerInstance> bredAnimal();
    private net.minecraft.advancements.triggers.Criterion<net.minecraft.advancements.triggers.InventoryChangeTrigger$TriggerInstance> has(net.minecraft.advancements.predicates.MinMaxBounds$Ints, net.minecraft.world.level.ItemLike);
    protected net.minecraft.advancements.triggers.Criterion<net.minecraft.advancements.triggers.InventoryChangeTrigger$TriggerInstance> has(net.minecraft.world.level.ItemLike);
    protected net.minecraft.advancements.triggers.Criterion<net.minecraft.advancements.triggers.InventoryChangeTrigger$TriggerInstance> has(net.minecraft.tags.TagKey<net.minecraft.world.item.Item>);
    private static net.minecraft.advancements.triggers.Criterion<net.minecraft.advancements.triggers.InventoryChangeTrigger$TriggerInstance> inventoryTrigger(net.minecraft.advancements.predicates.ItemPredicate$Builder...);
    private static net.minecraft.advancements.triggers.Criterion<net.minecraft.advancements.triggers.InventoryChangeTrigger$TriggerInstance> inventoryTrigger(net.minecraft.advancements.predicates.ItemPredicate...);
    protected static java.lang.String getHasName(net.minecraft.world.level.ItemLike);
    protected static java.lang.String getItemName(net.minecraft.world.level.ItemLike);
    protected static java.lang.String getSimpleRecipeName(net.minecraft.world.level.ItemLike);
    protected static java.lang.String getConversionRecipeName(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    protected static java.lang.String getSmeltingRecipeName(net.minecraft.world.level.ItemLike);
    protected static java.lang.String getBlastingRecipeName(net.minecraft.world.level.ItemLike);
    protected net.minecraft.world.item.crafting.Ingredient tag(net.minecraft.tags.TagKey<net.minecraft.world.item.Item>);
    protected net.minecraft.data.recipes.ShapedRecipeBuilder shaped(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike);
    protected net.minecraft.data.recipes.ShapedRecipeBuilder shaped(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, int);
    protected net.minecraft.data.recipes.ShapelessRecipeBuilder shapeless(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.ItemStackTemplate);
    protected net.minecraft.data.recipes.ShapelessRecipeBuilder shapeless(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike);
    protected net.minecraft.data.recipes.ShapelessRecipeBuilder shapeless(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, int);
    private static net.minecraft.advancements.predicates.ItemPredicate[] lambda$inventoryTrigger$0(int);
    private static void lambda$static$30(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static void lambda$static$29(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static void lambda$static$28(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static void lambda$static$27(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static void lambda$static$26(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static void lambda$static$25(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static void lambda$static$24(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static void lambda$static$23(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static void lambda$static$22(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static void lambda$static$21(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$20(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$19(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$18(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$17(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$16(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$15(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$14(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$13(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$12(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$11(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$10(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$9(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$8(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$7(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$6(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$5(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$4(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$3(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$2(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$1(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static net.minecraft.data.recipes.RecipeBuilder lambda$static$0(net.minecraft.data.recipes.RecipeProvider, net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike);
    private static java.lang.String lambda$getCraftingCriterionName$0(net.minecraft.world.level.ItemLike);
    private void lambda$generateStonecutterRecipe$0(net.minecraft.data.BlockFamily, net.minecraft.world.level.block.Block, net.minecraft.data.BlockFamily$Variant, net.minecraft.world.level.block.Block);
    private static void lambda$generateCraftingRecipe$0(net.minecraft.data.recipes.RecipeBuilder, net.minecraft.data.BlockFamily$Variant, java.lang.String);
    private void lambda$generateRecipes$0(net.minecraft.world.flag.FeatureFlagSet, net.minecraft.data.BlockFamily, net.minecraft.data.BlockFamily$Variant, net.minecraft.world.level.block.Block);
    private net.minecraft.world.item.crafting.Recipe lambda$dyedItem$0(net.minecraft.world.item.Item, net.minecraft.world.item.crafting.Recipe$CommonInfo, net.minecraft.world.item.crafting.CraftingRecipe$CraftingBookInfo);
    private void lambda$waxRecipes$0(net.minecraft.world.flag.FeatureFlagSet, net.minecraft.world.level.block.Block, net.minecraft.world.level.block.Block);
    private static net.minecraft.world.item.crafting.Recipe lambda$banner$0(net.minecraft.world.level.ItemLike);
    private static boolean lambda$colorWithDye$0(net.minecraft.world.item.Item, net.minecraft.world.item.Item);
    private void lambda$generateForEnabledBlockFamilies$0(net.minecraft.world.flag.FeatureFlagSet, net.minecraft.data.BlockFamily);
    static {};
}
```
