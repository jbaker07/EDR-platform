---
type: "interface"
fqcn: "net.minecraft.client.particle.ParticleEngine"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.ParticleEngine

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<clinit>` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| injects_into | `createParticleGroup` | `@Inject at NEW (Lnet/minecraft/client/particle/ParticleEngine;Lnet/minecraft/cli` | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (26, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.particle.ParticleEngine {
    private static final java.util.List<net.minecraft.client.particle.ParticleRenderType> RENDER_ORDER;
    protected net.minecraft.client.multiplayer.ClientLevel level;
    private final java.util.Map<net.minecraft.client.particle.ParticleRenderType, net.minecraft.client.particle.ParticleGroup<?>> particles;
    private final java.util.Queue<net.minecraft.client.particle.TrackingEmitter> trackingEmitters;
    private final java.util.Queue<net.minecraft.client.particle.Particle> particlesToAdd;
    private final it.unimi.dsi.fastutil.objects.Object2IntOpenHashMap<net.minecraft.core.particles.ParticleLimit> trackedParticleCounts;
    private final net.minecraft.client.particle.ParticleResources resourceManager;
    private final net.minecraft.util.RandomSource random;
    public net.minecraft.client.particle.ParticleEngine(net.minecraft.client.multiplayer.ClientLevel, net.minecraft.client.particle.ParticleResources);
    public void createTrackingEmitter(net.minecraft.world.entity.Entity, net.minecraft.core.particles.ParticleOptions);
    public void createTrackingEmitter(net.minecraft.world.entity.Entity, net.minecraft.core.particles.ParticleOptions, int);
    public net.minecraft.client.particle.Particle createParticle(net.minecraft.core.particles.ParticleOptions, double, double, double, double, double, double);
    private <T extends net.minecraft.core.particles.ParticleOptions> net.minecraft.client.particle.Particle makeParticle(T, double, double, double, double, double, double);
    public void add(net.minecraft.client.particle.Particle);
    public void tick();
    private net.minecraft.client.particle.ParticleGroup<?> createParticleGroup(net.minecraft.client.particle.ParticleRenderType);
    protected void updateCount(net.minecraft.core.particles.ParticleLimit, int);
    public void extract(net.minecraft.client.renderer.state.level.ParticlesRenderState, net.minecraft.client.renderer.culling.Frustum, net.minecraft.client.Camera, float);
    public void setLevel(net.minecraft.client.multiplayer.ClientLevel);
    public java.lang.String countParticles();
    private boolean hasSpaceInParticleLimit(net.minecraft.core.particles.ParticleLimit);
    public void clearParticles();
    public net.minecraft.util.RandomSource getRandom();
    private void lambda$tick$1(net.minecraft.core.particles.ParticleLimit);
    private static void lambda$tick$0(net.minecraft.client.particle.ParticleRenderType, net.minecraft.client.particle.ParticleGroup);
    static {};
}
```
