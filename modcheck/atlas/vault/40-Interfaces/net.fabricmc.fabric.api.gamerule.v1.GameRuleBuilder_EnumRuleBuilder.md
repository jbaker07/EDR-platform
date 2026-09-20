---
type: "interface"
fqcn: "net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$EnumRuleBuilder"
module: "fabric-game-rule-api-v1"
sha256: "58266b2e28d20444584f22f1940b99656437728bd45e1b226c4869637e704638"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$EnumRuleBuilder

Module: [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] -- kind: class

```java
net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$EnumRuleBuilder(E)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$EnumRuleBuilder<E> category(net.minecraft.world.level.gamerules.GameRuleCategory)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$EnumRuleBuilder<E> codec(com.mojang.serialization.Codec<E>)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$EnumRuleBuilder<E> argumentType(com.mojang.brigadier.arguments.ArgumentType<E>)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$EnumRuleBuilder<E> commandResultSupplier(java.util.function.ToIntFunction<E>)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$EnumRuleBuilder<E> requiredFeatures(net.minecraft.world.flag.FeatureFlagSet)
public final net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder$EnumRuleBuilder<E> supportedValues(E...)
public net.minecraft.world.level.gamerules.GameRule<E> build()
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder requiredFeatures(net.minecraft.world.flag.FeatureFlagSet)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder commandResultSupplier(java.util.function.ToIntFunction)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder argumentType(com.mojang.brigadier.arguments.ArgumentType)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder codec(com.mojang.serialization.Codec)
public net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder category(net.minecraft.world.level.gamerules.GameRuleCategory)
```
