---
type: "interface"
fqcn: "net.minecraft.data.tags.TagsProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.tags.TagsProvider

System: [[20-Systems/net.minecraft.data.tags|net.minecraft.data.tags]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>(Lnet/minecraft/data/PackOutput;Lnet/minecraft/resources/ResourceKey;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;)V` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `lambda$run$5` | `@ModifyArg at INVOKE Lnet/minecraft/data/DataProvider;saveStable(Lnet/minecraft/` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `lambda$run$5` | `@ModifyArg at INVOKE Lnet/minecraft/tags/TagFile;<init>(Ljava/util/List;Z)V` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (28, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.data.tags.TagsProvider<T> implements net.minecraft.data.DataProvider {
    protected final net.minecraft.data.PackOutput$PathProvider pathProvider;
    private final java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider> lookupProvider;
    private final java.util.concurrent.CompletableFuture<java.lang.Void> contentsDone;
    private final java.util.concurrent.CompletableFuture<net.minecraft.data.tags.TagsProvider$TagLookup<T>> parentProvider;
    protected final net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> registryKey;
    private final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.tags.TagBuilder> builders;
    protected net.minecraft.data.tags.TagsProvider(net.minecraft.data.PackOutput, net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider>);
    protected net.minecraft.data.tags.TagsProvider(net.minecraft.data.PackOutput, net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider>, java.util.concurrent.CompletableFuture<net.minecraft.data.tags.TagsProvider$TagLookup<T>>);
    public final java.lang.String getName();
    protected abstract void addTags(net.minecraft.core.HolderLookup$Provider);
    public java.util.concurrent.CompletableFuture<?> run(net.minecraft.data.CachedOutput);
    protected net.minecraft.tags.TagBuilder getOrCreateRawBuilder(net.minecraft.tags.TagKey<T>);
    public java.util.concurrent.CompletableFuture<net.minecraft.data.tags.TagsProvider$TagLookup<T>> contentsGetter();
    protected java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider> createContentsProvider();
    protected net.minecraft.data.tags.TagAppender<T> tag(net.minecraft.tags.TagKey<T>);
    protected net.minecraft.data.tags.TagAppender<T> tag(net.minecraft.tags.TagKey<T>, boolean);
    private net.minecraft.core.HolderLookup$Provider lambda$createContentsProvider$0(net.minecraft.core.HolderLookup$Provider);
    private net.minecraft.data.tags.TagsProvider$TagLookup lambda$contentsGetter$0(java.lang.Void);
    private java.util.Optional lambda$contentsGetter$1(net.minecraft.tags.TagKey);
    private static net.minecraft.tags.TagBuilder lambda$getOrCreateRawBuilder$0(net.minecraft.resources.Identifier);
    private java.util.concurrent.CompletionStage lambda$run$2(net.minecraft.data.CachedOutput, net.minecraft.data.tags.TagsProvider$1CombinedData);
    private static java.util.concurrent.CompletableFuture[] lambda$run$7(int);
    private java.util.concurrent.CompletableFuture lambda$run$5(java.util.function.Predicate, java.util.function.Predicate, net.minecraft.data.CachedOutput, net.minecraft.data.tags.TagsProvider$1CombinedData, java.util.Map$Entry);
    private static boolean lambda$run$6(java.util.function.Predicate, java.util.function.Predicate, net.minecraft.tags.TagEntry);
    private boolean lambda$run$4(net.minecraft.data.tags.TagsProvider$1CombinedData, net.minecraft.resources.Identifier);
    private boolean lambda$run$3(net.minecraft.core.HolderLookup$RegistryLookup, net.minecraft.resources.Identifier);
    private static net.minecraft.data.tags.TagsProvider$1CombinedData lambda$run$1(net.minecraft.core.HolderLookup$Provider, net.minecraft.data.tags.TagsProvider$TagLookup);
    private net.minecraft.core.HolderLookup$Provider lambda$run$0(net.minecraft.core.HolderLookup$Provider);
}
```
