---
type: "interface"
fqcn: "com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder

System: [[20-Systems/com.mojang.renderpearl.api|com.mojang.renderpearl.api]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderPipeline$Builder`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `buildSnippet` | `()Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;` | exact | invokevirtual@17 in `FabricRenderPipeline$Snippet.withPipelineDrawModeForGui` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `buildSnippet` | `()Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;` | exact | invokevirtual@16 in `FabricRenderPipeline$Snippet.withoutPipelineDrawModeForGui` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `build` | `()Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;` | name_only | @ModifyReturnValue at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `withSnippet` | `(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;)Lcom/moj` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `buildSnippet` | `()Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;` | name_only | @WrapOperation at ['NEW'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (13 fields, 26 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static nextPipelineSortKey : I
private location : Ljava/util/Optional;
private final shaders : Ljava/util/Map;
private definesBuilder : Ljava/util/Optional;
private bindGroupLayouts : Ljava/util/Optional;
private depthStencilState : Ljava/util/Optional;
private polygonMode : Ljava/util/Optional;
private cull : Ljava/util/Optional;
private final colorTargetStates : [Lcom/mojang/renderpearl/api/pipeline/ColorTargetState;
private activeColorTargetStateCount : I
private final vertexFormatPerBuffer : [Lcom/mojang/renderpearl/api/vertex/VertexFormat;
private primitiveTopology : Ljava/util/Optional;
private pushConstantSize : I
private <init>()V
public withLocation(Ljava/lang/String;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withLocation(Lnet/minecraft/resources/Identifier;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withFragmentShader(Ljava/lang/String;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withFragmentShader(Lnet/minecraft/resources/Identifier;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withVertexShader(Ljava/lang/String;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withVertexShader(Lnet/minecraft/resources/Identifier;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withShaderDefine(Ljava/lang/String;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withShaderDefine(Ljava/lang/String;I)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withShaderDefine(Ljava/lang/String;F)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withBindGroupLayout(Lcom/mojang/renderpearl/api/pipeline/BindGroupLayout;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withPolygonMode(Lcom/mojang/renderpearl/api/pipeline/PolygonMode;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withCull(Z)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withColorTargetState(ILcom/mojang/renderpearl/api/pipeline/ColorTargetState;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withColorTargetStates(IILjava/util/function/Supplier;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withUnusedColorTargetState(I)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withColorTargetState(Lcom/mojang/renderpearl/api/pipeline/ColorTargetState;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withDepthStencilState(Lcom/mojang/renderpearl/api/pipeline/DepthStencilState;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withDepthStencilState(Ljava/util/Optional;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withVertexBinding(ILcom/mojang/renderpearl/api/vertex/VertexFormat;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withPrimitiveTopology(Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withPushConstantSize(I)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public withSnippet(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;
public buildSnippet()Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public build()Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
private synthetic lambda$withSnippet$0(Ljava/util/List;)V
```
