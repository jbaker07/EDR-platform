---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.ShapelessRecipe"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.ShapelessRecipe

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `net/minecraft/world/item/crafting/NormalCraftingRecipe`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<init>` | `(Lnet/minecraft/world/item/crafting/Recipe$CommonInfo;Lnet/minecraft/w` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `matches` | `(Lnet/minecraft/world/item/crafting/CraftingInput;Lnet/minecraft/world` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `ingredients` | `Ljava/util/List;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | declared |

## Declared members (5 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final SERIALIZER : Lnet/minecraft/world/item/crafting/RecipeSerializer;
private final result : Lnet/minecraft/world/item/ItemStackTemplate;
private final ingredients : Ljava/util/List;
public <init>(Lnet/minecraft/world/item/crafting/Recipe$CommonInfo;Lnet/minecraft/world/item/crafting/CraftingRecipe$CraftingBookInfo;Lnet/minecraft/world/item/ItemStackTemplate;Ljava/util/List;)V
public getSerializer()Lnet/minecraft/world/item/crafting/RecipeSerializer;
protected createPlacementInfo()Lnet/minecraft/world/item/crafting/PlacementInfo;
public matches(Lnet/minecraft/world/item/crafting/CraftingInput;Lnet/minecraft/world/level/Level;)Z
public assemble(Lnet/minecraft/world/item/crafting/CraftingInput;)Lnet/minecraft/world/item/ItemStack;
public display()Ljava/util/List;
public synthetic assemble(Lnet/minecraft/world/item/crafting/RecipeInput;)Lnet/minecraft/world/item/ItemStack;
public synthetic matches(Lnet/minecraft/world/item/crafting/RecipeInput;Lnet/minecraft/world/level/Level;)Z
private static synthetic lambda$static$8(Lnet/minecraft/world/item/crafting/ShapelessRecipe;)Ljava/util/List;
private static synthetic lambda$static$7(Lnet/minecraft/world/item/crafting/ShapelessRecipe;)Lnet/minecraft/world/item/ItemStackTemplate;
private static synthetic lambda$static$6(Lnet/minecraft/world/item/crafting/ShapelessRecipe;)Lnet/minecraft/world/item/crafting/CraftingRecipe$CraftingBookInfo;
private static synthetic lambda$static$5(Lnet/minecraft/world/item/crafting/ShapelessRecipe;)Lnet/minecraft/world/item/crafting/Recipe$CommonInfo;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$4(Lnet/minecraft/world/item/crafting/ShapelessRecipe;)Ljava/util/List;
private static synthetic lambda$static$3(Lnet/minecraft/world/item/crafting/ShapelessRecipe;)Lnet/minecraft/world/item/ItemStackTemplate;
private static synthetic lambda$static$2(Lnet/minecraft/world/item/crafting/ShapelessRecipe;)Lnet/minecraft/world/item/crafting/CraftingRecipe$CraftingBookInfo;
private static synthetic lambda$static$1(Lnet/minecraft/world/item/crafting/ShapelessRecipe;)Lnet/minecraft/world/item/crafting/Recipe$CommonInfo;
static <clinit>()V
```
