---
type: "interface"
fqcn: "net.minecraft.data.recipes.ShapedRecipeBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.ShapedRecipeBuilder

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `save(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.recipes.ShapedRecipeBuilder implements net.minecraft.data.recipes.RecipeBuilder {
    private final net.minecraft.core.HolderGetter<net.minecraft.world.item.Item> items;
    private final net.minecraft.data.recipes.RecipeCategory category;
    private final net.minecraft.world.item.ItemStackTemplate result;
    private final java.util.List<java.lang.String> rows;
    private final java.util.Map<java.lang.Character, net.minecraft.world.item.crafting.Ingredient> key;
    private final net.minecraft.data.recipes.RecipeUnlockAdvancementBuilder advancementBuilder;
    private java.lang.String group;
    private boolean showNotification;
    private net.minecraft.data.recipes.ShapedRecipeBuilder(net.minecraft.core.HolderGetter<net.minecraft.world.item.Item>, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.item.ItemStackTemplate);
    private net.minecraft.data.recipes.ShapedRecipeBuilder(net.minecraft.core.HolderGetter<net.minecraft.world.item.Item>, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, int);
    public static net.minecraft.data.recipes.ShapedRecipeBuilder shaped(net.minecraft.core.HolderGetter<net.minecraft.world.item.Item>, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike);
    public static net.minecraft.data.recipes.ShapedRecipeBuilder shaped(net.minecraft.core.HolderGetter<net.minecraft.world.item.Item>, net.minecraft.data.recipes.RecipeCategory, net.minecraft.world.level.ItemLike, int);
    public net.minecraft.data.recipes.ShapedRecipeBuilder define(java.lang.Character, net.minecraft.tags.TagKey<net.minecraft.world.item.Item>);
    public net.minecraft.data.recipes.ShapedRecipeBuilder define(java.lang.Character, net.minecraft.world.level.ItemLike);
    public net.minecraft.data.recipes.ShapedRecipeBuilder define(java.lang.Character, net.minecraft.world.item.crafting.Ingredient);
    public net.minecraft.data.recipes.ShapedRecipeBuilder pattern(java.lang.String);
    public net.minecraft.data.recipes.ShapedRecipeBuilder unlockedBy(java.lang.String, net.minecraft.advancements.triggers.Criterion<?>);
    public net.minecraft.data.recipes.ShapedRecipeBuilder group(java.lang.String);
    public net.minecraft.data.recipes.ShapedRecipeBuilder showNotification(boolean);
    public net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>> defaultId();
    public void save(net.minecraft.data.recipes.RecipeOutput, net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>);
    public net.minecraft.data.recipes.RecipeBuilder group(java.lang.String);
    public net.minecraft.data.recipes.RecipeBuilder unlockedBy(java.lang.String, net.minecraft.advancements.triggers.Criterion);
}
```
