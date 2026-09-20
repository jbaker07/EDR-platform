---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.FeatureFrameContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.FeatureFrameContext

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `blockColors` | `()Lnet/minecraft/client/color/block/BlockColors;` | exact | invokevirtual@14 in `MovingBlockFeatureRendererMixin.beforeInitBlockRenderer` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `options` | `()Lnet/minecraft/client/renderer/state/OptionsRenderState;` | exact | invokevirtual@6 in `MovingBlockFeatureRendererMixin.beforeInitBlockRenderer` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (8 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final options : Lnet/minecraft/client/renderer/state/OptionsRenderState;
private final font : Lnet/minecraft/client/gui/Font;
private final blockStateModelSet : Lnet/minecraft/client/renderer/block/BlockStateModelSet;
private final blockColors : Lnet/minecraft/client/color/block/BlockColors;
private final textureManager : Lnet/minecraft/client/renderer/texture/TextureManager;
private final atlasManager : Lnet/minecraft/client/resources/model/sprite/AtlasManager;
private final lightmap : Lcom/mojang/renderpearl/api/textures/GpuTextureView;
private final stagedVertexBuffer : Lnet/minecraft/client/renderer/StagedVertexBuffer;
public <init>(Lnet/minecraft/client/renderer/state/OptionsRenderState;Lnet/minecraft/client/gui/Font;Lnet/minecraft/client/renderer/block/BlockStateModelSet;Lnet/minecraft/client/color/block/BlockColors;Lnet/minecraft/client/renderer/texture/TextureManager;Lnet/minecraft/client/resources/model/sprite/AtlasManager;Lcom/mojang/renderpearl/api/textures/GpuTextureView;Lnet/minecraft/client/renderer/StagedVertexBuffer;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public options()Lnet/minecraft/client/renderer/state/OptionsRenderState;
public font()Lnet/minecraft/client/gui/Font;
public blockStateModelSet()Lnet/minecraft/client/renderer/block/BlockStateModelSet;
public blockColors()Lnet/minecraft/client/color/block/BlockColors;
public textureManager()Lnet/minecraft/client/renderer/texture/TextureManager;
public atlasManager()Lnet/minecraft/client/resources/model/sprite/AtlasManager;
public lightmap()Lcom/mojang/renderpearl/api/textures/GpuTextureView;
public stagedVertexBuffer()Lnet/minecraft/client/renderer/StagedVertexBuffer;
```
