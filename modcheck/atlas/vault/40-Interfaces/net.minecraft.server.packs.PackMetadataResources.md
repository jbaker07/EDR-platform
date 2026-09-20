---
type: "interface"
fqcn: "net.minecraft.server.packs.PackMetadataResources"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.PackMetadataResources

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`interface` public abstract; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getMetadataSection` | `(Lnet/minecraft/server/packs/metadata/MetadataSectionType;)Ljava/lang/` | exact | invokeinterface@13 in `PackMixin.applyOverlayConditions` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (0 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract location()Lnet/minecraft/server/packs/PackLocationInfo;
public abstract getRootResource([Ljava/lang/String;)Lnet/minecraft/server/packs/resources/IoSupplier;
public abstract getMetadataSection(Lnet/minecraft/server/packs/metadata/MetadataSectionType;)Ljava/lang/Object;
public abstract close()V
```
