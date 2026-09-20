---
type: "interface"
fqcn: "net.minecraft.client.renderer.entity.EntityRenderDispatcher"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.entity.EntityRenderDispatcher

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/server/packs/resources/ResourceManagerReloadListener`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `onResourceManagerReload` | `(Lnet/minecraft/server/packs/resources/ResourceManager;)V` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (16 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private renderers : Ljava/util/Map;
private playerRenderers : Ljava/util/Map;
private mannequinRenderers : Ljava/util/Map;
public final textureManager : Lnet/minecraft/client/renderer/texture/TextureManager;
public camera : Lnet/minecraft/client/Camera;
public crosshairPickEntity : Lnet/minecraft/world/entity/Entity;
private final blockModelResolver : Lnet/minecraft/client/renderer/block/BlockModelResolver;
private final itemModelResolver : Lnet/minecraft/client/renderer/item/ItemModelResolver;
private final mapRenderer : Lnet/minecraft/client/renderer/MapRenderer;
private final atlasManager : Lnet/minecraft/client/resources/model/sprite/AtlasManager;
private final font : Lnet/minecraft/client/gui/Font;
public final options : Lnet/minecraft/client/Options;
private final entityModels : Ljava/util/function/Supplier;
private final equipmentAssets : Lnet/minecraft/client/resources/model/EquipmentAssetManager;
private final playerSkinRenderCache : Lnet/minecraft/client/renderer/PlayerSkinRenderCache;
private final palettedTextures : Lnet/minecraft/client/resources/palette/PalettedTextureManager;
public getPackedLightCoords(Lnet/minecraft/world/entity/Entity;F)I
public <init>(Lnet/minecraft/client/renderer/texture/TextureManager;Lnet/minecraft/client/renderer/block/BlockModelResolver;Lnet/minecraft/client/renderer/item/ItemModelResolver;Lnet/minecraft/client/renderer/MapRenderer;Lnet/minecraft/client/resources/model/sprite/AtlasManager;Lnet/minecraft/client/gui/Font;Lnet/minecraft/client/Options;Ljava/util/function/Supplier;Lnet/minecraft/client/resources/model/EquipmentAssetManager;Lnet/minecraft/client/renderer/PlayerSkinRenderCache;Lnet/minecraft/client/resources/palette/PalettedTextureManager;)V
public getRenderer(Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/client/renderer/entity/EntityRenderer;
private getAvatarRenderer(Ljava/util/Map;Lnet/minecraft/world/entity/Avatar;)Lnet/minecraft/client/renderer/entity/player/AvatarRenderer;
public getRenderer(Lnet/minecraft/client/renderer/entity/state/AvatarRenderState;)Lnet/minecraft/client/renderer/entity/player/AvatarRenderer;
public getRenderer(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;)Lnet/minecraft/client/renderer/entity/EntityRenderer;
public prepare(Lnet/minecraft/client/Camera;Lnet/minecraft/world/entity/Entity;)V
public shouldRender(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/client/renderer/culling/Frustum;DDDF)Z
public extractEntity(Lnet/minecraft/world/entity/Entity;F)Lnet/minecraft/client/renderer/entity/state/EntityRenderState;
public submit(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lnet/minecraft/client/renderer/state/level/CameraRenderState;DDDLcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;)V
private fillRendererDetails(Lnet/minecraft/client/renderer/entity/EntityRenderer;Lnet/minecraft/CrashReport;)Lnet/minecraft/CrashReportCategory;
public resetCamera()V
public distanceToSqr(Lnet/minecraft/world/entity/Entity;)D
public onResourceManagerReload(Lnet/minecraft/server/packs/resources/ResourceManager;)V
```
