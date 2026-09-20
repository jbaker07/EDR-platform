---
type: "interface"
fqcn: "net.minecraft.client.renderer.LevelRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.LevelRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `hasRenderedAllSections` | `()Z` | exact | invokevirtual@32 in `TestServerConnectionImpl.areChunksRendered` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `executeClassicTransparency` | `(Lnet/minecraft/client/renderer/chunk/ChunkSectionsToRender;Lnet/minec` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `executeOit` | `(Lnet/minecraft/client/renderer/chunk/ChunkSectionsToRender;Lnet/minec` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `executeSolid` | `(Lnet/minecraft/client/renderer/chunk/ChunkSectionsToRender;Lnet/minec` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `lambda$addMainPass$0` | `(Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;ZLnet/minecraft/cl` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `prepareChunkRenders` | `(Lorg/joml/Matrix4fc;Z)Lnet/minecraft/client/renderer/chunk/ChunkSecti` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `prepareChunkRendersIndirect` | `(Lorg/joml/Matrix4fc;Z)Lnet/minecraft/client/renderer/chunk/ChunkSecti` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `render` | `(Lcom/mojang/blaze3d/resource/GraphicsResourceAllocator;ZLnet/minecraf` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `submitBlockDestroyAnimation` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/S` | exact | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `submitBlockOutline` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/S` | name_only | @Inject at ['FIELD'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `submitFeatures` | `(Lnet/minecraft/client/renderer/state/level/LevelRenderState;Lnet/mine` | name_only | @ModifyExpressionValue at ['NEW'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `submitFeatures` | `(Lnet/minecraft/client/renderer/state/level/LevelRenderState;Lnet/mine` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `submitFeatures` | `(Lnet/minecraft/client/renderer/state/level/LevelRenderState;Lnet/mine` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `levelRenderState` | `Lnet/minecraft/client/renderer/state/level/LevelRenderState;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| reads | `renderBuffers` | `Lnet/minecraft/client/renderer/RenderBuffers;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| reads | `submitNodeStorage` | `Lnet/minecraft/client/renderer/SubmitNodeStorage;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| wraps | `executeClassicTransparency` | `(Lnet/minecraft/client/renderer/chunk/ChunkSectionsToRender;Lnet/minec` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `executeSolid` | `(Lnet/minecraft/client/renderer/chunk/ChunkSectionsToRender;Lnet/minec` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `submitBlockDestroyAnimation` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/S` | exact | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| wraps | `submitBlockDestroyAnimation` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/S` | exact | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (41 fields, 73 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final OIT_WAVELET_RANK : I
public static final OIT_COEFFICIENT_COUNT : I
public static final OIT_TRANSMITTANCE_TARGET_COUNT : I
private static final ENTITY_OUTLINE_POST_CHAIN_ID : Lnet/minecraft/resources/Identifier;
private static final MINIMUM_TRANSPARENT_SORT_COUNT : I
private static final CHUNK_VISIBILITY_THRESHOLD : F
private static final DEPTH_BOUNDS_CLEAR_COLOR : Lorg/joml/Vector4fc;
private static final ZERO_CLEAR_COLOR : Lorg/joml/Vector4fc;
private final gameRenderer : Lnet/minecraft/client/renderer/GameRenderer;
private final entityRenderDispatcher : Lnet/minecraft/client/renderer/entity/EntityRenderDispatcher;
private final blockEntityRenderDispatcher : Lnet/minecraft/client/renderer/blockentity/BlockEntityRenderDispatcher;
private final renderBuffers : Lnet/minecraft/client/renderer/RenderBuffers;
private final featureRenderDispatcher : Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher;
private final submitNodeStorage : Lnet/minecraft/client/renderer/SubmitNodeStorage;
private final modelManager : Lnet/minecraft/client/resources/model/ModelManager;
private final textureManager : Lnet/minecraft/client/renderer/texture/TextureManager;
private final atlasManager : Lnet/minecraft/client/resources/model/sprite/AtlasManager;
private final shaderManager : Lnet/minecraft/client/renderer/ShaderManager;
private final levelRenderState : Lnet/minecraft/client/renderer/state/level/LevelRenderState;
private final optionsRenderState : Lnet/minecraft/client/renderer/state/OptionsRenderState;
private skyRenderer : Lnet/minecraft/client/renderer/SkyRenderer;
private final cloudRenderer : Lnet/minecraft/client/renderer/CloudRenderer;
private final worldBorderRenderer : Lnet/minecraft/client/renderer/WorldBorderRenderer;
private final weatherEffectRenderer : Lnet/minecraft/client/renderer/WeatherEffectRenderer;
private final sectionOcclusionGraph : Lnet/minecraft/client/renderer/SectionOcclusionGraph;
private final visibleSections : Lit/unimi/dsi/fastutil/objects/ObjectArrayList;
private final nearbyVisibleSections : Lit/unimi/dsi/fastutil/objects/ObjectArrayList;
private final transientBlocks : Lit/unimi/dsi/fastutil/longs/Long2ObjectLinkedOpenHashMap;
private final transientBlockRemovalQueue : Ljava/util/concurrent/BlockingQueue;
private viewArea : Lnet/minecraft/client/renderer/ViewArea;
private final entityOutlineTarget : Lcom/mojang/blaze3d/pipeline/RenderTarget;
private final targets : Lnet/minecraft/client/renderer/LevelTargetBundle;
private sectionRenderDispatcher : Lnet/minecraft/client/renderer/chunk/SectionRenderDispatcher;
private lastTranslucentSortBlockPos : Lnet/minecraft/core/BlockPos;
private translucencyResortIterationIndex : I
private chunkLayerSampler : Lcom/mojang/renderpearl/api/textures/GpuSampler;
private currentFrameRendersEntityOutline : Z
private final multiDrawIndirectAvailable : Z
private usingMultiDrawIndirectForTerrain : Z
private final renderThreadGizmos : Lnet/minecraft/gizmos/SimpleGizmoCollector;
private finalizedGizmos : Lnet/minecraft/client/renderer/LevelRenderer$FinalizedGizmos;
public <init>(Lnet/minecraft/client/renderer/entity/EntityRenderDispatcher;Lnet/minecraft/client/renderer/blockentity/BlockEntityRenderDispatcher;Lnet/minecraft/client/resources/model/ModelManager;Lnet/minecraft/client/renderer/texture/TextureManager;Lnet/minecraft/client/resources/model/sprite/AtlasManager;Lnet/minecraft/client/renderer/ShaderManager;Lnet/minecraft/client/renderer/GameRenderer;II)V
public render(Lcom/mojang/blaze3d/resource/GraphicsResourceAllocator;ZLnet/minecraft/client/renderer/state/level/CameraRenderState;Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;Lorg/joml/Vector4f;ZZ)V
private submitFeatures(Lnet/minecraft/client/renderer/state/level/LevelRenderState;Lnet/minecraft/client/renderer/SubmitNodeCollector;Z)V
private repositionCamera(Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V
private addSkyPass(Lcom/mojang/blaze3d/framegraph/FrameGraphBuilder;Lnet/minecraft/client/renderer/state/level/CameraRenderState;Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;)V
private addMainPass(Lcom/mojang/blaze3d/framegraph/FrameGraphBuilder;Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;Lnet/minecraft/client/renderer/chunk/ChunkSectionsToRender;Z)V
private executeSeeThrough(Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;Lcom/mojang/blaze3d/pipeline/RenderTarget;)V
private executeAlwaysOnTop(Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;Lcom/mojang/blaze3d/pipeline/RenderTarget;Z)V
private frameHasAlwaysOnTopGizmos()Z
private executeSolid(Lnet/minecraft/client/renderer/chunk/ChunkSectionsToRender;Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;Lcom/mojang/renderpearl/api/commands/RenderPass;)V
private prepareTranslucents()V
private executeOit(Lnet/minecraft/client/renderer/chunk/ChunkSectionsToRender;Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;)V
private executeDepthBoundsCull()V
private executeOitWaterMask(Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;Lcom/mojang/blaze3d/pipeline/RenderTarget;)V
private executeClassicTransparency(Lnet/minecraft/client/renderer/chunk/ChunkSectionsToRender;Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;Lcom/mojang/renderpearl/api/commands/RenderPass;)V
private executeOutline(Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;)V
private extractSectionDrawGroups(ZLjava/util/List;Ljava/util/Map;)I
public prepareChunkRenders(Lorg/joml/Matrix4fc;Z)Lnet/minecraft/client/renderer/chunk/ChunkSectionsToRender;
public addTransientBlock(JLnet/minecraft/client/renderer/state/level/TransientBlockRenderState;)V
public removeTransientBlocksInSection(JJ)V
private performTransientBlockRemovals()V
public prepareChunkRendersIndirect(Lorg/joml/Matrix4fc;Z)Lnet/minecraft/client/renderer/chunk/ChunkSectionsToRender;
private compileSections(Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V
private checkPoseStack(Lcom/mojang/blaze3d/vertex/PoseStack;)V
private submitEntities(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/state/level/LevelRenderState;Lnet/minecraft/client/renderer/SubmitNodeCollector;)V
private submitBlockEntities(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/state/level/LevelRenderState;Lnet/minecraft/client/renderer/SubmitNodeCollector;)V
private submitBlockDestroyAnimation(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V
private submitTransientBlocks(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V
private submitBlockOutline(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V
private submitHitOutline(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/rendertype/RenderType;Lnet/minecraft/client/renderer/state/level/BlockOutlineRenderState;IFZ)V
public resize(II)V
public endFrame()V
public close()V
public blitEntityOutline()V
public invalidateCompiledGeometry(Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/client/Options;Lnet/minecraft/client/Camera;Lnet/minecraft/client/color/block/BlockColors;)V
private scheduleTranslucentSectionResort(Lnet/minecraft/world/phys/Vec3;)V
private scheduleResort(Lnet/minecraft/client/renderer/chunk/SectionRenderDispatcher$RenderSection;Lnet/minecraft/client/renderer/chunk/TranslucencyPointOfView;Lnet/minecraft/world/phys/Vec3;ZZ)V
public clearVisibleSections()V
public resetLevelRenderData()V
public hasRenderedAllSections()Z
public isSectionCompiledAndVisible(Lnet/minecraft/core/BlockPos;J)Z
public sectionRenderDispatcher()Lnet/minecraft/client/renderer/chunk/SectionRenderDispatcher;
public entityRenderDispatcher()Lnet/minecraft/client/renderer/entity/EntityRenderDispatcher;
public blockEntityRenderDispatcher()Lnet/minecraft/client/renderer/blockentity/BlockEntityRenderDispatcher;
public cloudRenderer()Lnet/minecraft/client/renderer/CloudRenderer;
public skyRenderer()Lnet/minecraft/client/renderer/SkyRenderer;
public weatherEffectRenderer()Lnet/minecraft/client/renderer/WeatherEffectRenderer;
public worldBorderRenderer()Lnet/minecraft/client/renderer/WorldBorderRenderer;
public viewArea()Lnet/minecraft/client/renderer/ViewArea;
public visibleSections()Lit/unimi/dsi/fastutil/objects/ObjectArrayList;
public nearbyVisibleSections()Lit/unimi/dsi/fastutil/objects/ObjectArrayList;
public expectedChunks()Lit/unimi/dsi/fastutil/longs/LongCollection;
public sectionOcclusionGraph()Lnet/minecraft/client/renderer/SectionOcclusionGraph;
public collectPerFrameRenderThreadGizmos()Lnet/minecraft/gizmos/Gizmos$TemporaryCollection;
public isChunkRenderingUsingMultiDrawIndirect()Z
private finalizeGizmoCollection()V
public addMainThreadGizmos(Ljava/util/List;)V
private static synthetic lambda$prepareChunkRenders$2(I[Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;Lcom/mojang/renderpearl/api/commands/RenderPass$UniformUploader;)V
private static synthetic lambda$prepareChunkRenders$1(Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;)Ljava/util/List;
private static synthetic lambda$prepareChunkRenders$0(Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;)Ljava/util/List;
private static synthetic lambda$executeOutline$0()Ljava/lang/String;
private static synthetic lambda$executeOitWaterMask$0()Ljava/lang/String;
private static synthetic lambda$executeDepthBoundsCull$0()Ljava/lang/String;
private static synthetic lambda$executeOit$1()Ljava/lang/String;
private static synthetic lambda$executeOit$0()Ljava/lang/String;
private static synthetic lambda$executeAlwaysOnTop$1()Ljava/lang/String;
private static synthetic lambda$executeAlwaysOnTop$0()Ljava/lang/String;
private static synthetic lambda$executeSeeThrough$0()Ljava/lang/String;
private synthetic lambda$addMainPass$0(Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;ZLnet/minecraft/client/renderer/chunk/ChunkSectionsToRender;Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;ZZ)V
private static synthetic lambda$addMainPass$1(Z)Ljava/lang/String;
private synthetic lambda$addSkyPass$0(Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;Lnet/minecraft/client/renderer/state/level/SkyRenderState;)V
private synthetic lambda$render$0(Lorg/joml/Vector4f;)V
static <clinit>()V
```
