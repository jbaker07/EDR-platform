---
type: "interface"
fqcn: "net.minecraft.util.InclusiveRange"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.InclusiveRange

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/lang/Comparable;)V` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (21, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.util.InclusiveRange<T extends java.lang.Comparable<T>> extends java.lang.Record {
    private final T minInclusive;
    private final T maxInclusive;
    public static final com.mojang.serialization.Codec<net.minecraft.util.InclusiveRange<java.lang.Integer>> INT;
    public net.minecraft.util.InclusiveRange(T, T);
    public net.minecraft.util.InclusiveRange(T);
    public static <T extends java.lang.Comparable<T>> com.mojang.serialization.Codec<net.minecraft.util.InclusiveRange<T>> codec(com.mojang.serialization.Codec<T>);
    public static <T extends java.lang.Comparable<T>> com.mojang.serialization.Codec<net.minecraft.util.InclusiveRange<T>> codec(com.mojang.serialization.Codec<T>, T, T);
    public static <T extends java.lang.Comparable<T>> com.mojang.serialization.DataResult<net.minecraft.util.InclusiveRange<T>> create(T, T);
    public <S extends java.lang.Comparable<S>> net.minecraft.util.InclusiveRange<S> map(java.util.function.Function<? super T, ? extends S>);
    public boolean isValueInRange(T);
    public boolean contains(net.minecraft.util.InclusiveRange<T>);
    public java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public T minInclusive();
    public T maxInclusive();
    private static java.lang.String lambda$create$0();
    private static com.mojang.serialization.DataResult lambda$codec$0(java.lang.Comparable, java.lang.Comparable, net.minecraft.util.InclusiveRange);
    private static java.lang.String lambda$codec$2(java.lang.Comparable, net.minecraft.util.InclusiveRange);
    private static java.lang.String lambda$codec$1(java.lang.Comparable, net.minecraft.util.InclusiveRange);
    static {};
}
```
