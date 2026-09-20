---
type: "interface"
fqcn: "net.minecraft.locale.Language"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.locale.Language

System: [[20-Systems/net.minecraft.locale|net.minecraft.locale]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getInstance` | `()Lnet/minecraft/locale/Language;` | exact | invokestatic@0 in `TranslationConventionLogWarnings.lambda$setupUntranslatedItemTagWarning | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `getInstance` | `()Lnet/minecraft/locale/Language;` | exact | invokestatic@60 in `RuleListEntryTypeVisitorMixin.displayProperEnumName` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `has` | `(Ljava/lang/String;)Z` | exact | invokevirtual@27 in `TranslationConventionLogWarnings.lambda$setupUntranslatedItemTagWarni | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `has` | `(Ljava/lang/String;)Z` | exact | invokevirtual@65 in `RuleListEntryTypeVisitorMixin.displayProperEnumName` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `loadFromJson` | `(Ljava/io/InputStream;Ljava/util/function/BiConsumer;)V` | exact | @Shadow declaration | server | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| reads | `LOGGER` | `Lorg/slf4j/Logger;` | exact | @Shadow declaration | server | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| wraps | `loadDefault` | `()Lnet/minecraft/locale/Language;` | name_only | @Redirect at ['INVOKE'] | server | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| wraps | `parseTranslations` | `(Ljava/util/function/BiConsumer;Ljava/lang/String;)V` | exact | @Redirect at ['INVOKE'] | server | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (6 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final GSON : Lcom/google/gson/Gson;
private static final UNSUPPORTED_FORMAT_PATTERN : Ljava/util/regex/Pattern;
public static final DEFAULT : Ljava/lang/String;
public static final DEFAULT_INSTANCE : Lnet/minecraft/locale/Language;
private static instance : Lnet/minecraft/locale/Language;
public <init>()V
private static loadDefault()Lnet/minecraft/locale/Language;
private static parseTranslations(Ljava/util/function/BiConsumer;Ljava/lang/String;)V
public static loadFromJson(Ljava/io/InputStream;Ljava/util/function/BiConsumer;)V
public static getInstance()Lnet/minecraft/locale/Language;
public static inject(Lnet/minecraft/locale/Language;)V
public getOrDefault(Ljava/lang/String;)Ljava/lang/String;
public abstract getOrDefault(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
public abstract has(Ljava/lang/String;)Z
public abstract isDefaultRightToLeft()Z
public abstract getVisualOrder(Lnet/minecraft/network/chat/FormattedText;)Lnet/minecraft/util/FormattedCharSequence;
public getVisualOrder(Ljava/util/List;)Ljava/util/List;
static <clinit>()V
```
