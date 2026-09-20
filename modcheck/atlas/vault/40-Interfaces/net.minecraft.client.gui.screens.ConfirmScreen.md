---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.ConfirmScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.ConfirmScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/screens/Screen`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lit/unimi/dsi/fastutil/booleans/BooleanConsumer;Lnet/minecraft/networ` | exact | invokespecial@72 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (8 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
protected final message : Lnet/minecraft/network/chat/Component;
protected final layout : Lnet/minecraft/client/gui/layouts/LinearLayout;
protected yesButtonComponent : Lnet/minecraft/network/chat/Component;
protected noButtonComponent : Lnet/minecraft/network/chat/Component;
protected yesButton : Lnet/minecraft/client/gui/components/Button;
protected noButton : Lnet/minecraft/client/gui/components/Button;
private delayTicker : I
protected final callback : Lit/unimi/dsi/fastutil/booleans/BooleanConsumer;
public <init>(Lit/unimi/dsi/fastutil/booleans/BooleanConsumer;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;)V
public <init>(Lit/unimi/dsi/fastutil/booleans/BooleanConsumer;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;)V
public getNarrationMessage()Lnet/minecraft/network/chat/Component;
protected init()V
protected repositionElements()V
protected addAdditionalText()V
protected addMessage()Lnet/minecraft/client/gui/layouts/LayoutElement;
protected addButtons(Lnet/minecraft/client/gui/layouts/LinearLayout;)V
public setDelay(I)V
public tick()V
public shouldCloseOnEsc()Z
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
private synthetic lambda$addButtons$1(Lnet/minecraft/client/gui/components/Button;)V
private synthetic lambda$addButtons$0(Lnet/minecraft/client/gui/components/Button;)V
```
