---
type: "interface"
fqcn: "net.minecraft.client.resources.model.sprite.Material$Baked"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.sprite.Material$Baked

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `forceTranslucent` | `()Z` | exact | invokevirtual@45 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `sprite` | `()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | invokevirtual@2 in `MutableQuadView.materialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `sprite` | `()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | invokevirtual@1 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `sprite` | `()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | invokevirtual@29 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `sprite` | `()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | invokevirtual@58 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `sprite` | `()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | invokevirtual@73 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final sprite : Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
private final forceTranslucent : Z
public <init>(Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;Z)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public sprite()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
public forceTranslucent()Z
```
