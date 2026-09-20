---
type: "interface"
fqcn: "net.minecraft.util.ParticleUtils"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.ParticleUtils

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `spawnSmashAttackParticles` | `(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos` | name_only | @ModifyExpressionValue at ['NEW'] | both | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (0 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>()V
public static spawnParticlesOnBlockFaces(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/particles/ParticleOptions;Lnet/minecraft/util/valueproviders/IntProvider;)V
public static spawnParticlesOnBlockFace(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/particles/ParticleOptions;Lnet/minecraft/util/valueproviders/IntProvider;Lnet/minecraft/core/Direction;Ljava/util/function/Supplier;D)V
private static getRandomSpeedRanges(Lnet/minecraft/util/RandomSource;)Lnet/minecraft/world/phys/Vec3;
public static spawnParticlesAlongAxis(Lnet/minecraft/core/Direction$Axis;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;DLnet/minecraft/core/particles/ParticleOptions;Lnet/minecraft/util/valueproviders/UniformInt;)V
public static spawnParticleOnFace(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/core/particles/ParticleOptions;Lnet/minecraft/world/phys/Vec3;D)V
public static spawnParticleBelow(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;Lnet/minecraft/core/particles/ParticleOptions;)V
public static spawnParticleInBlock(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;ILnet/minecraft/core/particles/ParticleOptions;)V
public static spawnParticles(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;IDDZLnet/minecraft/core/particles/ParticleOptions;)V
public static spawnSmashAttackParticles(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;I)V
private static synthetic lambda$spawnParticlesOnBlockFaces$0(Lnet/minecraft/util/RandomSource;)Lnet/minecraft/world/phys/Vec3;
```
