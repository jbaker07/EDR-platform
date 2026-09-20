---
type: "interface"
fqcn: "com.mojang.brigadier.tree.CommandNode"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.brigadier.tree.CommandNode

Package `com.mojang.brigadier.tree`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `addChild` | `(Lcom/mojang/brigadier/tree/CommandNode;)V` | exact | invokevirtual@131 in `ClientCommandInternals.copyChildren` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `canUse` | `(Ljava/lang/Object;)Z` | exact | invokevirtual@36 in `ClientCommandInternals.copyChildren` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `createBuilder` | `()Lcom/mojang/brigadier/builder/ArgumentBuilder;` | exact | invokevirtual@47 in `ClientCommandInternals.copyChildren` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getChildren` | `()Ljava/util/Collection;` | exact | invokevirtual@1 in `ClientCommandInternals.copyChildren` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getChildren` | `()Ljava/util/Collection;` | exact | invokevirtual@136 in `ClientCommandInternals.copyChildren` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
