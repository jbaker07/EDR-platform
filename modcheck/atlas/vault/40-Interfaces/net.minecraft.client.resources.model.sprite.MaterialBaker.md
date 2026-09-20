---
type: "interface"
fqcn: "net.minecraft.client.resources.model.sprite.MaterialBaker"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.sprite.MaterialBaker

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/renderer/v1/sprite/FabricMaterialBaker`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `spriteFinder` | `(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadAtlas;)Lnet/fabr` | inherited_exact | invokevirtual@45 in `SimpleUnbakedExtraModel.lambda$bakeResolved$0` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `spriteFinder` | `(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadAtlas;)Lnet/fabr` | inherited_exact | invokevirtual@49 in `SimpleModelWrapperMixin.lambda$analyzeMesh$0` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `blockAtlas` | `Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `itemAtlas` | `Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |

## Declared members (9 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final blockAtlas : Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;
private final itemAtlas : Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;
private final missingSprite : Lnet/minecraft/client/resources/model/sprite/Material$Baked;
private final missingSpriteForceTranslucent : Lnet/minecraft/client/resources/model/sprite/Material$Baked;
private final missingSprites : Lcom/google/common/collect/Multimap;
private final missingReferences : Lcom/google/common/collect/Multimap;
private final bakedMaterials : Ljava/util/Map;
private final bakerFunction : Ljava/util/function/Function;
public <init>(Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;)V
private replacementForMissingMaterial(Lnet/minecraft/client/resources/model/sprite/Material;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public get(Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/resources/model/ModelDebugName;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;
private bake(Lnet/minecraft/client/resources/model/sprite/Material;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;
private static bakeForAtlas(Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public resolveSlot(Lnet/minecraft/client/resources/model/sprite/TextureSlots;Ljava/lang/String;Lnet/minecraft/client/resources/model/ModelDebugName;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public reportMissingReference(Ljava/lang/String;Lnet/minecraft/client/resources/model/ModelDebugName;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public logMissingTextures()V
private static synthetic lambda$logMissingTextures$2(Ljava/lang/String;Ljava/util/Collection;)V
private static synthetic lambda$logMissingTextures$3(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$logMissingTextures$0(Ljava/lang/String;Ljava/util/Collection;)V
private static synthetic lambda$logMissingTextures$1(Lnet/minecraft/resources/Identifier;)Ljava/lang/String;
static <clinit>()V
```
