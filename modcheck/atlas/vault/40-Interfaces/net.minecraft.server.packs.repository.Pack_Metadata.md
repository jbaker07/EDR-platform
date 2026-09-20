---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.Pack$Metadata"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.Pack$Metadata

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `overlays` | `()Ljava/util/List;` | exact | invokevirtual@1 in `ResourceLoaderImpl$1.openResources` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `overlays` | `()Ljava/util/List;` | exact | invokevirtual@25 in `ResourceLoaderImpl$1.openResources` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `overlays` | `()Ljava/util/List;` | exact | invokevirtual@38 in `ResourceLoaderImpl$1.openResources` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `overlays` | `()Ljava/util/List;` | exact | invokevirtual@1 in `ModPackResourcesFactory.openResources` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `overlays` | `()Ljava/util/List;` | exact | invokevirtual@25 in `ModPackResourcesFactory.openResources` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `overlays` | `()Ljava/util/List;` | exact | invokevirtual@38 in `ModPackResourcesFactory.openResources` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (4 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final description : Lnet/minecraft/network/chat/Component;
private final compatibility : Lnet/minecraft/server/packs/repository/PackCompatibility;
private final requestedFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
private final overlays : Ljava/util/List;
public <init>(Lnet/minecraft/network/chat/Component;Lnet/minecraft/server/packs/repository/PackCompatibility;Lnet/minecraft/world/flag/FeatureFlagSet;Ljava/util/List;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public description()Lnet/minecraft/network/chat/Component;
public compatibility()Lnet/minecraft/server/packs/repository/PackCompatibility;
public requestedFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
public overlays()Ljava/util/List;
```
