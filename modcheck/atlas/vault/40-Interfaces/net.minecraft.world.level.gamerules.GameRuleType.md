---
type: "interface"
fqcn: "net.minecraft.world.level.gamerules.GameRuleType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.gamerules.GameRuleType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`enum` public final; extends `java/lang/Enum`; implements `net/minecraft/util/StringRepresentable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `BOOL` | `Lnet/minecraft/world/level/gamerules/GameRuleType;` | exact | getstatic@9 in `GameRuleBuilder$BooleanRuleBuilder.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| reads | `INT` | `Lnet/minecraft/world/level/gamerules/GameRuleType;` | exact | getstatic@9 in `GameRuleBuilder$IntegerRuleBuilder.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| reads | `INT` | `Lnet/minecraft/world/level/gamerules/GameRuleType;` | exact | getstatic@12 in `GameRuleBuilder.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (4 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final INT : Lnet/minecraft/world/level/gamerules/GameRuleType;
public static final BOOL : Lnet/minecraft/world/level/gamerules/GameRuleType;
private final name : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/world/level/gamerules/GameRuleType;
public static values()[Lnet/minecraft/world/level/gamerules/GameRuleType;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/world/level/gamerules/GameRuleType;
private <init>(Ljava/lang/String;ILjava/lang/String;)V
public getSerializedName()Ljava/lang/String;
private static synthetic $values()[Lnet/minecraft/world/level/gamerules/GameRuleType;
static <clinit>()V
```
