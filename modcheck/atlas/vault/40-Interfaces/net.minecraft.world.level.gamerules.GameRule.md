---
type: "interface"
fqcn: "net.minecraft.world.level.gamerules.GameRule"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.gamerules.GameRule

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `deserialize(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getCommandResult(Ljava/lang/Object;)I` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getDescriptionId()Ljava/lang/String;` | `` | client | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `id()Ljava/lang/String;` | `` | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `id()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `serialize(Ljava/lang/Object;)Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `toString()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `valueCodec()Lcom/mojang/serialization/Codec;` | `` | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.gamerules.GameRule<T> implements net.minecraft.world.flag.FeatureElement {
    private final net.minecraft.world.level.gamerules.GameRuleCategory category;
    private final net.minecraft.world.level.gamerules.GameRuleType gameRuleType;
    private final com.mojang.brigadier.arguments.ArgumentType<T> argument;
    private final net.minecraft.world.level.gamerules.GameRules$VisitorCaller<T> visitorCaller;
    private final com.mojang.serialization.Codec<T> valueCodec;
    private final java.util.function.ToIntFunction<T> commandResultFunction;
    private final T defaultValue;
    private final net.minecraft.world.flag.FeatureFlagSet requiredFeatures;
    public net.minecraft.world.level.gamerules.GameRule(net.minecraft.world.level.gamerules.GameRuleCategory, net.minecraft.world.level.gamerules.GameRuleType, com.mojang.brigadier.arguments.ArgumentType<T>, net.minecraft.world.level.gamerules.GameRules$VisitorCaller<T>, com.mojang.serialization.Codec<T>, java.util.function.ToIntFunction<T>, T, net.minecraft.world.flag.FeatureFlagSet);
    public java.lang.String toString();
    public java.lang.String id();
    public net.minecraft.resources.Identifier getIdentifier();
    public net.minecraft.resources.Identifier getIdentifierWithFallback();
    public java.lang.String getDescriptionId();
    public java.lang.String serialize(T);
    public com.mojang.serialization.DataResult<T> deserialize(java.lang.String);
    public java.lang.Class<T> valueClass();
    public void callVisitor(net.minecraft.world.level.gamerules.GameRuleTypeVisitor);
    public int getCommandResult(T);
    public net.minecraft.world.level.gamerules.GameRuleCategory category();
    public net.minecraft.world.level.gamerules.GameRuleType gameRuleType();
    public com.mojang.brigadier.arguments.ArgumentType<T> argument();
    public com.mojang.serialization.Codec<T> valueCodec();
    public T defaultValue();
    public net.minecraft.world.flag.FeatureFlagSet requiredFeatures();
    private static java.lang.String lambda$deserialize$1();
    private static java.lang.String lambda$deserialize$0();
}
```
