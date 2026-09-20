---
type: "interface"
fqcn: "net.minecraft.data.recipes.RecipeProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.RecipeProvider

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `buildRecipes` | `()V` | exact | invokevirtual@50 in `FabricRecipeProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `lambda$waxRecipes$0` | `(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/level/b` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `lambda$waxRecipes$0` | `(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/level/b` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `stonecutterResultFromBase` | `(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level` | exact | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (6 fields, 144 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final items : Lnet/minecraft/core/HolderGetter;
private final trimPatterns : Lnet/minecraft/core/HolderGetter;
protected final output : Lnet/minecraft/data/recipes/RecipeOutput;
protected final advancementOutput : Lnet/minecraft/data/worldgen/BootstrapContext;
private static final SHAPE_BUILDERS : Ljava/util/Map;
private static final STONECUTTER_RECIPE_BUILDERS : Ljava/util/Map;
protected <init>(Lnet/minecraft/data/worldgen/BootstrapContext;Lnet/minecraft/data/worldgen/BootstrapContext;)V
public abstract buildRecipes()V
public generateForEnabledBlockFamilies(Lnet/minecraft/world/flag/FeatureFlagSet;)V
public oneToOneConversionRecipe(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;Ljava/lang/String;)V
public oneToOneConversionRecipe(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;Ljava/lang/String;I)V
public oreSmelting(Ljava/util/List;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/CookingBookCategory;Lnet/minecraft/world/level/ItemLike;FILjava/lang/String;)V
public oreBlasting(Ljava/util/List;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/CookingBookCategory;Lnet/minecraft/world/level/ItemLike;FILjava/lang/String;)V
public final oreCooking(Lnet/minecraft/world/item/crafting/AbstractCookingRecipe$Factory;Ljava/util/List;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/CookingBookCategory;Lnet/minecraft/world/level/ItemLike;FILjava/lang/String;Ljava/lang/String;)V
public netheriteSmithing(Lnet/minecraft/world/item/Item;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/Item;)V
public trimSmithing(Lnet/minecraft/world/item/Item;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/ResourceKey;)V
public twoByTwoPacker(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public threeByThreePacker(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;Ljava/lang/String;)V
public threeByThreePacker(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public planksFromLog(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/tags/TagKey;I)V
public planksFromLogs(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/tags/TagKey;I)V
public woodFromLogs(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public woodenBoat(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public chestBoat(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public final buttonBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public doorBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public final fenceBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public final fenceGateBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public pressurePlate(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public final pressurePlateBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public slab(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public shelf(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public slabBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public stairBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public trapdoorBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public final signBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public hangingSignBuilder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public colorItemWithDye(Ljava/util/List;Ljava/util/List;Ljava/lang/String;Lnet/minecraft/data/recipes/RecipeCategory;)V
public colorWithDye(Ljava/util/List;Ljava/util/List;Lnet/minecraft/world/item/Item;Ljava/lang/String;Lnet/minecraft/data/recipes/RecipeCategory;)V
public carpet(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public bedFromPlanksAndWool(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public banner(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public stainedGlassFromGlassAndDye(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public dryGhast(Lnet/minecraft/world/level/ItemLike;)V
public harness(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public stainedGlassPaneFromStainedGlass(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public stainedGlassPaneFromGlassPaneAndDye(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public coloredTerracottaFromTerracottaAndDye(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public concretePowder(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public candle(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public wall(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public final wallBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public final bricksBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public final tilesBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public final pillarBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public polished(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public final polishedBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/RecipeBuilder;
public cut(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public final cutBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/ShapedRecipeBuilder;
public chiseled(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public mosaicBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public chiseledBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/ShapedRecipeBuilder;
public carpetBuilder(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/ShapedRecipeBuilder;
public stonecutterResultFromBase(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public stonecutterResultFromBase(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;I)V
public final smeltingResultFromBase(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public nineBlockStorageRecipes(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;)V
public nineBlockStorageRecipesWithCustomPacking(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Ljava/lang/String;Ljava/lang/String;)V
public nineBlockStorageRecipesRecipesWithCustomUnpacking(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Ljava/lang/String;Ljava/lang/String;)V
public final nineBlockStorageRecipes(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
public copySmithingTemplate(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
public copySmithingTemplate(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;)V
public cookRecipes(Ljava/lang/String;Lnet/minecraft/world/item/crafting/AbstractCookingRecipe$Factory;I)V
public final simpleCookingRecipe(Ljava/lang/String;Lnet/minecraft/world/item/crafting/AbstractCookingRecipe$Factory;ILnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;F)V
public waxRecipes(Lnet/minecraft/world/flag/FeatureFlagSet;)V
public grate(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V
public copperBulb(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V
public waxedChiseled(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V
public suspiciousStew(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/level/block/SuspiciousEffectHolder;)V
public dyedItem(Lnet/minecraft/world/item/Item;Ljava/lang/String;)V
public dyedShulkerBoxRecipe(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)V
public dyedBundleRecipe(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)V
public cushionRecipe(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)V
public generateRecipes(Lnet/minecraft/data/BlockFamily;Lnet/minecraft/world/flag/FeatureFlagSet;)V
public final generateCraftingRecipe(Lnet/minecraft/data/BlockFamily;Lnet/minecraft/data/BlockFamily$Variant;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/ItemLike;)V
public final generateSmeltingRecipe(Lnet/minecraft/data/BlockFamily$Variant;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/ItemLike;)V
public final generateStonecutterRecipe(Lnet/minecraft/data/BlockFamily;Lnet/minecraft/data/BlockFamily$Variant;Lnet/minecraft/world/level/block/Block;)V
public final getBaseBlockForCrafting(Lnet/minecraft/data/BlockFamily;Lnet/minecraft/data/BlockFamily$Variant;)Lnet/minecraft/world/level/block/Block;
public final getCraftingCriterionName(Lnet/minecraft/data/BlockFamily;Lnet/minecraft/data/BlockFamily$Variant;Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;
public static insideOf(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/advancements/triggers/Criterion;
public bredAnimal()Lnet/minecraft/advancements/triggers/Criterion;
public final has(Lnet/minecraft/advancements/predicates/MinMaxBounds$Ints;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/advancements/triggers/Criterion;
public has(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/advancements/triggers/Criterion;
public has(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/advancements/triggers/Criterion;
public static inventoryTrigger([Lnet/minecraft/advancements/predicates/ItemPredicate$Builder;)Lnet/minecraft/advancements/triggers/Criterion;
public static inventoryTrigger([Lnet/minecraft/advancements/predicates/ItemPredicate;)Lnet/minecraft/advancements/triggers/Criterion;
public static getHasName(Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;
public static getItemName(Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;
public static getSimpleRecipeName(Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;
public static getConversionRecipeName(Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;
public static getSmeltingRecipeName(Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;
public static getBlastingRecipeName(Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;
public tag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/item/crafting/Ingredient;
public shaped(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/ShapedRecipeBuilder;
public shaped(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;I)Lnet/minecraft/data/recipes/ShapedRecipeBuilder;
public shapeless(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/ItemStackTemplate;)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
public shapeless(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
public shapeless(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;I)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
private static synthetic lambda$inventoryTrigger$0(I)[Lnet/minecraft/advancements/predicates/ItemPredicate;
private static synthetic lambda$static$30(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
private static synthetic lambda$static$29(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
private static synthetic lambda$static$28(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
private static synthetic lambda$static$27(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
private static synthetic lambda$static$26(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
private static synthetic lambda$static$25(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
private static synthetic lambda$static$24(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
private static synthetic lambda$static$23(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
private static synthetic lambda$static$22(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
private static synthetic lambda$static$21(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)V
private static synthetic lambda$static$20(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$19(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$18(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$17(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$16(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$15(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$14(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$13(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$12(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$11(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$10(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$9(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$8(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$7(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$6(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$5(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$4(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$3(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$2(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$1(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$static$0(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$getCraftingCriterionName$0(Lnet/minecraft/world/level/ItemLike;)Ljava/lang/String;
private synthetic lambda$generateStonecutterRecipe$0(Lnet/minecraft/data/BlockFamily;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/data/BlockFamily$Variant;Lnet/minecraft/world/level/block/Block;)V
private static synthetic lambda$generateCraftingRecipe$0(Lnet/minecraft/data/recipes/RecipeBuilder;Lnet/minecraft/data/BlockFamily$Variant;Ljava/lang/String;)V
private synthetic lambda$generateRecipes$0(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/data/BlockFamily;Lnet/minecraft/data/BlockFamily$Variant;Lnet/minecraft/world/level/block/Block;)V
private synthetic lambda$dyedItem$0(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/crafting/Recipe$CommonInfo;Lnet/minecraft/world/item/crafting/CraftingRecipe$CraftingBookInfo;)Lnet/minecraft/world/item/crafting/Recipe;
private synthetic lambda$waxRecipes$0(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V
private static synthetic lambda$banner$0(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/item/crafting/Recipe;
private static synthetic lambda$colorWithDye$0(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)Z
private synthetic lambda$generateForEnabledBlockFamilies$0(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/data/BlockFamily;)V
static <clinit>()V
```
