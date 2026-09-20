---
type: "interface"
fqcn: "net.minecraft.client.particle.ParticleEngine"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.ParticleEngine

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<clinit>` | `()V` | exact | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| injects_into | `createParticleGroup` | `(Lnet/minecraft/client/particle/ParticleRenderType;)Lnet/minecraft/cli` | name_only | @Inject at ['NEW'] | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `RENDER_ORDER` | `Ljava/util/List;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | declared |

## Declared members (8 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final RENDER_ORDER : Ljava/util/List;
protected level : Lnet/minecraft/client/multiplayer/ClientLevel;
private final particles : Ljava/util/Map;
private final trackingEmitters : Ljava/util/Queue;
private final particlesToAdd : Ljava/util/Queue;
private final trackedParticleCounts : Lit/unimi/dsi/fastutil/objects/Object2IntOpenHashMap;
private final resourceManager : Lnet/minecraft/client/particle/ParticleResources;
private final random : Lnet/minecraft/util/RandomSource;
public <init>(Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/client/particle/ParticleResources;)V
public createTrackingEmitter(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/particles/ParticleOptions;)V
public createTrackingEmitter(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/particles/ParticleOptions;I)V
public createParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)Lnet/minecraft/client/particle/Particle;
private makeParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)Lnet/minecraft/client/particle/Particle;
public add(Lnet/minecraft/client/particle/Particle;)V
public tick()V
private createParticleGroup(Lnet/minecraft/client/particle/ParticleRenderType;)Lnet/minecraft/client/particle/ParticleGroup;
protected updateCount(Lnet/minecraft/core/particles/ParticleLimit;I)V
public extract(Lnet/minecraft/client/renderer/state/level/ParticlesRenderState;Lnet/minecraft/client/renderer/culling/Frustum;Lnet/minecraft/client/Camera;F)V
public setLevel(Lnet/minecraft/client/multiplayer/ClientLevel;)V
public countParticles()Ljava/lang/String;
private hasSpaceInParticleLimit(Lnet/minecraft/core/particles/ParticleLimit;)Z
public clearParticles()V
public getRandom()Lnet/minecraft/util/RandomSource;
private synthetic lambda$tick$1(Lnet/minecraft/core/particles/ParticleLimit;)V
private static synthetic lambda$tick$0(Lnet/minecraft/client/particle/ParticleRenderType;Lnet/minecraft/client/particle/ParticleGroup;)V
static <clinit>()V
```
