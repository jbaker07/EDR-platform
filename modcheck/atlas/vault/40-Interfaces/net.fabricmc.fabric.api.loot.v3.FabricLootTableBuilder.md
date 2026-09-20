---
type: "interface"
fqcn: "net.fabricmc.fabric.api.loot.v3.FabricLootTableBuilder"
module: "fabric-loot-api-v3"
sha256: "569540023c6d19e4b4854e14ea4bb99aed401946d5470f761ac1f1388cc7de2c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.loot.v3.FabricLootTableBuilder

Module: [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] -- kind: interface

```java
public net.minecraft.world.level.storage.loot.LootTable$Builder pool(net.minecraft.world.level.storage.loot.LootPool)
public net.minecraft.world.level.storage.loot.LootTable$Builder apply(net.minecraft.world.level.storage.loot.functions.LootItemFunction)
public net.minecraft.world.level.storage.loot.LootTable$Builder pools(java.util.Collection)
public net.minecraft.world.level.storage.loot.LootTable$Builder apply(java.util.Collection)
public net.minecraft.world.level.storage.loot.LootTable$Builder modifyPools(java.util.function.Consumer)
public static net.minecraft.world.level.storage.loot.LootTable$Builder copyOf(net.minecraft.world.level.storage.loot.LootTable)
```
