---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.Recipe"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.Recipe

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getSerializer()Lnet/minecraft/world/item/crafting/RecipeSerializer;` | `` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getType()Lnet/minecraft/world/item/crafting/RecipeType;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `matches(Lnet/minecraft/world/item/crafting/RecipeInput;Lnet/minecra` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.item.crafting.Recipe<T extends net.minecraft.world.item.crafting.RecipeInput> {
    public static final com.mojang.serialization.Codec<net.minecraft.world.item.crafting.Recipe<?>> DIRECT_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>> KEY_CODEC;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.world.item.crafting.Recipe<?>> STREAM_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.core.Holder<net.minecraft.world.item.crafting.Recipe<?>>> CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.core.HolderSet<net.minecraft.world.item.crafting.Recipe<?>>> LIST_CODEC;
    public abstract boolean matches(T, net.minecraft.world.level.Level);
    public abstract net.minecraft.world.item.ItemStack assemble(T);
    public default boolean isSpecial();
    public abstract boolean showNotification();
    public abstract java.lang.String group();
    public abstract net.minecraft.world.item.crafting.RecipeSerializer<? extends net.minecraft.world.item.crafting.Recipe<T>> getSerializer();
    public abstract net.minecraft.world.item.crafting.RecipeType<? extends net.minecraft.world.item.crafting.Recipe<T>> getType();
    public abstract net.minecraft.world.item.crafting.PlacementInfo placementInfo();
    public default java.util.List<net.minecraft.world.item.crafting.display.RecipeDisplay> display();
    public abstract net.minecraft.world.item.crafting.RecipeBookCategory recipeBookCategory();
    static {};
}
```
