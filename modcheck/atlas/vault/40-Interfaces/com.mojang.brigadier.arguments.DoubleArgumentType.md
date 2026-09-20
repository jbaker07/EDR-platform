---
type: "interface"
fqcn: "com.mojang.brigadier.arguments.DoubleArgumentType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.brigadier.arguments.DoubleArgumentType

Package `com.mojang.brigadier.arguments`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `doubleArg` | `()Lcom/mojang/brigadier/arguments/DoubleArgumentType;` | exact | invokestatic@25 in `GameRuleBuilder$DoubleRuleBuilder.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `doubleArg` | `(DD)Lcom/mojang/brigadier/arguments/DoubleArgumentType;` | exact | invokestatic@65 in `GameRuleBuilder$DoubleRuleBuilder.range` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
