---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.LootTable$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.LootTable$Builder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/level/storage/loot/functions/FunctionUserBuilder`, `net/fabricmc/fabric/api/loot/v3/FabricLootTableBuilder`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `apply` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/L` | exact | invokevirtual@8 in `LootTableBuilderMixin.apply` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/level/storage/loot/LootTable;` | exact | invokevirtual@29 in `FabricLootTableProviderImpl.lambda$run$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/level/storage/loot/LootTable;` | exact | invokevirtual@59 in `LootUtil.modifyLootTable` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `pools` | `(Ljava/util/Collection;)Lnet/minecraft/world/level/storage/loot/LootTa` | inherited_exact | invokevirtual@25 in `FabricLootTableBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `setParamSet` | `(Lnet/minecraft/util/context/ContextKeySet;)Lnet/minecraft/world/level` | exact | invokevirtual@26 in `FabricLootTableProviderImpl.lambda$run$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `setParamSet` | `(Lnet/minecraft/util/context/ContextKeySet;)Lnet/minecraft/world/level` | exact | invokevirtual@14 in `FabricLootTableBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| reads | `pools` | `Lcom/google/common/collect/ImmutableList$Builder;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | declared |

## Declared members (4 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final pools : Lcom/google/common/collect/ImmutableList$Builder;
private final functions : Lcom/google/common/collect/ImmutableList$Builder;
private paramSet : Lnet/minecraft/util/context/ContextKeySet;
private randomSequence : Ljava/util/Optional;
public <init>()V
public withPool(Lnet/minecraft/world/level/storage/loot/LootPool$Builder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public setParamSet(Lnet/minecraft/util/context/ContextKeySet;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public setRandomSequence(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public apply(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public unwrap()Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
public build()Lnet/minecraft/world/level/storage/loot/LootTable;
public synthetic unwrap()Lnet/minecraft/world/level/storage/loot/functions/FunctionUserBuilder;
public synthetic apply(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/functions/FunctionUserBuilder;
```
