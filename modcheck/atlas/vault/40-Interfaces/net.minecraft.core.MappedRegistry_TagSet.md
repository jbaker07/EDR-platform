---
type: "interface"
fqcn: "net.minecraft.core.MappedRegistry$TagSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.MappedRegistry$TagSet

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `get(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | `` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
interface net.minecraft.core.MappedRegistry$TagSet<T> {
    public static <T> net.minecraft.core.MappedRegistry$TagSet<T> unbound();
    public static <T> net.minecraft.core.MappedRegistry$TagSet<T> fromMap(java.util.Map<net.minecraft.tags.TagKey<T>, net.minecraft.core.HolderSet$Named<T>>);
    public abstract boolean isBound();
    public abstract java.util.Optional<net.minecraft.core.HolderSet$Named<T>> get(net.minecraft.tags.TagKey<T>);
    public abstract void forEach(java.util.function.BiConsumer<? super net.minecraft.tags.TagKey<T>, ? super net.minecraft.core.HolderSet$Named<T>>);
    public abstract java.util.stream.Stream<net.minecraft.core.HolderSet$Named<T>> getTags();
}
```
