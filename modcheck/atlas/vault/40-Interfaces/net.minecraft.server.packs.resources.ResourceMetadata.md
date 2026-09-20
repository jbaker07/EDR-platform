---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.ResourceMetadata"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.ResourceMetadata

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `fromJsonStream` | `(Ljava/io/InputStream;)Lnet/minecraft/server/packs/resources/ResourceM` | exact | invokestatic@23 in `ModNioPackResources.getMetadataSection` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getSection` | `(Lnet/minecraft/server/packs/metadata/MetadataSectionType;)Ljava/util/` | exact | invokeinterface@29 in `ModNioPackResources.getMetadataSection` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (2 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/server/packs/resources/ResourceMetadata;
public static final EMPTY_SUPPLIER : Lnet/minecraft/server/packs/resources/IoSupplier;
public static fromJsonStream(Ljava/io/InputStream;)Lnet/minecraft/server/packs/resources/ResourceMetadata;
public abstract getSection(Lnet/minecraft/server/packs/metadata/MetadataSectionType;)Ljava/util/Optional;
public getTypedSection(Lnet/minecraft/server/packs/metadata/MetadataSectionType;)Ljava/util/Optional;
public static of(Lnet/minecraft/server/packs/metadata/MetadataSectionType;Ljava/lang/Object;)Lnet/minecraft/server/packs/resources/ResourceMetadata;
public static of(Lnet/minecraft/server/packs/metadata/MetadataSectionType;Ljava/lang/Object;Lnet/minecraft/server/packs/metadata/MetadataSectionType;Ljava/lang/Object;)Lnet/minecraft/server/packs/resources/ResourceMetadata;
public getTypedSections(Ljava/util/Collection;)Ljava/util/List;
private static synthetic lambda$static$0()Lnet/minecraft/server/packs/resources/ResourceMetadata;
static <clinit>()V
```
