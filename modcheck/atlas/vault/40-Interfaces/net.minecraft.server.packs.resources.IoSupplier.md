---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.IoSupplier"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.IoSupplier

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `create` | `(Ljava/nio/file/Path;)Lnet/minecraft/server/packs/resources/IoSupplier` | exact | invokestatic@85 in `ModNioPackResources$1.visitFile` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `create` | `(Ljava/nio/file/Path;)Lnet/minecraft/server/packs/resources/IoSupplier` | exact | invokestatic@19 in `ModNioPackResources.getResource` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `get` | `()Ljava/lang/Object;` | exact | invokeinterface@13 in `ModNioPackResources.getMetadataSection` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (0 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static create(Ljava/nio/file/Path;)Lnet/minecraft/server/packs/resources/IoSupplier;
public static create(Ljava/util/zip/ZipFile;Ljava/util/zip/ZipEntry;)Lnet/minecraft/server/packs/resources/IoSupplier;
public abstract get()Ljava/lang/Object;
private static synthetic lambda$create$1(Ljava/util/zip/ZipFile;Ljava/util/zip/ZipEntry;)Ljava/io/InputStream;
private static synthetic lambda$create$0(Ljava/nio/file/Path;)Ljava/io/InputStream;
```
