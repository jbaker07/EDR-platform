---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.LootContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.LootContext

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getLevel` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@9 in `LootTableMixin.fabric$modifyDrops` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (4 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final params : Lnet/minecraft/world/level/storage/loot/LootParams;
private final random : Lnet/minecraft/util/RandomSource;
private final lootDataResolver : Lnet/minecraft/core/HolderGetter$Provider;
private final visitedElements : Ljava/util/Set;
private <init>(Lnet/minecraft/world/level/storage/loot/LootParams;Lnet/minecraft/util/RandomSource;Lnet/minecraft/core/HolderGetter$Provider;)V
public hasParameter(Lnet/minecraft/util/context/ContextKey;)Z
public getOptional(Lnet/minecraft/util/context/ContextKey;)Ljava/lang/Object;
public addDynamicDrops(Lnet/minecraft/resources/Identifier;Ljava/util/function/Consumer;)V
public hasVisitedElement(Lnet/minecraft/world/level/storage/loot/LootContext$VisitedEntry;)Z
public pushVisitedElement(Lnet/minecraft/world/level/storage/loot/LootContext$VisitedEntry;)Z
public popVisitedElement(Lnet/minecraft/world/level/storage/loot/LootContext$VisitedEntry;)V
public getResolver()Lnet/minecraft/core/HolderGetter$Provider;
public getRandom()Lnet/minecraft/util/RandomSource;
public getLuck()F
public getLevel()Lnet/minecraft/server/level/ServerLevel;
public static createVisitedEntry(Lnet/minecraft/world/level/storage/loot/LootTable;)Lnet/minecraft/world/level/storage/loot/LootContext$VisitedEntry;
public static createVisitedEntry(Lnet/minecraft/world/level/storage/loot/predicates/LootItemCondition;)Lnet/minecraft/world/level/storage/loot/LootContext$VisitedEntry;
public static createVisitedEntry(Lnet/minecraft/world/level/storage/loot/functions/LootItemFunction;)Lnet/minecraft/world/level/storage/loot/LootContext$VisitedEntry;
public static createVisitedEntry(Lnet/minecraft/world/item/slot/SlotSource;)Lnet/minecraft/world/level/storage/loot/LootContext$VisitedEntry;
```
