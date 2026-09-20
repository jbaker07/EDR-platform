---
type: "interface"
fqcn: "net.minecraft.resources.RegistryDataLoader$RegistryData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryDataLoader$RegistryData

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serializat` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.resources.RegistryDataLoader$RegistryData<T> extends java.lang.Record {
    private final net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> key;
    private final com.mojang.serialization.Codec<T> elementCodec;
    private final net.minecraft.resources.RegistryValidator<T> validator;
    private net.minecraft.resources.RegistryDataLoader$RegistryData(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, com.mojang.serialization.Codec<T>);
    public net.minecraft.resources.RegistryDataLoader$RegistryData(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, com.mojang.serialization.Codec<T>, net.minecraft.resources.RegistryValidator<T>);
    public void runWithArguments(java.util.function.BiConsumer<net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, com.mojang.serialization.Codec<T>>);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> key();
    public com.mojang.serialization.Codec<T> elementCodec();
    public net.minecraft.resources.RegistryValidator<T> validator();
}
```
