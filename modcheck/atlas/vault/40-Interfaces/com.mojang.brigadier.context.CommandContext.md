---
type: "interface"
fqcn: "com.mojang.brigadier.context.CommandContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.brigadier.context.CommandContext

Package `com.mojang.brigadier.context`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getNodes` | `()Ljava/util/List;` | exact | invokevirtual@51 in `CommandQueueEntryMixin.onExecute` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getNodes` | `()Ljava/util/List;` | exact | invokevirtual@1 in `ContextChainMixin.onRunExecutable` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getSource` | `()Ljava/lang/Object;` | exact | invokevirtual@10 in `ClientCommandInternals.executeArgumentHelp` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getSource` | `()Ljava/lang/Object;` | exact | invokevirtual@5 in `ClientCommandInternals.executeHelp` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getSource` | `()Ljava/lang/Object;` | exact | invokevirtual@48 in `ClientCommandInternals.executeHelp` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getSource` | `()Ljava/lang/Object;` | exact | invokevirtual@1 in `EnumRuleCommand.executeAndSetEnum` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getSource` | `()Ljava/lang/Object;` | exact | invokevirtual@1 in `EnumRuleCommand.lambda$register$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
