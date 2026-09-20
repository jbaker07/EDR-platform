---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.Ingredient"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.Ingredient

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public final; extends `java/lang/Object`; implements `java/util/function/Predicate`, `net/minecraft/world/entity/player/StackedContents$IngredientInfo`, `net/fabricmc/fabric/api/recipe/v1/ingredient/FabricIngredient`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/core/HolderSet;)V` | exact | invokespecial@17 in `CustomIngredientImpl.<init>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `equals` | `(Ljava/lang/Object;)Z` | exact | invokevirtual@37 in `ComponentsIngredient.equals` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `equals` | `(Ljava/lang/Object;)Z` | exact | invokevirtual@37 in `CustomDataIngredient.equals` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `equals` | `(Ljava/lang/Object;)Z` | exact | invokevirtual@37 in `DifferenceIngredient.equals` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `equals` | `(Ljava/lang/Object;)Z` | exact | invokevirtual@51 in `DifferenceIngredient.equals` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getCustomIngredient` | `()Lnet/fabricmc/fabric/api/recipe/v1/ingredient/CustomIngredient;` | inherited_exact | invokevirtual@1 in `CustomIngredientStreamCodec.encode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getCustomIngredient` | `()Lnet/fabricmc/fabric/api/recipe/v1/ingredient/CustomIngredient;` | inherited_exact | invokevirtual@26 in `OptionalCustomIngredientStreamCodec.encode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getCustomIngredient` | `()Lnet/fabricmc/fabric/api/recipe/v1/ingredient/CustomIngredient;` | inherited_exact | invokevirtual@1 in `IngredientMixin.lambda$injectCodec$2` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `items` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@16 in `AllIngredient.items` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `items` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@4 in `ComponentsIngredient.items` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `items` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@8 in `ComponentsIngredient.display` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `items` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@4 in `CustomDataIngredient.items` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `items` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@8 in `CustomDataIngredient.display` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `items` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@4 in `DifferenceIngredient.items` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `items` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@17 in `DifferenceIngredient.items` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `of` | `([Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/item/craft` | exact | invokestatic@18 in `DefaultCustomIngredients.components` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `requiresTesting` | `()Z` | inherited_exact | invokevirtual@30 in `CombinedIngredient.requiresTesting` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `requiresTesting` | `()Z` | inherited_exact | invokevirtual@4 in `DifferenceIngredient.requiresTesting` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `requiresTesting` | `()Z` | inherited_exact | invokevirtual@14 in `DifferenceIngredient.requiresTesting` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `requiresTesting` | `()Z` | inherited_exact | invokevirtual@33 in `ShapelessRecipeMixin.cacheRequiresTesting` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test` | `(Lnet/minecraft/world/item/ItemStack;)Z` | exact | invokevirtual@82 in `ShapelessMatch.isMatch` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test` | `(Lnet/minecraft/world/item/ItemStack;)Z` | exact | invokevirtual@31 in `AllIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test` | `(Lnet/minecraft/world/item/ItemStack;)Z` | exact | invokevirtual@13 in `AllIngredient.lambda$items$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test` | `(Lnet/minecraft/world/item/ItemStack;)Z` | exact | invokevirtual@31 in `AnyIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test` | `(Lnet/minecraft/world/item/ItemStack;)Z` | exact | invokevirtual@5 in `ComponentsIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test` | `(Lnet/minecraft/world/item/ItemStack;)Z` | exact | invokevirtual@5 in `CustomDataIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test` | `(Lnet/minecraft/world/item/ItemStack;)Z` | exact | invokevirtual@5 in `DifferenceIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `test` | `(Lnet/minecraft/world/item/ItemStack;)Z` | exact | invokevirtual@16 in `DifferenceIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `<clinit>` | `()V` | exact | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `<clinit>` | `()V` | exact | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `<clinit>` | `()V` | exact | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `equals` | `(Ljava/lang/Object;)Z` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `lambda$static$0` | `(Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/core/Ho` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `lambda$static$2` | `(Ljava/util/Optional;)Lnet/minecraft/core/HolderSet;` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `lambda$static$4` | `(Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/core/Ho` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | declared |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@0 in `AllIngredient.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@0 in `AnyIngredient.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@1 in `ComponentsIngredient$Serializer.lambda$static$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@1 in `CustomDataIngredient$Serializer.lambda$static$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@1 in `DifferenceIngredient$Serializer.lambda$static$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@19 in `DifferenceIngredient$Serializer.lambda$static$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CONTENTS_STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@15 in `CombinedIngredient$Serializer.<init>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CONTENTS_STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@21 in `ComponentsIngredient$Serializer.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CONTENTS_STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@21 in `CustomDataIngredient$Serializer.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CONTENTS_STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@21 in `DifferenceIngredient$Serializer.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CONTENTS_STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@29 in `DifferenceIngredient$Serializer.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (5 fields, 27 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CONTENTS_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final OPTIONAL_CONTENTS_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final NON_AIR_HOLDER_SET_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
private final values : Lnet/minecraft/core/HolderSet;
private <init>(Lnet/minecraft/core/HolderSet;)V
public static testOptionalIngredient(Ljava/util/Optional;Lnet/minecraft/world/item/ItemStack;)Z
public items()Ljava/util/stream/Stream;
public isEmpty()Z
public test(Lnet/minecraft/world/item/ItemStack;)Z
public acceptsItem(Lnet/minecraft/core/Holder;)Z
public equals(Ljava/lang/Object;)Z
public hashCode()I
public static of(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/item/crafting/Ingredient;
public static of([Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/item/crafting/Ingredient;
public static of(Ljava/util/stream/Stream;)Lnet/minecraft/world/item/crafting/Ingredient;
public static of(Lnet/minecraft/core/HolderSet;)Lnet/minecraft/world/item/crafting/Ingredient;
public display()Lnet/minecraft/world/item/crafting/display/SlotDisplay;
public static optionalIngredientToDisplay(Ljava/util/Optional;)Lnet/minecraft/world/item/crafting/display/SlotDisplay;
private static displayForSingleItem(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/item/crafting/display/SlotDisplay;
public getSingleItem()Ljava/util/Optional;
public synthetic test(Ljava/lang/Object;)Z
public synthetic acceptsItem(Ljava/lang/Object;)Z
private static synthetic lambda$of$0(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/core/Holder$Reference;
private static synthetic lambda$testOptionalIngredient$0(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/crafting/Ingredient;)Ljava/lang/Boolean;
private static synthetic lambda$new$0(Ljava/util/List;)V
private static synthetic lambda$static$4(Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/core/HolderSet;
private static synthetic lambda$static$2(Ljava/util/Optional;)Lnet/minecraft/core/HolderSet;
private static synthetic lambda$static$3(Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/core/HolderSet;
private static synthetic lambda$static$1(Lnet/minecraft/core/HolderSet;)Ljava/util/Optional;
private static synthetic lambda$static$0(Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/core/HolderSet;
static <clinit>()V
```
