---
type: "interface"
fqcn: "net.minecraft.core.Holder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.Holder

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `direct(Ljava/lang/Object;)Lnet/minecraft/core/Holder;` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `direct(Ljava/lang/Object;)Lnet/minecraft/core/Holder;` | `` | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `direct(Ljava/lang/Object;)Lnet/minecraft/core/Holder;` | `` | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `direct(Ljava/lang/Object;)Lnet/minecraft/core/Holder;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `equals(Ljava/lang/Object;)Z` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getRegisteredName()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `is(Lnet/minecraft/tags/TagKey;)Z` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `unwrapKey()Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.core.Holder<T> {
    public abstract T value();
    public abstract boolean isBound();
    public abstract boolean areComponentsBound();
    public abstract boolean is(net.minecraft.resources.Identifier);
    public abstract boolean is(net.minecraft.resources.ResourceKey<T>);
    public abstract boolean is(java.util.function.Predicate<net.minecraft.resources.ResourceKey<T>>);
    public abstract boolean is(net.minecraft.tags.TagKey<T>);
    public abstract boolean is(net.minecraft.core.Holder<T>);
    public abstract java.util.stream.Stream<net.minecraft.tags.TagKey<T>> tags();
    public abstract net.minecraft.core.component.DataComponentMap components();
    public abstract com.mojang.datafixers.util.Either<net.minecraft.resources.ResourceKey<T>, T> unwrap();
    public abstract java.util.Optional<net.minecraft.resources.ResourceKey<T>> unwrapKey();
    public abstract net.minecraft.core.Holder$Kind kind();
    public abstract boolean canSerializeIn(net.minecraft.core.HolderOwner<T>);
    public default java.util.Optional<java.lang.String> getRegisteredNameIfPresent();
    public default java.lang.String getRegisteredName();
    public static <T> net.minecraft.core.Holder<T> direct(T);
    public static <T> net.minecraft.core.Holder<T> direct(T, net.minecraft.core.component.DataComponentMap);
    private static java.lang.String lambda$getRegisteredNameIfPresent$0(net.minecraft.resources.ResourceKey);
}
```
