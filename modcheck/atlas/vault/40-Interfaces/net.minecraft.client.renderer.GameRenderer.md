---
type: "interface"
fqcn: "net.minecraft.client.renderer.GameRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.GameRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `java/lang/AutoCloseable`, `net/minecraft/world/waypoints/TrackedWaypoint$Projector`, `net/minecraft/server/packs/resources/ResourceManagerReloadListener`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `extract` | `(Lnet/minecraft/client/DeltaTracker;Z)V` | exact | invokevirtual@22 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `mainCamera` | `()Lnet/minecraft/client/Camera;` | exact | invokevirtual@34 in `LevelExtractorMixin.fabric_prepareLevelExtractionContext` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `mainRenderTarget` | `()Lcom/mojang/blaze3d/pipeline/RenderTarget;` | exact | invokevirtual@58 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `mainRenderTarget` | `()Lcom/mojang/blaze3d/pipeline/RenderTarget;` | exact | invokevirtual@13 in `GlCommandEncoderMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `mainRenderTarget` | `()Lcom/mojang/blaze3d/pipeline/RenderTarget;` | exact | invokevirtual@13 in `VulkanGpuSurfaceMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `panorama` | `()Lnet/minecraft/client/renderer/Panorama;` | exact | invokevirtual@6 in `ScreenMixin.disableRotatingPanoramaForClientGameTests` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `render` | `()V` | exact | invokevirtual@29 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `resize` | `(II)V` | exact | invokevirtual@34 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$4` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `resize` | `(II)V` | exact | invokevirtual@66 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `update` | `(Lnet/minecraft/client/DeltaTracker;)V` | exact | invokevirtual@13 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `useImprovedTransparency` | `()Z` | exact | invokevirtual@15 in `SubmitNodeCollectionMixin.submitBreakingBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/renderer/FirstP` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `extract` | `(Lnet/minecraft/client/DeltaTracker;Z)V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `guiRenderer` | `Lnet/minecraft/client/gui/render/GuiRenderer;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| reads | `minecraft` | `Lnet/minecraft/client/Minecraft;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |

