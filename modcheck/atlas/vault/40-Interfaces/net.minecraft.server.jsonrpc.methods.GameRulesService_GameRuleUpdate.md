---
type: "interface"
fqcn: "net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate

System: [[20-Systems/net.minecraft.server.jsonrpc|net.minecraft.server.jsonrpc]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Ob` | `` | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `gameRule()Lnet/minecraft/world/level/gamerules/GameRule;` | `` | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (18, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate<T> extends java.lang.Record {
    private final net.minecraft.world.level.gamerules.GameRule<T> gameRule;
    private final T value;
    public static final com.mojang.serialization.Codec<net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate<?>> TYPED_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate<?>> CODEC;
    public net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate(net.minecraft.world.level.gamerules.GameRule<T>, T);
    private static <T> com.mojang.serialization.MapCodec<? extends net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate<T>> getValueCodec(net.minecraft.world.level.gamerules.GameRule<T>);
    private static <T> com.mojang.serialization.MapCodec<? extends net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate<T>> getValueAndTypeCodec(net.minecraft.world.level.gamerules.GameRule<T>);
    private static <T> net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate<T> getUntypedRule(net.minecraft.world.level.gamerules.GameRule<T>, net.minecraft.world.level.gamerules.GameRuleType, T);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.level.gamerules.GameRule<T> gameRule();
    public T value();
    private static com.mojang.datafixers.kinds.App lambda$getValueAndTypeCodec$0(net.minecraft.world.level.gamerules.GameRule, com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate lambda$getValueAndTypeCodec$2(net.minecraft.world.level.gamerules.GameRule, net.minecraft.world.level.gamerules.GameRuleType, java.lang.Object);
    private static net.minecraft.world.level.gamerules.GameRuleType lambda$getValueAndTypeCodec$1(net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate);
    private static net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate lambda$getValueCodec$0(net.minecraft.world.level.gamerules.GameRule, java.lang.Object);
    static {};
}
```
