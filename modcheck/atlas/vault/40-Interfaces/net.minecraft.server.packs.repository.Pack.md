---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.Pack"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.Pack

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getId` | `()Ljava/lang/String;` | exact | invokevirtual@44 in `DefaultResourcePackStorage.updateTrackedPack` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getId` | `()Ljava/lang/String;` | exact | invokevirtual@112 in `ModPackResourcesUtil.refreshAutoEnabledPacks` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getId` | `()Ljava/lang/String;` | exact | invokevirtual@187 in `ModPackResourcesUtil.refreshAutoEnabledPacks` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getId` | `()Ljava/lang/String;` | exact | invokevirtual@220 in `ModPackResourcesUtil.refreshAutoEnabledPacks` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getId` | `()Ljava/lang/String;` | exact | invokevirtual@228 in `ModPackResourcesUtil.refreshAutoEnabledPacks` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getId` | `()Ljava/lang/String;` | exact | invokevirtual@104 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getId` | `()Ljava/lang/String;` | exact | invokevirtual@225 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getId` | `()Ljava/lang/String;` | exact | invokevirtual@238 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getId` | `()Ljava/lang/String;` | exact | invokevirtual@2 in `ModPackResourcesUtil.lambda$createTestServerSettings$0` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getId` | `()Ljava/lang/String;` | exact | invokevirtual@18 in `DataPackCommandMixin.errorOnInternalPack` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getPackSource` | `()Lnet/minecraft/server/packs/repository/PackSource;` | exact | invokevirtual@92 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getPackSource` | `()Lnet/minecraft/server/packs/repository/PackSource;` | exact | invokevirtual@30 in `MinecraftServerMixin.onCheckDisabled` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/server/packs/PackLocationInfo;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| calls | `open` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@149 in `DefaultResourcePackStorage.process` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `open` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@119 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `open` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@47 in `MinecraftServerMixin.onCheckDisabled` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `readMetaAndCreate` | `(Lnet/minecraft/server/packs/PackLocationInfo;Lnet/minecraft/server/pa` | exact | invokestatic@131 in `ResourceLoaderImpl.registerBuiltinResourcePacks` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `readMetaAndCreate` | `(Lnet/minecraft/server/packs/PackLocationInfo;Lnet/minecraft/server/pa` | exact | invokestatic@68 in `ModResourcePackCreator.registerModPack` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `open` | `()Ljava/util/stream/Stream;` | name_only | @ModifyReturnValue at ['RETURN'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `readPackMetadata` | `(Lnet/minecraft/server/packs/PackLocationInfo;Lnet/minecraft/server/pa` | name_only | @ModifyVariable at ['STORE'] | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (5 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final location : Lnet/minecraft/server/packs/PackLocationInfo;
private final resources : Lnet/minecraft/server/packs/repository/Pack$ResourcesSupplier;
private final metadata : Lnet/minecraft/server/packs/repository/Pack$Metadata;
private final selectionConfig : Lnet/minecraft/server/packs/PackSelectionConfig;
public static readMetaAndCreate(Lnet/minecraft/server/packs/PackLocationInfo;Lnet/minecraft/server/packs/repository/Pack$ResourcesSupplier;Lnet/minecraft/server/packs/PackType;Lnet/minecraft/server/packs/PackSelectionConfig;)Lnet/minecraft/server/packs/repository/Pack;
public <init>(Lnet/minecraft/server/packs/PackLocationInfo;Lnet/minecraft/server/packs/repository/Pack$ResourcesSupplier;Lnet/minecraft/server/packs/repository/Pack$Metadata;Lnet/minecraft/server/packs/PackSelectionConfig;)V
public static readPackMetadata(Lnet/minecraft/server/packs/PackLocationInfo;Lnet/minecraft/server/packs/repository/Pack$ResourcesSupplier;Lnet/minecraft/server/packs/metadata/pack/PackFormat;Lnet/minecraft/server/packs/PackType;)Lnet/minecraft/server/packs/repository/Pack$Metadata;
public location()Lnet/minecraft/server/packs/PackLocationInfo;
public getTitle()Lnet/minecraft/network/chat/Component;
public getDescription()Lnet/minecraft/network/chat/Component;
public getChatLink(Z)Lnet/minecraft/network/chat/Component;
public getCompatibility()Lnet/minecraft/server/packs/repository/PackCompatibility;
public getRequestedFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
public openMetadata()Lnet/minecraft/server/packs/PackMetadataResources;
public open()Ljava/util/stream/Stream;
public getId()Ljava/lang/String;
public selectionConfig()Lnet/minecraft/server/packs/PackSelectionConfig;
public isRequired()Z
public isFixedPosition()Z
public getDefaultPosition()Lnet/minecraft/server/packs/repository/Pack$Position;
public getPackSource()Lnet/minecraft/server/packs/repository/PackSource;
public equals(Ljava/lang/Object;)Z
public hashCode()I
static <clinit>()V
```
