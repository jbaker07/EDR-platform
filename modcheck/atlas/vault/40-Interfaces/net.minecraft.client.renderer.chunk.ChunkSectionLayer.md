---
type: "interface"
fqcn: "net.minecraft.client.renderer.chunk.ChunkSectionLayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.chunk.ChunkSectionLayer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `byTransparency` | `(Lcom/mojang/blaze3d/platform/Transparency;)Lnet/minecraft/client/rend` | exact | invokestatic@67 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@15 in `ChunkSectionLayerHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@30 in `ChunkSectionLayerHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@45 in `ChunkSectionLayerHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@4 in `ChunkSectionLayerHelper.getMovingBlockRenderType` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@8 in `EncodingFormat.chunkLayer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `translucent` | `()Z` | exact | invokevirtual@8 in `ModelHelper.computeMaterialFlags` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | invokestatic@0 in `ChunkSectionLayerHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | invokestatic@96 in `EncodingFormat.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `CUTOUT` | `Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | getstatic@27 in `ChunkSectionLayerHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `CUTOUT` | `Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | getstatic@47 in `MutableQuadViewImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `SOLID` | `Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | getstatic@1 in `FabricBlockStateModel.lambda$emitQuads$0` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `SOLID` | `Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | getstatic@12 in `ChunkSectionLayerHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `SOLID` | `Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | getstatic@1 in `SingleVariantMixin.lambda$emitQuads$0` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `SOLID` | `Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | getstatic@141 in `AoCalculator.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `TRANSLUCENT` | `Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | getstatic@42 in `ChunkSectionLayerHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `TRANSLUCENT` | `Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | getstatic@7 in `MovingBlockFeatureRendererMixin$1.accept` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (9 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SOLID : Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;
public static final CUTOUT : Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;
public static final TRANSLUCENT : Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;
private final pipeline : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
private final multiDrawPipeline : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
private final bufferSize : I
private final translucent : Z
private final label : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;
public static values()[Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;
private <init>(Ljava/lang/String;ILcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;IZ)V
public static byTransparency(Lcom/mojang/blaze3d/platform/Transparency;)Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;
public pipeline(Z)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public bufferSize()I
public label()Ljava/lang/String;
public translucent()Z
public vertexFormat()Lcom/mojang/renderpearl/api/vertex/VertexFormat;
private static synthetic $values()[Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;
static <clinit>()V
```
