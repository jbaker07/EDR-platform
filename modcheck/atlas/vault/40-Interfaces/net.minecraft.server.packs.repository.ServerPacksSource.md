---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.ServerPacksSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.ServerPacksSource

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`class` public; extends `net/minecraft/server/packs/repository/BuiltInPackSource`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/validation/DirectoryValidator;)V` | exact | invokespecial@26 in `ModPackResourcesUtil.createModdedRepository` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (7 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final VERSION_METADATA_SECTION : Lnet/minecraft/server/packs/metadata/pack/PackMetadataSection;
private static final FEATURE_FLAGS_METADATA_SECTION : Lnet/minecraft/server/packs/FeatureFlagsMetadataSection;
private static final BUILT_IN_METADATA : Lnet/minecraft/server/packs/resources/ResourceMetadata;
private static final VANILLA_PACK_INFO : Lnet/minecraft/server/packs/PackLocationInfo;
private static final VANILLA_SELECTION_CONFIG : Lnet/minecraft/server/packs/PackSelectionConfig;
private static final FEATURE_SELECTION_CONFIG : Lnet/minecraft/server/packs/PackSelectionConfig;
private static final PACKS_DIR : Lnet/minecraft/resources/Identifier;
public <init>(Lnet/minecraft/world/level/validation/DirectoryValidator;)V
private static createBuiltInPackLocation(Ljava/lang/String;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/server/packs/PackLocationInfo;
public static createVanillaPackSource()Lnet/minecraft/server/packs/VanillaPackResources;
protected getPackTitle(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
protected createVanillaPack(Lnet/minecraft/server/packs/repository/Pack$ResourcesSupplier;)Lnet/minecraft/server/packs/repository/Pack;
protected createBuiltinPack(Ljava/lang/String;Lnet/minecraft/server/packs/repository/Pack$ResourcesSupplier;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/server/packs/repository/Pack;
public static createPackRepository(Ljava/nio/file/Path;Lnet/minecraft/world/level/validation/DirectoryValidator;)Lnet/minecraft/server/packs/repository/PackRepository;
public static createVanillaTrustedRepository()Lnet/minecraft/server/packs/repository/PackRepository;
public static createPackRepository(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;)Lnet/minecraft/server/packs/repository/PackRepository;
private static synthetic lambda$createVanillaTrustedRepository$0(Ljava/nio/file/Path;)Z
static <clinit>()V
```
