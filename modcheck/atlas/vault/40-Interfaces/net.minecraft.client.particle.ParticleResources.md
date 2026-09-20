---
type: "interface"
fqcn: "net.minecraft.client.particle.ParticleResources"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.ParticleResources

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/server/packs/resources/PreparableReloadListener`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<init>` | `()V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `registerProviders` | `()V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `providers` | `Lit/unimi/dsi/fastutil/ints/Int2ObjectMap;` | exact | getfield@4 in `ParticleProviderRegistryImpl$DirectParticleProviderRegistry.register` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `providers` | `Lit/unimi/dsi/fastutil/ints/Int2ObjectMap;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| reads | `spriteSets` | `Ljava/util/Map;` | exact | getfield@22 in `ParticleProviderRegistryImpl$DirectParticleProviderRegistry.register` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (5 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final PARTICLE_LISTER : Lnet/minecraft/resources/FileToIdConverter;
private final spriteSets : Ljava/util/Map;
private final providers : Lit/unimi/dsi/fastutil/ints/Int2ObjectMap;
private onReload : Ljava/lang/Runnable;
public <init>()V
public onReload(Ljava/lang/Runnable;)V
private registerProviders()V
private register(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/client/particle/ParticleProvider;)V
private register(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/client/particle/ParticleResources$SpriteParticleRegistration;)V
public reload(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private loadParticleDescription(Lnet/minecraft/resources/Identifier;Lnet/minecraft/server/packs/resources/Resource;)Ljava/util/Optional;
public getProviders()Lit/unimi/dsi/fastutil/ints/Int2ObjectMap;
private synthetic lambda$reload$4(Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Ljava/lang/Void;)V
private synthetic lambda$reload$5(Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;Ljava/util/Set;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;Lnet/minecraft/client/particle/ParticleResources$1ParticleDefinition;)V
private synthetic lambda$reload$1(Ljava/util/concurrent/Executor;Ljava/util/Map;)Ljava/util/concurrent/CompletionStage;
private synthetic lambda$reload$2(Ljava/util/List;Ljava/util/concurrent/Executor;Lnet/minecraft/resources/Identifier;Lnet/minecraft/server/packs/resources/Resource;)V
private synthetic lambda$reload$3(Lnet/minecraft/resources/Identifier;Lnet/minecraft/server/packs/resources/Resource;)Lnet/minecraft/client/particle/ParticleResources$1ParticleDefinition;
private static synthetic lambda$reload$0(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljava/util/Map;
static <clinit>()V
```
