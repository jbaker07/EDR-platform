---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.RecipeMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.RecipeMap

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `byType(Lnet/minecraft/world/item/crafting/RecipeType;)Ljava/util/C` | `` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getRecipesFor(Lnet/minecraft/world/item/crafting/RecipeType;Lnet/minecraf` | `` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `values()Ljava/util/Collection;` | `` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `values()Ljava/util/Collection;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.item.crafting.RecipeMap {
    public static final net.minecraft.world.item.crafting.RecipeMap EMPTY;
    private final com.google.common.collect.Multimap<net.minecraft.world.item.crafting.RecipeType<?>, net.minecraft.world.item.crafting.RecipeHolder<?>> byType;
    private final java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>, net.minecraft.world.item.crafting.RecipeHolder<?>> byKey;
    private net.minecraft.world.item.crafting.RecipeMap(com.google.common.collect.Multimap<net.minecraft.world.item.crafting.RecipeType<?>, net.minecraft.world.item.crafting.RecipeHolder<?>>, java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>, net.minecraft.world.item.crafting.RecipeHolder<?>>);
    public static net.minecraft.world.item.crafting.RecipeMap create(net.minecraft.core.HolderLookup<net.minecraft.world.item.crafting.Recipe<?>>);
    public <I extends net.minecraft.world.item.crafting.RecipeInput, T extends net.minecraft.world.item.crafting.Recipe<I>> java.util.Collection<net.minecraft.world.item.crafting.RecipeHolder<T>> byType(net.minecraft.world.item.crafting.RecipeType<T>);
    public java.util.Collection<net.minecraft.world.item.crafting.RecipeHolder<?>> values();
    public net.minecraft.world.item.crafting.RecipeHolder<?> byKey(net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>);
    public <I extends net.minecraft.world.item.crafting.RecipeInput, T extends net.minecraft.world.item.crafting.Recipe<I>> java.util.stream.Stream<net.minecraft.world.item.crafting.RecipeHolder<T>> getRecipesFor(net.minecraft.world.item.crafting.RecipeType<T>, I, net.minecraft.world.level.Level);
    private static boolean lambda$getRecipesFor$0(net.minecraft.world.item.crafting.RecipeInput, net.minecraft.world.level.Level, net.minecraft.world.item.crafting.RecipeHolder);
    private static void lambda$create$0(com.google.common.collect.ImmutableMultimap$Builder, com.google.common.collect.ImmutableMap$Builder, net.minecraft.core.Holder$Reference);
    static {};
}
```
