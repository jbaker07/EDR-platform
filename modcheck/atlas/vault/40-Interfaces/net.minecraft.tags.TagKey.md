---
type: "interface"
fqcn: "net.minecraft.tags.TagKey"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.TagKey

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `codec(Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/serializa` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `create(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resourc` | `` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `create(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resourc` | `` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `getTranslationKey()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `isFor(Lnet/minecraft/resources/ResourceKey;)Z` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `location()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `location()Lnet/minecraft/resources/Identifier;` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `location()Lnet/minecraft/resources/Identifier;` | `` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `location()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `registry()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `registry()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `registry()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `registry()Lnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `registry()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `registry()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.tags.TagKey<T> extends java.lang.Record {
    private final net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> registry;
    private final net.minecraft.resources.Identifier location;
    private static final com.google.common.collect.Interner<net.minecraft.tags.TagKey<?>> VALUES;
    public net.minecraft.tags.TagKey(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, net.minecraft.resources.Identifier);
    public static <T> com.mojang.serialization.Codec<net.minecraft.tags.TagKey<T>> codec(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>);
    public static <T> com.mojang.serialization.Codec<net.minecraft.tags.TagKey<T>> hashedCodec(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>);
    public static <T> net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.tags.TagKey<T>> streamCodec(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>);
    public static <T> net.minecraft.tags.TagKey<T> create(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, net.minecraft.resources.Identifier);
    public boolean isFor(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>);
    public <E> java.util.Optional<net.minecraft.tags.TagKey<E>> cast(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<E>>);
    public java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> registry();
    public net.minecraft.resources.Identifier location();
    private static net.minecraft.tags.TagKey lambda$streamCodec$0(net.minecraft.resources.ResourceKey, net.minecraft.resources.Identifier);
    private static java.lang.String lambda$hashedCodec$3(net.minecraft.tags.TagKey);
    private static com.mojang.serialization.DataResult lambda$hashedCodec$0(net.minecraft.resources.ResourceKey, java.lang.String);
    private static java.lang.String lambda$hashedCodec$2();
    private static net.minecraft.tags.TagKey lambda$hashedCodec$1(net.minecraft.resources.ResourceKey, net.minecraft.resources.Identifier);
    private static net.minecraft.tags.TagKey lambda$codec$0(net.minecraft.resources.ResourceKey, net.minecraft.resources.Identifier);
    static {};
}
```
