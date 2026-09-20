---
type: "interface"
fqcn: "net.minecraft.server.commands.GameRuleCommand$1"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.commands.GameRuleCommand$1

System: [[20-Systems/net.minecraft.server.commands|net.minecraft.server.commands]]

`class` ; extends `java/lang/Object`; implements `net/minecraft/world/level/gamerules/GameRuleTypeVisitor`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `visit` | `(Lnet/minecraft/world/level/gamerules/GameRule;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| reads | `val$base` | `Lcom/mojang/brigadier/builder/LiteralArgumentBuilder;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | declared |

## Declared members (1 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
final synthetic val$base : Lcom/mojang/brigadier/builder/LiteralArgumentBuilder;
 <init>(Lcom/mojang/brigadier/builder/LiteralArgumentBuilder;)V
public visit(Lnet/minecraft/world/level/gamerules/GameRule;)V
```
