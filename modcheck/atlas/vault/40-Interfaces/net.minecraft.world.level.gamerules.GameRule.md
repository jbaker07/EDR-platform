---
type: "interface"
fqcn: "net.minecraft.world.level.gamerules.GameRule"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.gamerules.GameRule

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public final; extends `java/lang/Object`; implements `net/minecraft/world/flag/FeatureElement`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/gamerules/GameRuleCategory;Lnet/minecraft/` | exact | invokespecial@153 in `GameRuleBuilder.build` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `deserialize` | `(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;` | exact | invokevirtual@2 in `DoubleRuleEntry.lambda$new$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getCommandResult` | `(Ljava/lang/Object;)I` | exact | invokevirtual@62 in `EnumRuleCommand.executeAndSetEnum` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getDescriptionId` | `()Ljava/lang/String;` | exact | invokevirtual@42 in `RuleListEntryTypeVisitorMixin.displayProperEnumName` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getDescriptionId` | `()Ljava/lang/String;` | exact | invokevirtual@15 in `RuleListEntryTypeVisitorMixin.lambda$visitEnum$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `id` | `()Ljava/lang/String;` | exact | invokevirtual@9 in `EnumRuleCommand.lambda$executeAndSetEnum$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `id` | `()Ljava/lang/String;` | exact | invokevirtual@28 in `GameRulesServiceGameRuleUpdateMixin.fabric_checkType` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `serialize` | `(Ljava/lang/Object;)Ljava/lang/String;` | exact | invokevirtual@17 in `EnumRuleCommand.lambda$executeAndSetEnum$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `toString` | `()Ljava/lang/String;` | exact | invokevirtual@1 in `EnumRuleCommand.register` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `valueClass` | `()Ljava/lang/Class;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | declared |
| calls | `valueCodec` | `()Lcom/mojang/serialization/Codec;` | exact | invokevirtual@23 in `GameRulesServiceGameRuleUpdateMixin.lambda$fabric_createTypedCodec$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| wraps | `deserialize` | `(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;` | name_only | @WrapMethod | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (8 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final category : Lnet/minecraft/world/level/gamerules/GameRuleCategory;
private final gameRuleType : Lnet/minecraft/world/level/gamerules/GameRuleType;
private final argument : Lcom/mojang/brigadier/arguments/ArgumentType;
private final visitorCaller : Lnet/minecraft/world/level/gamerules/GameRules$VisitorCaller;
private final valueCodec : Lcom/mojang/serialization/Codec;
private final commandResultFunction : Ljava/util/function/ToIntFunction;
private final defaultValue : Ljava/lang/Object;
private final requiredFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
public <init>(Lnet/minecraft/world/level/gamerules/GameRuleCategory;Lnet/minecraft/world/level/gamerules/GameRuleType;Lcom/mojang/brigadier/arguments/ArgumentType;Lnet/minecraft/world/level/gamerules/GameRules$VisitorCaller;Lcom/mojang/serialization/Codec;Ljava/util/function/ToIntFunction;Ljava/lang/Object;Lnet/minecraft/world/flag/FeatureFlagSet;)V
public toString()Ljava/lang/String;
public id()Ljava/lang/String;
public getIdentifier()Lnet/minecraft/resources/Identifier;
public getIdentifierWithFallback()Lnet/minecraft/resources/Identifier;
public getDescriptionId()Ljava/lang/String;
public serialize(Ljava/lang/Object;)Ljava/lang/String;
public deserialize(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
public valueClass()Ljava/lang/Class;
public callVisitor(Lnet/minecraft/world/level/gamerules/GameRuleTypeVisitor;)V
public getCommandResult(Ljava/lang/Object;)I
public category()Lnet/minecraft/world/level/gamerules/GameRuleCategory;
public gameRuleType()Lnet/minecraft/world/level/gamerules/GameRuleType;
public argument()Lcom/mojang/brigadier/arguments/ArgumentType;
public valueCodec()Lcom/mojang/serialization/Codec;
public defaultValue()Ljava/lang/Object;
public requiredFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
private static synthetic lambda$deserialize$1()Ljava/lang/String;
private static synthetic lambda$deserialize$0()Ljava/lang/String;
```
