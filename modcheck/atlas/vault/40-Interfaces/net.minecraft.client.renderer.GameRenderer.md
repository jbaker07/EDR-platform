---
type: "interface"
fqcn: "net.minecraft.client.renderer.GameRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.GameRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `extract(Lnet/minecraft/client/DeltaTracker;Z)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `mainCamera()Lnet/minecraft/client/Camera;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `mainRenderTarget()Lcom/mojang/blaze3d/pipeline/RenderTarget;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `mainRenderTarget()Lcom/mojang/blaze3d/pipeline/RenderTarget;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `mainRenderTarget()Lcom/mojang/blaze3d/pipeline/RenderTarget;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `panorama()Lnet/minecraft/client/renderer/Panorama;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `render()V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `resize(II)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `update(Lnet/minecraft/client/DeltaTracker;)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `useImprovedTransparency()Z` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `extract` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (97, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.GameRenderer implements java.lang.AutoCloseable,net.minecraft.world.waypoints.TrackedWaypoint$Projector,net.minecraft.server.packs.resources.ResourceManagerReloadListener {
    private static final net.minecraft.resources.Identifier BLUR_POST_CHAIN_ID;
    public static net.minecraft.resources.Identifier END_OF_FRAME_POST_EFFECT;
    public static final int MAX_BLUR_RADIUS;
    private static final org.slf4j.Logger LOGGER;
    private final net.minecraft.client.Minecraft minecraft;
    private final net.minecraft.client.renderer.state.GameRenderState gameRenderState;
    public final net.minecraft.client.renderer.FirstPersonHandsAndItemsRenderer firstPersonHandsAndItemsRenderer;
    private final net.minecraft.client.renderer.ScreenEffectRenderer screenEffectRenderer;
    private final net.minecraft.client.renderer.DebugCrosshairRenderer debugCrosshairRenderer;
    private final net.minecraft.client.renderer.RenderBuffers renderBuffers;
    private final com.mojang.blaze3d.pipeline.RenderTarget mainRenderTarget;
    private final com.mojang.blaze3d.pipeline.RenderTarget hud3DTarget;
    private float bossOverlayWorldDarkening;
    private float bossOverlayWorldDarkeningO;
    private boolean renderBlockOutline;
    private long lastScreenshotAttempt;
    private boolean hasWorldScreenshot;
    private final net.minecraft.client.renderer.Lightmap lightmap;
    private final net.minecraft.client.renderer.LightmapRenderStateExtractor lightmapRenderStateExtractor;
    private final net.minecraft.client.renderer.UiLightmap uiLightmap;
    private boolean useUiLightmap;
    private final net.minecraft.client.renderer.texture.OverlayTexture overlayTexture;
    protected final net.minecraft.client.renderer.Panorama panorama;
    private final com.mojang.blaze3d.resource.CrossFrameResourcePool resourcePool;
    private final net.minecraft.client.renderer.fog.FogRenderer fogRenderer;
    private final net.minecraft.client.gui.render.GuiRenderer guiRenderer;
    private final net.minecraft.client.renderer.feature.FeatureRenderDispatcher featureRenderDispatcher;
    private final net.minecraft.client.renderer.SubmitNodeStorage handAndScreenSubmitNodeStorage;
    private net.minecraft.resources.Identifier spectatedEntityPostEffect;
    private boolean spectatedEntityEffectActive;
    private final net.minecraft.client.Camera mainCamera;
    private final net.minecraft.client.renderer.Projection hudProjection;
    private final com.mojang.blaze3d.platform.Lighting lighting;
    private final net.minecraft.client.renderer.GlobalSettingsUniform globalSettingsUniform;
    private final net.minecraft.client.renderer.ProjectionMatrixBuffer levelProjectionMatrixBuffer;
    private final net.minecraft.client.renderer.ProjectionMatrixBuffer hud3dProjectionMatrixBuffer;
    private final java.util.List<net.minecraft.resources.Identifier> requestedPostEffects;
    private final java.util.List<net.minecraft.client.renderer.PostChain> appliedPostEffects;
    private final java.util.List<net.minecraft.resources.Identifier> failedPostEffects;
    private volatile boolean shouldResetFailedPostEffects;
    public net.minecraft.client.renderer.GameRenderer(net.minecraft.client.Minecraft, net.minecraft.client.renderer.FirstPersonHandsAndItemsRenderer, net.minecraft.client.resources.model.ModelManager, net.minecraft.client.renderer.item.ItemModelResolver);
    public void close();
    public net.minecraft.client.renderer.RenderBuffers renderBuffers();
    public net.minecraft.client.renderer.feature.FeatureRenderDispatcher featureRenderDispatcher();
    public net.minecraft.client.renderer.state.GameRenderState gameRenderState();
    public void setRenderBlockOutline(boolean);
    public void clearSpectatedEntityPostEffect();
    public void toggleSpectatorPostEffect();
    public void checkEntityPostEffect(net.minecraft.world.entity.Entity);
    private void setSpectatedEntityPostEffect(net.minecraft.resources.Identifier);
    public void processBlurEffect();
    public static void preloadUiShader(net.minecraft.server.packs.resources.ResourceManager);
    public void tick();
    public net.minecraft.resources.Identifier spectatedEntityPostEffect();
    public java.util.List<net.minecraft.resources.Identifier> getRequestedPostEffects();
    public java.util.List<net.minecraft.resources.Identifier> getAppliedPostEffects();
    public void onResourceManagerReload(net.minecraft.server.packs.resources.ResourceManager);
    public void resize(int, int);
    private void bobHurt(net.minecraft.client.renderer.state.level.CameraRenderState, com.mojang.blaze3d.vertex.PoseStack);
    private void bobView(net.minecraft.client.renderer.state.level.CameraRenderState, com.mojang.blaze3d.vertex.PoseStack);
    private void renderItemInHand(net.minecraft.client.renderer.state.level.CameraRenderState, net.minecraft.client.renderer.state.level.PlayerRenderState, com.mojang.renderpearl.api.textures.GpuTextureView);
    public static float nightVisionScale(net.minecraft.world.entity.LivingEntity, float);
    public void update(net.minecraft.client.DeltaTracker);
    public void extract(net.minecraft.client.DeltaTracker, boolean);
    public void render();
    private void preparePostEffects(java.util.List<net.minecraft.resources.Identifier>);
    private void applyPostEffects();
    private void tryTakeScreenshotIfNeeded();
    private void takeAutoScreenshot(java.nio.file.Path);
    private boolean shouldRenderBlockOutline();
    public void renderLevel();
    private void render3dHud(net.minecraft.client.renderer.state.level.CameraRenderState, net.minecraft.client.renderer.state.level.PlayerRenderState, net.minecraft.client.renderer.state.OptionsRenderState, boolean);
    private void integrate3DHudDepth();
    private void extractWindow();
    private void extractOptions();
    private void extractCamera(net.minecraft.client.DeltaTracker, float);
    public void resetData();
    public float bossOverlayWorldDarkening(float);
    public net.minecraft.client.Camera mainCamera();
    public com.mojang.renderpearl.api.textures.GpuTextureView lightmap();
    public com.mojang.renderpearl.api.textures.GpuTextureView levelLightmap();
    public net.minecraft.client.renderer.texture.OverlayTexture overlayTexture();
    public com.mojang.blaze3d.pipeline.RenderTarget mainRenderTarget();
    public net.minecraft.world.phys.Vec3 projectPointToScreen(net.minecraft.world.phys.Vec3);
    public double projectHorizonToScreen();
    public com.mojang.blaze3d.platform.Lighting lighting();
    public void setLevel(net.minecraft.client.multiplayer.ClientLevel);
    public net.minecraft.client.renderer.Panorama panorama();
    public void registerPanoramaTextures(net.minecraft.client.renderer.texture.TextureManager);
    public boolean useImprovedTransparency();
    private static java.lang.String lambda$integrate3DHudDepth$0();
    private static java.lang.String lambda$render3dHud$0();
    private static void lambda$takeAutoScreenshot$0(java.nio.file.Path, com.mojang.blaze3d.platform.NativeImage);
    private static void lambda$takeAutoScreenshot$1(com.mojang.blaze3d.platform.NativeImage, java.nio.file.Path);
    private void lambda$tryTakeScreenshotIfNeeded$0(java.nio.file.Path);
    private static java.lang.String lambda$renderItemInHand$0();
    static {};
}
```
