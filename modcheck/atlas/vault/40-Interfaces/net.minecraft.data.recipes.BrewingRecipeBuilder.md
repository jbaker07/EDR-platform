---
type: "interface"
fqcn: "net.minecraft.data.recipes.BrewingRecipeBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.BrewingRecipeBuilder

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `save(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.recipes.BrewingRecipeBuilder implements net.minecraft.data.recipes.RecipeBuilder {
    private final net.minecraft.world.item.crafting.PotionIngredient input;
    private final net.minecraft.world.item.crafting.PotionIngredient reagent;
    private final net.minecraft.world.item.ItemStackTemplate output;
    private net.minecraft.data.recipes.BrewingRecipeBuilder(net.minecraft.world.item.crafting.PotionIngredient, net.minecraft.world.item.crafting.PotionIngredient, net.minecraft.world.item.ItemStackTemplate);
    private static net.minecraft.world.item.crafting.PotionIngredient potionIngredient(net.minecraft.world.item.Item, net.minecraft.core.Holder<net.minecraft.world.item.alchemy.Potion>);
    private static net.minecraft.world.item.ItemStackTemplate potionOutput(net.minecraft.world.item.Item, net.minecraft.core.Holder<net.minecraft.world.item.alchemy.Potion>);
    public static net.minecraft.data.recipes.BrewingRecipeBuilder brewingMix(net.minecraft.world.item.Item, net.minecraft.core.Holder<net.minecraft.world.item.alchemy.Potion>, net.minecraft.world.item.Item, net.minecraft.core.Holder<net.minecraft.world.item.alchemy.Potion>);
    public static net.minecraft.data.recipes.BrewingRecipeBuilder brewingContainerTransform(net.minecraft.world.item.Item, net.minecraft.core.Holder<net.minecraft.world.item.alchemy.Potion>, net.minecraft.world.item.Item, net.minecraft.world.item.Item);
    public net.minecraft.data.recipes.RecipeBuilder unlockedBy(java.lang.String, net.minecraft.advancements.triggers.Criterion<?>);
    public net.minecraft.data.recipes.BrewingRecipeBuilder group(java.lang.String);
    public static java.util.Optional<net.minecraft.core.Holder<net.minecraft.world.item.alchemy.Potion>> getExactPotion(net.minecraft.core.component.predicates.PotionsPredicate);
    private java.util.Optional<net.minecraft.core.Holder<net.minecraft.world.item.alchemy.Potion>> getIngredientPotion(net.minecraft.world.item.crafting.PotionIngredient);
    public net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>> defaultId();
    public void save(net.minecraft.data.recipes.RecipeOutput, net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>);
    public net.minecraft.data.recipes.RecipeBuilder group(java.lang.String);
    private static java.lang.String lambda$defaultId$0(net.minecraft.resources.ResourceKey, net.minecraft.resources.ResourceKey, java.lang.String);
}
```
