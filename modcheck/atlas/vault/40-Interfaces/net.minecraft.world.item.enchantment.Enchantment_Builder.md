---
type: "interface"
fqcn: "net.minecraft.world.item.enchantment.Enchantment$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.enchantment.Enchantment$Builder

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `exclusiveWith(Lnet/minecraft/core/HolderSet;)Lnet/minecraft/world/item/en` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `*` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.item.enchantment.Enchantment$Builder {
    private final net.minecraft.world.item.enchantment.Enchantment$EnchantmentDefinition definition;
    private net.minecraft.core.HolderSet<net.minecraft.world.item.enchantment.Enchantment> exclusiveSet;
    private final java.util.Map<net.minecraft.core.component.DataComponentType<?>, java.util.List<?>> effectLists;
    private final net.minecraft.core.component.DataComponentMap$Builder effectMapBuilder;
    public net.minecraft.world.item.enchantment.Enchantment$Builder(net.minecraft.world.item.enchantment.Enchantment$EnchantmentDefinition);
    public net.minecraft.world.item.enchantment.Enchantment$Builder exclusiveWith(net.minecraft.core.HolderSet<net.minecraft.world.item.enchantment.Enchantment>);
    public <E> net.minecraft.world.item.enchantment.Enchantment$Builder withEffect(net.minecraft.core.component.DataComponentType<java.util.List<net.minecraft.world.item.enchantment.ConditionalEffect<E>>>, E, net.minecraft.world.level.storage.loot.predicates.LootItemCondition$Builder);
    public <E> net.minecraft.world.item.enchantment.Enchantment$Builder withEffect(net.minecraft.core.component.DataComponentType<java.util.List<net.minecraft.world.item.enchantment.ConditionalEffect<E>>>, E);
    public <E> net.minecraft.world.item.enchantment.Enchantment$Builder withEffect(net.minecraft.core.component.DataComponentType<java.util.List<net.minecraft.world.item.enchantment.TargetedConditionalEffect<E>>>, net.minecraft.world.item.enchantment.EnchantmentTarget, net.minecraft.world.item.enchantment.EnchantmentTarget, E, net.minecraft.world.level.storage.loot.predicates.LootItemCondition$Builder);
    public <E> net.minecraft.world.item.enchantment.Enchantment$Builder withEffect(net.minecraft.core.component.DataComponentType<java.util.List<net.minecraft.world.item.enchantment.TargetedConditionalEffect<E>>>, net.minecraft.world.item.enchantment.EnchantmentTarget, net.minecraft.world.item.enchantment.EnchantmentTarget, E);
    public net.minecraft.world.item.enchantment.Enchantment$Builder withEffect(net.minecraft.core.component.DataComponentType<java.util.List<net.minecraft.world.item.enchantment.effects.EnchantmentAttributeEffect>>, net.minecraft.world.item.enchantment.effects.EnchantmentAttributeEffect);
    public <E> net.minecraft.world.item.enchantment.Enchantment$Builder withSpecialEffect(net.minecraft.core.component.DataComponentType<E>, E);
    public net.minecraft.world.item.enchantment.Enchantment$Builder withEffect(net.minecraft.core.component.DataComponentType<net.minecraft.util.Unit>);
    private <E> java.util.List<E> getEffectsList(net.minecraft.core.component.DataComponentType<java.util.List<E>>);
    public net.minecraft.world.item.enchantment.Enchantment build(net.minecraft.resources.Identifier);
    private java.util.List lambda$getEffectsList$0(net.minecraft.core.component.DataComponentType, net.minecraft.core.component.DataComponentType);
}
```
