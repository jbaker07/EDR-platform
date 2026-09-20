---
type: "interface"
fqcn: "net.minecraft.core.Holder$Reference"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.Holder$Reference

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `bindValue(Ljava/lang/Object;)V` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `createStandAlone(Lnet/minecraft/core/HolderOwner;Lnet/minecraft/resources/Re` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `is(Lnet/minecraft/tags/TagKey;)Z` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (30, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.Holder$Reference<T> implements net.minecraft.core.Holder<T> {
    private final net.minecraft.core.HolderOwner<T> owner;
    private java.util.Set<net.minecraft.tags.TagKey<T>> tags;
    private net.minecraft.core.component.DataComponentMap components;
    private final net.minecraft.core.Holder$Reference$Type type;
    private net.minecraft.resources.ResourceKey<T> key;
    private T value;
    protected net.minecraft.core.Holder$Reference(net.minecraft.core.Holder$Reference$Type, net.minecraft.core.HolderOwner<T>, net.minecraft.resources.ResourceKey<T>, T);
    public static <T> net.minecraft.core.Holder$Reference<T> createStandAlone(net.minecraft.core.HolderOwner<T>, net.minecraft.resources.ResourceKey<T>);
    public static <T> net.minecraft.core.Holder$Reference<T> createIntrusive(net.minecraft.core.HolderOwner<T>, T);
    public net.minecraft.resources.ResourceKey<T> key();
    public T value();
    public boolean is(net.minecraft.resources.Identifier);
    public boolean is(net.minecraft.resources.ResourceKey<T>);
    private java.util.Set<net.minecraft.tags.TagKey<T>> boundTags();
    public boolean is(net.minecraft.tags.TagKey<T>);
    public boolean is(net.minecraft.core.Holder<T>);
    public boolean is(java.util.function.Predicate<net.minecraft.resources.ResourceKey<T>>);
    public boolean canSerializeIn(net.minecraft.core.HolderOwner<T>);
    public com.mojang.datafixers.util.Either<net.minecraft.resources.ResourceKey<T>, T> unwrap();
    public java.util.Optional<net.minecraft.resources.ResourceKey<T>> unwrapKey();
    public net.minecraft.core.Holder$Kind kind();
    public boolean isBound();
    public boolean areComponentsBound();
    void bindKey(net.minecraft.resources.ResourceKey<T>);
    protected void bindValue(T);
    void bindTags(java.util.Collection<net.minecraft.tags.TagKey<T>>);
    public void bindComponents(net.minecraft.core.component.DataComponentMap);
    public java.util.stream.Stream<net.minecraft.tags.TagKey<T>> tags();
    public net.minecraft.core.component.DataComponentMap components();
    public java.lang.String toString();
}
```
