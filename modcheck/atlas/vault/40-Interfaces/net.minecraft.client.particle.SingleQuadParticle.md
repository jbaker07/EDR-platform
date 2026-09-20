---
type: "interface"
fqcn: "net.minecraft.client.particle.SingleQuadParticle"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.SingleQuadParticle

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

`abstract_class` public abstract; extends `net/minecraft/client/particle/Particle`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/client/multiplayer/ClientLevel;DDDLnet/minecraft/clien` | exact | invokespecial@6 in `TerrainParticleMixin.<init>` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (8 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
protected quadSize : F
protected rCol : F
protected gCol : F
protected bCol : F
protected alpha : F
protected roll : F
protected oRoll : F
protected sprite : Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
protected <init>(Lnet/minecraft/client/multiplayer/ClientLevel;DDDLnet/minecraft/client/renderer/texture/TextureAtlasSprite;)V
protected <init>(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/client/renderer/texture/TextureAtlasSprite;)V
public getFacingCameraMode()Lnet/minecraft/client/particle/SingleQuadParticle$FacingCameraMode;
public extract(Lnet/minecraft/client/renderer/state/level/QuadParticleRenderState;Lnet/minecraft/client/Camera;F)V
protected extractRotatedQuad(Lnet/minecraft/client/renderer/state/level/QuadParticleRenderState;Lnet/minecraft/client/Camera;Lorg/joml/Quaternionf;F)V
protected extractRotatedQuad(Lnet/minecraft/client/renderer/state/level/QuadParticleRenderState;Lorg/joml/Quaternionf;FFFF)V
public getQuadSize(F)F
public scale(F)Lnet/minecraft/client/particle/Particle;
public getGroup()Lnet/minecraft/client/particle/ParticleRenderType;
public setSpriteFromAge(Lnet/minecraft/client/particle/SpriteSet;)V
protected setSprite(Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;)V
protected getU0()F
protected getU1()F
protected getV0()F
protected getV1()F
protected abstract getLayer()Lnet/minecraft/client/particle/SingleQuadParticle$Layer;
public setColor(FFF)V
protected setAlpha(F)V
public toString()Ljava/lang/String;
```
