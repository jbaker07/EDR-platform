---
type: "interface"
fqcn: "com.mojang.brigadier.tree.LiteralCommandNode"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.brigadier.tree.LiteralCommandNode

Package `com.mojang.brigadier.tree`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `addChild` | `(Lcom/mojang/brigadier/tree/CommandNode;)V` | inherited_exact | invokevirtual@95 in `EnumRuleCommand.register` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getName` | `()Ljava/lang/String;` | exact | invokevirtual@108 in `CommandQueueEntryMixin.onExecute` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getName` | `()Ljava/lang/String;` | exact | invokevirtual@79 in `ContextChainMixin.onRunExecutable` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
