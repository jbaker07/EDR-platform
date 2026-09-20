---
type: "interface"
fqcn: "net.minecraft.server.commands.GameRuleCommand$1"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.commands.GameRuleCommand$1

System: [[20-Systems/net.minecraft.server.commands|net.minecraft.server.commands]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `visit` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (3, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
class net.minecraft.server.commands.GameRuleCommand$1 implements net.minecraft.world.level.gamerules.GameRuleTypeVisitor {
    final com.mojang.brigadier.builder.LiteralArgumentBuilder val$base;
    net.minecraft.server.commands.GameRuleCommand$1();
    public <T> void visit(net.minecraft.world.level.gamerules.GameRule<T>);
}
```
