---
type: "interface"
fqcn: "net.minecraft.world.level.gamerules.GameRuleCategory"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.gamerules.GameRuleCategory

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `MISC` | `Lnet/minecraft/world/level/gamerules/GameRuleCategory;` | exact | getstatic@5 in `GameRuleBuilder.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (9 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final id : Lnet/minecraft/resources/Identifier;
private static final SORT_ORDER : Ljava/util/List;
public static final PLAYER : Lnet/minecraft/world/level/gamerules/GameRuleCategory;
public static final MOBS : Lnet/minecraft/world/level/gamerules/GameRuleCategory;
public static final SPAWNING : Lnet/minecraft/world/level/gamerules/GameRuleCategory;
public static final DROPS : Lnet/minecraft/world/level/gamerules/GameRuleCategory;
public static final UPDATES : Lnet/minecraft/world/level/gamerules/GameRuleCategory;
public static final CHAT : Lnet/minecraft/world/level/gamerules/GameRuleCategory;
public static final MISC : Lnet/minecraft/world/level/gamerules/GameRuleCategory;
public <init>(Lnet/minecraft/resources/Identifier;)V
public getDescriptionId()Lnet/minecraft/resources/Identifier;
private static register(Ljava/lang/String;)Lnet/minecraft/world/level/gamerules/GameRuleCategory;
public static register(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/world/level/gamerules/GameRuleCategory;
public label()Lnet/minecraft/network/chat/MutableComponent;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public id()Lnet/minecraft/resources/Identifier;
static <clinit>()V
```
