---
type: "interface"
fqcn: "net.minecraft.client.gui.components.MultiLineTextWidget"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.MultiLineTextWidget

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getMessage()Lnet/minecraft/network/chat/Component;` | `` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.components.MultiLineTextWidget extends net.minecraft.client.gui.components.AbstractStringWidget {
    private java.util.OptionalInt maxWidth;
    private java.util.OptionalInt maxRows;
    private final net.minecraft.util.SingleKeyCache<net.minecraft.client.gui.components.MultiLineTextWidget$CacheKey, net.minecraft.client.gui.components.MultiLineLabel> cache;
    private boolean centered;
    public net.minecraft.client.gui.components.MultiLineTextWidget(net.minecraft.network.chat.Component, net.minecraft.client.gui.Font);
    public net.minecraft.client.gui.components.MultiLineTextWidget(int, int, net.minecraft.network.chat.Component, net.minecraft.client.gui.Font);
    public net.minecraft.client.gui.components.MultiLineTextWidget setMaxWidth(int);
    public net.minecraft.client.gui.components.MultiLineTextWidget setMaxRows(int);
    public net.minecraft.client.gui.components.MultiLineTextWidget setCentered(boolean);
    public int getWidth();
    public int getHeight();
    public void visitLines(net.minecraft.client.gui.ActiveTextCollector);
    protected int getTextX();
    protected int getTextY();
    private net.minecraft.client.gui.components.MultiLineTextWidget$CacheKey getFreshCacheKey();
    private static net.minecraft.client.gui.components.MultiLineLabel lambda$new$0(net.minecraft.client.gui.Font, net.minecraft.client.gui.components.MultiLineTextWidget$CacheKey);
}
```
