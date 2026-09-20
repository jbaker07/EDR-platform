---
type: "interface"
fqcn: "net.minecraft.client.InputType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.InputType

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `isKeyboard` | `()Z` | exact | invokevirtual@16 in `OptimizedScrollableLayout$Container.setFocused` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (5 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NONE : Lnet/minecraft/client/InputType;
public static final MOUSE : Lnet/minecraft/client/InputType;
public static final KEYBOARD_ARROW : Lnet/minecraft/client/InputType;
public static final KEYBOARD_TAB : Lnet/minecraft/client/InputType;
private static final synthetic $VALUES : [Lnet/minecraft/client/InputType;
public static values()[Lnet/minecraft/client/InputType;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/client/InputType;
private <init>(Ljava/lang/String;I)V
public isMouse()Z
public isKeyboard()Z
private static synthetic $values()[Lnet/minecraft/client/InputType;
static <clinit>()V
```
