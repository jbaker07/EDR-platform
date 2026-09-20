---
type: "interface"
fqcn: "net.minecraft.client.particle.ParticleResources$MutableSpriteSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.ParticleResources$MutableSpriteSet

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `first()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `get(II)Lnet/minecraft/client/renderer/texture/TextureAtlasSprit` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `get(Lnet/minecraft/util/RandomSource;)Lnet/minecraft/client/ren` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
class net.minecraft.client.particle.ParticleResources$MutableSpriteSet implements net.minecraft.client.particle.SpriteSet {
    private java.util.List<net.minecraft.client.renderer.texture.TextureAtlasSprite> sprites;
    private net.minecraft.client.particle.ParticleResources$MutableSpriteSet();
    public net.minecraft.client.renderer.texture.TextureAtlasSprite get(int, int);
    public net.minecraft.client.renderer.texture.TextureAtlasSprite get(net.minecraft.util.RandomSource);
    public net.minecraft.client.renderer.texture.TextureAtlasSprite first();
    public void rebind(java.util.List<net.minecraft.client.renderer.texture.TextureAtlasSprite>);
}
```
