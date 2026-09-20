---
type: "interface"
fqcn: "com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder

System: [[20-Systems/com.mojang.renderpearl.api|com.mojang.renderpearl.api]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `withSnippet` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (39, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder {
    private static int nextPipelineSortKey;
    private java.util.Optional<net.minecraft.resources.Identifier> location;
    private final java.util.Map<com.mojang.renderpearl.api.pipeline.ShaderType, net.minecraft.resources.Identifier> shaders;
    private java.util.Optional<net.minecraft.client.renderer.ShaderDefines$Builder> definesBuilder;
    private java.util.Optional<java.util.Set<com.mojang.renderpearl.api.pipeline.BindGroupLayout>> bindGroupLayouts;
    private java.util.Optional<com.mojang.renderpearl.api.pipeline.DepthStencilState> depthStencilState;
    private java.util.Optional<com.mojang.renderpearl.api.pipeline.PolygonMode> polygonMode;
    private java.util.Optional<java.lang.Boolean> cull;
    private final com.mojang.renderpearl.api.pipeline.ColorTargetState[] colorTargetStates;
    private int activeColorTargetStateCount;
    private final com.mojang.renderpearl.api.vertex.VertexFormat[] vertexFormatPerBuffer;
    private java.util.Optional<com.mojang.renderpearl.api.pipeline.PrimitiveTopology> primitiveTopology;
    private int pushConstantSize;
    private com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder();
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withLocation(java.lang.String);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withLocation(net.minecraft.resources.Identifier);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withFragmentShader(java.lang.String);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withFragmentShader(net.minecraft.resources.Identifier);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withVertexShader(java.lang.String);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withVertexShader(net.minecraft.resources.Identifier);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withShaderDefine(java.lang.String);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withShaderDefine(java.lang.String, int);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withShaderDefine(java.lang.String, float);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withBindGroupLayout(com.mojang.renderpearl.api.pipeline.BindGroupLayout);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withPolygonMode(com.mojang.renderpearl.api.pipeline.PolygonMode);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withCull(boolean);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withColorTargetState(int, com.mojang.renderpearl.api.pipeline.ColorTargetState);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withColorTargetStates(int, int, java.util.function.Supplier<com.mojang.renderpearl.api.pipeline.ColorTargetState>);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withUnusedColorTargetState(int);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withColorTargetState(com.mojang.renderpearl.api.pipeline.ColorTargetState);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withDepthStencilState(com.mojang.renderpearl.api.pipeline.DepthStencilState);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withDepthStencilState(java.util.Optional<com.mojang.renderpearl.api.pipeline.DepthStencilState>);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withVertexBinding(int, com.mojang.renderpearl.api.vertex.VertexFormat);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withPrimitiveTopology(com.mojang.renderpearl.api.pipeline.PrimitiveTopology);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withPushConstantSize(int);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Builder withSnippet(com.mojang.renderpearl.api.pipeline.RenderPipeline$Snippet);
    public com.mojang.renderpearl.api.pipeline.RenderPipeline$Snippet buildSnippet();
    public com.mojang.renderpearl.api.pipeline.RenderPipeline build();
    private void lambda$withSnippet$0(java.util.List);
}
```
