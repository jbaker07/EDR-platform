---
type: "interface"
fqcn: "net.minecraft.world.item.enchantment.Enchantment$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.enchantment.Enchantment$Builder

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `exclusiveWith` | `(Lnet/minecraft/core/HolderSet;)Lnet/minecraft/world/item/enchantment/` | exact | invokevirtual@29 in `EnchantmentUtil.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `*` | `?` | selector_unsupported | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (4 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final definition : Lnet/minecraft/world/item/enchantment/Enchantment$EnchantmentDefinition;
private exclusiveSet : Lnet/minecraft/core/HolderSet;
private final effectLists : Ljava/util/Map;
private final effectMapBuilder : Lnet/minecraft/core/component/DataComponentMap$Builder;
public <init>(Lnet/minecraft/world/item/enchantment/Enchantment$EnchantmentDefinition;)V
public exclusiveWith(Lnet/minecraft/core/HolderSet;)Lnet/minecraft/world/item/enchantment/Enchantment$Builder;
public withEffect(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;)Lnet/minecraft/world/item/enchantment/Enchantment$Builder;
public withEffect(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lnet/minecraft/world/item/enchantment/Enchantment$Builder;
public withEffect(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/world/item/enchantment/EnchantmentTarget;Lnet/minecraft/world/item/enchantment/EnchantmentTarget;Ljava/lang/Object;Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder;)Lnet/minecraft/world/item/enchantment/Enchantment$Builder;
public withEffect(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/world/item/enchantment/EnchantmentTarget;Lnet/minecraft/world/item/enchantment/EnchantmentTarget;Ljava/lang/Object;)Lnet/minecraft/world/item/enchantment/Enchantment$Builder;
public withEffect(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/world/item/enchantment/effects/EnchantmentAttributeEffect;)Lnet/minecraft/world/item/enchantment/Enchantment$Builder;
public withSpecialEffect(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lnet/minecraft/world/item/enchantment/Enchantment$Builder;
public withEffect(Lnet/minecraft/core/component/DataComponentType;)Lnet/minecraft/world/item/enchantment/Enchantment$Builder;
private getEffectsList(Lnet/minecraft/core/component/DataComponentType;)Ljava/util/List;
public build(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/world/item/enchantment/Enchantment;
private synthetic lambda$getEffectsList$0(Lnet/minecraft/core/component/DataComponentType;Lnet/minecraft/core/component/DataComponentType;)Ljava/util/List;
```
