---
type: "interface"
fqcn: "net.minecraft.ChatFormatting"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.ChatFormatting

System: [[20-Systems/net.minecraft|net.minecraft]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `DARK_GRAY` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@77 in `FluidVariantRendering.getTooltip` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `GOLD` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@284 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GRAY` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@32 in `BuiltinModPackSource.decorate` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `GREEN` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@56 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GREEN` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@79 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GREEN` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@242 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `RED` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@8 in `ClientSuggestionProviderMixin.sendError` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `RED` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@25 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@43 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@73 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@115 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@14 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@53 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@14 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@65 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@14 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@53 in `AttachmentTargetInfo$EntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@14 in `AttachmentTargetInfo$GlobalTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@14 in `AttachmentTargetInfo$LevelTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@115 in `ClientRegistrySyncHandler.missingRegistriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@185 in `ClientRegistrySyncHandler.missingEntriesError` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/ChatFormatting;` | exact | getstatic@161 in `RegistrySyncManager.getIncompatibleClientComponent` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (27 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final BLACK : Lnet/minecraft/ChatFormatting;
public static final DARK_BLUE : Lnet/minecraft/ChatFormatting;
public static final DARK_GREEN : Lnet/minecraft/ChatFormatting;
public static final DARK_AQUA : Lnet/minecraft/ChatFormatting;
public static final DARK_RED : Lnet/minecraft/ChatFormatting;
public static final DARK_PURPLE : Lnet/minecraft/ChatFormatting;
public static final GOLD : Lnet/minecraft/ChatFormatting;
public static final GRAY : Lnet/minecraft/ChatFormatting;
public static final DARK_GRAY : Lnet/minecraft/ChatFormatting;
public static final BLUE : Lnet/minecraft/ChatFormatting;
public static final GREEN : Lnet/minecraft/ChatFormatting;
public static final AQUA : Lnet/minecraft/ChatFormatting;
public static final RED : Lnet/minecraft/ChatFormatting;
public static final LIGHT_PURPLE : Lnet/minecraft/ChatFormatting;
public static final YELLOW : Lnet/minecraft/ChatFormatting;
public static final WHITE : Lnet/minecraft/ChatFormatting;
public static final OBFUSCATED : Lnet/minecraft/ChatFormatting;
public static final BOLD : Lnet/minecraft/ChatFormatting;
public static final STRIKETHROUGH : Lnet/minecraft/ChatFormatting;
public static final UNDERLINE : Lnet/minecraft/ChatFormatting;
public static final ITALIC : Lnet/minecraft/ChatFormatting;
public static final RESET : Lnet/minecraft/ChatFormatting;
public static final PREFIX_CODE : C
private static final STRIP_FORMATTING_PATTERN : Ljava/util/regex/Pattern;
private final code : C
private final toString : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/ChatFormatting;
public static values()[Lnet/minecraft/ChatFormatting;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/ChatFormatting;
private <init>(Ljava/lang/String;IC)V
public toString()Ljava/lang/String;
public static stripFormatting(Ljava/lang/String;)Ljava/lang/String;
public static getByCode(C)Lnet/minecraft/ChatFormatting;
private static synthetic $values()[Lnet/minecraft/ChatFormatting;
static <clinit>()V
```
