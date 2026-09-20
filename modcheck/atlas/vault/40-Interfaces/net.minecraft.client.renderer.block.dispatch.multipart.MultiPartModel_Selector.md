---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$Selector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$Selector

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `model()Ljava/lang/Object;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$Selector<T> extends java.lang.Record {
    private final java.util.function.Predicate<net.minecraft.world.level.block.state.BlockState> condition;
    private final T model;
    public net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$Selector(java.util.function.Predicate<net.minecraft.world.level.block.state.BlockState>, T);
    public <S> net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$Selector<S> with(S);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.util.function.Predicate<net.minecraft.world.level.block.state.BlockState> condition();
    public T model();
}
```
