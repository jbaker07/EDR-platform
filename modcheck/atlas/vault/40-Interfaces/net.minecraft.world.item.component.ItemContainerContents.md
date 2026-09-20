---
type: "interface"
fqcn: "net.minecraft.world.item.component.ItemContainerContents"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.component.ItemContainerContents

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public final; extends `java/lang/Object`; implements `net/minecraft/world/item/component/ContainerComponent`, `net/minecraft/world/item/component/TooltipProvider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `fromItems` | `(Ljava/util/List;)Lnet/minecraft/world/item/component/ItemContainerCon` | exact | invokestatic@84 in `ItemContainerContentsStorage$ContainerSlotWrapper.setStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `itemCopies` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@11 in `ItemContainerContentsStorage$ContainerSlotWrapper.setStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/item/component/ItemContainerContents;` | exact | getstatic@17 in `ItemContainerContentsStorage.container` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (7 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final NO_SLOT : I
public static final MAX_SIZE : I
public static final EMPTY : Lnet/minecraft/world/item/component/ItemContainerContents;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private final items : Ljava/util/List;
private final hashCode : I
private <init>(Ljava/util/List;)V
private static emptyContents(I)Ljava/util/List;
private static fromSlots(Ljava/util/List;)Lnet/minecraft/world/item/component/ItemContainerContents;
public static fromItems(Ljava/util/List;)Lnet/minecraft/world/item/component/ItemContainerContents;
private static findLastNonEmptySlot(Ljava/util/List;)I
private asSlots()Ljava/util/List;
private createStackFromSlot(I)Lnet/minecraft/world/item/ItemStack;
public copyInto(Lnet/minecraft/core/NonNullList;)V
public copyOne()Lnet/minecraft/world/item/ItemStack;
public itemCopies()Ljava/util/stream/Stream;
private nonEmptyItemsStream()Ljava/util/stream/Stream;
public nonEmptyItemCopyStream()Ljava/util/stream/Stream;
public nonEmptyItems()Ljava/lang/Iterable;
public size()I
public copyWithContents(Ljava/util/stream/Stream;)Lnet/minecraft/world/item/component/ItemContainerContents;
public asMutable()Lnet/minecraft/world/item/component/ItemContainerContents$Mutable;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public addToTooltip(Lnet/minecraft/world/item/Item$TooltipContext;Ljava/util/function/Consumer;Lnet/minecraft/world/item/TooltipFlag;Lnet/minecraft/core/component/DataComponentGetter;)V
public synthetic asMutable()Lnet/minecraft/world/item/component/ContainerComponent$Mutable;
public synthetic copyWithContents(Ljava/util/stream/Stream;)Lnet/minecraft/world/item/component/ContainerComponent;
private synthetic lambda$nonEmptyItems$0()Ljava/util/Iterator;
private static synthetic lambda$itemCopies$0(Ljava/util/Optional;)Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$static$0(Lnet/minecraft/world/item/component/ItemContainerContents;)Ljava/util/List;
static <clinit>()V
```
