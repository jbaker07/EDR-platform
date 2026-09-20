---
type: "interface"
fqcn: "net.minecraft.server.packs.OverlayedPackResources"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.OverlayedPackResources

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/server/packs/PackResources`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/server/packs/PackResources;Ljava/util/List;)V` | exact | invokespecial@98 in `ResourceLoaderImpl$1.openResources` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/server/packs/PackResources;Ljava/util/List;)V` | exact | invokespecial@98 in `ModPackResourcesFactory.openResources` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (2 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final primaryPackMetadataResources : Lnet/minecraft/server/packs/PackMetadataResources;
private final packResourcesStack : Ljava/util/List;
public <init>(Lnet/minecraft/server/packs/PackResources;Ljava/util/List;)V
public getRootResource([Ljava/lang/String;)Lnet/minecraft/server/packs/resources/IoSupplier;
public getResource(Lnet/minecraft/server/packs/PackType;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/server/packs/resources/IoSupplier;
public listResources(Lnet/minecraft/server/packs/PackType;Ljava/lang/String;Ljava/lang/String;Lnet/minecraft/server/packs/PackResources$ResourceOutput;)V
public getNamespaces(Lnet/minecraft/server/packs/PackType;)Ljava/util/Set;
public getMetadataSection(Lnet/minecraft/server/packs/metadata/MetadataSectionType;)Ljava/lang/Object;
public location()Lnet/minecraft/server/packs/PackLocationInfo;
public close()V
```
