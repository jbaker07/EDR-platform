---
type: "interface"
fqcn: "net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.jsonrpc.methods.GameRulesService$GameRuleUpdate

System: [[20-Systems/net.minecraft.server.jsonrpc|net.minecraft.server.jsonrpc]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;)V` | exact | invokespecial@46 in `GameRulesServiceGameRuleUpdateMixin.fabric_checkType` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `gameRule` | `()Lnet/minecraft/world/level/gamerules/GameRule;` | exact | invokevirtual@1 in `GameRulesServiceGameRuleUpdateMixin.lambda$fabric_createTypedCodec$1` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;)V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| injects_into | `getValueAndTypeCodec` | `(Lnet/minecraft/world/level/gamerules/GameRule;)Lcom/mojang/serializat` | name_only | @ModifyReturnValue at ['RETURN'] | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (4 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final gameRule : Lnet/minecraft/world/level/gamerules/GameRule;
private final value : Ljava/lang/Object;
public static final TYPED_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public <init>(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;)V
private static getValueCodec(Lnet/minecraft/world/level/gamerules/GameRule;)Lcom/mojang/serialization/MapCodec;
private static getValueAndTypeCodec(Lnet/minecraft/world/level/gamerules/GameRule;)Lcom/mojang/serialization/MapCodec;
private static getUntypedRule(Lnet/minecraft/world/level/gamerules/GameRule;Lnet/minecraft/world/level/gamerules/GameRuleType;Ljava/lang/Object;)Lnet/minecraft/server/jsonrpc/methods/GameRulesService$GameRuleUpdate;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public gameRule()Lnet/minecraft/world/level/gamerules/GameRule;
public value()Ljava/lang/Object;
private static synthetic lambda$getValueAndTypeCodec$0(Lnet/minecraft/world/level/gamerules/GameRule;Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$getValueAndTypeCodec$2(Lnet/minecraft/world/level/gamerules/GameRule;Lnet/minecraft/world/level/gamerules/GameRuleType;Ljava/lang/Object;)Lnet/minecraft/server/jsonrpc/methods/GameRulesService$GameRuleUpdate;
private static synthetic lambda$getValueAndTypeCodec$1(Lnet/minecraft/server/jsonrpc/methods/GameRulesService$GameRuleUpdate;)Lnet/minecraft/world/level/gamerules/GameRuleType;
private static synthetic lambda$getValueCodec$0(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;)Lnet/minecraft/server/jsonrpc/methods/GameRulesService$GameRuleUpdate;
static <clinit>()V
```
