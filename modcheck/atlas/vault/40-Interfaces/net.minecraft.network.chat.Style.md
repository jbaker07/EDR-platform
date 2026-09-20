---
type: "interface"
fqcn: "net.minecraft.network.chat.Style"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.Style

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `withColor` | `(Lnet/minecraft/network/chat/TextColor;)Lnet/minecraft/network/chat/St` | exact | invokevirtual@441 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withItalic` | `(Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Style;` | exact | invokevirtual@435 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/network/chat/Style;` | exact | getstatic@428 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (13 fields, 37 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/network/chat/Style;
public static final NO_SHADOW : I
private final color : Lnet/minecraft/network/chat/TextColor;
private final shadowColor : Ljava/lang/Integer;
private final bold : Ljava/lang/Boolean;
private final italic : Ljava/lang/Boolean;
private final underlined : Ljava/lang/Boolean;
private final strikethrough : Ljava/lang/Boolean;
private final obfuscated : Ljava/lang/Boolean;
private final clickEvent : Lnet/minecraft/network/chat/ClickEvent;
private final hoverEvent : Lnet/minecraft/network/chat/HoverEvent;
private final insertion : Ljava/lang/String;
private final font : Lnet/minecraft/network/chat/FontDescription;
private static create(Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;)Lnet/minecraft/network/chat/Style;
private <init>(Lnet/minecraft/network/chat/TextColor;Ljava/lang/Integer;Ljava/lang/Boolean;Ljava/lang/Boolean;Ljava/lang/Boolean;Ljava/lang/Boolean;Ljava/lang/Boolean;Lnet/minecraft/network/chat/ClickEvent;Lnet/minecraft/network/chat/HoverEvent;Ljava/lang/String;Lnet/minecraft/network/chat/FontDescription;)V
public getColor()Lnet/minecraft/network/chat/TextColor;
public getShadowColor()Ljava/lang/Integer;
public isBold()Z
public isItalic()Z
public isStrikethrough()Z
public isUnderlined()Z
public isObfuscated()Z
public isEmpty()Z
public getClickEvent()Lnet/minecraft/network/chat/ClickEvent;
public getHoverEvent()Lnet/minecraft/network/chat/HoverEvent;
public getInsertion()Ljava/lang/String;
public getFont()Lnet/minecraft/network/chat/FontDescription;
private static checkEmptyAfterChange(Lnet/minecraft/network/chat/Style;Ljava/lang/Object;Ljava/lang/Object;)Lnet/minecraft/network/chat/Style;
public withColor(Lnet/minecraft/network/chat/TextColor;)Lnet/minecraft/network/chat/Style;
public withColor(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/Style;
public withColor(I)Lnet/minecraft/network/chat/Style;
public withShadowColor(I)Lnet/minecraft/network/chat/Style;
public withoutShadow()Lnet/minecraft/network/chat/Style;
public withBold(Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Style;
public withItalic(Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Style;
public withUnderlined(Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Style;
public withStrikethrough(Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Style;
public withObfuscated(Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Style;
public withClickEvent(Lnet/minecraft/network/chat/ClickEvent;)Lnet/minecraft/network/chat/Style;
public withHoverEvent(Lnet/minecraft/network/chat/HoverEvent;)Lnet/minecraft/network/chat/Style;
public withInsertion(Ljava/lang/String;)Lnet/minecraft/network/chat/Style;
public withFont(Lnet/minecraft/network/chat/FontDescription;)Lnet/minecraft/network/chat/Style;
public applyFormat(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/Style;
public applyLegacyFormat(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/Style;
public applyFormats([Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/Style;
public applyTo(Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
public toString()Ljava/lang/String;
public equals(Ljava/lang/Object;)Z
public hashCode()I
static <clinit>()V
```
