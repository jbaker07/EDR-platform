---
type: "interface"
fqcn: "net.minecraft.server.ReloadableServerRegistries$LoadResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.ReloadableServerRegistries$LoadResult

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `layers` | `()Lnet/minecraft/core/LayeredRegistryAccess;` | exact | invokevirtual@2 in `ReloadableServerResourcesMixin.init` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `layers` | `()Lnet/minecraft/core/LayeredRegistryAccess;` | exact | invokevirtual@2 in `ReloadableServerResourcesMixin.storeDynamicRegistries` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `lookupWithUpdatedTags` | `()Lnet/minecraft/core/HolderLookup$Provider;` | exact | invokevirtual@18 in `ReloadableServerResourcesMixin.onSetupDataReloaders` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final layers : Lnet/minecraft/core/LayeredRegistryAccess;
private final lookupWithUpdatedTags : Lnet/minecraft/core/HolderLookup$Provider;
public <init>(Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/core/HolderLookup$Provider;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public layers()Lnet/minecraft/core/LayeredRegistryAccess;
public lookupWithUpdatedTags()Lnet/minecraft/core/HolderLookup$Provider;
```
