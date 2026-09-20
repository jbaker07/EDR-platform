---
type: "interface"
fqcn: "net.minecraft.client.gui.components.MultiLineTextWidget"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.MultiLineTextWidget

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/components/AbstractStringWidget`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getMessage` | `()Lnet/minecraft/network/chat/Component;` | inherited_exact | invokevirtual@63 in `TransferableSelectionListPackEntryMixin.onExtractContent` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (4 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private maxWidth : Ljava/util/OptionalInt;
private maxRows : Ljava/util/OptionalInt;
private final cache : Lnet/minecraft/util/SingleKeyCache;
private centered : Z
public <init>(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;)V
public <init>(IILnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;)V
public setMaxWidth(I)Lnet/minecraft/client/gui/components/MultiLineTextWidget;
public setMaxRows(I)Lnet/minecraft/client/gui/components/MultiLineTextWidget;
public setCentered(Z)Lnet/minecraft/client/gui/components/MultiLineTextWidget;
public getWidth()I
public getHeight()I
public visitLines(Lnet/minecraft/client/gui/ActiveTextCollector;)V
protected getTextX()I
protected getTextY()I
private getFreshCacheKey()Lnet/minecraft/client/gui/components/MultiLineTextWidget$CacheKey;
private static synthetic lambda$new$0(Lnet/minecraft/client/gui/Font;Lnet/minecraft/client/gui/components/MultiLineTextWidget$CacheKey;)Lnet/minecraft/client/gui/components/MultiLineLabel;
```
