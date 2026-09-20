---
type: "interface"
fqcn: "net.minecraft.client.particle.ParticleResources$MutableSpriteSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.ParticleResources$MutableSpriteSet

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

`class` ; extends `java/lang/Object`; implements `net/minecraft/client/particle/SpriteSet`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@4 in `ParticleProviderRegistryImpl$DirectParticleProviderRegistry.register` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `first` | `()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | invokevirtual@4 in `FabricSpriteSetImpl.first` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `get` | `(II)Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | invokevirtual@6 in `FabricSpriteSetImpl.get` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/util/RandomSource;)Lnet/minecraft/client/renderer/text` | exact | invokevirtual@5 in `FabricSpriteSetImpl.get` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `sprites` | `Ljava/util/List;` | exact | getfield@4 in `FabricSpriteSetImpl.getSprites` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (1 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private sprites : Ljava/util/List;
private <init>()V
public get(II)Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
public get(Lnet/minecraft/util/RandomSource;)Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
public first()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
public rebind(Ljava/util/List;)V
```
