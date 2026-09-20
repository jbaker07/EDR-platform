---
type: "interface"
fqcn: "net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder"
module: "fabric-game-rule-api-v1"
sha256: "58266b2e28d20444584f22f1940b99656437728bd45e1b226c4869637e704638"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder

Module: [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] -- kind: class

```java
protected final java.lang.Object defaultValue
protected net.minecraft.world.level.gamerules.GameRuleCategory category
protected net.minecraft.world.level.gamerules.GameRuleType type
protected net.fabricmc.fabric.impl.gamerule.rpc.FabricGameRuleType fabricType
protected com.mojang.brigadier.arguments.ArgumentType argumentType
protected net.minecraft.world.level.gamerules.GameRules$VisitorCaller acceptor
protected com.mojang.serialization.Codec codec
protected java.util.function.ToIntFunction commandResultSupplier
protected net.minecraft.world.flag.FeatureFlagSet requiredFeatures
protected <init>(java.lang.Object)
public static net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$BooleanRuleBuilder forBoolean(boolean)
public static net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$IntegerRuleBuilder forInteger(int)
public static net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$DoubleRuleBuilder forDouble(double)
public static net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$EnumRuleBuilder forEnum(java.lang.Enum)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder category(net.minecraft.world.level.gamerules.GameRuleCategory)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder codec(com.mojang.serialization.Codec)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder argumentType(com.mojang.brigadier.arguments.ArgumentType)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder commandResultSupplier(java.util.function.ToIntFunction)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder requiredFeatures(net.minecraft.world.flag.FeatureFlagSet)
public net.minecraft.world.level.gamerules.GameRule build()
public net.minecraft.world.level.gamerules.GameRule buildAndRegister(net.minecraft.resources.Identifier)
```
