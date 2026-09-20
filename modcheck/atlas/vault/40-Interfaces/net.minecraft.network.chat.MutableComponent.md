---
type: "interface"
fqcn: "net.minecraft.network.chat.MutableComponent"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.MutableComponent

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

`class` public final; extends `java/lang/Object`; implements `net/minecraft/network/chat/Component`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `append` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokevirtual@50 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `append` | `(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokevirtual@253 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@5 in `ClientSuggestionProviderMixin.sendError` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@31 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@37 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@45 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@83 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@89 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@125 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@131 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@24 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@30 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@63 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@69 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@24 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@30 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@75 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@81 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@24 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@30 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@63 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@69 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@24 in `AttachmentTargetInfo$GlobalTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@30 in `AttachmentTargetInfo$GlobalTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@24 in `AttachmentTargetInfo$LevelTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@30 in `AttachmentTargetInfo$LevelTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@45 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@56 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@22 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@46 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@62 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@73 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@121 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@129 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@174 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@43 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@68 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@85 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@96 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@191 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@199 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@246 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@167 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@177 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@228 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@248 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@259 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@264 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@270 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@276 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@290 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getString` | `()Ljava/lang/String;` | inherited_exact | invokevirtual@10 in `ClientGameTestContextImpl.tryClickScreenButtonImpl` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getString` | `()Ljava/lang/String;` | inherited_exact | invokevirtual@164 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withColor` | `(I)Lnet/minecraft/network/chat/MutableComponent;` | exact | invokevirtual@19 in `FluidVariantAttributeHandler.getColoredName` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `withColor` | `(Lnet/minecraft/network/chat/TextColor;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@31 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withColor` | `(Lnet/minecraft/network/chat/TextColor;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@70 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withColor` | `(Lnet/minecraft/network/chat/TextColor;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@360 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withColor` | `(Lnet/minecraft/network/chat/TextColor;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@520 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withColor` | `(Lnet/minecraft/network/chat/TextColor;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@606 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withColor` | `(Lnet/minecraft/network/chat/TextColor;)Lnet/minecraft/network/chat/Mu` | exact | invokevirtual@648 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@46 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@11 in `ClientSuggestionProviderMixin.sendError` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@28 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@76 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@118 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@17 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@56 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@17 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@68 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@17 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@56 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@17 in `AttachmentTargetInfo$GlobalTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@17 in `AttachmentTargetInfo$LevelTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@59 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@118 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@82 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@188 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@164 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@245 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@287 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@35 in `BuiltinModPackSource.decorate` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableCom` | exact | invokevirtual@80 in `FluidVariantRendering.getTooltip` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `withStyle` | `(Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Mutabl` | exact | invokevirtual@444 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (5 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final contents : Lnet/minecraft/network/chat/ComponentContents;
private final siblings : Ljava/util/List;
private style : Lnet/minecraft/network/chat/Style;
private visualOrderText : Lnet/minecraft/util/FormattedCharSequence;
private decomposedWith : Lnet/minecraft/locale/Language;
 <init>(Lnet/minecraft/network/chat/ComponentContents;Ljava/util/List;Lnet/minecraft/network/chat/Style;)V
public static create(Lnet/minecraft/network/chat/ComponentContents;)Lnet/minecraft/network/chat/MutableComponent;
public getContents()Lnet/minecraft/network/chat/ComponentContents;
public getSiblings()Ljava/util/List;
public setStyle(Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/MutableComponent;
public getStyle()Lnet/minecraft/network/chat/Style;
public append(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;
public append(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/MutableComponent;
public withStyle(Ljava/util/function/UnaryOperator;)Lnet/minecraft/network/chat/MutableComponent;
public withStyle(Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/MutableComponent;
public withStyle([Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableComponent;
public withStyle(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/MutableComponent;
public withColor(I)Lnet/minecraft/network/chat/MutableComponent;
public withColor(Lnet/minecraft/network/chat/TextColor;)Lnet/minecraft/network/chat/MutableComponent;
public withoutShadow()Lnet/minecraft/network/chat/MutableComponent;
public getVisualOrderText()Lnet/minecraft/util/FormattedCharSequence;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
```
