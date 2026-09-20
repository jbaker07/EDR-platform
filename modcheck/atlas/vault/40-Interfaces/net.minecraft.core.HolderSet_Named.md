---
type: "interface"
fqcn: "net.minecraft.core.HolderSet$Named"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.HolderSet$Named

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `key()Lnet/minecraft/tags/TagKey;` | `` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| reads | `contentsLjava/util/List;` | `` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.HolderSet$Named<T> extends net.minecraft.core.HolderSet$ListBacked<T> {
    private final net.minecraft.core.HolderOwner<T> owner;
    private final net.minecraft.tags.TagKey<T> key;
    private java.util.List<net.minecraft.core.Holder<T>> contents;
    net.minecraft.core.HolderSet$Named(net.minecraft.core.HolderOwner<T>, net.minecraft.tags.TagKey<T>);
    void bind(java.util.List<net.minecraft.core.Holder<T>>);
    public net.minecraft.tags.TagKey<T> key();
    protected java.util.List<net.minecraft.core.Holder<T>> contents();
    public boolean isBound();
    public com.mojang.datafixers.util.Either<net.minecraft.tags.TagKey<T>, java.util.List<net.minecraft.core.Holder<T>>> unwrap();
    public java.util.Optional<net.minecraft.tags.TagKey<T>> unwrapKey();
    public boolean contains(net.minecraft.core.Holder<T>);
    public java.lang.String toString();
    public boolean canSerializeIn(net.minecraft.core.HolderOwner<T>);
}
```
