---
type: "interface"
fqcn: "net.minecraft.client.particle.SingleQuadParticle"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.SingleQuadParticle

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/client/multiplayer/ClientLevel;DDDLnet/minec` | `` | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.client.particle.SingleQuadParticle extends net.minecraft.client.particle.Particle {
    protected float quadSize;
    protected float rCol;
    protected float gCol;
    protected float bCol;
    protected float alpha;
    protected float roll;
    protected float oRoll;
    protected net.minecraft.client.renderer.texture.TextureAtlasSprite sprite;
    protected net.minecraft.client.particle.SingleQuadParticle(net.minecraft.client.multiplayer.ClientLevel, double, double, double, net.minecraft.client.renderer.texture.TextureAtlasSprite);
    protected net.minecraft.client.particle.SingleQuadParticle(net.minecraft.client.multiplayer.ClientLevel, double, double, double, double, double, double, net.minecraft.client.renderer.texture.TextureAtlasSprite);
    public net.minecraft.client.particle.SingleQuadParticle$FacingCameraMode getFacingCameraMode();
    public void extract(net.minecraft.client.renderer.state.level.QuadParticleRenderState, net.minecraft.client.Camera, float);
    protected void extractRotatedQuad(net.minecraft.client.renderer.state.level.QuadParticleRenderState, net.minecraft.client.Camera, org.joml.Quaternionf, float);
    protected void extractRotatedQuad(net.minecraft.client.renderer.state.level.QuadParticleRenderState, org.joml.Quaternionf, float, float, float, float);
    public float getQuadSize(float);
    public net.minecraft.client.particle.Particle scale(float);
    public net.minecraft.client.particle.ParticleRenderType getGroup();
    public void setSpriteFromAge(net.minecraft.client.particle.SpriteSet);
    protected void setSprite(net.minecraft.client.renderer.texture.TextureAtlasSprite);
    protected float getU0();
    protected float getU1();
    protected float getV0();
    protected float getV1();
    protected abstract net.minecraft.client.particle.SingleQuadParticle$Layer getLayer();
    public void setColor(float, float, float);
    protected void setAlpha(float);
    public java.lang.String toString();
}
```