## Declared members (40 fields, 57 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final BLUR_POST_CHAIN_ID : Lnet/minecraft/resources/Identifier;
public static END_OF_FRAME_POST_EFFECT : Lnet/minecraft/resources/Identifier;
public static final MAX_BLUR_RADIUS : I
private static final LOGGER : Lorg/slf4j/Logger;
private final minecraft : Lnet/minecraft/client/Minecraft;
private final gameRenderState : Lnet/minecraft/client/renderer/state/GameRenderState;
public final firstPersonHandsAndItemsRenderer : Lnet/minecraft/client/renderer/FirstPersonHandsAndItemsRenderer;
private final screenEffectRenderer : Lnet/minecraft/client/renderer/ScreenEffectRenderer;
private final debugCrosshairRenderer : Lnet/minecraft/client/renderer/DebugCrosshairRenderer;
private final renderBuffers : Lnet/minecraft/client/renderer/RenderBuffers;
private final mainRenderTarget : Lcom/mojang/blaze3d/pipeline/RenderTarget;
private final hud3DTarget : Lcom/mojang/blaze3d/pipeline/RenderTarget;
private bossOverlayWorldDarkening : F
private bossOverlayWorldDarkeningO : F
private renderBlockOutline : Z
private lastScreenshotAttempt : J
private hasWorldScreenshot : Z
private final lightmap : Lnet/minecraft/client/renderer/Lightmap;
private final lightmapRenderStateExtractor : Lnet/minecraft/client/renderer/LightmapRenderStateExtractor;
private final uiLightmap : Lnet/minecraft/client/renderer/UiLightmap;
private useUiLightmap : Z
private final overlayTexture : Lnet/minecraft/client/renderer/texture/OverlayTexture;
protected final panorama : Lnet/minecraft/client/renderer/Panorama;
private final resourcePool : Lcom/mojang/blaze3d/resource/CrossFrameResourcePool;
private final fogRenderer : Lnet/minecraft/client/renderer/fog/FogRenderer;
private final guiRenderer : Lnet/minecraft/client/gui/render/GuiRenderer;
private final featureRenderDispatcher : Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher;
private final handAndScreenSubmitNodeStorage : Lnet/minecraft/client/renderer/SubmitNodeStorage;
private spectatedEntityPostEffect : Lnet/minecraft/resources/Identifier;
private spectatedEntityEffectActive : Z
private final mainCamera : Lnet/minecraft/client/Camera;
private final hudProjection : Lnet/minecraft/client/renderer/Projection;
private final lighting : Lcom/mojang/blaze3d/platform/Lighting;
private final globalSettingsUniform : Lnet/minecraft/client/renderer/GlobalSettingsUniform;
private final levelProjectionMatrixBuffer : Lnet/minecraft/client/renderer/ProjectionMatrixBuffer;
private final hud3dProjectionMatrixBuffer : Lnet/minecraft/client/renderer/ProjectionMatrixBuffer;
private final requestedPostEffects : Ljava/util/List;
private final appliedPostEffects : Ljava/util/List;
private final failedPostEffects : Ljava/util/List;
private shouldResetFailedPostEffects : Z
public <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/renderer/FirstPersonHandsAndItemsRenderer;Lnet/minecraft/client/resources/model/ModelManager;Lnet/minecraft/client/renderer/item/ItemModelResolver;)V
public close()V
public renderBuffers()Lnet/minecraft/client/renderer/RenderBuffers;
public featureRenderDispatcher()Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher;
public gameRenderState()Lnet/minecraft/client/renderer/state/GameRenderState;
public setRenderBlockOutline(Z)V
public clearSpectatedEntityPostEffect()V
public toggleSpectatorPostEffect()V
public checkEntityPostEffect(Lnet/minecraft/world/entity/Entity;)V
private setSpectatedEntityPostEffect(Lnet/minecraft/resources/Identifier;)V
public processBlurEffect()V
public static preloadUiShader(Lnet/minecraft/server/packs/resources/ResourceManager;)V
public tick()V
public spectatedEntityPostEffect()Lnet/minecraft/resources/Identifier;
public getRequestedPostEffects()Ljava/util/List;
public getAppliedPostEffects()Ljava/util/List;
public onResourceManagerReload(Lnet/minecraft/server/packs/resources/ResourceManager;)V
public resize(II)V
private bobHurt(Lnet/minecraft/client/renderer/state/level/CameraRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;)V
private bobView(Lnet/minecraft/client/renderer/state/level/CameraRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;)V
private renderItemInHand(Lnet/minecraft/client/renderer/state/level/CameraRenderState;Lnet/minecraft/client/renderer/state/level/PlayerRenderState;Lcom/mojang/renderpearl/api/textures/GpuTextureView;)V
public static nightVisionScale(Lnet/minecraft/world/entity/LivingEntity;F)F
public update(Lnet/minecraft/client/DeltaTracker;)V
public extract(Lnet/minecraft/client/DeltaTracker;Z)V
public render()V
private preparePostEffects(Ljava/util/List;)V
private applyPostEffects()V
private tryTakeScreenshotIfNeeded()V
private takeAutoScreenshot(Ljava/nio/file/Path;)V
private shouldRenderBlockOutline()Z
public renderLevel()V
private render3dHud(Lnet/minecraft/client/renderer/state/level/CameraRenderState;Lnet/minecraft/client/renderer/state/level/PlayerRenderState;Lnet/minecraft/client/renderer/state/OptionsRenderState;Z)V
private integrate3DHudDepth()V
private extractWindow()V
private extractOptions()V
private extractCamera(Lnet/minecraft/client/DeltaTracker;F)V
public resetData()V
public bossOverlayWorldDarkening(F)F
public mainCamera()Lnet/minecraft/client/Camera;
public lightmap()Lcom/mojang/renderpearl/api/textures/GpuTextureView;
public levelLightmap()Lcom/mojang/renderpearl/api/textures/GpuTextureView;
public overlayTexture()Lnet/minecraft/client/renderer/texture/OverlayTexture;
public mainRenderTarget()Lcom/mojang/blaze3d/pipeline/RenderTarget;
public projectPointToScreen(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
public projectHorizonToScreen()D
public lighting()Lcom/mojang/blaze3d/platform/Lighting;
public setLevel(Lnet/minecraft/client/multiplayer/ClientLevel;)V
public panorama()Lnet/minecraft/client/renderer/Panorama;
public registerPanoramaTextures(Lnet/minecraft/client/renderer/texture/TextureManager;)V
public useImprovedTransparency()Z
private static synthetic lambda$integrate3DHudDepth$0()Ljava/lang/String;
private static synthetic lambda$render3dHud$0()Ljava/lang/String;
private static synthetic lambda$takeAutoScreenshot$0(Ljava/nio/file/Path;Lcom/mojang/blaze3d/platform/NativeImage;)V
private static synthetic lambda$takeAutoScreenshot$1(Lcom/mojang/blaze3d/platform/NativeImage;Ljava/nio/file/Path;)V
private synthetic lambda$tryTakeScreenshotIfNeeded$0(Ljava/nio/file/Path;)V
private static synthetic lambda$renderItemInHand$0()Ljava/lang/String;
static <clinit>()V
```
