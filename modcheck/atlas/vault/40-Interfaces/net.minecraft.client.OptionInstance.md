---
type: "interface"
fqcn: "net.minecraft.client.OptionInstance"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.OptionInstance

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `get` | `()Ljava/lang/Object;` | exact | invokevirtual@5 in `ClientGameTestContextImpl$1.process` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `get` | `()Ljava/lang/Object;` | exact | invokevirtual@9 in `FabricBlockStateModel.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `get` | `()Ljava/lang/Object;` | exact | invokevirtual@9 in `SingleVariantMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `set` | `(Ljava/lang/Object;)V` | exact | invokevirtual@10 in `ClientGameTestContextImpl$2.process` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `set` | `(Ljava/lang/Object;)V` | exact | invokevirtual@14 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `set` | `(Ljava/lang/Object;)V` | exact | invokevirtual@25 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `set` | `(Ljava/lang/Object;)V` | exact | invokevirtual@36 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `set` | `(Ljava/lang/Object;)V` | exact | invokevirtual@52 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `set` | `(Ljava/lang/Object;)V` | exact | invokevirtual@66 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (12 fields, 24 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final BOOLEAN_VALUES : Lnet/minecraft/client/OptionInstance$Enum;
public static final BOOLEAN_TO_STRING : Lnet/minecraft/client/OptionInstance$CaptionBasedToString;
public static final NO_ACTION : Lnet/minecraft/client/OptionInstance$ValueUpdateListener;
private final tooltip : Lnet/minecraft/client/OptionInstance$TooltipSupplier;
private final toString : Ljava/util/function/Function;
private final values : Lnet/minecraft/client/OptionInstance$ValueSet;
private final codec : Lcom/mojang/serialization/Codec;
private final initialValue : Ljava/lang/Object;
private final onValueUpdate : Lnet/minecraft/client/OptionInstance$ValueUpdateListener;
private final caption : Lnet/minecraft/network/chat/Component;
private value : Ljava/lang/Object;
public static createBoolean(Ljava/lang/String;ZLnet/minecraft/client/OptionInstance$ValueUpdateListener;)Lnet/minecraft/client/OptionInstance;
public static createBoolean(Ljava/lang/String;Z)Lnet/minecraft/client/OptionInstance;
public static createBoolean(Ljava/lang/String;Lnet/minecraft/client/OptionInstance$TooltipSupplier;Z)Lnet/minecraft/client/OptionInstance;
public static createBoolean(Ljava/lang/String;Lnet/minecraft/client/OptionInstance$TooltipSupplier;ZLnet/minecraft/client/OptionInstance$ValueUpdateListener;)Lnet/minecraft/client/OptionInstance;
public static createBoolean(Ljava/lang/String;Lnet/minecraft/client/OptionInstance$TooltipSupplier;Lnet/minecraft/client/OptionInstance$CaptionBasedToString;ZLnet/minecraft/client/OptionInstance$ValueUpdateListener;)Lnet/minecraft/client/OptionInstance;
public <init>(Ljava/lang/String;Lnet/minecraft/client/OptionInstance$TooltipSupplier;Lnet/minecraft/client/OptionInstance$CaptionBasedToString;Lnet/minecraft/client/OptionInstance$ValueSet;Ljava/lang/Object;Lnet/minecraft/client/OptionInstance$ValueUpdateListener;)V
public <init>(Ljava/lang/String;Lnet/minecraft/client/OptionInstance$TooltipSupplier;Lnet/minecraft/client/OptionInstance$CaptionBasedToString;Lnet/minecraft/client/OptionInstance$ValueSet;Lcom/mojang/serialization/Codec;Ljava/lang/Object;Lnet/minecraft/client/OptionInstance$ValueUpdateListener;)V
public static noTooltip()Lnet/minecraft/client/OptionInstance$TooltipSupplier;
public static cachedConstantTooltip(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/client/OptionInstance$TooltipSupplier;
public createButton(Lnet/minecraft/client/Options;)Lnet/minecraft/client/gui/components/AbstractWidget;
public createButton(Lnet/minecraft/client/Options;III)Lnet/minecraft/client/gui/components/AbstractWidget;
public createButton(Lnet/minecraft/client/Options;IIILnet/minecraft/client/OptionInstance$ValueUpdateListener;)Lnet/minecraft/client/gui/components/AbstractWidget;
public get()Ljava/lang/Object;
public codec()Lcom/mojang/serialization/Codec;
public toString()Ljava/lang/String;
public set(Ljava/lang/Object;)V
public values()Lnet/minecraft/client/OptionInstance$ValueSet;
private synthetic lambda$set$0(Ljava/lang/Object;)Ljava/lang/Object;
private static synthetic lambda$cachedConstantTooltip$0(Lnet/minecraft/network/chat/Component;Ljava/lang/Object;)Lnet/minecraft/client/gui/components/Tooltip;
private static synthetic lambda$noTooltip$0(Ljava/lang/Object;)Lnet/minecraft/client/gui/components/Tooltip;
private synthetic lambda$new$0(Lnet/minecraft/client/OptionInstance$CaptionBasedToString;Ljava/lang/Object;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$static$1(Ljava/lang/Object;)V
private static synthetic lambda$static$0(Lnet/minecraft/network/chat/Component;Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Component;
static <clinit>()V
```
