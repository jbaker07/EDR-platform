---
type: "interface"
fqcn: "net.minecraft.resources.FileToIdConverter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.FileToIdConverter

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/String;Ljava/lang/String;)V` | exact | invokespecial@32 in `FabricGameTestRunner.<clinit>` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `fileToId` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Identif` | exact | invokevirtual@123 in `TagAliasLoader.prepare` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `idToFile` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Identif` | exact | invokevirtual@4 in `StructureTemplateManagerMixin$1.load` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `json` | `(Ljava/lang/String;)Lnet/minecraft/resources/FileToIdConverter;` | exact | invokestatic@59 in `TagAliasLoader.prepare` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `listMatchingResources` | `(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljava/util/Map` | exact | invokevirtual@9 in `StructureTemplateManagerMixin$1.list` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `listMatchingResources` | `(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljava/util/Map` | exact | invokevirtual@70 in `TagAliasLoader.prepare` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `prefix` | `()Ljava/lang/String;` | exact | invokevirtual@50 in `SimpleJsonResourceReloadListenerMixin.applyResourceConditions` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (2 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final prefix : Ljava/lang/String;
private final extension : Ljava/lang/String;
public <init>(Ljava/lang/String;Ljava/lang/String;)V
public static json(Ljava/lang/String;)Lnet/minecraft/resources/FileToIdConverter;
public static registry(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/resources/FileToIdConverter;
public idToFile(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Identifier;
public fileToId(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Identifier;
public extensionMatches(Lnet/minecraft/resources/Identifier;)Z
public prefixMatches(Lnet/minecraft/resources/Identifier;)Z
public matches(Lnet/minecraft/resources/Identifier;)Z
private extensionSelector()Lnet/minecraft/server/packs/resources/ResourceManager$Selector;
public listMatchingResources(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljava/util/Map;
public listMatchingResourceStacks(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljava/util/Map;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public prefix()Ljava/lang/String;
public extension()Ljava/lang/String;
```
