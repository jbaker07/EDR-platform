---
type: "interface"
fqcn: "net.minecraft.core.RegistryAccess$RegistryEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.RegistryAccess$RegistryEntry

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `key()Lnet/minecraft/resources/ResourceKey;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `value()Lnet/minecraft/core/Registry;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `value()Lnet/minecraft/core/Registry;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.core.RegistryAccess$RegistryEntry<T> extends java.lang.Record {
    private final net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> key;
    private final net.minecraft.core.Registry<T> value;
    public net.minecraft.core.RegistryAccess$RegistryEntry(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, net.minecraft.core.Registry<T>);
    private static <T, R extends net.minecraft.core.Registry<? extends T>> net.minecraft.core.RegistryAccess$RegistryEntry<T> fromMapEntry(java.util.Map$Entry<? extends net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>, R>);
    private static <T> net.minecraft.core.RegistryAccess$RegistryEntry<T> fromUntyped(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>, net.minecraft.core.Registry<?>);
    private net.minecraft.core.RegistryAccess$RegistryEntry<T> freeze();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> key();
    public net.minecraft.core.Registry<T> value();
}
```
