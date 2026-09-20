---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.LootTable$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.LootTable$Builder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `apply(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/stor` | `` | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `build()Lnet/minecraft/world/level/storage/loot/LootTable;` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `build()Lnet/minecraft/world/level/storage/loot/LootTable;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `setParamSet(Lnet/minecraft/util/context/ContextKeySet;)Lnet/minecraft/w` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.storage.loot.LootTable$Builder implements net.minecraft.world.level.storage.loot.functions.FunctionUserBuilder<net.minecraft.world.level.storage.loot.LootTable$Builder> {
    private final com.google.common.collect.ImmutableList$Builder<net.minecraft.world.level.storage.loot.LootPool> pools;
    private final com.google.common.collect.ImmutableList$Builder<net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.functions.LootItemFunction>> functions;
    private net.minecraft.util.context.ContextKeySet paramSet;
    private java.util.Optional<net.minecraft.resources.Identifier> randomSequence;
    public net.minecraft.world.level.storage.loot.LootTable$Builder();
    public net.minecraft.world.level.storage.loot.LootTable$Builder withPool(net.minecraft.world.level.storage.loot.LootPool$Builder);
    public net.minecraft.world.level.storage.loot.LootTable$Builder setParamSet(net.minecraft.util.context.ContextKeySet);
    public net.minecraft.world.level.storage.loot.LootTable$Builder setRandomSequence(net.minecraft.resources.Identifier);
    public net.minecraft.world.level.storage.loot.LootTable$Builder apply(net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.functions.LootItemFunction>);
    public net.minecraft.world.level.storage.loot.LootTable$Builder unwrap();
    public net.minecraft.world.level.storage.loot.LootTable build();
    public net.minecraft.world.level.storage.loot.functions.FunctionUserBuilder unwrap();
    public net.minecraft.world.level.storage.loot.functions.FunctionUserBuilder apply(net.minecraft.core.Holder);
}
```
