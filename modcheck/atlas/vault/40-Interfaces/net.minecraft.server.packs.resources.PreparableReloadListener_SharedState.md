---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.PreparableReloadListener$SharedState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.PreparableReloadListener$SharedState

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `get` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$StateK` | exact | invokevirtual@4 in `ResourceManagerHelperImpl$1.reload` | unknown | [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$StateK` | exact | invokevirtual@12 in `TagAliasLoader.prepare` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$StateK` | exact | invokevirtual@249 in `TagAliasLoader.apply` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `resourceManager` | `()Lnet/minecraft/server/packs/resources/ResourceManager;` | exact | invokevirtual@2 in `SimpleResourceReloadListener.reload` | unknown | [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] | direct_reference |
| calls | `resourceManager` | `()Lnet/minecraft/server/packs/resources/ResourceManager;` | exact | invokevirtual@3 in `SimpleResourceReloadListener.lambda$reload$0` | unknown | [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] | direct_reference |
| calls | `resourceManager` | `()Lnet/minecraft/server/packs/resources/ResourceManager;` | exact | invokevirtual@67 in `TagAliasLoader.prepare` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$StateK` | exact | invokevirtual@8 in `SetupMarkerResourceReloader.prepareSharedState` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$StateK` | exact | invokevirtual@19 in `SetupMarkerResourceReloader.prepareSharedState` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$StateK` | exact | invokevirtual@38 in `SetupMarkerResourceReloader.prepareSharedState` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (2 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final manager : Lnet/minecraft/server/packs/resources/ResourceManager;
private final state : Ljava/util/Map;
public <init>(Lnet/minecraft/server/packs/resources/ResourceManager;)V
public resourceManager()Lnet/minecraft/server/packs/resources/ResourceManager;
public set(Lnet/minecraft/server/packs/resources/PreparableReloadListener$StateKey;Ljava/lang/Object;)V
public get(Lnet/minecraft/server/packs/resources/PreparableReloadListener$StateKey;)Ljava/lang/Object;
```
