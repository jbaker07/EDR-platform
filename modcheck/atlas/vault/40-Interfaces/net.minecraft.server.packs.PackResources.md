---
type: "interface"
fqcn: "net.minecraft.server.packs.PackResources"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.PackResources

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/server/packs/PackMetadataResources`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `close` | `()V` | inherited_exact | invokeinterface@75 in `DefaultResourcePackStorage.updateTrackedPack` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `close` | `()V` | inherited_exact | invokeinterface@92 in `DefaultResourcePackStorage.updateTrackedPack` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `close` | `()V` | inherited_exact | invokeinterface@35 in `ModPackResourcesUtil.isEnabledByDefault` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `close` | `()V` | inherited_exact | invokeinterface@48 in `ModPackResourcesUtil.isEnabledByDefault` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `close` | `()V` | inherited_exact | invokeinterface@135 in `MinecraftServerMixin.onCheckDisabled` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `close` | `()V` | inherited_exact | invokeinterface@152 in `MinecraftServerMixin.onCheckDisabled` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/server/packs/PackLocationInfo;` | inherited_exact | invokeinterface@1 in `MinecraftServerMixin.lambda$init$0` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (2 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final METADATA_EXTENSION : Ljava/lang/String;
public static final PACK_META : Ljava/lang/String;
public abstract getResource(Lnet/minecraft/server/packs/PackType;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/server/packs/resources/IoSupplier;
public abstract listResources(Lnet/minecraft/server/packs/PackType;Ljava/lang/String;Ljava/lang/String;Lnet/minecraft/server/packs/PackResources$ResourceOutput;)V
public abstract getNamespaces(Lnet/minecraft/server/packs/PackType;)Ljava/util/Set;
public packId()Ljava/lang/String;
public knownPackInfo()Ljava/util/Optional;
```
