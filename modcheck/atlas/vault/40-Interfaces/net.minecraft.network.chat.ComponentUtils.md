---
type: "interface"
fqcn: "net.minecraft.network.chat.ComponentUtils"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.ComponentUtils

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `formatList` | `(Ljava/util/Collection;Lnet/minecraft/network/chat/Component;)Lnet/min` | exact | invokestatic@458 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `formatList` | `(Ljava/util/Collection;Lnet/minecraft/network/chat/Component;)Lnet/min` | exact | invokestatic@655 in `RegistryCustomContentState$Missing.asDetails` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `fromMessage` | `(Lcom/mojang/brigadier/Message;)Lnet/minecraft/network/chat/Component;` | exact | invokestatic@4 in `ClientCommandInternals.getErrorMessage` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (3 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DEFAULT_SEPARATOR_TEXT : Ljava/lang/String;
public static final DEFAULT_SEPARATOR : Lnet/minecraft/network/chat/Component;
public static final DEFAULT_NO_STYLE_SEPARATOR : Lnet/minecraft/network/chat/Component;
public <init>()V
public static mergeStyles(Lnet/minecraft/network/chat/MutableComponent;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/MutableComponent;
public static mergeStyles(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Component;
public static resolve(Lnet/minecraft/network/chat/ResolutionContext;Ljava/util/Optional;I)Ljava/util/Optional;
public static resolve(Lnet/minecraft/network/chat/ResolutionContext;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/MutableComponent;
public static resolve(Lnet/minecraft/network/chat/ResolutionContext;Lnet/minecraft/network/chat/Component;I)Lnet/minecraft/network/chat/MutableComponent;
private static resolveStyle(Lnet/minecraft/network/chat/ResolutionContext;Lnet/minecraft/network/chat/Style;I)Lnet/minecraft/network/chat/Style;
public static formatList(Ljava/util/Collection;)Lnet/minecraft/network/chat/Component;
public static formatAndSortList(Ljava/util/Collection;Ljava/util/function/Function;)Lnet/minecraft/network/chat/Component;
public static formatList(Ljava/util/Collection;Ljava/util/function/Function;)Lnet/minecraft/network/chat/Component;
public static formatList(Ljava/util/Collection;Ljava/util/Optional;Ljava/util/function/Function;)Lnet/minecraft/network/chat/MutableComponent;
public static formatList(Ljava/util/Collection;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
public static formatList(Ljava/util/Collection;Lnet/minecraft/network/chat/Component;Ljava/util/function/Function;)Lnet/minecraft/network/chat/MutableComponent;
public static wrapInSquareBrackets(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/MutableComponent;
public static fromMessage(Lcom/mojang/brigadier/Message;)Lnet/minecraft/network/chat/Component;
public static isTranslationResolvable(Lnet/minecraft/network/chat/Component;)Z
public static copyOnClickText(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableComponent;
private static synthetic lambda$copyOnClickText$0(Ljava/lang/String;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
private static synthetic lambda$formatList$0(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
static <clinit>()V
```
