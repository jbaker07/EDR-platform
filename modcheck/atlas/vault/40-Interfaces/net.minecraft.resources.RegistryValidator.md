---
type: "interface"
fqcn: "net.minecraft.resources.RegistryValidator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryValidator

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `none` | `()Lnet/minecraft/resources/RegistryValidator;` | exact | invokestatic@61 in `DynamicRegistriesImpl.register` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `none` | `()Lnet/minecraft/resources/RegistryValidator;` | exact | invokestatic@52 in `DynamicRegistriesImpl.addSyncedRegistry` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `none` | `()Lnet/minecraft/resources/RegistryValidator;` | exact | invokestatic@61 in `DynamicRegistriesImpl.registerReloadable` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NONE : Lnet/minecraft/resources/RegistryValidator;
public static final NON_EMPTY : Lnet/minecraft/resources/RegistryValidator;
public static none()Lnet/minecraft/resources/RegistryValidator;
public static nonEmpty()Lnet/minecraft/resources/RegistryValidator;
public abstract validate(Lnet/minecraft/core/Registry;Ljava/util/Map;)V
private static synthetic lambda$static$1(Lnet/minecraft/core/Registry;Ljava/util/Map;)V
private static synthetic lambda$static$0(Lnet/minecraft/core/Registry;Ljava/util/Map;)V
static <clinit>()V
```
