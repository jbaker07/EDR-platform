---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.LootPool$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.LootPool$Builder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `apply(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/stor` | `` | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `build()Lnet/minecraft/world/level/storage/loot/LootPool;` | `` | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `when(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/stor` | `` | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (18, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.storage.loot.LootPool$Builder implements net.minecraft.world.level.storage.loot.functions.FunctionUserBuilder<net.minecraft.world.level.storage.loot.LootPool$Builder>, net.minecraft.world.level.storage.loot.predicates.ConditionUserBuilder<net.minecraft.world.level.storage.loot.LootPool$Builder> {
    private final com.google.common.collect.ImmutableList$Builder<net.minecraft.world.level.storage.loot.entries.LootPoolEntryContainer> entries;
    private final com.google.common.collect.ImmutableList$Builder<net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.predicates.LootItemCondition>> conditions;
    private final com.google.common.collect.ImmutableList$Builder<net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.functions.LootItemFunction>> functions;
    private net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.providers.number.ints.ContextIntProvider> rolls;
    private net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.providers.number.floats.ContextFloatProvider> bonusRolls;
    public net.minecraft.world.level.storage.loot.LootPool$Builder();
    public net.minecraft.world.level.storage.loot.LootPool$Builder setRolls(net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.providers.number.ints.ContextIntProvider>);
    public net.minecraft.world.level.storage.loot.LootPool$Builder unwrap();
    public net.minecraft.world.level.storage.loot.LootPool$Builder setBonusRolls(net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.providers.number.floats.ContextFloatProvider>);
    public net.minecraft.world.level.storage.loot.LootPool$Builder add(net.minecraft.world.level.storage.loot.entries.LootPoolEntryContainer$Builder<?>);
    public net.minecraft.world.level.storage.loot.LootPool$Builder addAll(java.util.List<? extends net.minecraft.world.level.storage.loot.entries.UniformContainerBase$Builder<?>>);
    public net.minecraft.world.level.storage.loot.LootPool$Builder when(net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.predicates.LootItemCondition>);
    public net.minecraft.world.level.storage.loot.LootPool$Builder apply(net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.functions.LootItemFunction>);
    public net.minecraft.world.level.storage.loot.LootPool build();
    public net.minecraft.world.level.storage.loot.functions.FunctionUserBuilder unwrap();
    public net.minecraft.world.level.storage.loot.functions.FunctionUserBuilder apply(net.minecraft.core.Holder);
    public net.minecraft.world.level.storage.loot.predicates.ConditionUserBuilder unwrap();
    public net.minecraft.world.level.storage.loot.predicates.ConditionUserBuilder when(net.minecraft.core.Holder);
}
```
