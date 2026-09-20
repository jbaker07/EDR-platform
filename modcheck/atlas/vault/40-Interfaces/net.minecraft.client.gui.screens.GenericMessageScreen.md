---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.GenericMessageScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.GenericMessageScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/screens/Screen`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/network/chat/Component;)V` | exact | invokespecial@39 in `TestSingleplayerContextImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (1 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private textWidget : Lnet/minecraft/client/gui/components/FocusableTextWidget;
public <init>(Lnet/minecraft/network/chat/Component;)V
protected init()V
protected repositionElements()V
public shouldCloseOnEsc()Z
protected shouldNarrateNavigation()Z
public extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
```
