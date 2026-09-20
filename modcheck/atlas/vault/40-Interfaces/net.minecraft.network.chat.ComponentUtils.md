---
type: "interface"
fqcn: "net.minecraft.network.chat.ComponentUtils"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.ComponentUtils

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `fromMessage(Lcom/mojang/brigadier/Message;)Lnet/minecraft/network/chat/` | `` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.chat.ComponentUtils {
    public static final java.lang.String DEFAULT_SEPARATOR_TEXT;
    public static final net.minecraft.network.chat.Component DEFAULT_SEPARATOR;
    public static final net.minecraft.network.chat.Component DEFAULT_NO_STYLE_SEPARATOR;
    public net.minecraft.network.chat.ComponentUtils();
    public static net.minecraft.network.chat.MutableComponent mergeStyles(net.minecraft.network.chat.MutableComponent, net.minecraft.network.chat.Style);
    public static net.minecraft.network.chat.Component mergeStyles(net.minecraft.network.chat.Component, net.minecraft.network.chat.Style);
    public static java.util.Optional<net.minecraft.network.chat.MutableComponent> resolve(net.minecraft.network.chat.ResolutionContext, java.util.Optional<net.minecraft.network.chat.Component>, int) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    public static net.minecraft.network.chat.MutableComponent resolve(net.minecraft.network.chat.ResolutionContext, net.minecraft.network.chat.Component) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    public static net.minecraft.network.chat.MutableComponent resolve(net.minecraft.network.chat.ResolutionContext, net.minecraft.network.chat.Component, int) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static net.minecraft.network.chat.Style resolveStyle(net.minecraft.network.chat.ResolutionContext, net.minecraft.network.chat.Style, int) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    public static net.minecraft.network.chat.Component formatList(java.util.Collection<java.lang.String>);
    public static <T extends java.lang.Comparable<T>> net.minecraft.network.chat.Component formatAndSortList(java.util.Collection<T>, java.util.function.Function<T, net.minecraft.network.chat.Component>);
    public static <T> net.minecraft.network.chat.Component formatList(java.util.Collection<? extends T>, java.util.function.Function<T, net.minecraft.network.chat.Component>);
    public static <T> net.minecraft.network.chat.MutableComponent formatList(java.util.Collection<? extends T>, java.util.Optional<? extends net.minecraft.network.chat.Component>, java.util.function.Function<T, net.minecraft.network.chat.Component>);
    public static net.minecraft.network.chat.Component formatList(java.util.Collection<? extends net.minecraft.network.chat.Component>, net.minecraft.network.chat.Component);
    public static <T> net.minecraft.network.chat.MutableComponent formatList(java.util.Collection<? extends T>, net.minecraft.network.chat.Component, java.util.function.Function<T, net.minecraft.network.chat.Component>);
    public static net.minecraft.network.chat.MutableComponent wrapInSquareBrackets(net.minecraft.network.chat.Component);
    public static net.minecraft.network.chat.Component fromMessage(com.mojang.brigadier.Message);
    public static boolean isTranslationResolvable(net.minecraft.network.chat.Component);
    public static net.minecraft.network.chat.MutableComponent copyOnClickText(java.lang.String);
    private static net.minecraft.network.chat.Style lambda$copyOnClickText$0(java.lang.String, net.minecraft.network.chat.Style);
    private static net.minecraft.network.chat.Component lambda$formatList$0(java.lang.String);
    static {};
}
```
