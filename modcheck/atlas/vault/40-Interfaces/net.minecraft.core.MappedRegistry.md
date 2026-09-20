---
type: "interface"
fqcn: "net.minecraft.core.MappedRegistry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.MappedRegistry

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Lifecycle;Z)V` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `containsKey(Lnet/minecraft/resources/Identifier;)Z` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `containsKey(Lnet/minecraft/resources/ResourceKey;)Z` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `get(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `getOrCreateHolderOrThrow` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `getValue(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `getValue(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `register` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `registrationInfo` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (73, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.MappedRegistry<T> implements net.minecraft.core.WritableRegistry<T> {
    private final net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> key;
    private final it.unimi.dsi.fastutil.objects.ObjectList<net.minecraft.core.Holder$Reference<T>> byId;
    private final it.unimi.dsi.fastutil.objects.Reference2IntMap<T> toId;
    private final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.core.Holder$Reference<T>> byLocation;
    private final java.util.Map<net.minecraft.resources.ResourceKey<T>, net.minecraft.core.Holder$Reference<T>> byKey;
    private final java.util.Map<T, net.minecraft.core.Holder$Reference<T>> byValue;
    private final java.util.Map<net.minecraft.resources.ResourceKey<T>, net.minecraft.core.RegistrationInfo> registrationInfos;
    private com.mojang.serialization.Lifecycle registryLifecycle;
    private final java.util.Map<net.minecraft.tags.TagKey<T>, net.minecraft.core.HolderSet$Named<T>> frozenTags;
    private net.minecraft.core.MappedRegistry$TagSet<T> allTags;
    private net.minecraft.core.component.DataComponentLookup<T> componentLookup;
    private boolean frozen;
    private java.util.Map<T, net.minecraft.core.Holder$Reference<T>> unregisteredIntrusiveHolders;
    public java.util.stream.Stream<net.minecraft.core.HolderSet$Named<T>> listTags();
    public net.minecraft.core.MappedRegistry(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, com.mojang.serialization.Lifecycle);
    public net.minecraft.core.MappedRegistry(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, com.mojang.serialization.Lifecycle, boolean);
    public net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> key();
    public java.lang.String toString();
    private void validateWrite();
    private void validateWrite(net.minecraft.resources.ResourceKey<T>);
    public net.minecraft.core.Holder$Reference<T> register(net.minecraft.resources.ResourceKey<T>, T, net.minecraft.core.RegistrationInfo);
    public net.minecraft.resources.Identifier getKey(T);
    public java.util.Optional<net.minecraft.resources.ResourceKey<T>> getResourceKey(T);
    public int getId(T);
    public T getValue(net.minecraft.resources.ResourceKey<T>);
    public T byId(int);
    public java.util.Optional<net.minecraft.core.Holder$Reference<T>> get(int);
    public java.util.Optional<net.minecraft.core.Holder$Reference<T>> get(net.minecraft.resources.Identifier);
    public java.util.Optional<net.minecraft.core.Holder$Reference<T>> get(net.minecraft.resources.ResourceKey<T>);
    public java.util.Optional<net.minecraft.core.Holder$Reference<T>> getAny();
    public net.minecraft.core.Holder<T> wrapAsHolder(T);
    private net.minecraft.core.Holder$Reference<T> getOrCreateHolderOrThrow(net.minecraft.resources.ResourceKey<T>);
    public int size();
    public java.util.Optional<net.minecraft.core.RegistrationInfo> registrationInfo(net.minecraft.resources.ResourceKey<T>);
    public com.mojang.serialization.Lifecycle registryLifecycle();
    public java.util.Iterator<T> iterator();
    public T getValue(net.minecraft.resources.Identifier);
    private static <T> T getValueFromNullable(net.minecraft.core.Holder$Reference<T>);
    public java.util.Set<net.minecraft.resources.Identifier> keySet();
    public java.util.Set<net.minecraft.resources.ResourceKey<T>> registryKeySet();
    public java.util.Set<java.util.Map$Entry<net.minecraft.resources.ResourceKey<T>, T>> entrySet();
    public java.util.stream.Stream<net.minecraft.core.Holder$Reference<T>> listElements();
    public java.util.stream.Stream<net.minecraft.core.HolderSet$Named<T>> getTags();
    private net.minecraft.core.HolderSet$Named<T> getOrCreateTagForRegistration(net.minecraft.tags.TagKey<T>);
    private net.minecraft.core.HolderSet$Named<T> createTag(net.minecraft.tags.TagKey<T>);
    public boolean isEmpty();
    public java.util.Optional<net.minecraft.core.Holder$Reference<T>> getRandom(net.minecraft.util.RandomSource);
    public boolean containsKey(net.minecraft.resources.Identifier);
    public boolean containsKey(net.minecraft.resources.ResourceKey<T>);
    public net.minecraft.core.component.DataComponentLookup<T> componentLookup();
    public net.minecraft.core.Registry<T> freeze();
    public net.minecraft.core.Holder$Reference<T> createIntrusiveHolder(T);
    public java.util.Optional<net.minecraft.core.HolderSet$Named<T>> get(net.minecraft.tags.TagKey<T>);
    private net.minecraft.core.Holder$Reference<T> validateAndUnwrapTagElement(net.minecraft.tags.TagKey<T>, net.minecraft.core.Holder<T>);
    public void bindTags(java.util.Map<net.minecraft.tags.TagKey<T>, java.util.List<net.minecraft.core.Holder<T>>>);
    private void refreshTagsInHolders();
    public void bindAllTagsToEmpty();
    public net.minecraft.core.HolderGetter<T> createRegistrationLookup();
    public net.minecraft.core.Registry$PendingTags<T> prepareTagReload(net.minecraft.tags.TagLoader$LoadResult<T>);
    private void lambda$prepareTagReload$0(com.google.common.collect.ImmutableMap$Builder, java.util.Map, net.minecraft.tags.TagKey, java.util.List);
    private static void lambda$bindAllTagsToEmpty$0(net.minecraft.core.HolderSet$Named);
    private void lambda$refreshTagsInHolders$1(java.util.Map, net.minecraft.tags.TagKey, net.minecraft.core.HolderSet$Named);
    private static void lambda$refreshTagsInHolders$0(java.util.Map, net.minecraft.core.Holder$Reference);
    private void lambda$bindTags$0(net.minecraft.tags.TagKey, java.util.List);
    private net.minecraft.core.Holder$Reference lambda$createIntrusiveHolder$0(java.lang.Object);
    private static net.minecraft.resources.Identifier lambda$freeze$4(java.util.Map$Entry);
    private static boolean lambda$freeze$3(java.util.Map$Entry);
    private static net.minecraft.resources.Identifier lambda$freeze$2(java.util.Map$Entry);
    private static boolean lambda$freeze$1(java.util.Map$Entry);
    private static void lambda$freeze$0(java.lang.Object, net.minecraft.core.Holder$Reference);
    private net.minecraft.core.Holder$Reference lambda$getOrCreateHolderOrThrow$0(net.minecraft.resources.ResourceKey);
    private net.minecraft.core.Holder$Reference lambda$register$0(net.minecraft.resources.ResourceKey);
    private static void lambda$new$0(it.unimi.dsi.fastutil.objects.Reference2IntOpenHashMap);
}
```
