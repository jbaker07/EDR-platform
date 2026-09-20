---
type: "mechanism"
module: "fabric-game-rule-api-v1"
version: "4.0.10+3434d6d95d"
sha256: "58266b2e28d20444584f22f1940b99656437728bd45e1b226c4869637e704638"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-game-rule-api-v1

**Version** `4.0.10+3434d6d95d` -- **artifact sha256** `58266b2e28d20444584f22f1940b99656437728bd45e1b226c4869637e704638`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `null`
- mixin configs: `["fabric-game-rule-api-v1.mixins.json", {"config": "fabric-game-rule-api-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-game-rule-api-v1.classtweaker`

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `onGameRuleChanged` | injects_into `@Inject at RETURN` | both | `MinecraftServerMixin.handleGameRuleUpdate` |
| [[40-Interfaces/net.minecraft.server.commands.GameRuleCommand_1|GameRuleCommand$1]] | `visit` | injects_into `@Inject at HEAD` | both | `GameRuleCommandVisitorMixin.onRegisterCommand` |
| [[40-Interfaces/net.minecraft.server.jsonrpc.methods.GameRulesService_GameRuleUpdate|GameRulesService$GameRuleUpdate]] | `<init>` | injects_into `@Inject at RETURN` | both | `GameRulesServiceGameRuleUpdateMixin.updateFabricType` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.gamerule.v1.FabricGameRuleTypeVisitor|FabricGameRuleTypeVisitor]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder|GameRuleBuilder]] (class, 21 members)
- [[40-Interfaces/net.fabricmc.fabric.api.gamerule.v1.GameRuleEvents|GameRuleEvents]] (class, 1 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
