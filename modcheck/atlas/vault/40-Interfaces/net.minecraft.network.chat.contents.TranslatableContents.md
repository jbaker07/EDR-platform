---
type: "interface"
fqcn: "net.minecraft.network.chat.contents.TranslatableContents"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.contents.TranslatableContents

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/network/chat/ComponentContents`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getKey` | `()Ljava/lang/String;` | exact | invokevirtual@39 in `ClientGameTestImpl.isExperimentalWarningScreen` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getKey` | `()Ljava/lang/String;` | exact | invokevirtual@42 in `FabricLanguageProvider$TranslationBuilder.addCreativeModeTab` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (12 fields, 30 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NO_ARGS : [Ljava/lang/Object;
private static final PRIMITIVE_ARG_CODEC : Lcom/mojang/serialization/Codec;
private static final ARG_CODEC : Lcom/mojang/serialization/Codec;
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
private static final TEXT_PERCENT : Lnet/minecraft/network/chat/FormattedText;
private static final TEXT_NULL : Lnet/minecraft/network/chat/FormattedText;
private final key : Ljava/lang/String;
private final fallback : Ljava/lang/String;
private final args : [Ljava/lang/Object;
private decomposedWith : Lnet/minecraft/locale/Language;
private decomposedParts : Ljava/util/List;
private static final FORMAT_PATTERN : Ljava/util/regex/Pattern;
private static filterAllowedArguments(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public static isAllowedPrimitiveArgument(Ljava/lang/Object;)Z
private static adjustArgs([Ljava/lang/Object;)Ljava/util/Optional;
private static adjustArgs(Ljava/util/Optional;)[Ljava/lang/Object;
private static create(Ljava/lang/String;Ljava/util/Optional;Ljava/util/Optional;)Lnet/minecraft/network/chat/contents/TranslatableContents;
public <init>(Ljava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V
public codec()Lcom/mojang/serialization/MapCodec;
private decompose()V
private decomposeTemplate(Ljava/lang/String;Ljava/util/function/Consumer;)V
public final getArgument(I)Lnet/minecraft/network/chat/FormattedText;
public visit(Lnet/minecraft/network/chat/FormattedText$StyledContentConsumer;Lnet/minecraft/network/chat/Style;)Ljava/util/Optional;
public visit(Lnet/minecraft/network/chat/FormattedText$ContentConsumer;)Ljava/util/Optional;
public resolve(Lnet/minecraft/network/chat/ResolutionContext;I)Lnet/minecraft/network/chat/MutableComponent;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
public getKey()Ljava/lang/String;
public getFallback()Ljava/lang/String;
public getArgs()[Ljava/lang/Object;
private static synthetic lambda$adjustArgs$0(Ljava/util/List;)[Ljava/lang/Object;
private static synthetic lambda$static$4(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$7(Lnet/minecraft/network/chat/contents/TranslatableContents;)Ljava/util/Optional;
private static synthetic lambda$static$6(Lnet/minecraft/network/chat/contents/TranslatableContents;)Ljava/util/Optional;
private static synthetic lambda$static$5(Lnet/minecraft/network/chat/contents/TranslatableContents;)Ljava/lang/String;
private static synthetic lambda$static$3(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;
private static synthetic lambda$static$0(Lcom/mojang/datafixers/util/Either;)Ljava/lang/Object;
private static synthetic lambda$static$2(Lnet/minecraft/network/chat/Component;)Ljava/lang/Object;
private static synthetic lambda$static$1(Ljava/lang/Object;)Ljava/lang/Object;
private static synthetic lambda$filterAllowedArguments$0()Ljava/lang/String;
static <clinit>()V
```
