---
type: "interface"
fqcn: "net.minecraft.world.item.enchantment.ItemEnchantments"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.enchantment.ItemEnchantments

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/item/component/TooltipProvider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `keySet` | `()Ljava/util/Set;` | exact | invokevirtual@91 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (5 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/world/item/enchantment/ItemEnchantments;
private static final LEVEL_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private final enchantments : Lit/unimi/dsi/fastutil/objects/Object2IntOpenHashMap;
private <init>(Lit/unimi/dsi/fastutil/objects/Object2IntOpenHashMap;)V
public getLevel(Lnet/minecraft/core/Holder;)I
public addToTooltip(Lnet/minecraft/world/item/Item$TooltipContext;Ljava/util/function/Consumer;Lnet/minecraft/world/item/TooltipFlag;Lnet/minecraft/core/component/DataComponentGetter;)V
private static getTagOrEmpty(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/tags/TagKey;)Lnet/minecraft/core/HolderSet;
public keySet()Ljava/util/Set;
public entrySet()Ljava/util/Set;
public size()I
public isEmpty()Z
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
private static synthetic lambda$static$2(Lnet/minecraft/world/item/enchantment/ItemEnchantments;)Lit/unimi/dsi/fastutil/objects/Object2IntOpenHashMap;
private static synthetic lambda$static$1(Lnet/minecraft/world/item/enchantment/ItemEnchantments;)Ljava/util/Map;
private static synthetic lambda$static$0(Ljava/util/Map;)Lnet/minecraft/world/item/enchantment/ItemEnchantments;
static <clinit>()V
```
