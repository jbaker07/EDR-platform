---
type: "interface"
fqcn: "net.minecraft.world.effect.MobEffects"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.effect.MobEffects

System: [[20-Systems/net.minecraft.world.effect|net.minecraft.world.effect]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `DOLPHINS_GRACE` | `Lnet/minecraft/core/Holder;` | exact | getstatic@68 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (41 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final DARKNESS_EFFECT_FACTOR_PADDING_DURATION_TICKS : I
public static final SPEED : Lnet/minecraft/core/Holder;
public static final SLOWNESS : Lnet/minecraft/core/Holder;
public static final HASTE : Lnet/minecraft/core/Holder;
public static final MINING_FATIGUE : Lnet/minecraft/core/Holder;
public static final STRENGTH : Lnet/minecraft/core/Holder;
public static final INSTANT_HEALTH : Lnet/minecraft/core/Holder;
public static final INSTANT_DAMAGE : Lnet/minecraft/core/Holder;
public static final JUMP_BOOST : Lnet/minecraft/core/Holder;
public static final NAUSEA : Lnet/minecraft/core/Holder;
public static final REGENERATION : Lnet/minecraft/core/Holder;
public static final RESISTANCE : Lnet/minecraft/core/Holder;
public static final FIRE_RESISTANCE : Lnet/minecraft/core/Holder;
public static final WATER_BREATHING : Lnet/minecraft/core/Holder;
public static final INVISIBILITY : Lnet/minecraft/core/Holder;
public static final BLINDNESS : Lnet/minecraft/core/Holder;
public static final NIGHT_VISION : Lnet/minecraft/core/Holder;
public static final HUNGER : Lnet/minecraft/core/Holder;
public static final WEAKNESS : Lnet/minecraft/core/Holder;
public static final POISON : Lnet/minecraft/core/Holder;
public static final WITHER : Lnet/minecraft/core/Holder;
public static final HEALTH_BOOST : Lnet/minecraft/core/Holder;
public static final ABSORPTION : Lnet/minecraft/core/Holder;
public static final SATURATION : Lnet/minecraft/core/Holder;
public static final GLOWING : Lnet/minecraft/core/Holder;
public static final LEVITATION : Lnet/minecraft/core/Holder;
public static final LUCK : Lnet/minecraft/core/Holder;
public static final UNLUCK : Lnet/minecraft/core/Holder;
public static final SLOW_FALLING : Lnet/minecraft/core/Holder;
public static final CONDUIT_POWER : Lnet/minecraft/core/Holder;
public static final DOLPHINS_GRACE : Lnet/minecraft/core/Holder;
public static final BAD_OMEN : Lnet/minecraft/core/Holder;
public static final HERO_OF_THE_VILLAGE : Lnet/minecraft/core/Holder;
public static final DARKNESS : Lnet/minecraft/core/Holder;
public static final TRIAL_OMEN : Lnet/minecraft/core/Holder;
public static final RAID_OMEN : Lnet/minecraft/core/Holder;
public static final WIND_CHARGED : Lnet/minecraft/core/Holder;
public static final WEAVING : Lnet/minecraft/core/Holder;
public static final OOZING : Lnet/minecraft/core/Holder;
public static final INFESTED : Lnet/minecraft/core/Holder;
public static final BREATH_OF_THE_NAUTILUS : Lnet/minecraft/core/Holder;
public <init>()V
private static register(Ljava/lang/String;Lnet/minecraft/world/effect/MobEffect;)Lnet/minecraft/core/Holder;
public static bootstrap(Lnet/minecraft/core/Registry;)Lnet/minecraft/core/Holder;
private static synthetic lambda$static$2(Lnet/minecraft/util/RandomSource;)I
private static synthetic lambda$static$1(Lnet/minecraft/util/RandomSource;)I
private static synthetic lambda$static$0(Lnet/minecraft/util/RandomSource;)I
static <clinit>()V
```
