---
type: "interface"
fqcn: "net.minecraft.data.PackOutput$PathProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.PackOutput$PathProvider

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `file` | `(Lnet/minecraft/resources/Identifier;Ljava/lang/String;)Ljava/nio/file` | exact | invokevirtual@17 in `FabricCodecDataProvider.lambda$write$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `json` | `(Lnet/minecraft/resources/Identifier;)Ljava/nio/file/Path;` | exact | invokevirtual@8 in `FabricAdvancementProvider.getOutputPath` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `json` | `(Lnet/minecraft/resources/Identifier;)Ljava/nio/file/Path;` | exact | invokevirtual@157 in `FabricDynamicRegistryProvider.writeHolders` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `json` | `(Lnet/minecraft/resources/Identifier;)Ljava/nio/file/Path;` | exact | invokevirtual@23 in `FabricLanguageProvider.getLangFilePath` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `json` | `(Lnet/minecraft/resources/Identifier;)Ljava/nio/file/Path;` | exact | invokevirtual@53 in `FabricRecipeProvider.lambda$run$2` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `json` | `(Lnet/minecraft/resources/Identifier;)Ljava/nio/file/Path;` | exact | invokevirtual@53 in `FabricRecipeProvider.lambda$run$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `json` | `(Lnet/minecraft/resources/Identifier;)Ljava/nio/file/Path;` | exact | invokevirtual@2 in `TagAliasGenerator.writeTagAlias` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `json` | `(Lnet/minecraft/resources/Identifier;)Ljava/nio/file/Path;` | exact | invokevirtual@8 in `FabricLootTableProviderImpl.getOutputPath` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final root : Ljava/nio/file/Path;
private final kind : Ljava/lang/String;
private <init>(Lnet/minecraft/data/PackOutput;Lnet/minecraft/data/PackOutput$Target;Ljava/lang/String;)V
public file(Lnet/minecraft/resources/Identifier;Ljava/lang/String;)Ljava/nio/file/Path;
public json(Lnet/minecraft/resources/Identifier;)Ljava/nio/file/Path;
public json(Lnet/minecraft/resources/ResourceKey;)Ljava/nio/file/Path;
private synthetic lambda$json$0(Ljava/lang/String;)Ljava/lang/String;
private synthetic lambda$file$0(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
```
