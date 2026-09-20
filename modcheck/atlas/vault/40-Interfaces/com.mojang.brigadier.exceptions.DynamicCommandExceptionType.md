---
type: "interface"
fqcn: "com.mojang.brigadier.exceptions.DynamicCommandExceptionType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.brigadier.exceptions.DynamicCommandExceptionType

Package `com.mojang.brigadier.exceptions`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/function/Function;)V` | exact | invokespecial@9 in `DataPackCommandMixin.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `create` | `(Ljava/lang/Object;)Lcom/mojang/brigadier/exceptions/CommandSyntaxExce` | exact | invokevirtual@21 in `DataPackCommandMixin.errorOnInternalPack` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
