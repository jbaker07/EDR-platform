---
type: "interface"
fqcn: "com.mojang.brigadier.exceptions.BuiltInExceptionProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.brigadier.exceptions.BuiltInExceptionProvider

Package `com.mojang.brigadier.exceptions`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `dispatcherParseException` | `()Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;` | exact | invokeinterface@16 in `ClientCommandInternals.isIgnoredException` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `dispatcherUnknownCommand` | `()Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType;` | exact | invokeinterface@6 in `ClientCommandInternals.isIgnoredException` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
