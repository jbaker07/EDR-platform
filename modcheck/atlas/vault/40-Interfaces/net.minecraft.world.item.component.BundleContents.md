---
type: "interface"
fqcn: "net.minecraft.world.item.component.BundleContents"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.component.BundleContents

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public final; extends `java/lang/Object`; implements `net/minecraft/world/item/component/ContainerComponent`, `net/minecraft/world/inventory/tooltip/TooltipComponent`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/List;)V` | exact | invokespecial@168 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `asMutable` | `()Lnet/minecraft/world/item/component/BundleContents$Mutable;` | exact | invokevirtual@50 in `BundleContentsStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `canItemBeInBundle` | `(Lnet/minecraft/world/item/ItemStack;)Z` | exact | invokestatic@38 in `BundleContentsStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `items` | `()Ljava/util/List;` | exact | invokevirtual@28 in `BundleContentsStorage$BundleSlotWrapper.getStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `items` | `()Ljava/util/List;` | exact | invokevirtual@61 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `size` | `()I` | exact | invokevirtual@7 in `BundleContentsStorage$BundleSlotWrapper.getStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `size` | `()I` | exact | invokevirtual@23 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `size` | `()I` | exact | invokevirtual@4 in `BundleContentsStorage.updateSlotsIfNeeded` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `weight` | `()Lcom/mojang/serialization/DataResult;` | exact | invokevirtual@10 in `BundleContentsStorage$BundleSlotWrapper.getCapacity` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/item/component/BundleContents;` | exact | getstatic@17 in `BundleContentsStorage.bundleContents` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (10 fields, 26 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/world/item/component/BundleContents;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private static final BUNDLE_IN_BUNDLE_WEIGHT : Lorg/apache/commons/lang3/math/Fraction;
private static final NO_STACK_INDEX : I
public static final NO_SELECTED_ITEM_INDEX : I
public static final BEEHIVE_WEIGHT : Lcom/mojang/serialization/DataResult;
private final items : Ljava/util/List;
private final selectedItem : I
private final weight : Ljava/util/function/Supplier;
private <init>(Ljava/util/List;I)V
public <init>(Ljava/util/List;)V
private static computeContentWeight(Ljava/util/List;)Lcom/mojang/serialization/DataResult;
private static getWeight(Lnet/minecraft/world/item/ItemInstance;)Lcom/mojang/serialization/DataResult;
public static canItemBeInBundle(Lnet/minecraft/world/item/ItemStack;)Z
public getNumberOfItemsToShow()I
public itemCopies()Ljava/util/stream/Stream;
public items()Ljava/util/List;
public size()I
public weight()Lcom/mojang/serialization/DataResult;
public isEmpty()Z
public getSelectedItemIndex()I
public getSelectedItem()Lnet/minecraft/world/item/ItemStackTemplate;
public copyWithContents(Ljava/util/stream/Stream;)Lnet/minecraft/world/item/component/BundleContents;
public asMutable()Lnet/minecraft/world/item/component/BundleContents$Mutable;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
public synthetic asMutable()Lnet/minecraft/world/item/component/ContainerComponent$Mutable;
public synthetic copyWithContents(Ljava/util/stream/Stream;)Lnet/minecraft/world/item/component/ContainerComponent;
private static synthetic lambda$getWeight$0(Lorg/apache/commons/lang3/math/Fraction;)Lorg/apache/commons/lang3/math/Fraction;
private static synthetic lambda$computeContentWeight$0()Ljava/lang/String;
private synthetic lambda$new$0()Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$1(Lnet/minecraft/world/item/component/BundleContents;)Ljava/util/List;
private static synthetic lambda$static$0(Lnet/minecraft/world/item/component/BundleContents;)Ljava/util/List;
static <clinit>()V
```
