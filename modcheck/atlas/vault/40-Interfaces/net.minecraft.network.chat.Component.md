---
type: "interface"
fqcn: "net.minecraft.network.chat.Component"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.Component

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

`interface` public abstract; extends `java/lang/Object`; implements `com/mojang/brigadier/Message`, `net/minecraft/network/chat/FormattedText`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `copy` | `()Lnet/minecraft/network/chat/MutableComponent;` | exact | invokeinterface@37 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `copy` | `()Lnet/minecraft/network/chat/MutableComponent;` | exact | invokeinterface@7 in `FluidVariantAttributeHandler.getColoredName` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `empty` | `()Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@1 in `ClientSuggestionProviderMixin.sendError` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `empty` | `()Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@15 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `empty` | `()Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@0 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `empty` | `()Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@0 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getContents` | `()Lnet/minecraft/network/chat/ComponentContents;` | exact | invokeinterface@13 in `ClientGameTestImpl.isExperimentalWarningScreen` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getContents` | `()Lnet/minecraft/network/chat/ComponentContents;` | exact | invokeinterface@17 in `FabricLanguageProvider$TranslationBuilder.addCreativeModeTab` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getString` | `()Ljava/lang/String;` | exact | invokeinterface@27 in `ClientGameTestContextImpl.pressMatchingButton` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getString` | `()Ljava/lang/String;` | exact | invokeinterface@71 in `ClientGameTestContextImpl.pressMatchingButton` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getString` | `()Ljava/lang/String;` | exact | invokeinterface@364 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getString` | `()Ljava/lang/String;` | exact | invokeinterface@2 in `AttachmentSyncException.<init>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getString` | `()Ljava/lang/String;` | exact | invokeinterface@68 in `FabricLanguageProvider$TranslationBuilder.addCreativeModeTab` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getString` | `()Ljava/lang/String;` | exact | invokeinterface@2 in `RemapException.<init>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getVisualOrderText` | `()Lnet/minecraft/util/FormattedCharSequence;` | exact | invokeinterface@32 in `TransferableSelectionListPackEntryMixin.onExtractContent` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@23 in `TestDedicatedServerConnectionImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@40 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@61 in `ClientCommandInternals.executeHelp` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@9 in `FabricCreativeGuiComponents$Type.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@37 in `FabricCreativeGuiComponents$Type.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@70 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@112 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@50 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@62 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@50 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@38 in `EnumRuleCommand.executeAndSetEnum` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@2 in `ServerHandshakePacketListenerImplMixin.<clinit>` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@112 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@182 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@55 in `ClientRegistrySyncHandler.getComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@120 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@158 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@225 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@235 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@239 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@281 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@3 in `ResourceManagerHelper.registerBuiltinResourcePack` | unknown | [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@16 in `ResourceManagerHelper.registerBuiltinResourcePack` | unknown | [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@16 in `ResourceLoaderImpl.registerBuiltinPack` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@2 in `ModPackResourcesUtil.serializeMetadata` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@15 in `ModPackResourcesUtil.getName` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@74 in `FluidVariantRendering.getTooltip` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `nullToEmpty` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;` | exact | invokestatic@163 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@7 in `ClientGameTestContextImpl.tryClickScreenButtonImpl` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@23 in `TestSingleplayerContextImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@36 in `TestSingleplayerContextImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@28 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@55 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@22 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@11 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@11 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@11 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@11 in `AttachmentTargetInfo$GlobalTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@11 in `AttachmentTargetInfo$LevelTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@19 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@53 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@70 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@40 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@76 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@93 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@27 in `DetailedBackupConfirmScreen.init` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@10 in `DetailedBackupConfirmScreen.lambda$init$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@25 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@64 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@18 in `WorldOpenFlowsMixin.replaceBackupScreen` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@23 in `WorldOpenFlowsMixin.replaceBackupScreen` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@238 in `ModNioPackResources.create` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@14 in `ModResourcePackCreator$1.decorate` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokestatic@51 in `FluidVariantAttributeHandler.getName` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@50 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@41 in `ClientCommandInternals.getErrorMessage` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@171 in `FabricCreativeGuiComponents$CreativeModeTabButton.extractContents` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@80 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@122 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@21 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@60 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@21 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@72 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@21 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@60 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@21 in `AttachmentTargetInfo$GlobalTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@21 in `AttachmentTargetInfo$LevelTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@21 in `EnumRuleCommand.lambda$executeAndSetEnum$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@43 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@171 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@65 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@243 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@287 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@354 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@425 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@514 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@600 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@642 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@25 in `BuiltinModPackSource.decorate` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@29 in `BuiltinModPackSource.decorate` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@204 in `ModNioPackResources.create` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@242 in `ModNioPackResources.create` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@35 in `ModPackResourcesUtil.getName` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatable` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@18 in `ModResourcePackCreator$1.decorate` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatableEscape` | `(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/Mut` | exact | invokestatic@10 in `DataPackCommandMixin.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatableWithFallback` | `(Ljava/lang/String;Ljava/lang/String;)Lnet/minecraft/network/chat/Muta` | exact | invokestatic@21 in `FabricTagKey.getName` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `translatableWithFallback` | `(Ljava/lang/String;Ljava/lang/String;)Lnet/minecraft/network/chat/Muta` | exact | invokestatic@25 in `EnumRuleEntry.getValueComponent` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (0 fields, 37 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getStyle()Lnet/minecraft/network/chat/Style;
public abstract getContents()Lnet/minecraft/network/chat/ComponentContents;
public getString()Ljava/lang/String;
public getString(I)Ljava/lang/String;
public abstract getSiblings()Ljava/util/List;
public tryCollapseToString()Ljava/lang/String;
public plainCopy()Lnet/minecraft/network/chat/MutableComponent;
public copy()Lnet/minecraft/network/chat/MutableComponent;
public abstract getVisualOrderText()Lnet/minecraft/util/FormattedCharSequence;
public visit(Lnet/minecraft/network/chat/FormattedText$StyledContentConsumer;Lnet/minecraft/network/chat/Style;)Ljava/util/Optional;
public visit(Lnet/minecraft/network/chat/FormattedText$ContentConsumer;)Ljava/util/Optional;
public toFlatList()Ljava/util/List;
public toFlatList(Lnet/minecraft/network/chat/Style;)Ljava/util/List;
public contains(Lnet/minecraft/network/chat/Component;)Z
public static nullToEmpty(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
public static literal(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;
public static translatable(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;
public static translatable(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/MutableComponent;
public static translatableEscape(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/MutableComponent;
public static translatableWithFallback(Ljava/lang/String;Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;
public static translatableWithFallback(Ljava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/network/chat/MutableComponent;
public static empty()Lnet/minecraft/network/chat/MutableComponent;
public static keybind(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;
public static nbt(Lnet/minecraft/util/CompilableString;ZZLjava/util/Optional;Lnet/minecraft/network/chat/contents/data/DataSource;)Lnet/minecraft/network/chat/MutableComponent;
public static score(Lnet/minecraft/util/CompilableString;Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;
public static score(Ljava/lang/String;Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;
public static selector(Lnet/minecraft/util/CompilableString;Ljava/util/Optional;)Lnet/minecraft/network/chat/MutableComponent;
public static object(Lnet/minecraft/network/chat/contents/objects/ObjectInfo;)Lnet/minecraft/network/chat/MutableComponent;
public static object(Lnet/minecraft/network/chat/contents/objects/ObjectInfo;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/MutableComponent;
public static translationArg(Ljava/util/Date;)Lnet/minecraft/network/chat/Component;
public static translationArg(Lcom/mojang/brigadier/Message;)Lnet/minecraft/network/chat/Component;
public static translationArg(Ljava/util/UUID;)Lnet/minecraft/network/chat/Component;
public static translationArg(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/network/chat/Component;
public static translationArg(Lnet/minecraft/world/level/ChunkPos;)Lnet/minecraft/network/chat/Component;
public static translationArg(Ljava/net/URI;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$toFlatList$0(Ljava/util/List;Lnet/minecraft/network/chat/Style;Ljava/lang/String;)Ljava/util/Optional;
private static synthetic lambda$getString$0(ILjava/lang/StringBuilder;Ljava/lang/String;)Ljava/util/Optional;
```
