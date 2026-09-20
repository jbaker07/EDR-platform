---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.RecipeManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.RecipeManager

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (46, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.item.crafting.RecipeManager implements net.minecraft.world.item.crafting.RecipeAccess {
    private static final org.slf4j.Logger LOGGER;
    private static final java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.RecipePropertySet>, net.minecraft.world.item.crafting.RecipeManager$IngredientExtractor> RECIPE_PROPERTY_SETS;
    private final net.minecraft.world.item.crafting.RecipeMap recipes;
    private java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.RecipePropertySet>, net.minecraft.world.item.crafting.RecipePropertySet> propertySets;
    private net.minecraft.world.item.crafting.SelectableRecipe$SingleInputSet<net.minecraft.world.item.crafting.StonecutterRecipe> stonecutterRecipes;
    private java.util.List<net.minecraft.world.item.crafting.RecipeManager$ServerDisplayInfo> allDisplays;
    private java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>, java.util.List<net.minecraft.world.item.crafting.RecipeManager$ServerDisplayInfo>> recipeToDisplay;
    private final java.util.Collection<net.minecraft.world.item.crafting.RecipeHolder<?>> learnableRecipes;
    public net.minecraft.world.item.crafting.RecipeManager(net.minecraft.core.HolderLookup$Provider);
    public void finalizeRecipeLoading(net.minecraft.world.flag.FeatureFlagSet);
    private static java.util.List<net.minecraft.world.item.crafting.Ingredient> filterDisabled(net.minecraft.world.flag.FeatureFlagSet, java.util.List<net.minecraft.world.item.crafting.Ingredient>);
    private static boolean isIngredientEnabled(net.minecraft.world.flag.FeatureFlagSet, net.minecraft.world.item.crafting.Ingredient);
    public <I extends net.minecraft.world.item.crafting.RecipeInput, T extends net.minecraft.world.item.crafting.Recipe<I>> java.util.Optional<net.minecraft.world.item.crafting.RecipeHolder<T>> getRecipeFor(net.minecraft.world.item.crafting.RecipeType<T>, I, net.minecraft.world.level.Level, net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>);
    public <I extends net.minecraft.world.item.crafting.RecipeInput, T extends net.minecraft.world.item.crafting.Recipe<I>> java.util.Optional<net.minecraft.world.item.crafting.RecipeHolder<T>> getRecipeFor(net.minecraft.world.item.crafting.RecipeType<T>, I, net.minecraft.world.level.Level, net.minecraft.world.item.crafting.RecipeHolder<T>);
    public <I extends net.minecraft.world.item.crafting.RecipeInput, T extends net.minecraft.world.item.crafting.Recipe<I>> java.util.Optional<net.minecraft.world.item.crafting.RecipeHolder<T>> getRecipeFor(net.minecraft.world.item.crafting.RecipeType<T>, I, net.minecraft.world.level.Level);
    public java.util.Optional<net.minecraft.world.item.crafting.RecipeHolder<?>> byKey(net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>);
    private <T extends net.minecraft.world.item.crafting.Recipe<?>> net.minecraft.world.item.crafting.RecipeHolder<T> byKeyTyped(net.minecraft.world.item.crafting.RecipeType<T>, net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>);
    public java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.RecipePropertySet>, net.minecraft.world.item.crafting.RecipePropertySet> getSynchronizedItemProperties();
    public net.minecraft.world.item.crafting.SelectableRecipe$SingleInputSet<net.minecraft.world.item.crafting.StonecutterRecipe> getSynchronizedStonecutterRecipes();
    public net.minecraft.world.item.crafting.RecipePropertySet propertySet(net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.RecipePropertySet>);
    public net.minecraft.world.item.crafting.SelectableRecipe$SingleInputSet<net.minecraft.world.item.crafting.StonecutterRecipe> stonecutterRecipes();
    public java.util.Collection<net.minecraft.world.item.crafting.RecipeHolder<?>> getRecipes();
    public java.util.Collection<net.minecraft.world.item.crafting.RecipeHolder<?>> getLearnableRecipes();
    public net.minecraft.world.item.crafting.RecipeManager$ServerDisplayInfo getRecipeFromDisplay(net.minecraft.world.item.crafting.display.RecipeDisplayId);
    public void listDisplaysForRecipe(net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>, java.util.function.Consumer<net.minecraft.world.item.crafting.display.RecipeDisplayEntry>);
    public static <I extends net.minecraft.world.item.crafting.RecipeInput, T extends net.minecraft.world.item.crafting.Recipe<I>> net.minecraft.world.item.crafting.RecipeManager$CachedCheck<I, T> createCheck(net.minecraft.world.item.crafting.RecipeType<T>);
    private static java.util.List<net.minecraft.world.item.crafting.RecipeManager$ServerDisplayInfo> unpackRecipeInfo(java.lang.Iterable<net.minecraft.world.item.crafting.RecipeHolder<?>>, net.minecraft.world.flag.FeatureFlagSet);
    private static net.minecraft.world.item.crafting.RecipeManager$IngredientExtractor forSingleInput(net.minecraft.world.item.crafting.RecipeType<? extends net.minecraft.world.item.crafting.SingleItemRecipe>);
    private static java.util.Optional lambda$forSingleInput$0(net.minecraft.world.item.crafting.RecipeType, net.minecraft.world.item.crafting.Recipe);
    private static int lambda$unpackRecipeInfo$0(it.unimi.dsi.fastutil.objects.Object2IntMap, java.lang.Object);
    private static void lambda$listDisplaysForRecipe$0(java.util.function.Consumer, net.minecraft.world.item.crafting.RecipeManager$ServerDisplayInfo);
    private static boolean lambda$isIngredientEnabled$0(net.minecraft.world.flag.FeatureFlagSet, net.minecraft.core.Holder);
    private static boolean lambda$filterDisabled$0(net.minecraft.world.flag.FeatureFlagSet, net.minecraft.world.item.crafting.Ingredient);
    private static net.minecraft.resources.ResourceKey lambda$finalizeRecipeLoading$5(net.minecraft.world.item.crafting.RecipeManager$ServerDisplayInfo);
    private static net.minecraft.world.item.crafting.RecipePropertySet lambda$finalizeRecipeLoading$4(net.minecraft.world.flag.FeatureFlagSet, net.minecraft.world.item.crafting.RecipeManager$IngredientCollector);
    private static net.minecraft.resources.ResourceKey lambda$finalizeRecipeLoading$3(net.minecraft.world.item.crafting.RecipeManager$IngredientCollector);
    private static void lambda$finalizeRecipeLoading$1(java.util.List, net.minecraft.world.flag.FeatureFlagSet, java.util.List, net.minecraft.world.item.crafting.RecipeHolder);
    private static void lambda$finalizeRecipeLoading$2(net.minecraft.world.item.crafting.Recipe, net.minecraft.world.item.crafting.RecipeManager$IngredientCollector);
    private static net.minecraft.world.item.crafting.RecipeManager$IngredientCollector lambda$finalizeRecipeLoading$0(java.util.Map$Entry);
    private static boolean lambda$new$0(net.minecraft.world.item.crafting.RecipeHolder);
    private static java.util.Optional lambda$static$4(net.minecraft.world.item.crafting.Recipe);
    private static java.util.Optional lambda$static$3(net.minecraft.world.item.crafting.Recipe);
    private static java.util.Optional lambda$static$2(net.minecraft.world.item.crafting.Recipe);
    private static java.util.Optional lambda$static$1(net.minecraft.world.item.crafting.Recipe);
    private static java.util.Optional lambda$static$0(net.minecraft.world.item.crafting.Recipe);
    static {};
}
```
