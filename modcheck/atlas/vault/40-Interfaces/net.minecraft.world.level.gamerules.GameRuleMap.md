---
type: "interface"
fqcn: "net.minecraft.world.level.gamerules.GameRuleMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.gamerules.GameRuleMap

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public final; extends `net/minecraft/world/level/saveddata/SavedData`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `TYPE` | `Lnet/minecraft/world/level/saveddata/SavedDataType;` | exact | getstatic@98 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (3 fields, 22 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final TYPE : Lnet/minecraft/world/level/saveddata/SavedDataType;
private final map : Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;
private <init>(Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;)V
private static ofTrusted(Ljava/util/Map;)Lnet/minecraft/world/level/gamerules/GameRuleMap;
public static of()Lnet/minecraft/world/level/gamerules/GameRuleMap;
public static of(Ljava/util/stream/Stream;)Lnet/minecraft/world/level/gamerules/GameRuleMap;
public static copyOf(Lnet/minecraft/world/level/gamerules/GameRuleMap;)Lnet/minecraft/world/level/gamerules/GameRuleMap;
public has(Lnet/minecraft/world/level/gamerules/GameRule;)Z
public get(Lnet/minecraft/world/level/gamerules/GameRule;)Ljava/lang/Object;
public set(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;)V
public reset(Lnet/minecraft/world/level/gamerules/GameRule;)V
public remove(Lnet/minecraft/world/level/gamerules/GameRule;)Ljava/lang/Object;
public keySet()Ljava/util/Set;
public size()I
public toString()Ljava/lang/String;
public withOther(Lnet/minecraft/world/level/gamerules/GameRuleMap;)Lnet/minecraft/world/level/gamerules/GameRuleMap;
public setFromIf(Lnet/minecraft/world/level/gamerules/GameRuleMap;Ljava/util/function/Predicate;)V
private static setGameRule(Lnet/minecraft/world/level/gamerules/GameRuleMap;Lnet/minecraft/world/level/gamerules/GameRule;Lnet/minecraft/world/level/gamerules/GameRuleMap;)V
private map()Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;
public equals(Ljava/lang/Object;)Z
public hashCode()I
private static synthetic lambda$withOther$0(Lnet/minecraft/world/level/gamerules/GameRule;)Z
private static synthetic lambda$of$0(Lit/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap;Lnet/minecraft/world/level/gamerules/GameRule;)V
static <clinit>()V
```
