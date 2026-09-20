---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.GenericMessageScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.GenericMessageScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/network/chat/Component;)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.GenericMessageScreen extends net.minecraft.client.gui.screens.Screen {
    private net.minecraft.client.gui.components.FocusableTextWidget textWidget;
    public net.minecraft.client.gui.screens.GenericMessageScreen(net.minecraft.network.chat.Component);
    protected void init();
    protected void repositionElements();
    public boolean shouldCloseOnEsc();
    protected boolean shouldNarrateNavigation();
    public void extractBackground(net.minecraft.client.gui.GuiGraphicsExtractor, int, int, float);
}
```
