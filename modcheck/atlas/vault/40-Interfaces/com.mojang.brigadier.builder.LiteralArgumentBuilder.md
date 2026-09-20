---
type: "interface"
fqcn: "com.mojang.brigadier.builder.LiteralArgumentBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.brigadier.builder.LiteralArgumentBuilder

Package `com.mojang.brigadier.builder`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `build` | `()Lcom/mojang/brigadier/tree/LiteralCommandNode;` | exact | invokevirtual@27 in `EnumRuleCommand.register` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `build` | `()Lcom/mojang/brigadier/tree/LiteralCommandNode;` | exact | invokevirtual@92 in `EnumRuleCommand.register` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `executes` | `(Lcom/mojang/brigadier/Command;)Lcom/mojang/brigadier/builder/Argument` | inherited_exact | invokevirtual@29 in `ClientCommandInternals.finalizeInit` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `executes` | `(Lcom/mojang/brigadier/Command;)Lcom/mojang/brigadier/builder/Argument` | inherited_exact | invokevirtual@16 in `EnumRuleCommand.register` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `executes` | `(Lcom/mojang/brigadier/Command;)Lcom/mojang/brigadier/builder/Argument` | inherited_exact | invokevirtual@86 in `EnumRuleCommand.register` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lcom/mojang/brigadier/builder/LiteralArgumentBuild` | exact | invokestatic@1 in `ClientCommands.literal` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `redirect` | `(Lcom/mojang/brigadier/tree/CommandNode;)Lcom/mojang/brigadier/builder` | inherited_exact | invokevirtual@84 in `ClientCommandInternals.finalizeInit` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `then` | `(Lcom/mojang/brigadier/builder/ArgumentBuilder;)Lcom/mojang/brigadier/` | inherited_exact | invokevirtual@50 in `ClientCommandInternals.finalizeInit` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `then` | `(Lcom/mojang/brigadier/builder/ArgumentBuilder;)Lcom/mojang/brigadier/` | inherited_exact | invokevirtual@64 in `ClientCommandInternals.finalizeInit` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `then` | `(Lcom/mojang/brigadier/builder/ArgumentBuilder;)Lcom/mojang/brigadier/` | inherited_exact | invokevirtual@19 in `EnumRuleCommand.register` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `then` | `(Lcom/mojang/brigadier/tree/CommandNode;)Lcom/mojang/brigadier/builder` | inherited_exact | invokevirtual@103 in `EnumRuleCommand.register` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
