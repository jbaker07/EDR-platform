---
type: "interface"
fqcn: "net.minecraft.client.gui.layouts.Layout"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.layouts.Layout

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `arrangeElements()V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getWidth()I` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `removeChildren()V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.gui.layouts.Layout extends net.minecraft.client.gui.layouts.LayoutElement {
    public abstract void visitChildren(java.util.function.Consumer<net.minecraft.client.gui.layouts.LayoutElement>);
    public default void visitWidgets(java.util.function.Consumer<net.minecraft.client.gui.components.AbstractWidget>);
    public default void arrangeElements();
    public abstract void removeChildren();
    private static void lambda$arrangeElements$0(net.minecraft.client.gui.layouts.LayoutElement);
    private static void lambda$visitWidgets$0(java.util.function.Consumer, net.minecraft.client.gui.layouts.LayoutElement);
}
```
