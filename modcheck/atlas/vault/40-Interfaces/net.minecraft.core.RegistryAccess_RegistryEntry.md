---
type: "interface"
fqcn: "net.minecraft.core.RegistryAccess$RegistryEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.RegistryAccess$RegistryEntry

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/Registry;)V` | exact | invokespecial@11 in `DynamicRegistryViewImpl$1.entry` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@4 in `RegistrySynchronizationMixin.filterNonSyncedEntries` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `value` | `()Lnet/minecraft/core/Registry;` | exact | invokevirtual@18 in `RegistrySynchronizationMixin.filterNonSyncedEntries` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `value` | `()Lnet/minecraft/core/Registry;` | exact | invokevirtual@34 in `TagAliasLoader.applyToDynamicRegistries` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (2 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final key : Lnet/minecraft/resources/ResourceKey;
private final value : Lnet/minecraft/core/Registry;
public <init>(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/Registry;)V
private static fromMapEntry(Ljava/util/Map$Entry;)Lnet/minecraft/core/RegistryAccess$RegistryEntry;
private static fromUntyped(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/Registry;)Lnet/minecraft/core/RegistryAccess$RegistryEntry;
private freeze()Lnet/minecraft/core/RegistryAccess$RegistryEntry;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public key()Lnet/minecraft/resources/ResourceKey;
public value()Lnet/minecraft/core/Registry;
```
