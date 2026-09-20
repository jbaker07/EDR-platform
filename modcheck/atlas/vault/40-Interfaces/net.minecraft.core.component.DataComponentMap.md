---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentMap

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getOrDefault(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOrDefault(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOrDefault(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOrDefault(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `iterator()Ljava/util/Iterator;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `keySet()Ljava/util/Set;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `stream()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.core.component.DataComponentMap extends java.lang.Iterable<net.minecraft.core.component.TypedDataComponent<?>>, net.minecraft.core.component.DataComponentGetter {
    public static final net.minecraft.core.component.DataComponentMap EMPTY;
    public static final com.mojang.serialization.Codec<net.minecraft.core.component.DataComponentMap> CODEC;
    public static com.mojang.serialization.Codec<net.minecraft.core.component.DataComponentMap> makeCodec(com.mojang.serialization.Codec<net.minecraft.core.component.DataComponentType<?>>);
    public static com.mojang.serialization.Codec<net.minecraft.core.component.DataComponentMap> makeCodecFromMap(com.mojang.serialization.Codec<java.util.Map<net.minecraft.core.component.DataComponentType<?>, java.lang.Object>>);
    public static net.minecraft.core.component.DataComponentMap composite(net.minecraft.core.component.DataComponentMap, net.minecraft.core.component.DataComponentMap);
    public static net.minecraft.core.component.DataComponentMap$Builder builder();
    public abstract java.util.Set<net.minecraft.core.component.DataComponentType<?>> keySet();
    public default boolean has(net.minecraft.core.component.DataComponentType<?>);
    public default java.util.Iterator<net.minecraft.core.component.TypedDataComponent<?>> iterator();
    public default java.util.stream.Stream<net.minecraft.core.component.TypedDataComponent<?>> stream();
    public default int size();
    public default boolean isEmpty();
    public default net.minecraft.core.component.DataComponentMap filter(java.util.function.Predicate<net.minecraft.core.component.DataComponentType<?>>);
    private net.minecraft.core.component.TypedDataComponent lambda$iterator$0(net.minecraft.core.component.DataComponentType);
    private static com.mojang.serialization.DataResult lambda$makeCodecFromMap$0(net.minecraft.core.component.DataComponentMap);
    static {};
}
```
