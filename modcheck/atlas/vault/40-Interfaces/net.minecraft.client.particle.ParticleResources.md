---
type: "interface"
fqcn: "net.minecraft.client.particle.ParticleResources"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.ParticleResources

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `registerProviders` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.particle.ParticleResources implements net.minecraft.server.packs.resources.PreparableReloadListener {
    private static final org.slf4j.Logger LOGGER;
    private static final net.minecraft.resources.FileToIdConverter PARTICLE_LISTER;
    private final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.particle.ParticleResources$MutableSpriteSet> spriteSets;
    private final it.unimi.dsi.fastutil.ints.Int2ObjectMap<net.minecraft.client.particle.ParticleProvider<?>> providers;
    private java.lang.Runnable onReload;
    public net.minecraft.client.particle.ParticleResources();
    public void onReload(java.lang.Runnable);
    private void registerProviders();
    private <T extends net.minecraft.core.particles.ParticleOptions> void register(net.minecraft.core.particles.ParticleType<T>, net.minecraft.client.particle.ParticleProvider<T>);
    private <T extends net.minecraft.core.particles.ParticleOptions> void register(net.minecraft.core.particles.ParticleType<T>, net.minecraft.client.particle.ParticleResources$SpriteParticleRegistration<T>);
    public java.util.concurrent.CompletableFuture<java.lang.Void> reload(net.minecraft.server.packs.resources.PreparableReloadListener$SharedState, java.util.concurrent.Executor, net.minecraft.server.packs.resources.PreparableReloadListener$PreparationBarrier, java.util.concurrent.Executor);
    private java.util.Optional<java.util.List<net.minecraft.resources.Identifier>> loadParticleDescription(net.minecraft.resources.Identifier, net.minecraft.server.packs.resources.Resource);
    public it.unimi.dsi.fastutil.ints.Int2ObjectMap<net.minecraft.client.particle.ParticleProvider<?>> getProviders();
    private void lambda$reload$4(java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture, java.lang.Void);
    private void lambda$reload$5(net.minecraft.client.renderer.texture.SpriteLoader$Preparations, java.util.Set, net.minecraft.client.renderer.texture.TextureAtlasSprite, net.minecraft.client.particle.ParticleResources$1ParticleDefinition);
    private java.util.concurrent.CompletionStage lambda$reload$1(java.util.concurrent.Executor, java.util.Map);
    private void lambda$reload$2(java.util.List, java.util.concurrent.Executor, net.minecraft.resources.Identifier, net.minecraft.server.packs.resources.Resource);
    private net.minecraft.client.particle.ParticleResources$1ParticleDefinition lambda$reload$3(net.minecraft.resources.Identifier, net.minecraft.server.packs.resources.Resource);
    private static java.util.Map lambda$reload$0(net.minecraft.server.packs.resources.ResourceManager);
    static {};
}
```
