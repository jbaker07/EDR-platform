---
type: "interface"
fqcn: "com.mojang.brigadier.arguments.StringArgumentType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.brigadier.arguments.StringArgumentType

Package `com.mojang.brigadier.arguments`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getString` | `(Lcom/mojang/brigadier/context/CommandContext;Ljava/lang/String;)Ljava` | exact | invokestatic@6 in `ClientCommandInternals.executeArgumentHelp` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `greedyString` | `()Lcom/mojang/brigadier/arguments/StringArgumentType;` | exact | invokestatic@36 in `ClientCommandInternals.finalizeInit` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
