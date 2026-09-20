---
type: "interface"
fqcn: "net.minecraft.data.recipes.SimpleCookingRecipeBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.SimpleCookingRecipeBuilder

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `save(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.recipes.SimpleCookingRecipeBuilder implements net.minecraft.data.recipes.RecipeBuilder {
    private final net.minecraft.data.recipes.RecipeCategory craftingCategory;
    private final net.minecraft.world.item.crafting.CookingBookCategory cookingCategory;
    private final net.minecraft.world.item.ItemStackTemplate result;
    private final net.minecraft.world.item.crafting.Ingredient ingredient;
    private final float experience;
    private final int cookingTime;
    private final net.minecraft.data.recipes.RecipeUnlockAdvancementBuilder advancementBuilder;
    private java.lang.String group;
    private final net.minecraft.world.item.crafting.AbstractCookingRecipe$Factory<?> factory;
    private net.minecraft.data.recipes.SimpleCookingRecipeBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.CookingBookCategory, net.minecraft.world.item.ItemStackTemplate, net.minecraft.world.item.crafting.Ingredient, float, int, net.minecraft.world.item.crafting.AbstractCookingRecipe$Factory<?>);
    private net.minecraft.data.recipes.SimpleCookingRecipeBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.CookingBookCategory, net.minecraft.world.level.ItemLike, net.minecraft.world.item.crafting.Ingredient, float, int, net.minecraft.world.item.crafting.AbstractCookingRecipe$Factory<?>);
    public static <T extends net.minecraft.world.item.crafting.AbstractCookingRecipe> net.minecraft.data.recipes.SimpleCookingRecipeBuilder generic(net.minecraft.world.item.crafting.Ingredient, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.CookingBookCategory, net.minecraft.world.level.ItemLike, float, int, net.minecraft.world.item.crafting.AbstractCookingRecipe$Factory<T>);
    public static net.minecraft.data.recipes.SimpleCookingRecipeBuilder campfireCooking(net.minecraft.world.item.crafting.Ingredient, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, float, int);
    public static net.minecraft.data.recipes.SimpleCookingRecipeBuilder blasting(net.minecraft.world.item.crafting.Ingredient, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.CookingBookCategory, net.minecraft.world.level.ItemLike, float, int);
    public static net.minecraft.data.recipes.SimpleCookingRecipeBuilder smelting(net.minecraft.world.item.crafting.Ingredient, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.CookingBookCategory, net.minecraft.world.level.ItemLike, float, int);
    public static net.minecraft.data.recipes.SimpleCookingRecipeBuilder smoking(net.minecraft.world.item.crafting.Ingredient, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, float, int);
    public net.minecraft.data.recipes.SimpleCookingRecipeBuilder unlockedBy(java.lang.String, net.minecraft.advancements.triggers.Criterion<?>);
    public net.minecraft.data.recipes.SimpleCookingRecipeBuilder group(java.lang.String);
    public net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>> defaultId();
    public void save(net.minecraft.data.recipes.RecipeOutput, net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>);
    public net.minecraft.data.recipes.RecipeBuilder group(java.lang.String);
    public net.minecraft.data.recipes.RecipeBuilder unlockedBy(java.lang.String, net.minecraft.advancements.triggers.Criterion);
}
```
