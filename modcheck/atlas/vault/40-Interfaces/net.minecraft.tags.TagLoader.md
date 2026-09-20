---
type: "interface"
fqcn: "net.minecraft.tags.TagLoader"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.TagLoader

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `build` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| injects_into | `lambda$build$0` | `@ModifyArg at INVOKE Lnet/minecraft/tags/TagLoader$SortingEntry;<init>(Ljava/uti` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| injects_into | `load` | `@Inject at INVOKE Ljava/util/List;clear()V` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| injects_into | `load` | `@Inject at INVOKE Ljava/util/List;forEach(Ljava/util/function/Consumer;)V` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (25, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.tags.TagLoader<T> {
    private static final org.slf4j.Logger LOGGER;
    private final net.minecraft.tags.TagLoader$ElementLookup<T> elementLookup;
    private final java.lang.String directory;
    public net.minecraft.tags.TagLoader(net.minecraft.tags.TagLoader$ElementLookup<T>, java.lang.String);
    public java.util.Map<net.minecraft.resources.Identifier, java.util.List<net.minecraft.tags.TagLoader$EntryWithSource>> load(net.minecraft.server.packs.resources.ResourceManager);
    private com.mojang.datafixers.util.Either<java.util.List<net.minecraft.tags.TagLoader$EntryWithSource>, java.util.List<T>> tryBuildTag(net.minecraft.tags.TagEntry$Lookup<T>, java.util.List<net.minecraft.tags.TagLoader$EntryWithSource>);
    public java.util.Map<net.minecraft.resources.Identifier, java.util.List<T>> build(java.util.Map<net.minecraft.resources.Identifier, java.util.List<net.minecraft.tags.TagLoader$EntryWithSource>>);
    public static <T> java.util.Map<net.minecraft.tags.TagKey<T>, java.util.List<net.minecraft.core.Holder<T>>> loadTagsFromNetwork(net.minecraft.tags.TagNetworkSerialization$NetworkPayload, net.minecraft.core.Registry<T>);
    public static java.util.List<net.minecraft.core.Registry$PendingTags<?>> loadTagsForExistingRegistries(net.minecraft.server.packs.resources.ResourceManager, net.minecraft.core.RegistryAccess);
    public static <T> void loadTagsForRegistry(net.minecraft.server.packs.resources.ResourceManager, net.minecraft.core.WritableRegistry<T>);
    public static <T> java.util.Map<net.minecraft.tags.TagKey<T>, java.util.List<net.minecraft.core.Holder<T>>> loadTagsForRegistry(net.minecraft.server.packs.resources.ResourceManager, net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, net.minecraft.tags.TagLoader$ElementLookup<net.minecraft.core.Holder<T>>);
    private static <T> java.util.Map<net.minecraft.tags.TagKey<T>, java.util.List<net.minecraft.core.Holder<T>>> wrapTags(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, java.util.Map<net.minecraft.resources.Identifier, java.util.List<net.minecraft.core.Holder<T>>>);
    private static <T> java.util.Optional<net.minecraft.core.Registry$PendingTags<T>> loadPendingTags(net.minecraft.server.packs.resources.ResourceManager, net.minecraft.core.Registry<T>);
    public static java.util.List<net.minecraft.core.HolderLookup$RegistryLookup<?>> buildUpdatedLookups(net.minecraft.core.RegistryAccess$Frozen, java.util.List<net.minecraft.core.Registry$PendingTags<?>>);
    private static net.minecraft.core.Registry$PendingTags<?> findTagsForRegistry(java.util.List<net.minecraft.core.Registry$PendingTags<?>>, net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>);
    private static void lambda$buildUpdatedLookups$0(java.util.List, java.util.List, net.minecraft.core.RegistryAccess$RegistryEntry);
    private static net.minecraft.tags.TagKey lambda$wrapTags$0(net.minecraft.resources.ResourceKey, java.util.Map$Entry);
    private static java.util.Optional lambda$loadTagsForExistingRegistries$0(net.minecraft.server.packs.resources.ResourceManager, net.minecraft.core.RegistryAccess$RegistryEntry);
    private void lambda$build$1(net.minecraft.tags.TagEntry$Lookup, java.util.Map, net.minecraft.resources.Identifier, net.minecraft.tags.TagLoader$SortingEntry);
    private static void lambda$build$3(java.util.Map, net.minecraft.resources.Identifier, java.util.List);
    private static void lambda$build$2(net.minecraft.resources.Identifier, java.util.List);
    private static void lambda$build$0(net.minecraft.util.DependencySorter, net.minecraft.resources.Identifier, java.util.List);
    private static void lambda$load$1(java.util.List, java.lang.String, net.minecraft.tags.TagEntry);
    private static java.util.List lambda$load$0(net.minecraft.resources.Identifier);
    static {};
}
```
