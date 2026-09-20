---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentPatch"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentPatch

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `builder()Lnet/minecraft/core/component/DataComponentPatch$Builder;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `builder()Lnet/minecraft/core/component/DataComponentPatch$Builder;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `builder()Lnet/minecraft/core/component/DataComponentPatch$Builder;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `equals(Ljava/lang/Object;)Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `isEmpty()Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `isEmpty()Z` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEmpty()Z` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `split()Lnet/minecraft/core/component/DataComponentPatch$SplitResu` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `split()Lnet/minecraft/core/component/DataComponentPatch$SplitResu` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTYLnet/minecraft/core/component/DataComponentPatch;` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTYLnet/minecraft/core/component/DataComponentPatch;` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.core.component.DataComponentPatch {
    public static final net.minecraft.core.component.DataComponentPatch EMPTY;
    public static final com.mojang.serialization.Codec<net.minecraft.core.component.DataComponentPatch> CODEC;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.core.component.DataComponentPatch> STREAM_CODEC;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.core.component.DataComponentPatch> DELIMITED_STREAM_CODEC;
    private static final java.lang.String REMOVED_PREFIX;
    final it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.core.component.DataComponentType<?>, java.lang.Object> map;
    private static net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.core.component.DataComponentPatch> createStreamCodec(net.minecraft.core.component.DataComponentPatch$CodecGetter);
    net.minecraft.core.component.DataComponentPatch(it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.core.component.DataComponentType<?>, java.lang.Object>);
    public static net.minecraft.core.component.DataComponentPatch$Builder builder();
    public <T> T get(net.minecraft.core.component.DataComponentGetter, net.minecraft.core.component.DataComponentType<? extends T>);
    static <T> T getFromPatchAndPrototype(it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.core.component.DataComponentType<?>, java.lang.Object>, net.minecraft.core.component.DataComponentGetter, net.minecraft.core.component.DataComponentType<? extends T>);
    public int size();
    public net.minecraft.core.component.DataComponentPatch forget(java.util.function.Predicate<net.minecraft.core.component.DataComponentType<?>>);
    public boolean isEmpty();
    public net.minecraft.core.component.DataComponentPatch$SplitResult split();
    public boolean equals(java.lang.Object);
    public int hashCode();
    public java.lang.String toString();
    static java.lang.String toString(it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.core.component.DataComponentType<?>, java.lang.Object>);
    private static void lambda$split$0(net.minecraft.core.component.DataComponentMap$Builder, java.util.Set, net.minecraft.core.component.DataComponentType, java.lang.Object);
    private static java.util.Map lambda$static$1(net.minecraft.core.component.DataComponentPatch);
    private static net.minecraft.core.component.DataComponentPatch lambda$static$0(java.util.Map);
    static {};
}
```
