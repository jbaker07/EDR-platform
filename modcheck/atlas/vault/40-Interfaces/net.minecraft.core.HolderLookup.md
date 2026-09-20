---
type: "interface"
fqcn: "net.minecraft.core.HolderLookup"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.HolderLookup

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `listElements()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.core.HolderLookup<T> extends net.minecraft.core.HolderGetter<T> {
    public abstract java.util.stream.Stream<net.minecraft.core.Holder$Reference<T>> listElements();
    public default java.util.stream.Stream<net.minecraft.resources.ResourceKey<T>> listElementIds();
    public abstract java.util.stream.Stream<net.minecraft.core.HolderSet$Named<T>> listTags();
    public default java.util.stream.Stream<net.minecraft.tags.TagKey<T>> listTagIds();
}
```
