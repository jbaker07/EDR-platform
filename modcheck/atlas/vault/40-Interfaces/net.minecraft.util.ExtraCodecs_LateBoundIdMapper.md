---
type: "interface"
fqcn: "net.minecraft.util.ExtraCodecs$LateBoundIdMapper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.ExtraCodecs$LateBoundIdMapper

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `codec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `put(Ljava/lang/Object;Ljava/lang/Object;)Lnet/minecraft/util/Ex` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `put(Ljava/lang/Object;Ljava/lang/Object;)Lnet/minecraft/util/Ex` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.ExtraCodecs$LateBoundIdMapper<I, V> {
    private final com.google.common.collect.BiMap<I, V> idToValue;
    public net.minecraft.util.ExtraCodecs$LateBoundIdMapper();
    public com.mojang.serialization.Codec<V> codec(com.mojang.serialization.Codec<I>);
    public net.minecraft.util.ExtraCodecs$LateBoundIdMapper<I, V> put(I, V);
    public java.util.Set<V> values();
    private static java.lang.String lambda$put$0(java.lang.Object);
}
```
