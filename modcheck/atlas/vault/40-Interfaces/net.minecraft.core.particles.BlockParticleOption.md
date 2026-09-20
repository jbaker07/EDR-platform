---
type: "interface"
fqcn: "net.minecraft.core.particles.BlockParticleOption"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.particles.BlockParticleOption

System: [[20-Systems/net.minecraft.core.particles|net.minecraft.core.particles]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/core/particles/ParticleOptions`, `net/fabricmc/fabric/api/particle/v1/FabricBlockParticleOption`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/world/level` | exact | invokespecial@6 in `BlockParticleOptionFactoryImpl.create` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `getBlockPos` | `()Lnet/minecraft/core/BlockPos;` | inherited_exact | invokevirtual@1 in `ExtendedBlockParticleOptionStreamCodec.encode` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `getBlockPos` | `()Lnet/minecraft/core/BlockPos;` | inherited_exact | invokevirtual@2 in `TerrainParticleMixin.constructTerrainParticle` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| injects_into | `streamCodec` | `(Lnet/minecraft/core/particles/ParticleType;)Lnet/minecraft/network/co` | name_only | @ModifyReturnValue at ['RETURN'] | both | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (3 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final BLOCK_STATE_CODEC : Lcom/mojang/serialization/Codec;
private final type : Lnet/minecraft/core/particles/ParticleType;
private final state : Lnet/minecraft/world/level/block/state/BlockState;
public static codec(Lnet/minecraft/core/particles/ParticleType;)Lcom/mojang/serialization/MapCodec;
public static streamCodec(Lnet/minecraft/core/particles/ParticleType;)Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/world/level/block/state/BlockState;)V
public getType()Lnet/minecraft/core/particles/ParticleType;
public getState()Lnet/minecraft/world/level/block/state/BlockState;
private static synthetic lambda$streamCodec$1(Lnet/minecraft/core/particles/BlockParticleOption;)Lnet/minecraft/world/level/block/state/BlockState;
private static synthetic lambda$streamCodec$0(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/particles/BlockParticleOption;
private static synthetic lambda$codec$1(Lnet/minecraft/core/particles/BlockParticleOption;)Lnet/minecraft/world/level/block/state/BlockState;
private static synthetic lambda$codec$0(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/particles/BlockParticleOption;
static <clinit>()V
```
