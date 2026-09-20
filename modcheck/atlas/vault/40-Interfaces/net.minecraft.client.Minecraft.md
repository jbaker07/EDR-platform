---
type: "interface"
fqcn: "net.minecraft.client.Minecraft"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.Minecraft

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

`class` public; extends `net/minecraft/util/thread/ReentrantBlockableEventLoop`; implements `com/mojang/blaze3d/platform/WindowEventHandler`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `createWorldOpenFlows` | `()Lnet/minecraft/client/gui/screens/worldselection/WorldOpenFlows;` | exact | invokevirtual@1 in `TestWorldSaveImpl.lambda$open$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `disconnect` | `(Lnet/minecraft/client/gui/screens/Screen;Z)V` | exact | invokevirtual@43 in `TestSingleplayerContextImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `disconnect` | `(Lnet/minecraft/client/gui/screens/Screen;Z)V` | exact | invokevirtual@5 in `MinecraftMixin.lambda$deferDisconnect$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `disconnectWithSavingScreen` | `()V` | exact | invokevirtual@30 in `TestDedicatedServerConnectionImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `doWorldLoad` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAcc` | exact | invokevirtual@9 in `MinecraftMixin.lambda$deferStartIntegratedServer$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `execute` | `(Ljava/lang/Runnable;)V` | inherited_exact | invokevirtual@5 in `ClientCommonNetworkAddon.schedule` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `execute` | `(Ljava/lang/Runnable;)V` | inherited_exact | invokevirtual@46 in `ClientRegistrySyncHandler.receivePacket` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getAtlasManager` | `()Lnet/minecraft/client/resources/model/sprite/AtlasManager;` | exact | invokevirtual@3 in `FabricSpriteSetImpl.getAtlas` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `getAtlasManager` | `()Lnet/minecraft/client/resources/model/sprite/AtlasManager;` | exact | invokevirtual@10 in `PictureInPictureRendererRegistryImpl.lambda$registerVanillaFactories$ | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | invokevirtual@3 in `ClientCommands.refreshCommandCompletions` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |
| calls | `getConnection` | `()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | invokevirtual@6 in `ClientPlayNetworking$Context.packetContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | invokevirtual@3 in `ClientPlayNetworking.canSend` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | invokevirtual@12 in `ClientPlayNetworking.canSend` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | invokevirtual@3 in `ClientPlayNetworking.getSender` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | invokevirtual@12 in `ClientPlayNetworking.getSender` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | invokevirtual@34 in `ClientPlayNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | invokevirtual@43 in `ClientPlayNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | invokevirtual@3 in `ClientNetworkingImpl.getClientPlayAddon` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | invokevirtual@16 in `ClientNetworkingImpl.getClientPlayAddon` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | invokevirtual@70 in `AdvancementToastMixin.extractAdvancementIcon` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getDeltaTracker` | `()Lnet/minecraft/client/DeltaTracker;` | exact | invokevirtual@11 in `HudMixin.wrapArmorBar` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getDeltaTracker` | `()Lnet/minecraft/client/DeltaTracker;` | exact | invokevirtual@11 in `HudMixin.wrapHealthBar` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getDeltaTracker` | `()Lnet/minecraft/client/DeltaTracker;` | exact | invokevirtual@11 in `HudMixin.wrapFoodBar` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getDeltaTracker` | `()Lnet/minecraft/client/DeltaTracker;` | exact | invokevirtual@11 in `HudMixin.wrapAirBar` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getDeltaTracker` | `()Lnet/minecraft/client/DeltaTracker;` | exact | invokevirtual@10 in `SubtitleOverlayMixin.wrapExtractRenderState` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getEntityRenderDispatcher` | `()Lnet/minecraft/client/renderer/entity/EntityRenderDispatcher;` | exact | invokevirtual@10 in `PictureInPictureRendererRegistryImpl.lambda$registerVanillaFactories$ | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getGameProfile` | `()Lcom/mojang/authlib/GameProfile;` | exact | invokevirtual@15 in `TestServerConnectionImpl.getServerPlayer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@7 in `FabricClientGameTestImpl.onInitializeClient` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `FabricClientGameTestRunner.start` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@24 in `ClientGameTestContextImpl.runOnClient` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@24 in `ClientGameTestContextImpl.computeOnClient` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@2 in `ClientGameTestContextImpl.lambda$computeOnClient$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@1 in `ClientGameTestContextImpl.lambda$runOnClient$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `ClientGameTestContextImpl.lambda$restoreDefaultGameOptions$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@5 in `TestServerConnectionImpl.getClientPlayer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@5 in `TestServerConnectionImpl.getClientLevel` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@12 in `TestServerConnectionImpl.getServerPlayer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@12 in `TestServerConnectionImpl.getServerLevel` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `ThreadingImpl.lambda$runTestThread$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `ScreenMixin.disableRotatingPanoramaForClientGameTests` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@7 in `GlCommandEncoderMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@22 in `GlCommandEncoderMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@7 in `VulkanGpuSurfaceMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@22 in `VulkanGpuSurfaceMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `MinecraftMixin.deferDisconnect` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `MinecraftMixin.lambda$deferDisconnect$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `MinecraftMixin.lambda$deferStartIntegratedServer$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@18 in `MinecraftServerMixin.onCrash` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `ClientCommands.refreshCommandCompletions` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@135 in `FabricCreativeGuiComponents$CreativeModeTabButton.extractContents` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `DebugRendererMixin.registerRenderers` | unknown | [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@23 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@19 in `EnumRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `KeyMappingRegistryImpl.registerKeyMapping` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@86 in `ClientNetworking.openScreen` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `ClientPlayNetworking.canSend` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@9 in `ClientPlayNetworking.canSend` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `ClientPlayNetworking.getSender` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@9 in `ClientPlayNetworking.getSender` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@31 in `ClientPlayNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@40 in `ClientPlayNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `ClientNetworkingImpl.getLoginConnection` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@37 in `ClientNetworkingImpl.getLoginConnection` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@52 in `ClientNetworkingImpl.getLoginConnection` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `ClientNetworkingImpl.getClientPlayAddon` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@13 in `ClientNetworkingImpl.getClientPlayAddon` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `FabricSpriteSetImpl.getAtlas` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `FabricBlockStateModel.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `SingleVariantMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@9 in `SubmitNodeCollectionMixin.submitBreakingBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@52 in `HudStatusBarHeightRegistryImpl.getHeight` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@23 in `HudStatusBarHeightRegistryImpl.getElementHeight` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `HudStatusBarHeightRegistryImpl.lambda$replaceVanillaElement$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `HudStatusBarHeightRegistryImpl.lambda$static$4` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `HudStatusBarHeightRegistryImpl.lambda$static$3` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `HudStatusBarHeightRegistryImpl.lambda$static$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@0 in `GuiRendererMixin.fabric_onReady` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@38 in `GuiRendererMixin.substituteSpecialElementRenderer` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@4 in `LevelRendererMixin.beforeRender` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@7 in `SubtitleOverlayMixin.wrapExtractRenderState` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@67 in `AdvancementToastMixin.extractAdvancementIcon` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@14 in `GameOptionsWriteVisitorMixin.toPackListString` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@210 in `ScreenMixin.beforeInit` | unknown | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@9 in `ScreenMixin.afterInit` | unknown | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@5 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@11 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@20 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@32 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@11 in `FluidVariantRenderHandler.getColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@2 in `FluidVariantRendering.getTooltip` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | exact | invokestatic@2 in `FluidVariantRendering.getTooltip` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getLastInputType` | `()Lnet/minecraft/client/InputType;` | exact | invokevirtual@13 in `OptimizedScrollableLayout$Container.setFocused` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getLevelSource` | `()Lnet/minecraft/world/level/storage/LevelStorageSource;` | exact | invokevirtual@86 in `TestWorldBuilderImpl.lambda$navigateCreateWorldScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getModelManager` | `()Lnet/minecraft/client/resources/model/ModelManager;` | exact | invokevirtual@14 in `FluidVariantRenderHandler.getColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getNarrator` | `()Lnet/minecraft/client/GameNarrator;` | exact | invokevirtual@21 in `ClientSuggestionProviderMixin.sendFeedback` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getResourcePackRepository` | `()Lnet/minecraft/server/packs/repository/PackRepository;` | exact | invokevirtual@17 in `GameOptionsWriteVisitorMixin.toPackListString` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getRunningThread` | `()Ljava/lang/Thread;` | exact | invokevirtual@15 in `GuiMixin.checkThreadOnDev` | unknown | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| calls | `getSingleplayerServer` | `()Lnet/minecraft/client/server/IntegratedServer;` | exact | invokevirtual@3 in `MinecraftMixin.deferDisconnect` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@3 in `FabricClientGameTestRunner.start` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@1 in `FabricClientGameTestRunner.lambda$setupAndCheckFinalGameTestState$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@1 in `FabricClientGameTestRunner.lambda$setupAndCheckFinalGameTestState$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@1 in `FabricClientGameTestRunner.lambda$setupAndCheckFinalGameTestState$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@44 in `TestInputImpl.pressOrReleaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@88 in `TestInputImpl.pressOrReleaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@1 in `TestInputImpl.lambda$resizeWindow$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@34 in `TestInputImpl.lambda$moveCursor$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@10 in `TestInputImpl.lambda$setCursorPos$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@10 in `TestInputImpl.lambda$scroll$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@8 in `TestInputImpl.lambda$typeChars$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@8 in `TestInputImpl.lambda$typeChar$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@1 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$4` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@12 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$4` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@1 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@9 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@33 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@44 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@5 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@12 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@25 in `GlCommandEncoderMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow` | `()Lcom/mojang/blaze3d/platform/Window;` | exact | invokevirtual@25 in `VulkanGpuSurfaceMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `isLocalServer` | `()Z` | exact | invokevirtual@12 in `ClientRegistrySyncHandler.receivePacket` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `isSameThread` | `()Z` | inherited_exact | invokevirtual@17 in `ClientGameTestContextImpl.runOnClient` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `isSameThread` | `()Z` | inherited_exact | invokevirtual@17 in `ClientGameTestContextImpl.computeOnClient` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `isSameThread` | `()Z` | inherited_exact | invokevirtual@3 in `ThreadingImpl.checkOnClientThread` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `isSameThread` | `()Z` | inherited_exact | invokevirtual@12 in `ThreadingImpl.checkOnGametestOrClientThread` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `packetProcessor` | `()Lnet/minecraft/network/PacketProcessor;` | exact | invokevirtual@4 in `ClientPlayNetworkAddon.isOnReceiveThread` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `packetProcessor` | `()Lnet/minecraft/network/PacketProcessor;` | exact | invokevirtual@97 in `ClientCommonPacketListenerImplMixin.onCustomPayload` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `setScreenAndShow` | `(Lnet/minecraft/client/gui/screens/Screen;)V` | exact | invokevirtual@21 in `DetailedBackupConfirmScreen.lambda$init$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `stop` | `()V` | exact | invokevirtual@3 in `ThreadingImpl.lambda$runTestThread$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `stop` | `()V` | exact | invokevirtual@21 in `MinecraftServerMixin.onCrash` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/client/main/GameConfig;)V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/client/main/GameConfig;)V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/client/main/GameConfig;)V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `continueAttack` | `(Z)V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `disconnect` | `(Lnet/minecraft/client/gui/screens/Screen;Z)V` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `disconnect` | `(Lnet/minecraft/client/gui/screens/Screen;ZZ)V` | exact | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `disconnect` | `(Lnet/minecraft/client/gui/screens/Screen;ZZ)V` | exact | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `doWorldLoad` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAcc` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `doWorldLoad` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAcc` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `doWorldLoad` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAcc` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `doWorldLoad` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAcc` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `emergencySave` | `()V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `exitWorldAndClose` | `()V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `exitWorldAndClose` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `getInstance` | `()Lnet/minecraft/client/Minecraft;` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleKeybinds` | `()V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `run` | `()V` | name_only | @Inject at ['FIELD'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `runTick` | `(Z)V` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `runTick` | `(Z)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `startAttack` | `()Z` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `startUseItem` | `()V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `tick` | `()V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `tick` | `()V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `tick` | `()V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `updateLevelInEngines` | `(Lnet/minecraft/client/multiplayer/ClientLevel;Z)V` | exact | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `LOGGER` | `Lorg/slf4j/Logger;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| reads | `font` | `Lnet/minecraft/client/gui/Font;` | exact | getfield@138 in `FabricCreativeGuiComponents$CreativeModeTabButton.extractContents` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `font` | `Lnet/minecraft/client/gui/Font;` | exact | getfield@26 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| reads | `font` | `Lnet/minecraft/client/gui/Font;` | exact | getfield@22 in `EnumRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| reads | `font` | `Lnet/minecraft/client/gui/Font;` | exact | getfield@22 in `TransferableSelectionListPackEntryMixin.onExtractContent` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `font` | `Lnet/minecraft/client/gui/Font;` | exact | getfield@56 in `TransferableSelectionListPackEntryMixin.onExtractContent` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `font` | `Lnet/minecraft/client/gui/Font;` | exact | getfield@120 in `TransferableSelectionListPackEntryMixin.onExtractContent` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `gameMode` | `Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@23 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$4` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@9 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@17 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@26 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@55 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@55 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@3 in `ScreenMixin.disableRotatingPanoramaForClientGameTests` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@10 in `GlCommandEncoderMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@10 in `VulkanGpuSurfaceMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@12 in `SubmitNodeCollectionMixin.submitBreakingBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@8 in `LevelExtractorMixin.fabric_prepareLevelExtractionContext` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@31 in `LevelExtractorMixin.fabric_prepareLevelExtractionContext` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `gameRenderer` | `Lnet/minecraft/client/renderer/GameRenderer;` | exact | getfield@7 in `LevelRendererMixin.beforeRender` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@36 in `FabricClientGameTestRunner.lambda$setupAndCheckFinalGameTestState$3` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@72 in `FabricClientGameTestRunner.lambda$setupAndCheckFinalGameTestState$3` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@1 in `ClientGameTestContextImpl.lambda$tryClickScreenButton$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@1 in `ClientGameTestContextImpl.lambda$clickScreenButton$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@32 in `ClientGameTestContextImpl.lambda$clickScreenButton$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@1 in `ClientGameTestContextImpl.lambda$setScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@2 in `ClientGameTestContextImpl.lambda$waitForScreen$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@1 in `ClientGameTestContextImpl.lambda$waitForScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@18 in `TestDedicatedServerContextImpl.lambda$connect$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@10 in `ClientGameTestImpl.isWorldLoadingFinished` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@29 in `ClientGameTestImpl.isWorldLoadingFinished` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@1 in `ClientGameTestImpl.lambda$waitForWorldLoad$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@1 in `ClientGameTestImpl.lambda$waitForWorldLoad$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@1 in `TestWorldBuilderImpl.lambda$navigateCreateWorldScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@20 in `TestWorldBuilderImpl.lambda$navigateCreateWorldScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@1 in `TestWorldBuilderImpl.lambda$navigateCreateWorldScreen$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@5 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@78 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@11 in `ClientCommandInternals.lambda$openSendConfirmationWindow$0` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@4 in `ClientSuggestionProviderMixin.sendFeedback` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@149 in `ClientNetworking.openScreen` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@40 in `ClientNetworkingImpl.getLoginConnection` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@55 in `ClientNetworkingImpl.getLoginConnection` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@4 in `DetailsScreen.onClose` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@55 in `HudStatusBarHeightRegistryImpl.getHeight` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@26 in `HudStatusBarHeightRegistryImpl.getElementHeight` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@3 in `HudStatusBarHeightRegistryImpl.lambda$replaceVanillaElement$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@3 in `HudStatusBarHeightRegistryImpl.lambda$static$4` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@3 in `HudStatusBarHeightRegistryImpl.lambda$static$3` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | getfield@3 in `HudStatusBarHeightRegistryImpl.lambda$static$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `gui` | `Lnet/minecraft/client/gui/Gui;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | declared |
| reads | `hitResult` | `Lnet/minecraft/world/phys/HitResult;` | exact | getfield@17 in `LevelExtractorMixin.afterBlockOutlineExtraction` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `keyboardHandler` | `Lnet/minecraft/client/KeyboardHandler;` | exact | getfield@37 in `TestInputImpl.pressOrReleaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `keyboardHandler` | `Lnet/minecraft/client/KeyboardHandler;` | exact | getfield@1 in `TestInputImpl.lambda$typeChars$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `keyboardHandler` | `Lnet/minecraft/client/KeyboardHandler;` | exact | getfield@1 in `TestInputImpl.lambda$typeChar$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@1 in `FabricClientGameTestRunner.lambda$setupAndCheckFinalGameTestState$3` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@1 in `TestDedicatedServerConnectionImpl.lambda$close$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@1 in `TestDedicatedServerConnectionImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@18 in `TestDedicatedServerConnectionImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@9 in `TestServerConnectionImpl.areChunksLoaded` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@1 in `TestServerConnectionImpl.areChunksRendered` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@8 in `TestServerConnectionImpl.getClientLevel` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@15 in `TestServerConnectionImpl.getServerLevel` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@7 in `TestSingleplayerContextImpl.lambda$close$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@1 in `TestSingleplayerContextImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@18 in `TestSingleplayerContextImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@6 in `ClientGameTestImpl.isWorldLoadingFinished` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@22 in `ClientGameTestImpl.isWorldLoadingFinished` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@4 in `ClientSuggestionProviderMixin.getLevel` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@10 in `AttachmentSyncClient.lambda$onInitializeClient$1` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@20 in `MultiPlayerGameModeMixin.fabric_fireAttackBlockCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@78 in `MultiPlayerGameModeMixin.fabric_fireAttackBlockCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@13 in `MultiPlayerGameModeMixin.fabric$onBlockBroken` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@14 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@23 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | getfield@35 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| reads | `levelExtractor` | `Lnet/minecraft/client/renderer/extract/LevelExtractor;` | exact | getfield@4 in `GameRendererMixin.beforeExtract` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `levelRenderer` | `Lnet/minecraft/client/renderer/LevelRenderer;` | exact | getfield@29 in `TestServerConnectionImpl.areChunksRendered` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `levelRenderer` | `Lnet/minecraft/client/renderer/LevelRenderer;` | exact | getfield@15 in `LevelExtractorMixin.fabric_prepareLevelExtractionContext` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `mouseHandler` | `Lnet/minecraft/client/MouseHandler;` | exact | getfield@81 in `TestInputImpl.pressOrReleaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `mouseHandler` | `Lnet/minecraft/client/MouseHandler;` | exact | getfield@2 in `TestInputImpl.lambda$moveCursor$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `mouseHandler` | `Lnet/minecraft/client/MouseHandler;` | exact | getfield@14 in `TestInputImpl.lambda$moveCursor$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `mouseHandler` | `Lnet/minecraft/client/MouseHandler;` | exact | getfield@26 in `TestInputImpl.lambda$moveCursor$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `mouseHandler` | `Lnet/minecraft/client/MouseHandler;` | exact | getfield@2 in `TestInputImpl.lambda$setCursorPos$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `mouseHandler` | `Lnet/minecraft/client/MouseHandler;` | exact | getfield@21 in `TestInputImpl.lambda$setCursorPos$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `mouseHandler` | `Lnet/minecraft/client/MouseHandler;` | exact | getfield@31 in `TestInputImpl.lambda$setCursorPos$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `mouseHandler` | `Lnet/minecraft/client/MouseHandler;` | exact | getfield@2 in `TestInputImpl.lambda$scroll$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `options` | `Lnet/minecraft/client/Options;` | exact | getfield@2 in `TestInputImpl.lambda$holdKeyFor$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `options` | `Lnet/minecraft/client/Options;` | exact | getfield@2 in `TestInputImpl.lambda$pressKey$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `options` | `Lnet/minecraft/client/Options;` | exact | getfield@2 in `TestInputImpl.lambda$releaseKey$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `options` | `Lnet/minecraft/client/Options;` | exact | getfield@2 in `TestInputImpl.lambda$holdKey$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `options` | `Lnet/minecraft/client/Options;` | exact | getfield@3 in `ClientGameTestContextImpl.lambda$restoreDefaultGameOptions$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `options` | `Lnet/minecraft/client/Options;` | exact | getfield@1 in `TestServerConnectionImpl.areChunksLoaded` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `options` | `Lnet/minecraft/client/Options;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |
| reads | `options` | `Lnet/minecraft/client/Options;` | exact | getfield@3 in `KeyMappingRegistryImpl.registerKeyMapping` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| reads | `options` | `Lnet/minecraft/client/Options;` | exact | getfield@3 in `FabricBlockStateModel.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `options` | `Lnet/minecraft/client/Options;` | exact | getfield@3 in `SingleVariantMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `options` | `Lnet/minecraft/client/Options;` | exact | getfield@5 in `FluidVariantRendering.getTooltip` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `options` | `Lnet/minecraft/client/Options;` | exact | getfield@5 in `FluidVariantRendering.getTooltip` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@1 in `TestInputImpl.lambda$lookAt$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@19 in `TestInputImpl.lambda$lookAt$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@1 in `TestInputImpl.lambda$lookAt$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@19 in `TestInputImpl.lambda$lookAt$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@27 in `TestInputImpl.lambda$lookAt$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@8 in `TestServerConnectionImpl.getClientPlayer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@4 in `ClientSuggestionProviderMixin.getPlayer` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@4 in `MultiPlayerGameModeMixin.method_2902` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@13 in `MultiPlayerGameModeMixin.fabric_fireAttackBlockCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@20 in `MultiPlayerGameModeMixin.fabric$onBlockBroken` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@23 in `MultiPlayerGameModeMixin.fabricItemContinueBlockBreakingInject` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@53 in `MultiPlayerGameModeMixin.fabricItemContinueBlockBreakingInject` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@93 in `ClientNetworking.openScreen` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/client/player/LocalPlayer;` | exact | getfield@4 in `ClientPlayNetworkAddon$ContextImpl.player` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| wraps | `run` | `()V` | name_only | @WrapMethod | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (118 fields, 238 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static instance : Lnet/minecraft/client/Minecraft;
private static final LOGGER : Lorg/slf4j/Logger;
private static final TRACY_CURRENT_LEVEL : Lcom/mojang/jtracy/SectionCategory;
private static final MAX_TICKS_PER_UPDATE : I
public static final DEFAULT_FONT : Lnet/minecraft/resources/Identifier;
private static final REGIONAL_COMPLIANCIES : Lnet/minecraft/resources/Identifier;
private static final RESOURCE_RELOAD_INITIAL_TASK : Ljava/util/concurrent/CompletableFuture;
public static final UPDATE_DRIVERS_ADVICE : Ljava/lang/String;
private final canary : J
private final resourcePackDirectory : Ljava/nio/file/Path;
private final profileFuture : Ljava/util/concurrent/CompletableFuture;
private final textureManager : Lnet/minecraft/client/renderer/texture/TextureManager;
private final shaderManager : Lnet/minecraft/client/renderer/ShaderManager;
private final fixerUpper : Lcom/mojang/datafixers/DataFixer;
private final monitorManager : Lcom/mojang/blaze3d/platform/MonitorManager;
private final window : Lcom/mojang/blaze3d/platform/Window;
private final windowSurface : Lcom/mojang/renderpearl/api/device/GpuSurface;
private final sdlEventHandler : Lcom/mojang/blaze3d/platform/SDLEventHandler;
private final textInputManager : Lcom/mojang/blaze3d/platform/TextInputManager;
private final deltaTracker : Lnet/minecraft/client/DeltaTracker$Timer;
public final levelExtractor : Lnet/minecraft/client/renderer/extract/LevelExtractor;
public final levelRenderer : Lnet/minecraft/client/renderer/LevelRenderer;
private final entityRenderDispatcher : Lnet/minecraft/client/renderer/entity/EntityRenderDispatcher;
private final itemModelResolver : Lnet/minecraft/client/renderer/item/ItemModelResolver;
private final mapRenderer : Lnet/minecraft/client/renderer/MapRenderer;
public final particleEngine : Lnet/minecraft/client/particle/ParticleEngine;
private final user : Lnet/minecraft/client/User;
public final font : Lnet/minecraft/client/gui/Font;
public final fontFilterFishy : Lnet/minecraft/client/gui/Font;
public final gameRenderer : Lnet/minecraft/client/renderer/GameRenderer;
public final gui : Lnet/minecraft/client/gui/Gui;
public final options : Lnet/minecraft/client/Options;
public final debugEntries : Lnet/minecraft/client/gui/components/debug/DebugScreenEntryList;
private final hotbarManager : Lnet/minecraft/client/HotbarManager;
public final mouseHandler : Lnet/minecraft/client/MouseHandler;
public final keyboardHandler : Lnet/minecraft/client/KeyboardHandler;
private lastInputType : Lnet/minecraft/client/InputType;
public final gameDirectory : Ljava/io/File;
private final launchedVersion : Ljava/lang/String;
private final proxy : Ljava/net/Proxy;
private final offlineDeveloperMode : Z
private final levelSource : Lnet/minecraft/world/level/storage/LevelStorageSource;
private final demo : Z
private final allowsMultiplayer : Z
private final allowsChat : Z
private final resourceManager : Lnet/minecraft/server/packs/resources/ReloadableResourceManager;
private final vanillaPackResources : Lnet/minecraft/server/packs/VanillaPackResources;
private final downloadedPackSource : Lnet/minecraft/client/resources/server/DownloadedPackSource;
private final resourcePackRepository : Lnet/minecraft/server/packs/repository/PackRepository;
private final languageManager : Lnet/minecraft/client/resources/language/LanguageManager;
private final blockColors : Lnet/minecraft/client/color/block/BlockColors;
private final tracyFrameCapture : Lcom/mojang/blaze3d/TracyFrameCapture;
private final soundManager : Lnet/minecraft/client/sounds/SoundManager;
private final musicManager : Lnet/minecraft/client/sounds/MusicManager;
private final fontManager : Lnet/minecraft/client/gui/font/FontManager;
private final gpuWarnlistManager : Lnet/minecraft/client/renderer/GpuWarnlistManager;
private final regionalCompliancies : Lnet/minecraft/client/PeriodicNotificationManager;
private final userApiService : Lcom/mojang/authlib/minecraft/UserApiService;
private final userPropertiesFuture : Ljava/util/concurrent/CompletableFuture;
private final skinManager : Lnet/minecraft/client/resources/SkinManager;
private final atlasManager : Lnet/minecraft/client/resources/model/sprite/AtlasManager;
private final modelManager : Lnet/minecraft/client/resources/model/ModelManager;
private final palettedTextureManager : Lnet/minecraft/client/resources/palette/PalettedTextureManager;
private final mapTextureManager : Lnet/minecraft/client/resources/MapTextureManager;
private final tutorial : Lnet/minecraft/client/tutorial/Tutorial;
private final playerSocialManager : Lnet/minecraft/client/gui/screens/social/PlayerSocialManager;
private final remoteFriendListUpdateHandler : Lnet/minecraft/client/gui/screens/social/RemoteFriendListUpdateHandler;
private final blockEntityRenderDispatcher : Lnet/minecraft/client/renderer/blockentity/BlockEntityRenderDispatcher;
private final telemetryManager : Lnet/minecraft/client/telemetry/ClientTelemetryManager;
private final profileKeyPairManager : Lnet/minecraft/client/multiplayer/ProfileKeyPairManager;
private final realmsDataFetcher : Lcom/mojang/realmsclient/gui/RealmsDataFetcher;
private final quickPlayLog : Lnet/minecraft/client/quickplay/QuickPlayLog;
private final services : Lnet/minecraft/server/Services;
private final playerSkinRenderCache : Lnet/minecraft/client/renderer/PlayerSkinRenderCache;
private final timerQuery : Lcom/mojang/blaze3d/systems/TimerQuery;
public gameMode : Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;
public level : Lnet/minecraft/client/multiplayer/ClientLevel;
public player : Lnet/minecraft/client/player/LocalPlayer;
private singleplayerServer : Lnet/minecraft/client/server/IntegratedServer;
private pendingConnection : Lnet/minecraft/network/Connection;
private isLocalServer : Z
private tracyLevelSection : Lcom/mojang/jtracy/Section;
public crosshairPickEntity : Lnet/minecraft/world/entity/Entity;
public hitResult : Lnet/minecraft/world/phys/HitResult;
private rightClickDelay : I
public missTime : I
private pause : Z
private lastNanoTime : J
private lastTime : J
private frames : I
private gameThread : Ljava/lang/Thread;
private running : Z
private static fps : I
private frameTimeNs : J
private final framerateLimitTracker : Lcom/mojang/blaze3d/platform/FramerateLimitTracker;
public multiDrawIndirect : Z
public wireframe : Z
public smartCull : Z
private lastActiveTime : J
private pendingReload : Ljava/util/concurrent/CompletableFuture;
private fpsPieRenderTicks : I
private final fpsPieProfiler : Lnet/minecraft/util/profiling/ContinuousProfiler;
private metricsRecorder : Lnet/minecraft/util/profiling/metrics/profiling/MetricsRecorder;
private final reloadStateTracker : Lnet/minecraft/client/ResourceLoadStateTracker;
private savedCpuDuration : J
private gpuUtilization : D
private final narrator : Lnet/minecraft/client/GameNarrator;
private reportingContext : Lnet/minecraft/client/multiplayer/chat/report/ReportingContext;
private final directoryValidator : Lnet/minecraft/world/level/validation/DirectoryValidator;
private gameLoadFinished : Z
private final clientStartTimeMs : J
private clientTickCount : J
private final packetProcessor : Lnet/minecraft/network/PacketProcessor;
private final perTickGizmos : Lnet/minecraft/gizmos/SimpleGizmoCollector;
private drainedLatestTickGizmos : Ljava/util/List;
private windowSurfaceNeedsReconfiguring : Z
private surfaceIsInvalid : Z
private backendCreationException : Lcom/mojang/renderpearl/api/device/BackendCreationException;
public <init>(Lnet/minecraft/client/main/GameConfig;)V
public hasShiftDown()Z
public hasControlDown()Z
public hasAltDown()Z
private onResourceLoadFinished(Lnet/minecraft/client/GameLoadCookie;)V
private onGameLoadFinished(Lnet/minecraft/client/GameLoadCookie;)V
public isGameLoadFinished()Z
private static countryEqualsISO3(Ljava/lang/Object;)Z
public updateTitle()V
private createTitle()Ljava/lang/String;
private static createUserApiService(Lcom/mojang/authlib/services/MinecraftServicesDiscoveryService;Lnet/minecraft/client/main/GameConfig;)Lcom/mojang/authlib/minecraft/UserApiService;
public isOfflineDeveloperMode()Z
public static checkModStatus()Lnet/minecraft/util/ModCheck;
private loadCriticalShaders()V
private rollbackResourcePacks(Ljava/lang/Throwable;Lnet/minecraft/client/GameLoadCookie;)V
public clearResourcePacksOnError(Ljava/lang/Throwable;Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/GameLoadCookie;)V
private abortResourcePackRecovery()V
private addResourcePackLoadFailToast(Lnet/minecraft/network/chat/Component;)V
public triggerResourcePackRecovery(Ljava/lang/Exception;)V
public run()V
 updateFontOptions()V
public getLaunchedVersion()Ljava/lang/String;
public delayCrash(Lnet/minecraft/CrashReport;)V
public emergencySaveAndCrash(Lnet/minecraft/CrashReport;)V
public static saveReport(Ljava/io/File;Lnet/minecraft/CrashReport;)V
public static saveReport(Ljava/io/File;Lnet/minecraft/CrashReport;I)I
public static crash(Lnet/minecraft/client/Minecraft;Ljava/io/File;Lnet/minecraft/CrashReport;I)V
public static saveReportAndShutdownSoundManager(Lnet/minecraft/client/Minecraft;Ljava/io/File;Lnet/minecraft/CrashReport;I)I
public isEnforceUnicode()Z
public reloadResourcePacks()Ljava/util/concurrent/CompletableFuture;
private reloadResourcePacks(ZLnet/minecraft/client/GameLoadCookie;)Ljava/util/concurrent/CompletableFuture;
private selfTest()V
public getLevelSource()Lnet/minecraft/world/level/storage/LevelStorageSource;
public exitWorldAndClose()V
public close()V
private runTick(Z)V
public renderFrame(Z)V
private pauseIfInactive()V
private constructProfiler(ZLnet/minecraft/util/profiling/SingleTickProfiler;)Lnet/minecraft/util/profiling/ProfilerFiller;
private finishProfilers(ZLnet/minecraft/util/profiling/SingleTickProfiler;)V
public framebufferSizeChanged()V
public resizeGui()V
public cursorEntered()V
public fullscreenStateChanged(Z)V
public getFps()I
public getFrameTimeNs()J
public sendLowDiskSpaceWarning()V
private emergencySave()V
public debugClientMetricsStart(Ljava/util/function/Consumer;)Z
private debugClientMetricsStop()V
private debugClientMetricsCancel()V
private archiveProfilingReport(Lnet/minecraft/SystemReport;Ljava/util/List;)Ljava/nio/file/Path;
public stop()V
public isRunning()Z
public pauseGame(Z)V
private continueAttack(Z)V
private startAttack()Z
private startUseItem()V
public getMusicManager()Lnet/minecraft/client/sounds/MusicManager;
public tick()V
private isLevelRunningNormally()Z
public isMultiplayerServer()Z
private handleKeybinds()V
private getQuickActionsDialog()Ljava/util/Optional;
public getTelemetryManager()Lnet/minecraft/client/telemetry/ClientTelemetryManager;
public getMetricsRecorder()Lnet/minecraft/util/profiling/metrics/profiling/MetricsRecorder;
public getGpuUtilization()D
public getProfileKeyPairManager()Lnet/minecraft/client/multiplayer/ProfileKeyPairManager;
public createWorldOpenFlows()Lnet/minecraft/client/gui/screens/worldselection/WorldOpenFlows;
public doWorldLoad(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;Z)V
public setLevel(Lnet/minecraft/client/multiplayer/ClientLevel;)V
public disconnectFromWorld(Lnet/minecraft/network/chat/Component;)V
public disconnectWithSavingScreen()V
public disconnectWithProgressScreen()V
public disconnectWithProgressScreen(Z)V
public disconnect(Lnet/minecraft/client/gui/screens/Screen;Z)V
public disconnect(Lnet/minecraft/client/gui/screens/Screen;ZZ)V
public clearDownloadedResourcePacks()V
public clearClientLevel(Lnet/minecraft/client/gui/screens/Screen;)V
public setScreenAndShow(Lnet/minecraft/client/gui/screens/Screen;)V
private updateLevelInEngines(Lnet/minecraft/client/multiplayer/ClientLevel;)V
private updateLevelInEngines(Lnet/minecraft/client/multiplayer/ClientLevel;Z)V
private userProperties()Lcom/mojang/authlib/minecraft/UserApiService$UserProperties;
public telemetryOptInExtra()Z
public extraTelemetryAvailable()Z
public allowsTelemetry()Z
public allowsMultiplayer()Z
public allowsRealms()Z
public friendsEnabled()Z
public handleGlobalKeyPress(Lcom/mojang/blaze3d/platform/InputConstants$Key;Z)Z
private toggleFullscreen()V
private toggleFriendsScreen()Z
public allowFriendRequests()Z
public allowChatOnlyWithFriend()Z
public multiplayerBan()Lcom/mojang/authlib/minecraft/BanDetails;
public isNameBanned()Z
public isBlocked(Ljava/util/UUID;)Z
public isFriendOnlyRestricted(Ljava/util/UUID;)Z
private isLocalOrUnknownPlayer(Ljava/util/UUID;)Z
public computeChatAbilities()Lnet/minecraft/client/multiplayer/chat/ChatAbilities;
public final isDemo()Z
public final canSwitchGameMode()Z
public getConnection()Lnet/minecraft/client/multiplayer/ClientPacketListener;
private pickBlockOrEntity()V
public fillReport(Lnet/minecraft/CrashReport;)Lnet/minecraft/CrashReport;
public static fillReport(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/resources/language/LanguageManager;Ljava/lang/String;Lnet/minecraft/client/Options;Lnet/minecraft/CrashReport;)V
private static formatSeconds(D)Ljava/lang/String;
private fillUptime(Lnet/minecraft/CrashReportCategory;)V
private static fillSystemReport(Lnet/minecraft/SystemReport;Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/resources/language/LanguageManager;Ljava/lang/String;Lnet/minecraft/client/Options;)Lnet/minecraft/SystemReport;
public static getInstance()Lnet/minecraft/client/Minecraft;
public delayTextureReload()Ljava/util/concurrent/CompletableFuture;
public updateReportEnvironment(Lnet/minecraft/client/multiplayer/chat/report/ReportEnvironment;)V
public getCurrentServer()Lnet/minecraft/client/multiplayer/ServerData;
public isLocalServer()Z
public hasSingleplayerServer()Z
public getSingleplayerServer()Lnet/minecraft/client/server/IntegratedServer;
public isLocalPlayer(Ljava/util/UUID;)Z
public getUser()Lnet/minecraft/client/User;
public getProfileResult()Lcom/mojang/authlib/services/ProfileResult;
public getGameProfile()Lcom/mojang/authlib/GameProfile;
public getProxy()Ljava/net/Proxy;
public getTextureManager()Lnet/minecraft/client/renderer/texture/TextureManager;
public getShaderManager()Lnet/minecraft/client/renderer/ShaderManager;
public getResourceManager()Lnet/minecraft/server/packs/resources/ResourceManager;
public getResourcePackRepository()Lnet/minecraft/server/packs/repository/PackRepository;
public getVanillaPackResources()Lnet/minecraft/server/packs/VanillaPackResources;
public getDownloadedPackSource()Lnet/minecraft/client/resources/server/DownloadedPackSource;
public getResourcePackDirectory()Ljava/nio/file/Path;
public getLanguageManager()Lnet/minecraft/client/resources/language/LanguageManager;
public isPaused()Z
public getGpuWarnlistManager()Lnet/minecraft/client/renderer/GpuWarnlistManager;
public getSoundManager()Lnet/minecraft/client/sounds/SoundManager;
public getSituationalMusic()Lnet/minecraft/sounds/Music;
public getMusicVolume()F
public services()Lnet/minecraft/server/Services;
public getSkinManager()Lnet/minecraft/client/resources/SkinManager;
public getCameraEntity()Lnet/minecraft/world/entity/Entity;
public setCameraEntity(Lnet/minecraft/world/entity/Entity;)V
public shouldEntityAppearGlowing(Lnet/minecraft/world/entity/Entity;)Z
public getRunningThread()Ljava/lang/Thread;
public wrapRunnable(Ljava/lang/Runnable;)Ljava/lang/Runnable;
protected shouldRun(Ljava/lang/Runnable;)Z
public getEntityRenderDispatcher()Lnet/minecraft/client/renderer/entity/EntityRenderDispatcher;
public getBlockEntityRenderDispatcher()Lnet/minecraft/client/renderer/blockentity/BlockEntityRenderDispatcher;
public getMapRenderer()Lnet/minecraft/client/renderer/MapRenderer;
public getFixerUpper()Lcom/mojang/datafixers/DataFixer;
public getDeltaTracker()Lnet/minecraft/client/DeltaTracker;
public getBlockColors()Lnet/minecraft/client/color/block/BlockColors;
public showOnlyReducedInfo()Z
public getTutorial()Lnet/minecraft/client/tutorial/Tutorial;
public isWindowActive()Z
public getHotbarManager()Lnet/minecraft/client/HotbarManager;
public getModelManager()Lnet/minecraft/client/resources/model/ModelManager;
public getAtlasManager()Lnet/minecraft/client/resources/model/sprite/AtlasManager;
public getPalettedTextureManager()Lnet/minecraft/client/resources/palette/PalettedTextureManager;
public getMapTextureManager()Lnet/minecraft/client/resources/MapTextureManager;
public grabPanoramixScreenshot(Ljava/io/File;)Lnet/minecraft/network/chat/Component;
public getPlayerSocialManager()Lnet/minecraft/client/gui/screens/social/PlayerSocialManager;
public getWindow()Lcom/mojang/blaze3d/platform/Window;
public textInputManager()Lcom/mojang/blaze3d/platform/TextInputManager;
public onTextInputFocusChange(Lnet/minecraft/client/gui/components/events/GuiEventListener;Z)V
public windowSurface()Lcom/mojang/renderpearl/api/device/GpuSurface;
public getFramerateLimitTracker()Lcom/mojang/blaze3d/platform/FramerateLimitTracker;
public getDebugOverlay()Lnet/minecraft/client/gui/components/DebugScreenOverlay;
public updateMaxMipLevel(I)V
public getEntityModels()Lnet/minecraft/client/model/geom/EntityModelSet;
public isTextFilteringEnabled()Z
public prepareForMultiplayer()V
public getLastInputType()Lnet/minecraft/client/InputType;
public setLastInputType(Lnet/minecraft/client/InputType;)V
public getNarrator()Lnet/minecraft/client/GameNarrator;
public getReportingContext()Lnet/minecraft/client/multiplayer/chat/report/ReportingContext;
public realmsDataFetcher()Lcom/mojang/realmsclient/gui/RealmsDataFetcher;
public quickPlayLog()Lnet/minecraft/client/quickplay/QuickPlayLog;
public directoryValidator()Lnet/minecraft/world/level/validation/DirectoryValidator;
public playerSkinRenderCache()Lnet/minecraft/client/renderer/PlayerSkinRenderCache;
private getTickTargetMillis(F)F
public getItemModelResolver()Lnet/minecraft/client/renderer/item/ItemModelResolver;
public canInterruptScreen()Z
public invalidateSurfaceConfiguration()V
public static getLauncherBrand()Ljava/lang/String;
public packetProcessor()Lnet/minecraft/network/PacketProcessor;
public collectPerTickGizmos()Lnet/minecraft/gizmos/Gizmos$TemporaryCollection;
public getPerTickGizmos()Ljava/util/Collection;
private pick(F)V
public showDebugChat(Lnet/minecraft/network/chat/Component;)V
private static synthetic lambda$grabPanoramixScreenshot$1(Ljava/io/File;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
private static synthetic lambda$grabPanoramixScreenshot$0(Lnet/minecraft/network/chat/Component;)V
private static synthetic lambda$delayTextureReload$0(Ljava/util/concurrent/CompletableFuture;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$fillSystemReport$11()Ljava/lang/Object;
private static synthetic lambda$fillSystemReport$10()Ljava/lang/Object;
private static synthetic lambda$fillSystemReport$9(Lnet/minecraft/client/resources/language/LanguageManager;)Ljava/lang/Object;
private static synthetic lambda$fillSystemReport$8(Lnet/minecraft/client/Minecraft;)Ljava/lang/Object;
private static synthetic lambda$fillSystemReport$7(Lnet/minecraft/client/Minecraft;)Ljava/lang/Object;
private static synthetic lambda$fillSystemReport$6(Lnet/minecraft/client/Minecraft;)Ljava/lang/Object;
private static synthetic lambda$fillSystemReport$5(Lnet/minecraft/client/Minecraft;)Ljava/lang/Object;
private static synthetic lambda$fillSystemReport$4()Ljava/lang/Object;
private static synthetic lambda$fillSystemReport$3(Lcom/mojang/renderpearl/api/device/GpuDevice;)Ljava/lang/Object;
private static synthetic lambda$fillSystemReport$2(Lnet/minecraft/client/Minecraft;)Ljava/lang/Object;
private static synthetic lambda$fillSystemReport$1(Lnet/minecraft/client/Minecraft;)Ljava/lang/Object;
private static synthetic lambda$fillSystemReport$0(Ljava/lang/String;)Ljava/lang/Object;
private synthetic lambda$fillUptime$3()Ljava/lang/String;
private static synthetic lambda$fillUptime$2()Ljava/lang/String;
private synthetic lambda$fillUptime$1()Ljava/lang/String;
private static synthetic lambda$fillUptime$0()Ljava/lang/String;
private synthetic lambda$toggleFriendsScreen$0(Lnet/minecraft/client/gui/screens/Screen;)V
private static synthetic lambda$doWorldLoad$3(Lnet/minecraft/network/chat/Component;)V
private static synthetic lambda$doWorldLoad$2(J)Z
private static synthetic lambda$doWorldLoad$1(Lnet/minecraft/server/WorldStem;)Ljava/lang/String;
private synthetic lambda$doWorldLoad$0(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;Lnet/minecraft/server/level/progress/LevelLoadListener;Ljava/lang/Thread;)Lnet/minecraft/client/server/IntegratedServer;
private static synthetic lambda$getQuickActionsDialog$0(Lnet/minecraft/core/Registry;Lnet/minecraft/core/HolderSet$Named;)Ljava/util/Optional;
private synthetic lambda$handleKeybinds$0(Lnet/minecraft/core/Holder;)V
private static synthetic lambda$tick$0()Z
private synthetic lambda$debugClientMetricsStart$9(Ljava/util/function/Consumer;Lnet/minecraft/util/profiling/ProfileResults;)V
private static synthetic lambda$debugClientMetricsStart$8(Lnet/minecraft/util/profiling/ProfileResults;)V
private static synthetic lambda$debugClientMetricsStart$7(Ljava/util/function/Consumer;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;)V
private static synthetic lambda$debugClientMetricsStart$6(Ljava/util/function/Consumer;Ljava/nio/file/Path;)V
private synthetic lambda$debugClientMetricsStart$5(Lnet/minecraft/SystemReport;Ljava/util/function/Consumer;Ljava/util/List;)V
private synthetic lambda$debugClientMetricsStart$2(Ljava/util/function/Consumer;Ljava/nio/file/Path;)V
private static synthetic lambda$debugClientMetricsStart$4(Ljava/util/function/Consumer;Lnet/minecraft/network/chat/Component;)V
private static synthetic lambda$debugClientMetricsStart$3(Ljava/nio/file/Path;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
private synthetic lambda$debugClientMetricsStart$0(Ljava/util/function/Consumer;Lnet/minecraft/util/profiling/ProfileResults;)V
private static synthetic lambda$debugClientMetricsStart$1(Ljava/util/function/Consumer;DI)V
private synthetic lambda$sendLowDiskSpaceWarning$0()V
private static synthetic lambda$runTick$0(Ljava/util/concurrent/CompletableFuture;)V
private synthetic lambda$reloadResourcePacks$0(ZLnet/minecraft/client/GameLoadCookie;Ljava/util/concurrent/CompletableFuture;Ljava/util/Optional;)V
private synthetic lambda$reloadResourcePacks$2(Ljava/util/concurrent/CompletableFuture;Lnet/minecraft/client/GameLoadCookie;)V
private synthetic lambda$reloadResourcePacks$1(ZLnet/minecraft/client/GameLoadCookie;Ljava/lang/Throwable;)V
private synthetic lambda$clearResourcePacksOnError$0(Lnet/minecraft/network/chat/Component;)V
private synthetic lambda$new$7(Lnet/minecraft/client/telemetry/TelemetryPropertyMap$Builder;)V
private synthetic lambda$new$6()I
private synthetic lambda$new$3(Lnet/minecraft/client/GameLoadCookie;Ljava/util/Optional;)V
private synthetic lambda$new$5(Lnet/minecraft/client/GameLoadCookie;)V
private synthetic lambda$new$4(Lnet/minecraft/client/GameLoadCookie;Ljava/lang/Throwable;)V
private synthetic lambda$new$2(Lnet/minecraft/client/resources/language/ClientLanguage;)V
private synthetic lambda$new$1()Lcom/mojang/authlib/minecraft/UserApiService$UserProperties;
private synthetic lambda$new$0()Lcom/mojang/authlib/services/ProfileResult;
static <clinit>()V
```
