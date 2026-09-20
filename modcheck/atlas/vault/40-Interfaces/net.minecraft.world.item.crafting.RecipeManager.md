---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.RecipeManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.RecipeManager

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/item/crafting/RecipeAccess`, `net/fabricmc/fabric/api/recipe/v1/FabricRecipeManager`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<init>` | `(Lnet/minecraft/core/HolderLookup$Provider;)V` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `recipes` | `Lnet/minecraft/world/item/crafting/RecipeMap;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | declared |

## Declared members (8 fields, 38 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final RECIPE_PROPERTY_SETS : Ljava/util/Map;
private final recipes : Lnet/minecraft/world/item/crafting/RecipeMap;
private propertySets : Ljava/util/Map;
private stonecutterRecipes : Lnet/minecraft/world/item/crafting/SelectableRecipe$SingleInputSet;
private allDisplays : Ljava/util/List;
private recipeToDisplay : Ljava/util/Map;
private final learnableRecipes : Ljava/util/Collection;
public <init>(Lnet/minecraft/core/HolderLookup$Provider;)V
public finalizeRecipeLoading(Lnet/minecraft/world/flag/FeatureFlagSet;)V
private static filterDisabled(Lnet/minecraft/world/flag/FeatureFlagSet;Ljava/util/List;)Ljava/util/List;
private static isIngredientEnabled(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/item/crafting/Ingredient;)Z
public getRecipeFor(Lnet/minecraft/world/item/crafting/RecipeType;Lnet/minecraft/world/item/crafting/RecipeInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public getRecipeFor(Lnet/minecraft/world/item/crafting/RecipeType;Lnet/minecraft/world/item/crafting/RecipeInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/crafting/RecipeHolder;)Ljava/util/Optional;
public getRecipeFor(Lnet/minecraft/world/item/crafting/RecipeType;Lnet/minecraft/world/item/crafting/RecipeInput;Lnet/minecraft/world/level/Level;)Ljava/util/Optional;
public byKey(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
private byKeyTyped(Lnet/minecraft/world/item/crafting/RecipeType;Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/crafting/RecipeHolder;
public getSynchronizedItemProperties()Ljava/util/Map;
public getSynchronizedStonecutterRecipes()Lnet/minecraft/world/item/crafting/SelectableRecipe$SingleInputSet;
public propertySet(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/crafting/RecipePropertySet;
public stonecutterRecipes()Lnet/minecraft/world/item/crafting/SelectableRecipe$SingleInputSet;
public getRecipes()Ljava/util/Collection;
public getLearnableRecipes()Ljava/util/Collection;
public getRecipeFromDisplay(Lnet/minecraft/world/item/crafting/display/RecipeDisplayId;)Lnet/minecraft/world/item/crafting/RecipeManager$ServerDisplayInfo;
public listDisplaysForRecipe(Lnet/minecraft/resources/ResourceKey;Ljava/util/function/Consumer;)V
public static createCheck(Lnet/minecraft/world/item/crafting/RecipeType;)Lnet/minecraft/world/item/crafting/RecipeManager$CachedCheck;
private static unpackRecipeInfo(Ljava/lang/Iterable;Lnet/minecraft/world/flag/FeatureFlagSet;)Ljava/util/List;
private static forSingleInput(Lnet/minecraft/world/item/crafting/RecipeType;)Lnet/minecraft/world/item/crafting/RecipeManager$IngredientExtractor;
private static synthetic lambda$forSingleInput$0(Lnet/minecraft/world/item/crafting/RecipeType;Lnet/minecraft/world/item/crafting/Recipe;)Ljava/util/Optional;
private static synthetic lambda$unpackRecipeInfo$0(Lit/unimi/dsi/fastutil/objects/Object2IntMap;Ljava/lang/Object;)I
private static synthetic lambda$listDisplaysForRecipe$0(Ljava/util/function/Consumer;Lnet/minecraft/world/item/crafting/RecipeManager$ServerDisplayInfo;)V
private static synthetic lambda$isIngredientEnabled$0(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/core/Holder;)Z
private static synthetic lambda$filterDisabled$0(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/item/crafting/Ingredient;)Z
private static synthetic lambda$finalizeRecipeLoading$5(Lnet/minecraft/world/item/crafting/RecipeManager$ServerDisplayInfo;)Lnet/minecraft/resources/ResourceKey;
private static synthetic lambda$finalizeRecipeLoading$4(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/item/crafting/RecipeManager$IngredientCollector;)Lnet/minecraft/world/item/crafting/RecipePropertySet;
private static synthetic lambda$finalizeRecipeLoading$3(Lnet/minecraft/world/item/crafting/RecipeManager$IngredientCollector;)Lnet/minecraft/resources/ResourceKey;
private static synthetic lambda$finalizeRecipeLoading$1(Ljava/util/List;Lnet/minecraft/world/flag/FeatureFlagSet;Ljava/util/List;Lnet/minecraft/world/item/crafting/RecipeHolder;)V
private static synthetic lambda$finalizeRecipeLoading$2(Lnet/minecraft/world/item/crafting/Recipe;Lnet/minecraft/world/item/crafting/RecipeManager$IngredientCollector;)V
private static synthetic lambda$finalizeRecipeLoading$0(Ljava/util/Map$Entry;)Lnet/minecraft/world/item/crafting/RecipeManager$IngredientCollector;
private static synthetic lambda$new$0(Lnet/minecraft/world/item/crafting/RecipeHolder;)Z
private static synthetic lambda$static$4(Lnet/minecraft/world/item/crafting/Recipe;)Ljava/util/Optional;
private static synthetic lambda$static$3(Lnet/minecraft/world/item/crafting/Recipe;)Ljava/util/Optional;
private static synthetic lambda$static$2(Lnet/minecraft/world/item/crafting/Recipe;)Ljava/util/Optional;
private static synthetic lambda$static$1(Lnet/minecraft/world/item/crafting/Recipe;)Ljava/util/Optional;
private static synthetic lambda$static$0(Lnet/minecraft/world/item/crafting/Recipe;)Ljava/util/Optional;
static <clinit>()V
```
