---
type: "interface"
fqcn: "net.minecraft.data.tags.TagAppender"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.tags.TagAppender

System: [[20-Systems/net.minecraft.data.tags|net.minecraft.data.tags]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getBuilder()Lnet/minecraft/tags/TagBuilder;` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.data.tags.TagAppender<T> {
    public abstract net.minecraft.data.tags.TagAppender<T> add(net.minecraft.resources.ResourceKey<T>);
    public default net.minecraft.data.tags.TagAppender<T> add(net.minecraft.resources.ResourceKey<T>...);
    public default net.minecraft.data.tags.TagAppender<T> addAll(java.util.Collection<net.minecraft.resources.ResourceKey<T>>);
    public default net.minecraft.data.tags.TagAppender<T> addAll(java.util.stream.Stream<net.minecraft.resources.ResourceKey<T>>);
    public abstract net.minecraft.data.tags.TagAppender<T> addOptional(net.minecraft.resources.ResourceKey<T>);
    public abstract net.minecraft.data.tags.TagAppender<T> addTag(net.minecraft.tags.TagKey<T>);
    public abstract net.minecraft.data.tags.TagAppender<T> addOptionalTag(net.minecraft.tags.TagKey<T>);
    public static <T> net.minecraft.data.tags.TagAppender<T> forBuilder(net.minecraft.tags.TagBuilder);
}
```
