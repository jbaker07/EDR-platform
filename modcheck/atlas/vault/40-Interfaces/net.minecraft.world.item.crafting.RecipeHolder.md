---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.RecipeHolder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.RecipeHolder

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `id()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `value()Lnet/minecraft/world/item/crafting/Recipe;` | `` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `value()Lnet/minecraft/world/item/crafting/Recipe;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.item.crafting.RecipeHolder<T extends net.minecraft.world.item.crafting.Recipe<?>> extends java.lang.Record {
    private final net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>> id;
    private final T value;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.world.item.crafting.RecipeHolder<?>> STREAM_CODEC;
    public net.minecraft.world.item.crafting.RecipeHolder(net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>, T);
    public boolean equals(java.lang.Object);
    public int hashCode();
    public java.lang.String toString();
    public net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>> id();
    public T value();
    static {};
}
```
