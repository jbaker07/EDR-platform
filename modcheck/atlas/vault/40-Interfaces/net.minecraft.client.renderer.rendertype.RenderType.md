---
type: "interface"
fqcn: "net.minecraft.client.renderer.rendertype.RenderType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.rendertype.RenderType

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `hasBlending` | `()Z` | exact | invokevirtual@6 in `MeshViewRenderTypeGroups$FilteredMeshView.lambda$forEach$0` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasBlending` | `()Z` | exact | invokevirtual@7 in `MeshViewRenderTypeGroups.lambda$split$0` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasBlending` | `()Z` | exact | invokevirtual@134 in `BlockModelRenderStateMixin.submitMesh` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasBlending` | `()Z` | exact | invokevirtual@170 in `BlockModelRenderStateMixin.submitMesh` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `isOutline` | `()Z` | exact | invokevirtual@12 in `SubmitNodeCollectionMixin.lambda$submitBlockModel$0` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `outline` | `()Ljava/util/Optional;` | exact | invokevirtual@31 in `MovingBlockFeatureRendererMixin$1.accept` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `outline` | `()Ljava/util/Optional;` | exact | invokevirtual@68 in `MovingBlockFeatureRendererMixin$1.accept` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `outline` | `()Ljava/util/Optional;` | exact | invokevirtual@4 in `ExtendedItemFeatureRenderer.bufferOutline` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (8 fields, 17 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final MEGABYTE : I
public static final BIG_BUFFER_SIZE : I
public static final SMALL_BUFFER_SIZE : I
public static final TRANSIENT_BUFFER_SIZE : I
private final state : Lnet/minecraft/client/renderer/rendertype/RenderSetup;
private final hasBlending : Z
private final outline : Ljava/util/Optional;
protected final name : Ljava/lang/String;
private <init>(Ljava/lang/String;Lnet/minecraft/client/renderer/rendertype/RenderSetup;)V
public static create(Ljava/lang/String;Lnet/minecraft/client/renderer/rendertype/RenderSetup;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public toString()Ljava/lang/String;
public hasBlending()Z
private calculateHasBlending()Z
public prepare()Lnet/minecraft/client/renderer/rendertype/PreparedRenderType;
private writeDynamicTransforms(Lorg/joml/Matrix4f;)Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;
public format()Lcom/mojang/renderpearl/api/vertex/VertexFormat;
public primitiveTopology()Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
public outline()Ljava/util/Optional;
public isOutline()Z
public pipeline()Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public affectsCrumbling()Z
public canConsolidateConsecutiveGeometry()Z
public sortOnUpload()Z
public forceSolidModelPhase()Z
private static synthetic lambda$new$0(Lnet/minecraft/client/renderer/rendertype/RenderSetup;Lnet/minecraft/client/renderer/rendertype/RenderSetup$TextureBinding;)Lnet/minecraft/client/renderer/rendertype/RenderType;
```
