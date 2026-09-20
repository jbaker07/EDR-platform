---
type: "interface"
fqcn: "net.minecraft.server.packs.PackType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.PackType

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getDirectory` | `()Ljava/lang/String;` | exact | invokevirtual@3 in `ServerLanguageUtil.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getDirectory` | `()Ljava/lang/String;` | exact | invokevirtual@71 in `ModNioPackResources.readNamespaces` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getDirectory` | `()Ljava/lang/String;` | exact | invokevirtual@74 in `ModNioPackResources.listResources` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getDirectory` | `()Ljava/lang/String;` | exact | invokevirtual@13 in `ModNioPackResources.getFilename` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getDirectory` | `()Ljava/lang/String;` | exact | invokevirtual@26 in `ModNioPackResources.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getDirectory` | `()Ljava/lang/String;` | exact | invokevirtual@40 in `ModNioPackResources.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/server/packs/PackType;` | exact | invokestatic@10 in `ModNioPackResources.readNamespaces` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `CLIENT_RESOURCES` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@4 in `ResourceLoaderImpl.extractSetupMarker` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `CLIENT_RESOURCES` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@44 in `ResourceLoaderImpl.registerBuiltinPack` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `CLIENT_RESOURCES` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@85 in `ServerLanguageUtil.getModLanguageFiles` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `CLIENT_RESOURCES` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@0 in `ServerLanguageUtil.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `CLIENT_RESOURCES` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@17 in `ModNioPackResources.hasAbsentNs` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `CLIENT_RESOURCES` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@23 in `ModNioPackResources.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `CLIENT_RESOURCES` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@13 in `ModResourcePackCreator.loadPacks` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `CLIENT_RESOURCES` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@50 in `ModResourcePackCreator.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATA` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@1 in `DataResourceLoaderImpl.<init>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATA` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@61 in `ResourceLoaderImpl.registerBuiltinPack` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATA` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@1 in `ResourceLoaderImpl.lambda$get$0` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATA` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@41 in `ModNioPackResources.hasAbsentNs` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATA` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@37 in `ModNioPackResources.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATA` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@4 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATA` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@12 in `ModPackResourcesUtil.createTestServerSettings` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATA` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@36 in `ModPackResourcesUtil.createModdedRepository` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATA` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@106 in `PackRepositoryMixin.construct` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATA` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@8 in `CreateWorldScreenMixin.onCreateResManagerInit` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATA` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@11 in `CreateWorldScreenMixin.onScanPacks` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER_DATA` | `Lnet/minecraft/server/packs/PackType;` | exact | getstatic@0 in `TagInit.onInitialize` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (4 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CLIENT_RESOURCES : Lnet/minecraft/server/packs/PackType;
public static final SERVER_DATA : Lnet/minecraft/server/packs/PackType;
private final directory : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/server/packs/PackType;
public static values()[Lnet/minecraft/server/packs/PackType;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/server/packs/PackType;
private <init>(Ljava/lang/String;ILjava/lang/String;)V
public getDirectory()Ljava/lang/String;
private static synthetic $values()[Lnet/minecraft/server/packs/PackType;
static <clinit>()V
```
