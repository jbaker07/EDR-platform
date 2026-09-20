---
type: "interface"
fqcn: "net.minecraft.WorldVersion"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.WorldVersion

System: [[20-Systems/net.minecraft|net.minecraft]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `packVersion` | `(Lnet/minecraft/server/packs/PackType;)Lnet/minecraft/server/packs/met` | exact | invokeinterface@118 in `ModPackResourcesUtil.openDefault` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (0 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract dataVersion()Lnet/minecraft/world/level/storage/DataVersion;
public abstract id()Ljava/lang/String;
public abstract name()Ljava/lang/String;
public abstract protocolVersion()I
public abstract packVersion(Lnet/minecraft/server/packs/PackType;)Lnet/minecraft/server/packs/metadata/pack/PackFormat;
public abstract buildTime()Ljava/util/Date;
public abstract stable()Z
```
