---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.Ingredient"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.Ingredient

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/core/HolderSet;)V` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `equals(Ljava/lang/Object;)Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `equals(Ljava/lang/Object;)Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `equals(Ljava/lang/Object;)Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getCustomIngredient()Lnet/fabricmc/fabric/api/recipe/v1/ingredient/CustomIngred` | `` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getCustomIngredient()Lnet/fabricmc/fabric/api/recipe/v1/ingredient/CustomIngred` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getCustomIngredient()Lnet/fabricmc/fabric/api/recipe/v1/ingredient/CustomIngred` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `items()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `items()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `items()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `items()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `requiresTesting()Z` | `` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `requiresTesting()Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `requiresTesting()Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test(Lnet/minecraft/world/item/ItemStack;)Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test(Lnet/minecraft/world/item/ItemStack;)Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test(Lnet/minecraft/world/item/ItemStack;)Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test(Lnet/minecraft/world/item/ItemStack;)Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test(Lnet/minecraft/world/item/ItemStack;)Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test(Lnet/minecraft/world/item/ItemStack;)Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `<clinit>` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `equals(Ljava/lang/Object;)Z` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `lambda$static$0` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `lambda$static$2` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `lambda$static$4` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (32, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.item.crafting.Ingredient implements java.util.function.Predicate<net.minecraft.world.item.ItemStack>, net.minecraft.world.entity.player.StackedContents$IngredientInfo<net.minecraft.core.Holder<net.minecraft.world.item.Item>> {
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.world.item.crafting.Ingredient> CONTENTS_STREAM_CODEC;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, java.util.Optional<net.minecraft.world.item.crafting.Ingredient>> OPTIONAL_CONTENTS_STREAM_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.core.HolderSet<net.minecraft.world.item.Item>> NON_AIR_HOLDER_SET_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.world.item.crafting.Ingredient> CODEC;
    private final net.minecraft.core.HolderSet<net.minecraft.world.item.Item> values;
    private net.minecraft.world.item.crafting.Ingredient(net.minecraft.core.HolderSet<net.minecraft.world.item.Item>);
    public static boolean testOptionalIngredient(java.util.Optional<net.minecraft.world.item.crafting.Ingredient>, net.minecraft.world.item.ItemStack);
    public java.util.stream.Stream<net.minecraft.core.Holder<net.minecraft.world.item.Item>> items();
    public boolean isEmpty();
    public boolean test(net.minecraft.world.item.ItemStack);
    public boolean acceptsItem(net.minecraft.core.Holder<net.minecraft.world.item.Item>);
    public boolean equals(java.lang.Object);
    public int hashCode();
    public static net.minecraft.world.item.crafting.Ingredient of(net.minecraft.world.level.ItemLike);
    public static net.minecraft.world.item.crafting.Ingredient of(net.minecraft.world.level.ItemLike...);
    public static net.minecraft.world.item.crafting.Ingredient of(java.util.stream.Stream<? extends net.minecraft.world.level.ItemLike>);
    public static net.minecraft.world.item.crafting.Ingredient of(net.minecraft.core.HolderSet<net.minecraft.world.item.Item>);
    public net.minecraft.world.item.crafting.display.SlotDisplay display();
    public static net.minecraft.world.item.crafting.display.SlotDisplay optionalIngredientToDisplay(java.util.Optional<net.minecraft.world.item.crafting.Ingredient>);
    private static net.minecraft.world.item.crafting.display.SlotDisplay displayForSingleItem(net.minecraft.core.Holder<net.minecraft.world.item.Item>);
    public java.util.Optional<net.minecraft.core.Holder<net.minecraft.world.item.Item>> getSingleItem();
    public boolean test(java.lang.Object);
    public boolean acceptsItem(java.lang.Object);
    private static net.minecraft.core.Holder$Reference lambda$of$0(net.minecraft.world.level.ItemLike);
    private static java.lang.Boolean lambda$testOptionalIngredient$0(net.minecraft.world.item.ItemStack, net.minecraft.world.item.crafting.Ingredient);
    private static void lambda$new$0(java.util.List);
    private static net.minecraft.core.HolderSet lambda$static$4(net.minecraft.world.item.crafting.Ingredient);
    private static net.minecraft.core.HolderSet lambda$static$2(java.util.Optional);
    private static net.minecraft.core.HolderSet lambda$static$3(net.minecraft.world.item.crafting.Ingredient);
    private static java.util.Optional lambda$static$1(net.minecraft.core.HolderSet);
    private static net.minecraft.core.HolderSet lambda$static$0(net.minecraft.world.item.crafting.Ingredient);
    static {};
}
```
