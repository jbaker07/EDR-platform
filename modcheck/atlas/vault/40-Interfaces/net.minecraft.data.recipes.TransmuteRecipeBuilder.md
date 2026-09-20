---
type: "interface"
fqcn: "net.minecraft.data.recipes.TransmuteRecipeBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.TransmuteRecipeBuilder

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `save(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (21, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.recipes.TransmuteRecipeBuilder implements net.minecraft.data.recipes.RecipeBuilder {
    private final net.minecraft.data.recipes.RecipeCategory category;
    private final net.minecraft.world.item.crafting.TransmuteResult result;
    private final net.minecraft.world.item.crafting.Ingredient input;
    private final net.minecraft.world.item.crafting.Ingredient material;
    private final net.minecraft.data.recipes.RecipeUnlockAdvancementBuilder advancementBuilder;
    private java.lang.String group;
    private net.minecraft.advancements.predicates.MinMaxBounds$Ints materialCount;
    private boolean addMaterialCountToOutput;
    private net.minecraft.data.recipes.TransmuteRecipeBuilder(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.TransmuteResult, net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.item.crafting.Ingredient);
    public static net.minecraft.data.recipes.TransmuteRecipeBuilder transmute(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.item.Item);
    public static net.minecraft.data.recipes.TransmuteRecipeBuilder transmute(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.item.ItemStackTemplate);
    public static net.minecraft.data.recipes.TransmuteRecipeBuilder transmute(net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.item.crafting.TransmuteResult);
    public net.minecraft.data.recipes.TransmuteRecipeBuilder unlockedBy(java.lang.String, net.minecraft.advancements.triggers.Criterion<?>);
    public net.minecraft.data.recipes.TransmuteRecipeBuilder group(java.lang.String);
    public net.minecraft.data.recipes.TransmuteRecipeBuilder addMaterialCountToOutput();
    public net.minecraft.data.recipes.TransmuteRecipeBuilder setMaterialCount(net.minecraft.advancements.predicates.MinMaxBounds$Ints);
    public net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>> defaultId();
    public void save(net.minecraft.data.recipes.RecipeOutput, net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>);
    public net.minecraft.data.recipes.RecipeBuilder group(java.lang.String);
    public net.minecraft.data.recipes.RecipeBuilder unlockedBy(java.lang.String, net.minecraft.advancements.triggers.Criterion);
    private net.minecraft.resources.ResourceKey lambda$defaultId$0(net.minecraft.core.Holder);
}
```
