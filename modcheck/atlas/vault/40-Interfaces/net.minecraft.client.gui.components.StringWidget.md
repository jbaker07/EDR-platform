---
type: "interface"
fqcn: "net.minecraft.client.gui.components.StringWidget"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.StringWidget

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/components/AbstractStringWidget`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getMessage` | `()Lnet/minecraft/network/chat/Component;` | inherited_exact | invokevirtual@29 in `TransferableSelectionListPackEntryMixin.onExtractContent` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getMessage` | `()Lnet/minecraft/network/chat/Component;` | inherited_exact | invokevirtual@47 in `TransferableSelectionListPackEntryMixin.onExtractContent` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getWidth` | `()I` | exact | invokevirtual@15 in `TransferableSelectionListPackEntryMixin.onExtractContent` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (5 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final TEXT_MARGIN : I
private maxWidth : I
private cachedWidth : I
private cachedWidthDirty : Z
private textOverflow : Lnet/minecraft/client/gui/components/StringWidget$TextOverflow;
public <init>(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;)V
public <init>(IILnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;)V
public <init>(IIIILnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;)V
public setMessage(Lnet/minecraft/network/chat/Component;)V
public setMaxWidth(I)Lnet/minecraft/client/gui/components/StringWidget;
public setMaxWidth(ILnet/minecraft/client/gui/components/StringWidget$TextOverflow;)Lnet/minecraft/client/gui/components/StringWidget;
public getWidth()I
public visitLines(Lnet/minecraft/client/gui/ActiveTextCollector;)V
```
