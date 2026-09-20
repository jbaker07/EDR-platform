---
type: "interface"
fqcn: "net.minecraft.world.item.ItemStackTemplate"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.ItemStackTemplate

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/world/item/ItemInstance`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/DataCompone` | exact | invokespecial@14 in `ComponentsIngredient.createEntryDisplay` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/DataCompone` | exact | invokespecial@31 in `CustomDataIngredient.createEntryDisplay` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/DataCompone` | exact | invokespecial@149 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `components` | `()Lnet/minecraft/core/component/DataComponentPatch;` | exact | invokevirtual@146 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `count` | `()I` | exact | invokevirtual@85 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `count` | `()I` | exact | invokevirtual@98 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `count` | `()I` | exact | invokevirtual@138 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `create` | `()Lnet/minecraft/world/item/ItemStack;` | exact | invokevirtual@43 in `BundleContentsStorage$BundleSlotWrapper.getStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `item` | `()Lnet/minecraft/core/Holder;` | exact | invokevirtual@133 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (7 fields, 23 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final item : Lnet/minecraft/core/Holder;
private final count : I
private final components : Lnet/minecraft/core/component/DataComponentPatch;
private static final LOGGER : Lorg/slf4j/Logger;
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/world/item/Item;)V
public <init>(Lnet/minecraft/world/item/Item;I)V
public <init>(Lnet/minecraft/world/item/Item;Lnet/minecraft/core/component/DataComponentPatch;)V
public <init>(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/DataComponentPatch;)V
public static fromNonEmptyStack(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemStackTemplate;
public static fromStack(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemStackTemplate;
public static fromNonEmptyStack(Lnet/minecraft/world/item/ItemStack;I)Lnet/minecraft/world/item/ItemStackTemplate;
public withCount(I)Lnet/minecraft/world/item/ItemStackTemplate;
public create()Lnet/minecraft/world/item/ItemStack;
private validate(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemStack;
public apply(Lnet/minecraft/core/component/DataComponentPatch;)Lnet/minecraft/world/item/ItemStack;
public apply(ILnet/minecraft/core/component/DataComponentPatch;)Lnet/minecraft/world/item/ItemStack;
public typeHolder()Lnet/minecraft/core/Holder;
public get(Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public item()Lnet/minecraft/core/Holder;
public count()I
public components()Lnet/minecraft/core/component/DataComponentPatch;
private static synthetic lambda$static$1(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/item/ItemStackTemplate;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
