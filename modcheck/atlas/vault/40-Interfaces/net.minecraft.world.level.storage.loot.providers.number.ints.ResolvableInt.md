---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `get` | `(Lnet/minecraft/world/level/storage/loot/LootContext;I)I` | exact | invokeinterface@50 in `ComposterWrapper.getLayersToAdd` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (2 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private static wrap(Lnet/minecraft/world/level/storage/loot/providers/number/ints/ResolvableInt;)Lcom/mojang/datafixers/util/Either;
public abstract get(Lnet/minecraft/world/level/storage/loot/LootContext;I)I
public static fromKey(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/level/storage/loot/providers/number/ints/ResolvableInt;
public static getFromItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/component/DataComponentType;Ljava/util/function/Function;Lnet/minecraft/world/level/storage/loot/LootContext;I)I
static <clinit>()V
```
