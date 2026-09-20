---
type: "interface"
fqcn: "net.minecraft.data.recipes.SmithingTransformRecipeBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.SmithingTransformRecipeBuilder

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `save(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.recipes.SmithingTransformRecipeBuilder {
    private final net.minecraft.world.item.crafting.Ingredient template;
    private final net.minecraft.world.item.crafting.Ingredient base;
    private final net.minecraft.world.item.crafting.Ingredient addition;
    private final net.minecraft.data.recipes.RecipeCategory category;
    private final net.minecraft.world.item.ItemStackTemplate result;
    private final net.minecraft.data.recipes.RecipeUnlockAdvancementBuilder advancementBuilder;
    public net.minecraft.data.recipes.SmithingTransformRecipeBuilder(net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.item.crafting.Ingredient, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.ItemStackTemplate);
    public static net.minecraft.data.recipes.SmithingTransformRecipeBuilder smithing(net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.item.crafting.Ingredient, net.minecraft.world.item.crafting.Ingredient, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.Item);
    public net.minecraft.data.recipes.SmithingTransformRecipeBuilder unlocks(java.lang.String, net.minecraft.advancements.triggers.Criterion<?>);
    public void save(net.minecraft.data.recipes.RecipeOutput, java.lang.String);
    public void save(net.minecraft.data.recipes.RecipeOutput, net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>);
}
```
