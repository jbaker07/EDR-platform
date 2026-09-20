---
type: "interface"
fqcn: "net.minecraft.client.resources.model.UnbakedModel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.UnbakedModel

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `ambientOcclusion` | `()Ljava/lang/Boolean;` | exact | invokeinterface@4 in `WrapperUnbakedModel.ambientOcclusion` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `geometry` | `()Lnet/minecraft/client/resources/model/geometry/UnbakedGeometry;` | exact | invokeinterface@4 in `WrapperUnbakedModel.geometry` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `guiLight` | `()Lnet/minecraft/client/resources/model/UnbakedModel$GuiLight;` | exact | invokeinterface@4 in `WrapperUnbakedModel.guiLight` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `parent` | `()Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@4 in `WrapperUnbakedModel.parent` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `textureSlots` | `()Lnet/minecraft/client/resources/model/sprite/TextureSlots$Data;` | exact | invokeinterface@4 in `WrapperUnbakedModel.textureSlots` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `transforms` | `()Lnet/minecraft/client/resources/model/cuboid/ItemTransforms;` | exact | invokeinterface@4 in `WrapperUnbakedModel.transforms` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (1 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final PARTICLE_TEXTURE_REFERENCE : Ljava/lang/String;
public ambientOcclusion()Ljava/lang/Boolean;
public guiLight()Lnet/minecraft/client/resources/model/UnbakedModel$GuiLight;
public transforms()Lnet/minecraft/client/resources/model/cuboid/ItemTransforms;
public textureSlots()Lnet/minecraft/client/resources/model/sprite/TextureSlots$Data;
public geometry()Lnet/minecraft/client/resources/model/geometry/UnbakedGeometry;
public parent()Lnet/minecraft/resources/Identifier;
```
