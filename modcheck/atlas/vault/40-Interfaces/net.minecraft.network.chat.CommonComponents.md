---
type: "interface"
fqcn: "net.minecraft.network.chat.CommonComponents"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.CommonComponents

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `EMPTY` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@16 in `OptimizedScrollableLayout$Container.<init>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GUI_BACK` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@63 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `GUI_BACK` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@4 in `DetailsScreen.addFooter` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GUI_CANCEL` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@69 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@34 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@42 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@86 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@128 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@27 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@66 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@27 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@78 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@27 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@66 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@27 in `AttachmentTargetInfo$GlobalTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@27 in `AttachmentTargetInfo$LevelTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@42 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@53 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@126 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@196 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@174 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@256 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@267 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@273 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@455 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `NEW_LINE` | `Lnet/minecraft/network/chat/Component;` | exact | getstatic@652 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (25 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/network/chat/Component;
public static final OPTION_ON : Lnet/minecraft/network/chat/Component;
public static final OPTION_OFF : Lnet/minecraft/network/chat/Component;
public static final GUI_DONE : Lnet/minecraft/network/chat/Component;
public static final GUI_CANCEL : Lnet/minecraft/network/chat/Component;
public static final GUI_YES : Lnet/minecraft/network/chat/Component;
public static final GUI_REMOVE : Lnet/minecraft/network/chat/Component;
public static final GUI_NO : Lnet/minecraft/network/chat/Component;
public static final GUI_OK : Lnet/minecraft/network/chat/Component;
public static final GUI_PROCEED : Lnet/minecraft/network/chat/Component;
public static final GUI_CONTINUE : Lnet/minecraft/network/chat/Component;
public static final GUI_BACK : Lnet/minecraft/network/chat/Component;
public static final GUI_TO_TITLE : Lnet/minecraft/network/chat/Component;
public static final GUI_ACKNOWLEDGE : Lnet/minecraft/network/chat/Component;
public static final GUI_OPEN_IN_BROWSER : Lnet/minecraft/network/chat/Component;
public static final GUI_COPY_TO_CLIPBOARD : Lnet/minecraft/network/chat/Component;
public static final GUI_COPY_LINK_TO_CLIPBOARD : Lnet/minecraft/network/chat/Component;
public static final GUI_DISCONNECT : Lnet/minecraft/network/chat/Component;
public static final GUI_RETURN_TO_MENU : Lnet/minecraft/network/chat/Component;
public static final TRANSFER_CONNECT_FAILED : Lnet/minecraft/network/chat/Component;
public static final CONNECT_FAILED : Lnet/minecraft/network/chat/Component;
public static final NEW_LINE : Lnet/minecraft/network/chat/Component;
public static final NARRATION_SEPARATOR : Lnet/minecraft/network/chat/Component;
public static final ELLIPSIS : Lnet/minecraft/network/chat/Component;
public static final SPACE : Lnet/minecraft/network/chat/Component;
public <init>()V
public static space()Lnet/minecraft/network/chat/MutableComponent;
public static days(J)Lnet/minecraft/network/chat/MutableComponent;
public static hours(J)Lnet/minecraft/network/chat/MutableComponent;
public static minutes(J)Lnet/minecraft/network/chat/MutableComponent;
public static optionStatus(Z)Lnet/minecraft/network/chat/Component;
public static disconnectButtonLabel(Z)Lnet/minecraft/network/chat/Component;
public static optionStatus(Lnet/minecraft/network/chat/Component;Z)Lnet/minecraft/network/chat/MutableComponent;
public static optionNameValue(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/MutableComponent;
public static joinForNarration([Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/MutableComponent;
public static joinLines([Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
public static joinLines(Ljava/util/Collection;)Lnet/minecraft/network/chat/Component;
static <clinit>()V
```
