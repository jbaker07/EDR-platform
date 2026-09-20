---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.FolderRepositorySource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.FolderRepositorySource

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/server/packs/repository/RepositorySource`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `packSource` | `Lnet/minecraft/server/packs/repository/PackSource;` | exact | getfield@63 in `PackRepositoryMixin.construct` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `packSource` | `Lnet/minecraft/server/packs/repository/PackSource;` | exact | getfield@77 in `PackRepositoryMixin.construct` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (6 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final DISCOVERED_PACK_SELECTION_CONFIG : Lnet/minecraft/server/packs/PackSelectionConfig;
private final folder : Ljava/nio/file/Path;
private final packType : Lnet/minecraft/server/packs/PackType;
private final packSource : Lnet/minecraft/server/packs/repository/PackSource;
private final validator : Lnet/minecraft/world/level/validation/DirectoryValidator;
public <init>(Ljava/nio/file/Path;Lnet/minecraft/server/packs/PackType;Lnet/minecraft/server/packs/repository/PackSource;Lnet/minecraft/world/level/validation/DirectoryValidator;)V
private static nameFromPath(Ljava/nio/file/Path;)Ljava/lang/String;
public loadPacks(Ljava/util/function/Consumer;)V
private createDiscoveredFilePackInfo(Ljava/nio/file/Path;)Lnet/minecraft/server/packs/PackLocationInfo;
public static discoverPacks(Ljava/nio/file/Path;Lnet/minecraft/world/level/validation/DirectoryValidator;Ljava/util/function/BiConsumer;)V
private synthetic lambda$loadPacks$0(Ljava/util/function/Consumer;Ljava/nio/file/Path;Lnet/minecraft/server/packs/repository/Pack$ResourcesSupplier;)V
static <clinit>()V
```
