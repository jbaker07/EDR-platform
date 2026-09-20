---
type: "interface"
fqcn: "com.mojang.brigadier.exceptions.CommandSyntaxException"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.brigadier.exceptions.CommandSyntaxException

Package `com.mojang.brigadier.exceptions`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getContext` | `()Ljava/lang/String;` | exact | invokevirtual@9 in `ClientCommandInternals.getErrorMessage` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getCursor` | `()I` | exact | invokevirtual@30 in `ClientCommandInternals.getErrorMessage` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getRawMessage` | `()Lcom/mojang/brigadier/Message;` | exact | invokevirtual@1 in `ClientCommandInternals.getErrorMessage` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getType` | `()Lcom/mojang/brigadier/exceptions/CommandExceptionType;` | exact | invokevirtual@70 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `BUILT_IN_EXCEPTIONS` | `Lcom/mojang/brigadier/exceptions/BuiltInExceptionProvider;` | exact | getstatic@0 in `ClientCommandInternals.isIgnoredException` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
