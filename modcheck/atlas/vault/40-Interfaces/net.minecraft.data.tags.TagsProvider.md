---
type: "interface"
fqcn: "net.minecraft.data.tags.TagsProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.tags.TagsProvider

System: [[20-Systems/net.minecraft.data.tags|net.minecraft.data.tags]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/data/DataProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/data/PackOutput;Lnet/minecraft/resources/ResourceKey;L` | exact | invokespecial@4 in `FabricTagsProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/data/PackOutput;Lnet/minecraft/resources/ResourceKey;L` | exact | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `lambda$run$5` | `(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Lnet/mine` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `lambda$run$5` | `(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Lnet/mine` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `registryKey` | `Lnet/minecraft/resources/ResourceKey;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | declared |
| wraps | `lambda$run$2` | `(Lnet/minecraft/data/CachedOutput;Lnet/minecraft/data/tags/TagsProvide` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (6 fields, 22 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
protected final pathProvider : Lnet/minecraft/data/PackOutput$PathProvider;
private final lookupProvider : Ljava/util/concurrent/CompletableFuture;
private final contentsDone : Ljava/util/concurrent/CompletableFuture;
private final parentProvider : Ljava/util/concurrent/CompletableFuture;
protected final registryKey : Lnet/minecraft/resources/ResourceKey;
private final builders : Ljava/util/Map;
protected <init>(Lnet/minecraft/data/PackOutput;Lnet/minecraft/resources/ResourceKey;Ljava/util/concurrent/CompletableFuture;)V
protected <init>(Lnet/minecraft/data/PackOutput;Lnet/minecraft/resources/ResourceKey;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;)V
public getName()Ljava/lang/String;
protected abstract addTags(Lnet/minecraft/core/HolderLookup$Provider;)V
public run(Lnet/minecraft/data/CachedOutput;)Ljava/util/concurrent/CompletableFuture;
protected getOrCreateRawBuilder(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/tags/TagBuilder;
public contentsGetter()Ljava/util/concurrent/CompletableFuture;
protected createContentsProvider()Ljava/util/concurrent/CompletableFuture;
protected tag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/data/tags/TagAppender;
protected tag(Lnet/minecraft/tags/TagKey;Z)Lnet/minecraft/data/tags/TagAppender;
private synthetic lambda$createContentsProvider$0(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/HolderLookup$Provider;
private synthetic lambda$contentsGetter$0(Ljava/lang/Void;)Lnet/minecraft/data/tags/TagsProvider$TagLookup;
private synthetic lambda$contentsGetter$1(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;
private static synthetic lambda$getOrCreateRawBuilder$0(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagBuilder;
private synthetic lambda$run$2(Lnet/minecraft/data/CachedOutput;Lnet/minecraft/data/tags/TagsProvider$1CombinedData;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$run$7(I)[Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$run$5(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Lnet/minecraft/data/CachedOutput;Lnet/minecraft/data/tags/TagsProvider$1CombinedData;Ljava/util/Map$Entry;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$run$6(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Lnet/minecraft/tags/TagEntry;)Z
private synthetic lambda$run$4(Lnet/minecraft/data/tags/TagsProvider$1CombinedData;Lnet/minecraft/resources/Identifier;)Z
private synthetic lambda$run$3(Lnet/minecraft/core/HolderLookup$RegistryLookup;Lnet/minecraft/resources/Identifier;)Z
private static synthetic lambda$run$1(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/data/tags/TagsProvider$TagLookup;)Lnet/minecraft/data/tags/TagsProvider$1CombinedData;
private synthetic lambda$run$0(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/HolderLookup$Provider;
```
