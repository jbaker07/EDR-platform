---
type: "interface"
fqcn: "net.minecraft.world.level.gameevent.vibrations.VibrationSystem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.gameevent.vibrations.VibrationSystem

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `VIBRATION_FREQUENCY_FOR_EVENT` | `Ljava/util/function/ToIntFunction;` | exact | getstatic@31 in `VibrationFrequencyRegistry.register` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (3 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final RESONANCE_EVENTS : Ljava/util/List;
public static final NO_VIBRATION_FREQUENCY : I
public static final VIBRATION_FREQUENCY_FOR_EVENT : Ljava/util/function/ToIntFunction;
public abstract getVibrationData()Lnet/minecraft/world/level/gameevent/vibrations/VibrationSystem$Data;
public abstract getVibrationUser()Lnet/minecraft/world/level/gameevent/vibrations/VibrationSystem$User;
public static getGameEventFrequency(Lnet/minecraft/core/Holder;)I
public static getGameEventFrequency(Lnet/minecraft/resources/ResourceKey;)I
public static getResonanceEventByFrequency(I)Lnet/minecraft/resources/ResourceKey;
public static getRedstoneStrengthForDistance(FI)I
private static synthetic lambda$static$0(Lit/unimi/dsi/fastutil/objects/Reference2IntOpenHashMap;)V
static <clinit>()V
```
