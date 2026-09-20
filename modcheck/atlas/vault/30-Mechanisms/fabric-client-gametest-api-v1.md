---
type: "mechanism"
module: "fabric-client-gametest-api-v1"
version: "6.0.7+4be74c3f5d"
sha256: "09d7d48475eef47a347c12c51bd32c6cffc065a6785f3023f5aa6be4b815cd38"
lifecycle: "experimental"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-client-gametest-api-v1

**Version** `6.0.7+4be74c3f5d` -- **artifact sha256** `09d7d48475eef47a347c12c51bd32c6cffc065a6785f3023f5aa6be4b815cd38`

## Declared (fabric.mod.json)

- environment: `client`
- depends: `{"fabricloader": ">=0.19.3", "fabric-resource-loader-v1": "*", "fabric-networking-api-v1": "*"}`
- entrypoints: `{"client": ["net.fabricmc.fabric.impl.client.gametest.FabricClientGameTestImpl"]}`
- mixin configs: `["fabric-client-gametest-api-v1.mixins.json"]`
- access widener: `fabric-client-gametest-api-v1.classtweaker`
- mixin classes: 25 found by annotation, 25 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/com.mojang.blaze3d.platform.InputConstants|InputConstants]].`grabMouse` | `(Lcom/mojang/blaze3d/platform/Window;DD)V` | name_only | @Inject | HEAD | both | 1000 (default) | `InputConstantsMixin.disableCursorGrabbing` |
| [[40-Interfaces/com.mojang.blaze3d.platform.InputConstants|InputConstants]].`isKeyDown` | `(I)Z` | name_only | @Inject | HEAD | both | 1000 (default) | `InputConstantsMixin.useGameTestInputForKeyDown` |
| [[40-Interfaces/com.mojang.blaze3d.platform.InputConstants|InputConstants]].`releaseMouse` | `(Lcom/mojang/blaze3d/platform/Window;DD)V` | name_only | @Inject | HEAD | both | 1000 (default) | `InputConstantsMixin.disableCursorGrabbing` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]].`handleDropBeginEvent` | `()V` | name_only | @Inject | HEAD | both | 1000 (default) | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]].`handleDropCompleteEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]].`handleDropFileEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]].`handleKeyEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]].`handleMouseButtonEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]].`handleMouseMotionEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]].`handleMouseWheelEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]].`handleTextEditingEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]].`handleTextInputEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]].`<init>` | `(Lcom/mojang/blaze3d/platform/WindowEventHandler;Lcom/mojang/blaze3d/platform/DisplayData;Ljava/lang/String;ZLjava/lang/String;Lcom/mojang/blaze3d/platform/MonitorManager;Lcom/mojang/renderpearl/api/device/GpuBackend;I)V` | exact | @Inject | RETURN | both | 1000 (default) | `WindowMixin.onInit` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]].`handleEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `WindowMixin.cancelDirectEvents` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]].`onFocus` | `(Z)V` | name_only | @Inject | HEAD | both | 1000 (default) | `WindowMixin.cancelEvents` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]].`onFramebufferResize` | `(II)V` | name_only | @Inject | HEAD | both | 1000 (default) | `WindowMixin.cancelFramebufferResize` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]].`onIconified` | `(Z)V` | name_only | @Inject | HEAD | both | 1000 (default) | `WindowMixin.cancelEvents` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]].`onResize` | `(II)V` | name_only | @Inject | HEAD | both | 1000 (default) | `WindowMixin.cancelResize` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]].`setMode` | `()V` | name_only | @WrapMethod | - | both | 1000 (default) | `WindowMixin.wrapSetMode` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]].`setWindowed` | `(II)V` | name_only | @Inject | HEAD | both | 1000 (default) | `WindowMixin.setWindowedSize` |
| [[40-Interfaces/com.mojang.renderpearl.backend.opengl.GlCommandEncoder|GlCommandEncoder]].`presentTexture` | `(JLcom/mojang/renderpearl/api/textures/GpuTextureView;II)V` | name_only | @WrapOperation | INVOKE `Lcom/mojang/renderpearl/backend/opengl/DirectStateAccess;blitFrameBuffers(IIIIIIIIIIII)V` (exact) | both | 1000 (default) | `GlCommandEncoderMixin.blitFrameBuffer` |
| [[40-Interfaces/com.mojang.renderpearl.backend.vulkan.VulkanGpuSurface|VulkanGpuSurface]].`blitFromTexture` | `(Lcom/mojang/renderpearl/backend/api/CommandEncoderBackend;Lcom/mojang/renderpearl/api/textures/GpuTextureView;)V` | name_only | @WrapOperation | INVOKE `Lorg/lwjgl/vulkan/VkImageBlit$Buffer;dstOffsets(Lorg/lwjgl/vulkan/VkOffset3D$Buffer;)Lorg/lwjgl/vulkan/VkImageBlit$Buffer;` (exact) | both | 1000 (default) | `VulkanGpuSurfaceMixin.blitFrameBuffer` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`disconnect` | `(Lnet/minecraft/client/gui/screens/Screen;Z)V` | exact | @Inject | HEAD | both | 1000 (default) | `MinecraftMixin.deferDisconnect` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`disconnect` | `(Lnet/minecraft/client/gui/screens/Screen;ZZ)V` | exact | @Inject | INVOKE `Lnet/minecraft/client/Minecraft;renderFrame(Z)V` (exact) | both | 1000 (default) | `MinecraftMixin.onDisconnectBusyWait` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`doWorldLoad` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;Z)V` | name_only | @Inject | HEAD | both | 1000 (default) | `MinecraftMixin.deferStartIntegratedServer` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`doWorldLoad` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;Z)V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/Minecraft;managedBlock(Ljava/util/function/BooleanSupplier;)V` (inherited_exact) | both | 1000 (default) | `MinecraftMixin.onStartIntegratedServerBusyWait` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`emergencySave` | `()V` | name_only | @Inject | HEAD | both | 1000 (default) | `MinecraftMixin.deregisterAfterCrash` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`getInstance` | `()Lnet/minecraft/client/Minecraft;` | name_only | @Inject | HEAD | both | 1000 (default) | `MinecraftMixin.checkThreadOnGetInstance` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`run` | `()V` | name_only | @WrapMethod | - | both | 1000 (default) | `MinecraftMixin.onRun` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`runTick` | `(Z)V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/client/DeltaTracker$Timer;advanceGameTime(J)I` (exact) | both | 1000 (default) | `MinecraftMixin.captureTicksPerFrame` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`runTick` | `(Z)V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/Minecraft;runAllTasks()V` (inherited_exact) | both | 1000 (default) | `MinecraftMixin.postRunTasksHook` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`tick` | `()V` | name_only | @Inject | HEAD | both | 1000 (default) | `MinecraftMixin.onTick` |
| [[40-Interfaces/net.minecraft.client.Options|Options]].`<init>` | `(Lnet/minecraft/client/Minecraft;Ljava/io/File;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `OptionsMixin.onCreateGameOptions` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]].`extractPanorama` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;F)V` | name_only | @Inject | HEAD | both | 1000 (default) | `ScreenMixin.disableRotatingPanoramaForClientGameTests` |
| [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.CreateWorldScreen|CreateWorldScreen]].`onCreate` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/gui/screens/worldselection/WorldOpenFlows;confirmWorldCreation(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;Lcom/mojang/serialization/Lifecycle;Ljava/lang/Runnable;Z)V` (exact) | both | 1000 (default) | `CreateWorldScreenMixin.createLevelDataForServers` |
| [[40-Interfaces/net.minecraft.server.Main|Main]].`main` | `([Ljava/lang/String;)V` | name_only | @WrapWithCondition | INVOKE `Lnet/minecraft/util/Util;startTimerHackThread()V` (exact) | both | 1000 (default) | `MainMixin.dontStartAnotherTimerHack` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`runServer` | `()V` | name_only | @WrapMethod | - | both | 1000 (default) | `MinecraftServerMixin.onRunServer` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`runServer` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/MinecraftServer;onServerCrash(Lnet/minecraft/CrashReport;)V` (exact) | both | 1000 (default) | `MinecraftServerMixin.onCrash` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`runServer` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/MinecraftServer;waitUntilNextTick()V` (exact) | both | 1000 (default) | `MinecraftServerMixin.postRunTasks` |
| [[40-Interfaces/net.minecraft.server.dedicated.DedicatedServer|DedicatedServer]].`initServer` | `()Z` | name_only | @Inject | INVOKE `Lnet/minecraft/server/dedicated/DedicatedServer;loadLevel()V` (inherited_exact) | both | 1000 (default) | `DedicatedServerMixin.captureServerInstance` |
| [[40-Interfaces/net.minecraft.server.dedicated.DedicatedServer|DedicatedServer]].`stopServer` | `()V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/util/Util;shutdownExecutors()V` (exact) | both | 1000 (default) | `DedicatedServerMixin.dontStopExecutors` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.FabricClientGameTest|FabricClientGameTest]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.TestInput|TestInput]] (interface, 35 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.context.ClientGameTestContext|ClientGameTestContext]] (interface, 21 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.context.TestDedicatedServerConnection|TestDedicatedServerConnection]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.context.TestDedicatedServerContext|TestDedicatedServerContext]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.context.TestServerConnection|TestServerConnection]] (interface, 14 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.context.TestServerContext|TestServerContext]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.context.TestSingleplayerContext|TestSingleplayerContext]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotCommonOptions|TestScreenshotCommonOptions]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonAlgorithm|TestScreenshotComparisonAlgorithm]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonOptions|TestScreenshotComparisonOptions]] (interface, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotOptions|TestScreenshotOptions]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.world.TestWorldBuilder|TestWorldBuilder]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.world.TestWorldSave|TestWorldSave]] (interface, 2 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
