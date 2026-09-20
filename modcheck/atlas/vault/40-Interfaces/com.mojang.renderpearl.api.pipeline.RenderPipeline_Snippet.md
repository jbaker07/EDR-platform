---
type: "interface"
fqcn: "com.mojang.renderpearl.api.pipeline.RenderPipeline$Snippet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.renderpearl.api.pipeline.RenderPipeline$Snippet

System: [[20-Systems/com.mojang.renderpearl.api|com.mojang.renderpearl.api]]

`record` public final; extends `java/lang/Record`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderPipeline$Snippet`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `equals` | `(Ljava/lang/Object;)Z` | name_only | @ModifyReturnValue at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `hashCode` | `()I` | name_only | @ModifyReturnValue at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `toString` | `()Ljava/lang/String;` | name_only | @ModifyReturnValue at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (11 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final shaders : Ljava/util/Map;
private final shaderDefines : Ljava/util/Optional;
private final bindGroupLayouts : Ljava/util/Optional;
private final colorTargetStates : [Lcom/mojang/renderpearl/api/pipeline/ColorTargetState;
private final activeColorTargetStateCount : I
private final depthStencilState : Ljava/util/Optional;
private final polygonMode : Ljava/util/Optional;
private final cull : Ljava/util/Optional;
private final vertexFormatPerBuffer : [Lcom/mojang/renderpearl/api/vertex/VertexFormat;
private final vertexFormatMode : Ljava/util/Optional;
private final pushConstantSize : I
public <init>(Ljava/util/Map;Ljava/util/Optional;Ljava/util/Optional;[Lcom/mojang/renderpearl/api/pipeline/ColorTargetState;ILjava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;[Lcom/mojang/renderpearl/api/vertex/VertexFormat;Ljava/util/Optional;I)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public shaders()Ljava/util/Map;
public shaderDefines()Ljava/util/Optional;
public bindGroupLayouts()Ljava/util/Optional;
public colorTargetStates()[Lcom/mojang/renderpearl/api/pipeline/ColorTargetState;
public activeColorTargetStateCount()I
public depthStencilState()Ljava/util/Optional;
public polygonMode()Ljava/util/Optional;
public cull()Ljava/util/Optional;
public vertexFormatPerBuffer()[Lcom/mojang/renderpearl/api/vertex/VertexFormat;
public vertexFormatMode()Ljava/util/Optional;
public pushConstantSize()I
```
