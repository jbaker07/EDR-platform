---
type: "interface"
fqcn: "net.minecraft.core.component.TypedDataComponent"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.TypedDataComponent

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `type()Lnet/minecraft/core/component/DataComponentType;` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `type()Lnet/minecraft/core/component/DataComponentType;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.core.component.TypedDataComponent<T> extends java.lang.Record {
    private final net.minecraft.core.component.DataComponentType<T> type;
    private final T value;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.core.component.TypedDataComponent<?>> STREAM_CODEC;
    public net.minecraft.core.component.TypedDataComponent(net.minecraft.core.component.DataComponentType<T>, T);
    static net.minecraft.core.component.TypedDataComponent<?> fromEntryUnchecked(java.util.Map$Entry<net.minecraft.core.component.DataComponentType<?>, java.lang.Object>);
    public static <T> net.minecraft.core.component.TypedDataComponent<T> createUnchecked(net.minecraft.core.component.DataComponentType<T>, java.lang.Object);
    public void applyTo(net.minecraft.core.component.PatchedDataComponentMap);
    public <D> com.mojang.serialization.DataResult<D> encodeValue(com.mojang.serialization.DynamicOps<D>);
    public java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.core.component.DataComponentType<T> type();
    public T value();
    private java.lang.String lambda$encodeValue$0();
    static {};
}
```
