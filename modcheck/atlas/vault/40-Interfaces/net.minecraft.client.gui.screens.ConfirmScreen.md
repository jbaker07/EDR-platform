---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.ConfirmScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.ConfirmScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lit/unimi/dsi/fastutil/booleans/BooleanConsumer;Lnet/minecr` | `` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.ConfirmScreen extends net.minecraft.client.gui.screens.Screen {
    protected final net.minecraft.network.chat.Component message;
    protected final net.minecraft.client.gui.layouts.LinearLayout layout;
    protected net.minecraft.network.chat.Component yesButtonComponent;
    protected net.minecraft.network.chat.Component noButtonComponent;
    protected net.minecraft.client.gui.components.Button yesButton;
    protected net.minecraft.client.gui.components.Button noButton;
    private int delayTicker;
    protected final it.unimi.dsi.fastutil.booleans.BooleanConsumer callback;
    public net.minecraft.client.gui.screens.ConfirmScreen(it.unimi.dsi.fastutil.booleans.BooleanConsumer, net.minecraft.network.chat.Component, net.minecraft.network.chat.Component);
    public net.minecraft.client.gui.screens.ConfirmScreen(it.unimi.dsi.fastutil.booleans.BooleanConsumer, net.minecraft.network.chat.Component, net.minecraft.network.chat.Component, net.minecraft.network.chat.Component, net.minecraft.network.chat.Component);
    public net.minecraft.network.chat.Component getNarrationMessage();
    protected void init();
    protected void repositionElements();
    protected void addAdditionalText();
    protected net.minecraft.client.gui.layouts.LayoutElement addMessage();
    protected void addButtons(net.minecraft.client.gui.layouts.LinearLayout);
    public void setDelay(int);
    public void tick();
    public boolean shouldCloseOnEsc();
    public boolean keyPressed(net.minecraft.client.input.KeyEvent);
    private void lambda$addButtons$1(net.minecraft.client.gui.components.Button);
    private void lambda$addButtons$0(net.minecraft.client.gui.components.Button);
}
```
