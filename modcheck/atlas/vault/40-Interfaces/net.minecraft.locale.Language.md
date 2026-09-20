---
type: "interface"
fqcn: "net.minecraft.locale.Language"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.locale.Language

System: [[20-Systems/net.minecraft.locale|net.minecraft.locale]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getInstance()Lnet/minecraft/locale/Language;` | `` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/locale/Language;` | `` | client | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `has(Ljava/lang/String;)Z` | `` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `has(Ljava/lang/String;)Z` | `` | client | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| wraps | `loadDefault` | `@Redirect at INVOKE Ljava/util/Map;copyOf(Ljava/util/Map;)Ljava/util/Map;` | server | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| wraps | `parseTranslations(Ljava/util/function/BiConsumer;Ljava/lang/String;)V` | `@Redirect at INVOKE Ljava/lang/Class;getResourceAsStream(Ljava/lang/String;)Ljav` | server | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.locale.Language {
    private static final org.slf4j.Logger LOGGER;
    private static final com.google.gson.Gson GSON;
    private static final java.util.regex.Pattern UNSUPPORTED_FORMAT_PATTERN;
    public static final java.lang.String DEFAULT;
    public static final net.minecraft.locale.Language DEFAULT_INSTANCE;
    private static volatile net.minecraft.locale.Language instance;
    public net.minecraft.locale.Language();
    private static net.minecraft.locale.Language loadDefault();
    private static void parseTranslations(java.util.function.BiConsumer<java.lang.String, java.lang.String>, java.lang.String);
    public static void loadFromJson(java.io.InputStream, java.util.function.BiConsumer<java.lang.String, java.lang.String>);
    public static net.minecraft.locale.Language getInstance();
    public static void inject(net.minecraft.locale.Language);
    public java.lang.String getOrDefault(java.lang.String);
    public abstract java.lang.String getOrDefault(java.lang.String, java.lang.String);
    public abstract boolean has(java.lang.String);
    public abstract boolean isDefaultRightToLeft();
    public abstract net.minecraft.util.FormattedCharSequence getVisualOrder(net.minecraft.network.chat.FormattedText);
    public java.util.List<net.minecraft.util.FormattedCharSequence> getVisualOrder(java.util.List<net.minecraft.network.chat.FormattedText>);
    static {};
}
```
