---
type: "interface"
fqcn: "net.minecraft.server.ReloadableServerRegistries$LoadResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.ReloadableServerRegistries$LoadResult

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `layers()Lnet/minecraft/core/LayeredRegistryAccess;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `layers()Lnet/minecraft/core/LayeredRegistryAccess;` | `` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `lookupWithUpdatedTags()Lnet/minecraft/core/HolderLookup$Provider;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.ReloadableServerRegistries$LoadResult extends java.lang.Record {
    private final net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer> layers;
    private final net.minecraft.core.HolderLookup$Provider lookupWithUpdatedTags;
    public net.minecraft.server.ReloadableServerRegistries$LoadResult(net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>, net.minecraft.core.HolderLookup$Provider);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer> layers();
    public net.minecraft.core.HolderLookup$Provider lookupWithUpdatedTags();
}
```
