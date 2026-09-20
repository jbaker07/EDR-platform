---
type: "interface"
fqcn: "com.mojang.brigadier.builder.RequiredArgumentBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.brigadier.builder.RequiredArgumentBuilder

Package `com.mojang.brigadier.builder`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `argument` | `(Ljava/lang/String;Lcom/mojang/brigadier/arguments/ArgumentType;)Lcom/` | exact | invokestatic@2 in `ClientCommands.argument` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `executes` | `(Lcom/mojang/brigadier/Command;)Lcom/mojang/brigadier/builder/Argument` | inherited_exact | invokevirtual@47 in `ClientCommandInternals.finalizeInit` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
