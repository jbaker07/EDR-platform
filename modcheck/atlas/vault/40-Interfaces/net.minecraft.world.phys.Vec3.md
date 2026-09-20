---
type: "interface"
fqcn: "net.minecraft.world.phys.Vec3"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.phys.Vec3

System: [[20-Systems/net.minecraft.world.phys|net.minecraft.world.phys]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/core/Position`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(DDD)V` | exact | invokespecial@110 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `add` | `(DDD)Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@102 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `add` | `(DDD)Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@170 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `add` | `(DDD)Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@30 in `ServerGamePacketListenerImplMixin.handleInteract` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `atCenterOf` | `(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/world/phys/Vec3;` | exact | invokestatic@26 in `TestInputImpl.lambda$lookAt$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `atCenterOf` | `(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/world/phys/Vec3;` | exact | invokestatic@23 in `ComposterWrapper.getLayersToAdd` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `multiply` | `(DDD)Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@7 in `SimpleConfiguredFluidBehavior$Builder.lambda$movementSlowdown$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `multiply` | `(DDD)Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@24 in `SimpleConfiguredFluidBehavior$Builder.lambda$new$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `multiply` | `(DDD)Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@124 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `scale` | `(D)Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@26 in `FluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `scale` | `(D)Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@9 in `SimpleConfiguredFluidBehavior$Builder.lambda$movementSlowdown$2` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `scale` | `(D)Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@3 in `SimpleConfiguredFluidBehavior$Builder.lambda$movementSlowdown$0` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `scale` | `(D)Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@8 in `SimpleConfiguredFluidBehavior$Builder.lambda$new$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `subtract` | `(DDD)Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@70 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `x` | `D` | exact | getfield@100 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `x` | `D` | exact | getfield@17 in `AltModelBlockRendererImpl.tesselateBlock` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `y` | `D` | exact | getfield@25 in `AltModelBlockRendererImpl.tesselateBlock` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `z` | `D` | exact | getfield@107 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `z` | `D` | exact | getfield@34 in `AltModelBlockRendererImpl.tesselateBlock` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (10 fields, 60 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final LP_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final ZERO : Lnet/minecraft/world/phys/Vec3;
public static final X_AXIS : Lnet/minecraft/world/phys/Vec3;
public static final Y_AXIS : Lnet/minecraft/world/phys/Vec3;
public static final Z_AXIS : Lnet/minecraft/world/phys/Vec3;
public final x : D
public final y : D
public final z : D
public static atLowerCornerOf(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/world/phys/Vec3;
public static atLowerCornerWithOffset(Lnet/minecraft/core/Vec3i;DDD)Lnet/minecraft/world/phys/Vec3;
public static atCenterOf(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/world/phys/Vec3;
public static atCenterOfWithY(Lnet/minecraft/core/Vec3i;D)Lnet/minecraft/world/phys/Vec3;
public static atBottomCenterOf(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/world/phys/Vec3;
public static upFromBottomCenterOf(Lnet/minecraft/core/Vec3i;D)Lnet/minecraft/world/phys/Vec3;
public <init>(DDD)V
public <init>(Lorg/joml/Vector3fc;)V
public <init>(Lnet/minecraft/core/Vec3i;)V
public vectorTo(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
public normalize()Lnet/minecraft/world/phys/Vec3;
public dot(Lnet/minecraft/world/phys/Vec3;)D
public cross(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
public subtract(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
public subtract(D)Lnet/minecraft/world/phys/Vec3;
public subtract(DDD)Lnet/minecraft/world/phys/Vec3;
public add(D)Lnet/minecraft/world/phys/Vec3;
public add(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
public add(DDD)Lnet/minecraft/world/phys/Vec3;
public closerThan(Lnet/minecraft/core/Position;D)Z
public distanceTo(Lnet/minecraft/world/phys/Vec3;)D
public distanceToSqr(Lnet/minecraft/world/phys/Vec3;)D
public distanceToSqr(DDD)D
public closerThan(Lnet/minecraft/world/phys/Vec3;DD)Z
public scale(D)Lnet/minecraft/world/phys/Vec3;
public reverse()Lnet/minecraft/world/phys/Vec3;
public multiply(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
public multiply(DDD)Lnet/minecraft/world/phys/Vec3;
public horizontal()Lnet/minecraft/world/phys/Vec3;
public offsetRandom(Lnet/minecraft/util/RandomSource;F)Lnet/minecraft/world/phys/Vec3;
public offsetRandomXZ(Lnet/minecraft/util/RandomSource;F)Lnet/minecraft/world/phys/Vec3;
public length()D
public lengthSqr()D
public horizontalDistance()D
public horizontalDistanceSqr()D
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
public lerp(Lnet/minecraft/world/phys/Vec3;D)Lnet/minecraft/world/phys/Vec3;
public xRot(F)Lnet/minecraft/world/phys/Vec3;
public yRot(F)Lnet/minecraft/world/phys/Vec3;
public zRot(F)Lnet/minecraft/world/phys/Vec3;
public rotateClockwise90()Lnet/minecraft/world/phys/Vec3;
public static directionFromRotation(Lnet/minecraft/world/phys/Vec2;)Lnet/minecraft/world/phys/Vec3;
public static directionFromRotation(FF)Lnet/minecraft/world/phys/Vec3;
public rotation()Lnet/minecraft/world/phys/Vec2;
public align(Ljava/util/EnumSet;)Lnet/minecraft/world/phys/Vec3;
public get(Lnet/minecraft/core/Direction$Axis;)D
public with(Lnet/minecraft/core/Direction$Axis;D)Lnet/minecraft/world/phys/Vec3;
public relative(Lnet/minecraft/core/Direction;D)Lnet/minecraft/world/phys/Vec3;
public final x()D
public final y()D
public final z()D
public toVector3f()Lorg/joml/Vector3f;
public projectedOn(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
public isFinite()Z
private static synthetic lambda$static$2(Lnet/minecraft/world/phys/Vec3;)Ljava/util/List;
private static synthetic lambda$static$0(Ljava/util/List;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$1(Ljava/util/List;)Lnet/minecraft/world/phys/Vec3;
static <clinit>()V
```
