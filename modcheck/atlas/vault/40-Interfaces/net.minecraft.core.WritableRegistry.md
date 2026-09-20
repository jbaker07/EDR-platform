---
type: "interface"
fqcn: "net.minecraft.core.WritableRegistry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.WritableRegistry

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/core/Registry`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `bindTags` | `(Ljava/util/Map;)V` | exact | invokeinterface@31 in `TagLoaderMixin.loadTagsForRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | inherited_exact | invokeinterface@45 in `FabricGameTestModInitializer.registerDynamicEntries` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | inherited_exact | invokeinterface@4 in `FabricRegistryBuilder.buildAndRegister` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | inherited_exact | invokeinterface@65 in `RegistryDataLoaderMixin.beforeLoad` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `register` | `(Lnet/minecraft/resources/ResourceKey;Ljava/lang/Object;Lnet/minecraft` | exact | invokeinterface@62 in `FabricRegistryBuilder.buildAndRegister` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (0 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract register(Lnet/minecraft/resources/ResourceKey;Ljava/lang/Object;Lnet/minecraft/core/RegistrationInfo;)Lnet/minecraft/core/Holder$Reference;
public abstract bindTags(Ljava/util/Map;)V
public abstract isEmpty()Z
public abstract createRegistrationLookup()Lnet/minecraft/core/HolderGetter;
```
