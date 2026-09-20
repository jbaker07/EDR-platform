---
type: "interface"
fqcn: "net.minecraft.network.chat.TextColor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.TextColor

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

`class` public final; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getValue` | `()I` | exact | invokevirtual@57 in `PackTooltipComponent.extractImage` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getValue` | `()I` | exact | invokevirtual@3 in `FluidVariantAttributes$2.getAssociatedColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getValue` | `()I` | exact | invokevirtual@3 in `FluidVariantAttributes$3.getAssociatedColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `BLUE` | `Lnet/minecraft/network/chat/TextColor;` | exact | getstatic@0 in `FluidVariantAttributes$2.getAssociatedColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `GOLD` | `Lnet/minecraft/network/chat/TextColor;` | exact | getstatic@67 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GRAY` | `Lnet/minecraft/network/chat/TextColor;` | exact | getstatic@357 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GRAY` | `Lnet/minecraft/network/chat/TextColor;` | exact | getstatic@438 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GRAY` | `Lnet/minecraft/network/chat/TextColor;` | exact | getstatic@603 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GRAY` | `Lnet/minecraft/network/chat/TextColor;` | exact | getstatic@54 in `PackTooltipComponent.extractImage` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `RED` | `Lnet/minecraft/network/chat/TextColor;` | exact | getstatic@28 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `RED` | `Lnet/minecraft/network/chat/TextColor;` | exact | getstatic@0 in `FluidVariantAttributes$3.getAssociatedColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/network/chat/TextColor;` | exact | getstatic@517 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `YELLOW` | `Lnet/minecraft/network/chat/TextColor;` | exact | getstatic@645 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (21 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final CUSTOM_COLOR_PREFIX : Ljava/lang/String;
public static final CODEC : Lcom/mojang/serialization/Codec;
private static final NAMED_COLORS : Ljava/util/Map;
public static final BLACK : Lnet/minecraft/network/chat/TextColor;
public static final DARK_BLUE : Lnet/minecraft/network/chat/TextColor;
public static final DARK_GREEN : Lnet/minecraft/network/chat/TextColor;
public static final DARK_AQUA : Lnet/minecraft/network/chat/TextColor;
public static final DARK_RED : Lnet/minecraft/network/chat/TextColor;
public static final DARK_PURPLE : Lnet/minecraft/network/chat/TextColor;
public static final GOLD : Lnet/minecraft/network/chat/TextColor;
public static final GRAY : Lnet/minecraft/network/chat/TextColor;
public static final DARK_GRAY : Lnet/minecraft/network/chat/TextColor;
public static final BLUE : Lnet/minecraft/network/chat/TextColor;
public static final GREEN : Lnet/minecraft/network/chat/TextColor;
public static final AQUA : Lnet/minecraft/network/chat/TextColor;
public static final RED : Lnet/minecraft/network/chat/TextColor;
public static final LIGHT_PURPLE : Lnet/minecraft/network/chat/TextColor;
public static final YELLOW : Lnet/minecraft/network/chat/TextColor;
public static final WHITE : Lnet/minecraft/network/chat/TextColor;
private final value : I
private final name : Ljava/lang/String;
private <init>(ILjava/lang/String;)V
private <init>(I)V
private static named(Ljava/lang/String;I)Lnet/minecraft/network/chat/TextColor;
public getValue()I
public serialize()Ljava/lang/String;
public final formatValue()Ljava/lang/String;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
public static fromLegacyFormat(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/TextColor;
public static fromRgb(I)Lnet/minecraft/network/chat/TextColor;
public static parseColor(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$parseColor$2(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$parseColor$1(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$parseColor$0(Ljava/lang/String;)Ljava/lang/String;
static <clinit>()V
```
