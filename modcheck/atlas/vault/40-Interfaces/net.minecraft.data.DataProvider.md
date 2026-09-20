---
type: "interface"
fqcn: "net.minecraft.data.DataProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.DataProvider

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `saveStable` | `(Lnet/minecraft/data/CachedOutput;Lcom/google/gson/JsonElement;Ljava/n` | exact | invokestatic@166 in `FabricAdvancementProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `saveStable` | `(Lnet/minecraft/data/CachedOutput;Lcom/google/gson/JsonElement;Ljava/n` | exact | invokestatic@32 in `FabricCodecDataProvider.lambda$write$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `saveStable` | `(Lnet/minecraft/data/CachedOutput;Lcom/google/gson/JsonElement;Ljava/n` | exact | invokestatic@90 in `FabricDynamicRegistryProvider.writeToPath` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `saveStable` | `(Lnet/minecraft/data/CachedOutput;Lcom/google/gson/JsonElement;Ljava/n` | exact | invokestatic@103 in `FabricLanguageProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `saveStable` | `(Lnet/minecraft/data/CachedOutput;Lcom/google/gson/JsonElement;Ljava/n` | exact | invokestatic@56 in `FabricRecipeProvider.lambda$run$2` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `saveStable` | `(Lnet/minecraft/data/CachedOutput;Lcom/google/gson/JsonElement;Ljava/n` | exact | invokestatic@56 in `FabricRecipeProvider.lambda$run$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `saveStable` | `(Lnet/minecraft/data/CachedOutput;Lcom/google/gson/JsonElement;Ljava/n` | exact | invokestatic@143 in `FabricLootTableProviderImpl.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `saveStable` | `(Lnet/minecraft/data/CachedOutput;Lcom/mojang/serialization/Codec;Ljav` | exact | invokestatic@23 in `TagAliasGenerator.writeTagAlias` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `saveStable` | `(Lnet/minecraft/data/CachedOutput;Lnet/minecraft/core/HolderLookup$Pro` | exact | invokestatic@47 in `FabricSoundsProvider.lambda$run$3` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `lambda$static$0` | `(Lit/unimi/dsi/fastutil/objects/Object2IntOpenHashMap;)V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `lambda$static$0` | `(Lit/unimi/dsi/fastutil/objects/Object2IntOpenHashMap;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| reads | `FIXED_ORDER_FIELDS` | `Ljava/util/function/ToIntFunction;` | exact | getstatic@108 in `FabricDataGenHelper.runInternal` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (3 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final FIXED_ORDER_FIELDS : Ljava/util/function/ToIntFunction;
public static final KEY_COMPARATOR : Ljava/util/Comparator;
public static final LOGGER : Lorg/slf4j/Logger;
public abstract run(Lnet/minecraft/data/CachedOutput;)Ljava/util/concurrent/CompletableFuture;
public abstract getName()Ljava/lang/String;
public static saveAll(Lnet/minecraft/data/CachedOutput;Lcom/mojang/serialization/Codec;Lnet/minecraft/data/PackOutput$PathProvider;Ljava/util/Map;)Ljava/util/concurrent/CompletableFuture;
public static saveAll(Lnet/minecraft/data/CachedOutput;Lcom/mojang/serialization/Codec;Ljava/util/function/Function;Ljava/util/Map;)Ljava/util/concurrent/CompletableFuture;
public static saveAll(Lnet/minecraft/data/CachedOutput;Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/Map;)Ljava/util/concurrent/CompletableFuture;
public static saveStable(Lnet/minecraft/data/CachedOutput;Lnet/minecraft/core/HolderLookup$Provider;Lcom/mojang/serialization/Codec;Ljava/lang/Object;Ljava/nio/file/Path;)Ljava/util/concurrent/CompletableFuture;
public static saveStable(Lnet/minecraft/data/CachedOutput;Lcom/mojang/serialization/Codec;Ljava/lang/Object;Ljava/nio/file/Path;)Ljava/util/concurrent/CompletableFuture;
private static saveStable(Lnet/minecraft/data/CachedOutput;Lcom/mojang/serialization/DynamicOps;Lcom/mojang/serialization/Codec;Ljava/lang/Object;Ljava/nio/file/Path;)Ljava/util/concurrent/CompletableFuture;
public static saveStable(Lnet/minecraft/data/CachedOutput;Lcom/google/gson/JsonElement;Ljava/nio/file/Path;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$saveStable$0(Lcom/google/gson/JsonElement;Lnet/minecraft/data/CachedOutput;Ljava/nio/file/Path;)V
private static synthetic lambda$saveAll$2(I)[Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$saveAll$1(Ljava/util/function/Function;Ljava/util/function/Function;Lnet/minecraft/data/CachedOutput;Ljava/util/Map$Entry;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$saveAll$0(Lcom/mojang/serialization/Codec;Ljava/lang/Object;)Lcom/google/gson/JsonElement;
private static synthetic lambda$static$1(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$static$0(Lit/unimi/dsi/fastutil/objects/Object2IntOpenHashMap;)V
static <clinit>()V
```
