---
type: "interface"
fqcn: "net.minecraft.data.PackOutput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.PackOutput

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/nio/file/Path;)V` | exact | invokespecial@2 in `FabricPackOutput.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `createPathProvider` | `(Lnet/minecraft/data/PackOutput$Target;Ljava/lang/String;)Lnet/minecra` | exact | invokevirtual@9 in `TagsProviderMixin.initPathResolver` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getOutputFolder` | `()Ljava/nio/file/Path;` | exact | invokevirtual@4 in `FabricDataGenerator.createBuiltinResourcePack` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getOutputFolder` | `(Lnet/minecraft/data/PackOutput$Target;)Ljava/nio/file/Path;` | exact | invokevirtual@7 in `FabricSoundsProvider.lambda$run$3` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (1 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final outputFolder : Ljava/nio/file/Path;
public <init>(Ljava/nio/file/Path;)V
public getOutputFolder()Ljava/nio/file/Path;
public getOutputFolder(Lnet/minecraft/data/PackOutput$Target;)Ljava/nio/file/Path;
public createPathProvider(Lnet/minecraft/data/PackOutput$Target;Ljava/lang/String;)Lnet/minecraft/data/PackOutput$PathProvider;
public createRegistryElementsPathProvider(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/PackOutput$PathProvider;
public createRegistryTagsPathProvider(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/PackOutput$PathProvider;
public createRegistryComponentPathProvider(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/PackOutput$PathProvider;
```
