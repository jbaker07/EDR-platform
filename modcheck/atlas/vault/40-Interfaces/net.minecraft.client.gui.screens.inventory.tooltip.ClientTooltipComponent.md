---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `create` | `(Lnet/minecraft/world/inventory/tooltip/TooltipComponent;)Lnet/minecra` | exact | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `create` | `(Lnet/minecraft/world/inventory/tooltip/TooltipComponent;)Lnet/minecra` | exact | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (0 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static create(Lnet/minecraft/util/FormattedCharSequence;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;
public static create(Lnet/minecraft/world/inventory/tooltip/TooltipComponent;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;
public abstract getHeight(Lnet/minecraft/client/gui/Font;)I
public abstract getWidth(Lnet/minecraft/client/gui/Font;)I
public showTooltipWithItemInHand()Z
public extractText(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/gui/Font;II)V
public extractImage(Lnet/minecraft/client/gui/Font;IIIILnet/minecraft/client/gui/GuiGraphicsExtractor;)V
```
