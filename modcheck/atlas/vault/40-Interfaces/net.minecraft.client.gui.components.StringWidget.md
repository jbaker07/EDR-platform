---
type: "interface"
fqcn: "net.minecraft.client.gui.components.StringWidget"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.StringWidget

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getMessage()Lnet/minecraft/network/chat/Component;` | `` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getWidth()I` | `` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.components.StringWidget extends net.minecraft.client.gui.components.AbstractStringWidget {
    private static final int TEXT_MARGIN;
    private int maxWidth;
    private int cachedWidth;
    private boolean cachedWidthDirty;
    private net.minecraft.client.gui.components.StringWidget$TextOverflow textOverflow;
    public net.minecraft.client.gui.components.StringWidget(net.minecraft.network.chat.Component, net.minecraft.client.gui.Font);
    public net.minecraft.client.gui.components.StringWidget(int, int, net.minecraft.network.chat.Component, net.minecraft.client.gui.Font);
    public net.minecraft.client.gui.components.StringWidget(int, int, int, int, net.minecraft.network.chat.Component, net.minecraft.client.gui.Font);
    public void setMessage(net.minecraft.network.chat.Component);
    public net.minecraft.client.gui.components.StringWidget setMaxWidth(int);
    public net.minecraft.client.gui.components.StringWidget setMaxWidth(int, net.minecraft.client.gui.components.StringWidget$TextOverflow);
    public int getWidth();
    public void visitLines(net.minecraft.client.gui.ActiveTextCollector);
}
```
