---
type: "interface"
fqcn: "net.minecraft.core.Registry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.Registry

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `byId(I)Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `byId(I)Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `containsKey(Lnet/minecraft/resources/Identifier;)Z` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `containsKey(Lnet/minecraft/resources/Identifier;)Z` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `entrySet()Ljava/util/Set;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `entrySet()Ljava/util/Set;` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `forEach(Ljava/util/function/Consumer;)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `get(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getClass()Ljava/lang/Class;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getId(Ljava/lang/Object;)I` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getId(Ljava/lang/Object;)I` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `getId(Ljava/lang/Object;)I` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getId(Ljava/lang/Object;)I` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | both | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | client | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOptional(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getOptional(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getOptional(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/H` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/H` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/H` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `getResourceKey(Ljava/lang/Object;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getResourceKey(Ljava/lang/Object;)Ljava/util/Optional;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getResourceKey(Ljava/lang/Object;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getTags()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `getValue(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getValue(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getValue(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `getValue(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | `` | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getValue(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getValue(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getValueOrThrow(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getValueOrThrow(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getValueOrThrow(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `iterator()Ljava/util/Iterator;` | `` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `iterator()Ljava/util/Iterator;` | `` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `iterator()Ljava/util/Iterator;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `iterator()Ljava/util/Iterator;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `keySet()Ljava/util/Set;` | `` | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `keySet()Ljava/util/Set;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `listElements()Ljava/util/stream/Stream;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `listElements()Ljava/util/stream/Stream;` | `` | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `register(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/Ident` | `` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `register(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/Ident` | `` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `registrationInfo(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `registryKeySet()Ljava/util/Set;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `size()I` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `stream()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (47, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.core.Registry<T> extends net.minecraft.core.IdMap<T>, com.mojang.serialization.Keyable, net.minecraft.core.HolderLookup$RegistryLookup<T> {
    public abstract net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> key();
    public default com.mojang.serialization.Codec<T> byNameCodec();
    public default com.mojang.serialization.Codec<net.minecraft.core.Holder<T>> holderByNameCodec();
    private com.mojang.serialization.Codec<net.minecraft.core.Holder$Reference<T>> referenceHolderWithLifecycle();
    private com.mojang.serialization.DataResult<net.minecraft.core.Holder$Reference<T>> safeCastToReference(net.minecraft.core.Holder<T>);
    public default <U> java.util.stream.Stream<U> keys(com.mojang.serialization.DynamicOps<U>);
    public abstract net.minecraft.resources.Identifier getKey(T);
    public abstract java.util.Optional<net.minecraft.resources.ResourceKey<T>> getResourceKey(T);
    public abstract int getId(T);
    public abstract T getValue(net.minecraft.resources.ResourceKey<T>);
    public abstract T getValue(net.minecraft.resources.Identifier);
    public abstract java.util.Optional<net.minecraft.core.RegistrationInfo> registrationInfo(net.minecraft.resources.ResourceKey<T>);
    public default java.util.Optional<T> getOptional(net.minecraft.resources.Identifier);
    public default java.util.Optional<T> getOptional(net.minecraft.resources.ResourceKey<T>);
    public abstract java.util.Optional<net.minecraft.core.Holder$Reference<T>> getAny();
    public default T getValueOrThrow(net.minecraft.resources.ResourceKey<T>);
    public abstract java.util.Set<net.minecraft.resources.Identifier> keySet();
    public abstract java.util.Set<java.util.Map$Entry<net.minecraft.resources.ResourceKey<T>, T>> entrySet();
    public abstract java.util.Set<net.minecraft.resources.ResourceKey<T>> registryKeySet();
    public abstract java.util.Optional<net.minecraft.core.Holder$Reference<T>> getRandom(net.minecraft.util.RandomSource);
    public default java.util.stream.Stream<T> stream();
    public abstract boolean containsKey(net.minecraft.resources.Identifier);
    public abstract boolean containsKey(net.minecraft.resources.ResourceKey<T>);
    public static <T> T register(net.minecraft.core.Registry<? super T>, java.lang.String, T);
    public static <V, T extends V> T register(net.minecraft.core.Registry<V>, net.minecraft.resources.Identifier, T);
    public static <V, T extends V> T register(net.minecraft.core.Registry<V>, net.minecraft.resources.ResourceKey<V>, T);
    public static <R, T extends R> net.minecraft.core.Holder$Reference<T> registerForHolder(net.minecraft.core.Registry<R>, net.minecraft.resources.ResourceKey<R>, T);
    public static <R, T extends R> net.minecraft.core.Holder$Reference<T> registerForHolder(net.minecraft.core.Registry<R>, net.minecraft.resources.Identifier, T);
    public abstract net.minecraft.core.Registry<T> freeze();
    public abstract net.minecraft.core.Holder$Reference<T> createIntrusiveHolder(T);
    public abstract java.util.Optional<net.minecraft.core.Holder$Reference<T>> get(int);
    public abstract java.util.Optional<net.minecraft.core.Holder$Reference<T>> get(net.minecraft.resources.Identifier);
    public abstract net.minecraft.core.Holder<T> wrapAsHolder(T);
    public default java.lang.Iterable<net.minecraft.core.Holder<T>> getTagOrEmpty(net.minecraft.tags.TagKey<T>);
    public abstract java.util.stream.Stream<net.minecraft.core.HolderSet$Named<T>> getTags();
    public default net.minecraft.core.IdMap<net.minecraft.core.Holder<T>> asHolderIdMap();
    public abstract net.minecraft.core.Registry$PendingTags<T> prepareTagReload(net.minecraft.tags.TagLoader$LoadResult<T>);
    public abstract net.minecraft.core.component.DataComponentLookup<T> componentLookup();
    private static java.lang.Object lambda$keys$0(com.mojang.serialization.DynamicOps, net.minecraft.resources.Identifier);
    private java.lang.String lambda$safeCastToReference$0(net.minecraft.core.Holder);
    private com.mojang.serialization.Lifecycle lambda$referenceHolderWithLifecycle$4(net.minecraft.core.Holder$Reference);
    private static net.minecraft.resources.Identifier lambda$referenceHolderWithLifecycle$3(net.minecraft.core.Holder$Reference);
    private com.mojang.serialization.DataResult lambda$referenceHolderWithLifecycle$0(net.minecraft.resources.Identifier);
    private com.mojang.serialization.DataResult lambda$referenceHolderWithLifecycle$1(net.minecraft.resources.Identifier);
    private java.lang.String lambda$referenceHolderWithLifecycle$2(net.minecraft.resources.Identifier);
    private static net.minecraft.core.Holder lambda$holderByNameCodec$0(net.minecraft.core.Holder$Reference);
    private com.mojang.serialization.DataResult lambda$byNameCodec$0(java.lang.Object);
}
```
