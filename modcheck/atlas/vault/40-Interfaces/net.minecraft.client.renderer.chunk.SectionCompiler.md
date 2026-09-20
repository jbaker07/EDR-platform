---
type: "interface"
fqcn: "net.minecraft.client.renderer.chunk.SectionCompiler"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.chunk.SectionCompiler

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getOrBeginLayer` | `(Ljava/util/Map;Lnet/minecraft/client/renderer/SectionBufferBuilderPac` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| injects_into | `compile` | `(Lnet/minecraft/core/SectionPos;Lnet/minecraft/client/renderer/chunk/R` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `ambientOcclusion` | `Z` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `blockColors` | `Lnet/minecraft/client/color/block/BlockColors;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| wraps | `compile` | `(Lnet/minecraft/core/SectionPos;Lnet/minecraft/client/renderer/chunk/R` | name_only | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (5 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final ambientOcclusion : Z
private final cutoutLeaves : Z
private final blockModelSet : Lnet/minecraft/client/renderer/block/BlockStateModelSet;
private final fluidModelSet : Lnet/minecraft/client/renderer/block/FluidStateModelSet;
private final blockColors : Lnet/minecraft/client/color/block/BlockColors;
public <init>(ZZLnet/minecraft/client/renderer/block/BlockStateModelSet;Lnet/minecraft/client/renderer/block/FluidStateModelSet;Lnet/minecraft/client/color/block/BlockColors;)V
public compile(Lnet/minecraft/core/SectionPos;Lnet/minecraft/client/renderer/chunk/RenderSectionRegion;Lcom/mojang/blaze3d/vertex/VertexSorting;Lnet/minecraft/client/renderer/SectionBufferBuilderPack;)Lnet/minecraft/client/renderer/chunk/SectionCompiler$Results;
private getOrBeginLayer(Ljava/util/Map;Lnet/minecraft/client/renderer/SectionBufferBuilderPack;Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;)Lcom/mojang/blaze3d/vertex/BufferBuilder;
private handleBlockEntity(Lnet/minecraft/client/renderer/chunk/SectionCompiler$Results;Lnet/minecraft/world/level/block/entity/BlockEntity;)V
private synthetic lambda$compile$2(Ljava/util/Map;Lnet/minecraft/client/renderer/SectionBufferBuilderPack;Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;)Lcom/mojang/blaze3d/vertex/VertexConsumer;
private synthetic lambda$compile$1(Ljava/util/Map;Lnet/minecraft/client/renderer/SectionBufferBuilderPack;FFFLnet/minecraft/client/resources/model/geometry/BakedQuad;Lcom/mojang/blaze3d/vertex/QuadInstance;)V
private synthetic lambda$compile$0(Ljava/util/Map;Lnet/minecraft/client/renderer/SectionBufferBuilderPack;FFFLnet/minecraft/client/resources/model/geometry/BakedQuad;Lcom/mojang/blaze3d/vertex/QuadInstance;)V
```
