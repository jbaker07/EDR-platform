---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.LootTable"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.LootTable

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/level/storage/loot/Validatable`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getParamSet` | `()Lnet/minecraft/util/context/ContextKeySet;` | exact | invokevirtual@11 in `FabricLootTableBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `lootTable` | `()Lnet/minecraft/world/level/storage/loot/LootTable$Builder;` | exact | invokestatic@0 in `FabricLootTableBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| reads | `DIRECT_CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@68 in `FabricLootTableProviderImpl.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| wraps | `getRandomItemsRaw` | `(Lnet/minecraft/world/level/storage/loot/LootContext;Ljava/util/functi` | exact | @WrapMethod | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (12 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final KEY_CODEC : Lcom/mojang/serialization/Codec;
public static final DEFAULT_PARAM_SET : Lnet/minecraft/util/context/ContextKeySet;
public static final RANDOMIZE_SEED : J
public static final DIRECT_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final LIST_CODEC : Lcom/mojang/serialization/Codec;
public static final EMPTY : Lnet/minecraft/world/level/storage/loot/LootTable;
private final paramSet : Lnet/minecraft/util/context/ContextKeySet;
private final randomSequence : Ljava/util/Optional;
private final pools : Ljava/util/List;
private final modifier : Ljava/util/Optional;
private <init>(Lnet/minecraft/util/context/ContextKeySet;Ljava/util/Optional;Ljava/util/List;Ljava/util/Optional;)V
public static createStackSplitter(Lnet/minecraft/server/level/ServerLevel;Ljava/util/function/Consumer;)Ljava/util/function/Consumer;
public getRandomItemsRaw(Lnet/minecraft/world/level/storage/loot/LootParams;Ljava/util/function/Consumer;)V
public getRandomItemsRaw(Lnet/minecraft/world/level/storage/loot/LootContext;Ljava/util/function/Consumer;)V
public getRandomItems(Lnet/minecraft/world/level/storage/loot/LootParams;JLjava/util/function/Consumer;)V
public getRandomItems(Lnet/minecraft/world/level/storage/loot/LootParams;Ljava/util/function/Consumer;)V
public getRandomItems(Lnet/minecraft/world/level/storage/loot/LootContext;Ljava/util/function/Consumer;)V
public getRandomItems(Lnet/minecraft/world/level/storage/loot/LootParams;Lnet/minecraft/util/RandomSource;)Lit/unimi/dsi/fastutil/objects/ObjectArrayList;
public getRandomItems(Lnet/minecraft/world/level/storage/loot/LootParams;J)Lit/unimi/dsi/fastutil/objects/ObjectArrayList;
public getRandomItems(Lnet/minecraft/world/level/storage/loot/LootParams;)Lit/unimi/dsi/fastutil/objects/ObjectArrayList;
private getRandomItems(Lnet/minecraft/world/level/storage/loot/LootContext;)Lit/unimi/dsi/fastutil/objects/ObjectArrayList;
public getParamSet()Lnet/minecraft/util/context/ContextKeySet;
public validate(Lnet/minecraft/world/level/storage/loot/ValidationContext;)V
public fill(Lnet/minecraft/world/Container;Lnet/minecraft/world/level/storage/loot/LootParams;J)V
private shuffleAndSplitItems(Lit/unimi/dsi/fastutil/objects/ObjectArrayList;ILnet/minecraft/util/RandomSource;)V
private getAvailableSlots(Lnet/minecraft/world/Container;Lnet/minecraft/util/RandomSource;)Ljava/util/List;
public static lootTable()Lnet/minecraft/world/level/storage/loot/LootTable$Builder;
private static synthetic lambda$createStackSplitter$0(Lnet/minecraft/server/level/ServerLevel;Ljava/util/function/Consumer;Lnet/minecraft/world/item/ItemStack;)V
private static synthetic lambda$static$0()Lcom/mojang/serialization/Codec;
private static synthetic lambda$static$1(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$5(Lnet/minecraft/world/level/storage/loot/LootTable;)Ljava/util/Optional;
private static synthetic lambda$static$4(Lnet/minecraft/world/level/storage/loot/LootTable;)Ljava/util/List;
private static synthetic lambda$static$3(Lnet/minecraft/world/level/storage/loot/LootTable;)Ljava/util/Optional;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/storage/loot/LootTable;)Lnet/minecraft/util/context/ContextKeySet;
static <clinit>()V
```
