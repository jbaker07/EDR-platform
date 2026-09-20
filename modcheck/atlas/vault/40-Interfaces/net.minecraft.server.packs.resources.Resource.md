---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.Resource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.Resource

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/resource/v1/FabricResource`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getFabricPackSource` | `()Lnet/minecraft/server/packs/repository/PackSource;` | inherited_exact | invokevirtual@5 in `AdvancementUtil.determineSource` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `getFabricPackSource` | `()Lnet/minecraft/server/packs/repository/PackSource;` | inherited_exact | invokevirtual@5 in `ResourceUtil.determineSource` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getFabricPackSource` | `()Lnet/minecraft/server/packs/repository/PackSource;` | inherited_exact | invokevirtual@5 in `LootUtil.determineSource` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `openAsReader` | `()Ljava/io/BufferedReader;` | exact | invokevirtual@33 in `StructureTemplateManagerMixin$1.load` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `openAsReader` | `()Ljava/io/BufferedReader;` | exact | invokevirtual@138 in `TagAliasLoader.prepare` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `source` | `()Lnet/minecraft/server/packs/PackResources;` | exact | invokevirtual@6 in `ResourceMixin.getFabricPackSource` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (4 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final source : Lnet/minecraft/server/packs/PackResources;
private final streamSupplier : Lnet/minecraft/server/packs/resources/IoSupplier;
private final metadataSupplier : Lnet/minecraft/server/packs/resources/IoSupplier;
private cachedMetadata : Lnet/minecraft/server/packs/resources/ResourceMetadata;
public <init>(Lnet/minecraft/server/packs/PackResources;Lnet/minecraft/server/packs/resources/IoSupplier;Lnet/minecraft/server/packs/resources/IoSupplier;)V
public <init>(Lnet/minecraft/server/packs/PackResources;Lnet/minecraft/server/packs/resources/IoSupplier;)V
public source()Lnet/minecraft/server/packs/PackResources;
public sourcePackId()Ljava/lang/String;
public knownPackInfo()Ljava/util/Optional;
public open()Ljava/io/InputStream;
public openAsReader()Ljava/io/BufferedReader;
public readAllAsString()Ljava/lang/String;
public metadata()Lnet/minecraft/server/packs/resources/ResourceMetadata;
```
