---
type: "interface"
fqcn: "com.mojang.renderpearl.api.pipeline.RenderPipeline"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.renderpearl.api.pipeline.RenderPipeline

System: [[20-Systems/com.mojang.renderpearl.api|com.mojang.renderpearl.api]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderPipeline`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `builder` | `([Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;)Lcom/mo` | exact | invokestatic@8 in `FabricRenderPipeline$Snippet.withPipelineDrawModeForGui` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `builder` | `([Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;)Lcom/mo` | exact | invokestatic@8 in `FabricRenderPipeline$Snippet.withoutPipelineDrawModeForGui` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getPrimitiveTopology` | `()Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;` | exact | invokevirtual@5 in `GuiRendererMixin.uploadPrimitivesIndividually` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (13 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final location : Lnet/minecraft/resources/Identifier;
private final shaders : Ljava/util/Map;
private final shaderDefines : Lnet/minecraft/client/renderer/ShaderDefines;
private final bindGroupLayouts : Ljava/util/List;
private final depthStencilState : Lcom/mojang/renderpearl/api/pipeline/DepthStencilState;
private final polygonMode : Lcom/mojang/renderpearl/api/pipeline/PolygonMode;
private final cull : Z
private final colorTargetStates : Ljava/util/List;
private final vertexFormatPerBuffer : Ljava/util/List;
private final primitiveTopology : Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
private final pushConstantSize : I
private final sortKey : I
private static sortKeySeed : I
protected <init>(Lnet/minecraft/resources/Identifier;Ljava/util/Map;Lnet/minecraft/client/renderer/ShaderDefines;Ljava/util/Collection;[Lcom/mojang/renderpearl/api/pipeline/ColorTargetState;Lcom/mojang/renderpearl/api/pipeline/DepthStencilState;Lcom/mojang/renderpearl/api/pipeline/PolygonMode;Z[Lcom/mojang/renderpearl/api/vertex/VertexFormat;Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;II)V
public getSortKey()I
public static updateSortKeySeed()V
public toString()Ljava/lang/String;
public getPolygonMode()Lcom/mojang/renderpearl/api/pipeline/PolygonMode;
public isCull()Z
public getColorTargetStates()Ljava/util/List;
public getDepthStencilState()Lcom/mojang/renderpearl/api/pipeline/DepthStencilState;
public getLocation()Lnet/minecraft/resources/Identifier;
public getVertexFormatBindings()Ljava/util/List;
public getVertexFormatBinding(I)Lcom/mojang/renderpearl/api/vertex/VertexFormat;
public getPrimitiveTopology()Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
public getShaders()Ljava/util/Map;
public getShaderDefines()Lnet/minecraft/client/renderer/ShaderDefines;
public getBindGroupLayouts()Ljava/util/List;
public wantsDepthTexture()Z
public pushConstantSize()I
public static builder([Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
```
