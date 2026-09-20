---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.InputConstants$Type"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.InputConstants$Type

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getOrCreate` | `(I)Lcom/mojang/blaze3d/platform/InputConstants$Key;` | exact | invokevirtual@7 in `TestInputImpl.isKeyDown` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getOrCreate` | `(I)Lcom/mojang/blaze3d/platform/InputConstants$Key;` | exact | invokevirtual@10 in `TestInputImpl.holdKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getOrCreate` | `(I)Lcom/mojang/blaze3d/platform/InputConstants$Key;` | exact | invokevirtual@10 in `TestInputImpl.holdMouse` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getOrCreate` | `(I)Lcom/mojang/blaze3d/platform/InputConstants$Key;` | exact | invokevirtual@10 in `TestInputImpl.releaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getOrCreate` | `(I)Lcom/mojang/blaze3d/platform/InputConstants$Key;` | exact | invokevirtual@10 in `TestInputImpl.releaseMouse` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getOrCreate` | `(I)Lcom/mojang/blaze3d/platform/InputConstants$Key;` | exact | invokevirtual@10 in `TestInputImpl.pressKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getOrCreate` | `(I)Lcom/mojang/blaze3d/platform/InputConstants$Key;` | exact | invokevirtual@10 in `TestInputImpl.pressMouse` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getOrCreate` | `(I)Lcom/mojang/blaze3d/platform/InputConstants$Key;` | exact | invokevirtual@24 in `TestInputImpl.holdKeyFor` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getOrCreate` | `(I)Lcom/mojang/blaze3d/platform/InputConstants$Key;` | exact | invokevirtual@24 in `TestInputImpl.holdMouseFor` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@15 in `TestInputImpl$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@30 in `TestInputImpl$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@7 in `TestInputImpl.pressOrReleaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `values` | `()[Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | invokestatic@0 in `TestInputImpl$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `KEYBOARD` | `Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | getstatic@12 in `TestInputImpl$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `KEYBOARD` | `Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | getstatic@3 in `TestInputImpl.isKeyDown` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `KEYBOARD` | `Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | getstatic@6 in `TestInputImpl.holdKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `KEYBOARD` | `Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | getstatic@6 in `TestInputImpl.releaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `KEYBOARD` | `Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | getstatic@6 in `TestInputImpl.pressKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `KEYBOARD` | `Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | getstatic@20 in `TestInputImpl.holdKeyFor` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `MOUSE` | `Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | getstatic@27 in `TestInputImpl$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `MOUSE` | `Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | getstatic@6 in `TestInputImpl.holdMouse` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `MOUSE` | `Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | getstatic@6 in `TestInputImpl.releaseMouse` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `MOUSE` | `Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | getstatic@6 in `TestInputImpl.pressMouse` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `MOUSE` | `Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | getstatic@20 in `TestInputImpl.holdMouseFor` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (7 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final KEYBOARD : Lcom/mojang/blaze3d/platform/InputConstants$Type;
public static final MOUSE : Lcom/mojang/blaze3d/platform/InputConstants$Type;
private static final KEY_KEYBOARD_UNKNOWN : Ljava/lang/String;
private final map : Lit/unimi/dsi/fastutil/ints/Int2ObjectMap;
private final defaultPrefix : Ljava/lang/String;
private final displayTextSupplier : Ljava/util/function/BiFunction;
private static final synthetic $VALUES : [Lcom/mojang/blaze3d/platform/InputConstants$Type;
public static values()[Lcom/mojang/blaze3d/platform/InputConstants$Type;
public static valueOf(Ljava/lang/String;)Lcom/mojang/blaze3d/platform/InputConstants$Type;
private static addKey(Lcom/mojang/blaze3d/platform/InputConstants$Type;Ljava/lang/String;I)V
private <init>(Ljava/lang/String;ILjava/lang/String;Ljava/util/function/BiFunction;)V
public getOrCreate(I)Lcom/mojang/blaze3d/platform/InputConstants$Key;
private static synthetic $values()[Lcom/mojang/blaze3d/platform/InputConstants$Type;
private synthetic lambda$getOrCreate$0(I)Lcom/mojang/blaze3d/platform/InputConstants$Key;
private static synthetic lambda$static$1(Ljava/lang/Integer;Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$static$0(Ljava/lang/Integer;Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
static <clinit>()V
```
