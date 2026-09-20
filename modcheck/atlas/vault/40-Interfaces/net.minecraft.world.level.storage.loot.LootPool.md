---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.LootPool"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.LootPool

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/level/storage/loot/Validatable`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `lootPool` | `()Lnet/minecraft/world/level/storage/loot/LootPool$Builder;` | exact | invokestatic@5 in `FabricLootPoolBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (6 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public final entries : Ljava/util/List;
public final condition : Ljava/util/Optional;
public final modifier : Ljava/util/Optional;
public final rolls : Lnet/minecraft/core/Holder;
public final bonusRolls : Lnet/minecraft/core/Holder;
private <init>(Ljava/util/List;Ljava/util/Optional;Ljava/util/Optional;Lnet/minecraft/core/Holder;Lnet/minecraft/core/Holder;)V
private addRandomItem(Ljava/util/function/Consumer;Lnet/minecraft/world/level/storage/loot/LootContext;)V
public addRandomItems(Ljava/util/function/Consumer;Lnet/minecraft/world/level/storage/loot/LootContext;)V
public validate(Lnet/minecraft/world/level/storage/loot/ValidationContext;)V
public static lootPool()Lnet/minecraft/world/level/storage/loot/LootPool$Builder;
private static synthetic lambda$addRandomItem$0(Lnet/minecraft/world/level/storage/loot/LootContext;Ljava/util/List;Lorg/apache/commons/lang3/mutable/MutableInt;Lnet/minecraft/world/level/storage/loot/entries/LootPoolEntry;)V
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$5(Lnet/minecraft/world/level/storage/loot/LootPool;)Lnet/minecraft/core/Holder;
private static synthetic lambda$static$4(Lnet/minecraft/world/level/storage/loot/LootPool;)Lnet/minecraft/core/Holder;
private static synthetic lambda$static$3(Lnet/minecraft/world/level/storage/loot/LootPool;)Ljava/util/Optional;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/storage/loot/LootPool;)Ljava/util/Optional;
private static synthetic lambda$static$1(Lnet/minecraft/world/level/storage/loot/LootPool;)Ljava/util/List;
static <clinit>()V
```
