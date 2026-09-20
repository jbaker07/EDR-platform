---
type: "interface"
fqcn: "net.minecraft.world.item.component.CustomData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.component.CustomData

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `matchedBy` | `(Lnet/minecraft/nbt/CompoundTag;)Z` | exact | invokevirtual@33 in `CustomDataIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/item/component/C` | exact | invokestatic@10 in `CustomDataIngredient.createEntryDisplay` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (5 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/world/item/component/CustomData;
public static final COMPOUND_TAG_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private final tag : Lnet/minecraft/nbt/CompoundTag;
private <init>(Lnet/minecraft/nbt/CompoundTag;)V
public static of(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/item/component/CustomData;
public matchedBy(Lnet/minecraft/nbt/CompoundTag;)Z
public static update(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/world/item/ItemStack;Ljava/util/function/Consumer;)V
public static set(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/nbt/CompoundTag;)V
public update(Ljava/util/function/Consumer;)Lnet/minecraft/world/item/component/CustomData;
public isEmpty()Z
public copyTag()Lnet/minecraft/nbt/CompoundTag;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
private static synthetic lambda$static$1(Lnet/minecraft/world/item/component/CustomData;)Lnet/minecraft/nbt/CompoundTag;
private static synthetic lambda$static$0(Lnet/minecraft/world/item/component/CustomData;)Lnet/minecraft/nbt/CompoundTag;
static <clinit>()V
```
