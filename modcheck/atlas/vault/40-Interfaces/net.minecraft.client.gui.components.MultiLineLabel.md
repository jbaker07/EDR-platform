---
type: "interface"
fqcn: "net.minecraft.client.gui.components.MultiLineLabel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.MultiLineLabel

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getLineCount` | `()I` | exact | invokeinterface@13 in `DetailedBackupConfirmScreen.init` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (1 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/client/gui/components/MultiLineLabel;
public static create(Lnet/minecraft/client/gui/Font;[Lnet/minecraft/network/chat/Component;)Lnet/minecraft/client/gui/components/MultiLineLabel;
public static create(Lnet/minecraft/client/gui/Font;I[Lnet/minecraft/network/chat/Component;)Lnet/minecraft/client/gui/components/MultiLineLabel;
public static create(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;I)Lnet/minecraft/client/gui/components/MultiLineLabel;
public static create(Lnet/minecraft/client/gui/Font;II[Lnet/minecraft/network/chat/Component;)Lnet/minecraft/client/gui/components/MultiLineLabel;
public abstract visitLines(Lnet/minecraft/client/gui/TextAlignment;IIILnet/minecraft/client/gui/ActiveTextCollector;)I
public abstract getLineCount()I
public abstract getWidth()I
static <clinit>()V
```
