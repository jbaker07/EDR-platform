---
type: "interface"
fqcn: "net.minecraft.client.resources.model.ResolvedModel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.ResolvedModel

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/client/resources/model/ModelDebugName`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `bakeTopGeometry` | `(Lnet/minecraft/client/resources/model/sprite/TextureSlots;Lnet/minecr` | exact | invokeinterface@29 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `debugName` | `()Ljava/lang/String;` | inherited_exact | invokeinterface@196 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getTopAmbientOcclusion` | `()Z` | exact | invokeinterface@8 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getTopTextureSlots` | `()Lnet/minecraft/client/resources/model/sprite/TextureSlots;` | exact | invokeinterface@1 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `resolveParticleMaterial` | `(Lnet/minecraft/client/resources/model/sprite/TextureSlots;Lnet/minecr` | exact | invokeinterface@18 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (2 fields, 17 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DEFAULT_AMBIENT_OCCLUSION : Z
public static final DEFAULT_GUI_LIGHT : Lnet/minecraft/client/resources/model/UnbakedModel$GuiLight;
public abstract wrapped()Lnet/minecraft/client/resources/model/UnbakedModel;
public abstract parent()Lnet/minecraft/client/resources/model/ResolvedModel;
public static findTopTextureSlots(Lnet/minecraft/client/resources/model/ResolvedModel;)Lnet/minecraft/client/resources/model/sprite/TextureSlots;
public getTopTextureSlots()Lnet/minecraft/client/resources/model/sprite/TextureSlots;
public static findTopAmbientOcclusion(Lnet/minecraft/client/resources/model/ResolvedModel;)Z
public getTopAmbientOcclusion()Z
public static findTopGuiLight(Lnet/minecraft/client/resources/model/ResolvedModel;)Lnet/minecraft/client/resources/model/UnbakedModel$GuiLight;
public getTopGuiLight()Lnet/minecraft/client/resources/model/UnbakedModel$GuiLight;
public static findTopGeometry(Lnet/minecraft/client/resources/model/ResolvedModel;)Lnet/minecraft/client/resources/model/geometry/UnbakedGeometry;
public getTopGeometry()Lnet/minecraft/client/resources/model/geometry/UnbakedGeometry;
public bakeTopGeometry(Lnet/minecraft/client/resources/model/sprite/TextureSlots;Lnet/minecraft/client/resources/model/ModelBaker;Lnet/minecraft/client/renderer/block/dispatch/ModelState;)Lnet/minecraft/client/resources/model/geometry/QuadCollection;
public static resolveParticleMaterial(Lnet/minecraft/client/resources/model/sprite/TextureSlots;Lnet/minecraft/client/resources/model/ModelBaker;Lnet/minecraft/client/resources/model/ModelDebugName;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public resolveParticleMaterial(Lnet/minecraft/client/resources/model/sprite/TextureSlots;Lnet/minecraft/client/resources/model/ModelBaker;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public static findTopTransform(Lnet/minecraft/client/resources/model/ResolvedModel;Lnet/minecraft/world/item/ItemDisplayContext;)Lnet/minecraft/client/resources/model/cuboid/ItemTransform;
public static findTopTransforms(Lnet/minecraft/client/resources/model/ResolvedModel;)Lnet/minecraft/client/resources/model/cuboid/ItemTransforms;
public getTopTransforms()Lnet/minecraft/client/resources/model/cuboid/ItemTransforms;
static <clinit>()V
```
