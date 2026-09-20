---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.LootPool$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.LootPool$Builder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/level/storage/loot/functions/FunctionUserBuilder`, `net/minecraft/world/level/storage/loot/predicates/ConditionUserBuilder`, `net/fabricmc/fabric/api/loot/v3/FabricLootPoolBuilder`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `add` | `(Ljava/util/Collection;)Lnet/minecraft/world/level/storage/loot/LootPo` | inherited_exact | invokevirtual@32 in `FabricLootPoolBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `apply` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/L` | exact | invokevirtual@8 in `LootPoolBuilderMixin.apply` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/level/storage/loot/LootPool;` | exact | invokevirtual@54 in `LootTableBuilderMixin.modifyPools` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `setBonusRolls` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/L` | exact | invokevirtual@23 in `FabricLootPoolBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `setRolls` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/L` | exact | invokevirtual@14 in `FabricLootPoolBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `when` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/L` | exact | invokevirtual@8 in `LootPoolBuilderMixin.when` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| reads | `entries` | `Lcom/google/common/collect/ImmutableList$Builder;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | declared |

## Declared members (5 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final entries : Lcom/google/common/collect/ImmutableList$Builder;
private final conditions : Lcom/google/common/collect/ImmutableList$Builder;
private final functions : Lcom/google/common/collect/ImmutableList$Builder;
private rolls : Lnet/minecraft/core/Holder;
private bonusRolls : Lnet/minecraft/core/Holder;
public <init>()V
public setRolls(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/LootPool$Builder;
public unwrap()Lnet/minecraft/world/level/storage/loot/LootPool$Builder;
public setBonusRolls(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/LootPool$Builder;
public add(Lnet/minecraft/world/level/storage/loot/entries/LootPoolEntryContainer$Builder;)Lnet/minecraft/world/level/storage/loot/LootPool$Builder;
public addAll(Ljava/util/List;)Lnet/minecraft/world/level/storage/loot/LootPool$Builder;
public when(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/LootPool$Builder;
public apply(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/LootPool$Builder;
public build()Lnet/minecraft/world/level/storage/loot/LootPool;
public synthetic unwrap()Lnet/minecraft/world/level/storage/loot/functions/FunctionUserBuilder;
public synthetic apply(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/functions/FunctionUserBuilder;
public synthetic unwrap()Lnet/minecraft/world/level/storage/loot/predicates/ConditionUserBuilder;
public synthetic when(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/storage/loot/predicates/ConditionUserBuilder;
```
