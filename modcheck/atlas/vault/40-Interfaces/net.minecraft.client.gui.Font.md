---
type: "interface"
fqcn: "net.minecraft.client.gui.Font"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.Font

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `split` | `(Lnet/minecraft/network/chat/FormattedText;I)Ljava/util/List;` | exact | invokevirtual@29 in `EnumRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `split` | `(Lnet/minecraft/network/chat/FormattedText;I)Ljava/util/List;` | exact | invokevirtual@86 in `TransferableSelectionListPackEntryMixin.onExtractContent` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `width` | `(Lnet/minecraft/util/FormattedCharSequence;)I` | exact | invokevirtual@37 in `TransferableSelectionListPackEntryMixin.onExtractContent` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (8 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final EFFECT_DEPTH : F
private static final OVER_EFFECT_DEPTH : F
private static final UNDER_EFFECT_DEPTH : F
public static final SHADOW_DEPTH : F
public final lineHeight : I
private final random : Lnet/minecraft/util/RandomSource;
private final provider : Lnet/minecraft/client/gui/Font$Provider;
private final splitter : Lnet/minecraft/client/StringSplitter;
public <init>(Lnet/minecraft/client/gui/Font$Provider;)V
private getGlyphSource(Lnet/minecraft/network/chat/FontDescription;)Lnet/minecraft/client/gui/GlyphSource;
public bidirectionalShaping(Ljava/lang/String;)Ljava/lang/String;
public prepare8xTextOutline(Lnet/minecraft/util/FormattedCharSequence;FFI)Lnet/minecraft/client/gui/Font$PreparedText;
public prepareBackground(FFFFI)Lnet/minecraft/client/gui/font/TextRenderable;
private getGlyph(ILnet/minecraft/network/chat/Style;)Lnet/minecraft/client/gui/font/glyphs/BakedGlyph;
public prepareText(Ljava/lang/String;FFIZI)Lnet/minecraft/client/gui/Font$PreparedText;
public prepareText(Lnet/minecraft/util/FormattedCharSequence;FFIZZI)Lnet/minecraft/client/gui/Font$PreparedText;
public width(Ljava/lang/String;)I
public width(Lnet/minecraft/network/chat/FormattedText;)I
public width(Lnet/minecraft/util/FormattedCharSequence;)I
public plainSubstrByWidth(Ljava/lang/String;IZ)Ljava/lang/String;
public plainSubstrByWidth(Ljava/lang/String;I)Ljava/lang/String;
public substrByWidth(Lnet/minecraft/network/chat/FormattedText;I)Lnet/minecraft/network/chat/FormattedText;
public wordWrapHeight(Lnet/minecraft/network/chat/FormattedText;I)I
public split(Lnet/minecraft/network/chat/FormattedText;I)Ljava/util/List;
public splitIgnoringLanguage(Lnet/minecraft/network/chat/FormattedText;I)Ljava/util/List;
public isBidirectional()Z
public getSplitter()Lnet/minecraft/client/StringSplitter;
private synthetic lambda$prepare8xTextOutline$0(Lnet/minecraft/client/gui/Font$PreparedTextBuilder;[FIFIIILnet/minecraft/network/chat/Style;I)Z
private synthetic lambda$new$0(ILnet/minecraft/network/chat/Style;)F
```
