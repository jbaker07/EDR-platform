---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.ShapelessRecipe"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.ShapelessRecipe

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `matches(Lnet/minecraft/world/item/crafting/CraftingInput;Lnet/minecraft/world/level/Level;)Z` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.item.crafting.ShapelessRecipe extends net.minecraft.world.item.crafting.NormalCraftingRecipe {
    public static final com.mojang.serialization.MapCodec<net.minecraft.world.item.crafting.ShapelessRecipe> MAP_CODEC;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.world.item.crafting.ShapelessRecipe> STREAM_CODEC;
    public static final net.minecraft.world.item.crafting.RecipeSerializer<net.minecraft.world.item.crafting.ShapelessRecipe> SERIALIZER;
    private final net.minecraft.world.item.ItemStackTemplate result;
    private final java.util.List<net.minecraft.world.item.crafting.Ingredient> ingredients;
    public net.minecraft.world.item.crafting.ShapelessRecipe(net.minecraft.world.item.crafting.Recipe$CommonInfo, net.minecraft.world.item.crafting.CraftingRecipe$CraftingBookInfo, net.minecraft.world.item.ItemStackTemplate, java.util.List<net.minecraft.world.item.crafting.Ingredient>);
    public net.minecraft.world.item.crafting.RecipeSerializer<net.minecraft.world.item.crafting.ShapelessRecipe> getSerializer();
    protected net.minecraft.world.item.crafting.PlacementInfo createPlacementInfo();
    public boolean matches(net.minecraft.world.item.crafting.CraftingInput, net.minecraft.world.level.Level);
    public net.minecraft.world.item.ItemStack assemble(net.minecraft.world.item.crafting.CraftingInput);
    public java.util.List<net.minecraft.world.item.crafting.display.RecipeDisplay> display();
    public net.minecraft.world.item.ItemStack assemble(net.minecraft.world.item.crafting.RecipeInput);
    public boolean matches(net.minecraft.world.item.crafting.RecipeInput, net.minecraft.world.level.Level);
    private static java.util.List lambda$static$8(net.minecraft.world.item.crafting.ShapelessRecipe);
    private static net.minecraft.world.item.ItemStackTemplate lambda$static$7(net.minecraft.world.item.crafting.ShapelessRecipe);
    private static net.minecraft.world.item.crafting.CraftingRecipe$CraftingBookInfo lambda$static$6(net.minecraft.world.item.crafting.ShapelessRecipe);
    private static net.minecraft.world.item.crafting.Recipe$CommonInfo lambda$static$5(net.minecraft.world.item.crafting.ShapelessRecipe);
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static java.util.List lambda$static$4(net.minecraft.world.item.crafting.ShapelessRecipe);
    private static net.minecraft.world.item.ItemStackTemplate lambda$static$3(net.minecraft.world.item.crafting.ShapelessRecipe);
    private static net.minecraft.world.item.crafting.CraftingRecipe$CraftingBookInfo lambda$static$2(net.minecraft.world.item.crafting.ShapelessRecipe);
    private static net.minecraft.world.item.crafting.Recipe$CommonInfo lambda$static$1(net.minecraft.world.item.crafting.ShapelessRecipe);
    static {};
}
```
