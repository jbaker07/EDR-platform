---
type: "interface"
fqcn: "com.mojang.brigadier.builder.ArgumentBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.brigadier.builder.ArgumentBuilder

Package `com.mojang.brigadier.builder`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `build` | `()Lcom/mojang/brigadier/tree/CommandNode;` | exact | invokevirtual@112 in `ClientCommandInternals.copyChildren` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `executes` | `(Lcom/mojang/brigadier/Command;)Lcom/mojang/brigadier/builder/Argument` | exact | invokevirtual@78 in `ClientCommandInternals.copyChildren` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getCommand` | `()Lcom/mojang/brigadier/Command;` | exact | invokevirtual@65 in `ClientCommandInternals.copyChildren` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getRedirect` | `()Lcom/mojang/brigadier/tree/CommandNode;` | exact | invokevirtual@84 in `ClientCommandInternals.copyChildren` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getRedirect` | `()Lcom/mojang/brigadier/tree/CommandNode;` | exact | invokevirtual@95 in `ClientCommandInternals.copyChildren` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `redirect` | `(Lcom/mojang/brigadier/tree/CommandNode;)Lcom/mojang/brigadier/builder` | exact | invokevirtual@106 in `ClientCommandInternals.copyChildren` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `requires` | `(Ljava/util/function/Predicate;)Lcom/mojang/brigadier/builder/Argument` | exact | invokevirtual@59 in `ClientCommandInternals.copyChildren` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
