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
- mixin classes: 8 found by annotation, 8 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen_RuleList_1|AbstractGameRulesScreen$RuleList$1]].`addEntry` | `(Lnet/minecraft/world/level/gamerules/GameRule;Lnet/minecraft/client/gui/screens/worldselection/AbstractGameRulesScreen$EntryFactory;)V` | exact | @WrapOperation | INVOKE `Lnet/minecraft/world/level/gamerules/GameRule;serialize(Ljava/lang/Object;)Ljava/lang/String;` (exact) | client | 1000 (default) | `RuleListEntryTypeVisitorMixin.displayProperEnumName` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`onGameRuleChanged` | `(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `MinecraftServerMixin.handleGameRuleUpdate` |
| [[40-Interfaces/net.minecraft.server.commands.GameRuleCommand_1|GameRuleCommand$1]].`visit` | `(Lnet/minecraft/world/level/gamerules/GameRule;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `GameRuleCommandVisitorMixin.onRegisterCommand` |
| [[40-Interfaces/net.minecraft.server.jsonrpc.methods.GameRulesService_GameRuleUpdate|GameRulesService$GameRuleUpdate]].`<init>` | `(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `GameRulesServiceGameRuleUpdateMixin.updateFabricType` |
| [[40-Interfaces/net.minecraft.server.jsonrpc.methods.GameRulesService_GameRuleUpdate|GameRulesService$GameRuleUpdate]].`getValueAndTypeCodec` | `(Lnet/minecraft/world/level/gamerules/GameRule;)Lcom/mojang/serialization/MapCodec;` | name_only | @ModifyReturnValue | RETURN | both | 1000 (default) | `GameRulesServiceGameRuleUpdateMixin.getValueAndFabricTypeCodec` |
| [[40-Interfaces/net.minecraft.world.level.gamerules.GameRule|GameRule]].`deserialize` | `(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;` | name_only | @WrapMethod | - | both | 1000 (default) | `GameRuleMixin.deserializeEnum` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.gamerule.v1.FabricGameRuleTypeVisitor|FabricGameRuleTypeVisitor]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder|GameRuleBuilder]] (class, 21 members)
- [[40-Interfaces/net.fabricmc.fabric.api.gamerule.v1.GameRuleEvents|GameRuleEvents]] (class, 1 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
