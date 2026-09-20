---
type: "interface"
fqcn: "net.minecraft.client.gui.components.MultiLineLabel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.MultiLineLabel

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getLineCount()I` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.gui.components.MultiLineLabel {
    public static final net.minecraft.client.gui.components.MultiLineLabel EMPTY;
    public static net.minecraft.client.gui.components.MultiLineLabel create(net.minecraft.client.gui.Font, net.minecraft.network.chat.Component...);
    public static net.minecraft.client.gui.components.MultiLineLabel create(net.minecraft.client.gui.Font, int, net.minecraft.network.chat.Component...);
    public static net.minecraft.client.gui.components.MultiLineLabel create(net.minecraft.client.gui.Font, net.minecraft.network.chat.Component, int);
    public static net.minecraft.client.gui.components.MultiLineLabel create(net.minecraft.client.gui.Font, int, int, net.minecraft.network.chat.Component...);
    public abstract int visitLines(net.minecraft.client.gui.TextAlignment, int, int, int, net.minecraft.client.gui.ActiveTextCollector);
    public abstract int getLineCount();
    public abstract int getWidth();
    static {};
}
```
