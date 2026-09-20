---
type: "interface"
fqcn: "net.minecraft.world.entity.player.Abilities"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.player.Abilities

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `instabuild` | `Z` | exact | getfield@10 in `MultiPlayerGameModeMixin.method_2902` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (14 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final DEFAULT_INVULNERABLE : Z
private static final DEFAULY_FLYING : Z
private static final DEFAULT_MAY_FLY : Z
private static final DEFAULT_INSTABUILD : Z
private static final DEFAULT_MAY_BUILD : Z
private static final DEFAULT_FLYING_SPEED : F
private static final DEFAULT_WALKING_SPEED : F
public invulnerable : Z
public flying : Z
public mayfly : Z
public instabuild : Z
public mayBuild : Z
private flyingSpeed : F
private walkingSpeed : F
public <init>()V
public getFlyingSpeed()F
public setFlyingSpeed(F)V
public getWalkingSpeed()F
public setWalkingSpeed(F)V
public pack()Lnet/minecraft/world/entity/player/Abilities$Packed;
public apply(Lnet/minecraft/world/entity/player/Abilities$Packed;)V
```
