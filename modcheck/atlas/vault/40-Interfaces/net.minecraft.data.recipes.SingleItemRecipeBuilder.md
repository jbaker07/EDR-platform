---
type: "interface"
fqcn: "net.minecraft.data.recipes.SingleItemRecipeBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.SingleItemRecipeBuilder

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `save(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.recipes.SingleItemRecipeBuilder implements net.minecraft.data.recipes.RecipeBuilder {
    private final net.minecraft.data.recipes.RecipeCategory category;
    private final net.minecraft.world.item.ItemStackTemplate result;
    private final net.minecraft.world.item.crafting.Ingredient ingredient;
    private final net.minecraft.data.recipes.RecipeUnlockAdvancementBuilder advancementBuilder;
    private final net.minecraft.world.item.crafting.SingleItemRecipe$Factory<?> factory;
    private net.minecraft.data.recipes.SingleItemRecipeBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.SingleItemRecipe$Factory<?>, net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.item.ItemStackTemplate);
    public net.minecraft.data.recipes.SingleItemRecipeBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.SingleItemRecipe$Factory<?>, net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.level.ItemLike, int);
    public static net.minecraft.data.recipes.SingleItemRecipeBuilder stonecutting(net.minecraft.world.item.crafting.Ingredient, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, int);
    public net.minecraft.data.recipes.SingleItemRecipeBuilder unlockedBy(java.lang.String, net.minecraft.advancements.triggers.Criterion<?>);
    public net.minecraft.data.recipes.SingleItemRecipeBuilder group(java.lang.String);
    public net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>> defaultId();
    public void save(net.minecraft.data.recipes.RecipeOutput, net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>);
    public net.minecraft.data.recipes.RecipeBuilder group(java.lang.String);
    public net.minecraft.data.recipes.RecipeBuilder unlockedBy(java.lang.String, net.minecraft.advancements.triggers.Criterion);
}
```
