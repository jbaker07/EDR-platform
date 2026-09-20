---
type: "interface"
fqcn: "net.minecraft.client.gui.components.FocusableTextWidget$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.FocusableTextWidget$Builder

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `alwaysShowBorder(Z)Lnet/minecraft/client/gui/components/FocusableTextWidget$` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `backgroundFill(Lnet/minecraft/client/gui/components/FocusableTextWidget$Ba` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `build()Lnet/minecraft/client/gui/components/FocusableTextWidget;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `maxWidth(I)Lnet/minecraft/client/gui/components/FocusableTextWidget$` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.components.FocusableTextWidget$Builder {
    private final net.minecraft.network.chat.Component message;
    private final net.minecraft.client.gui.Font font;
    private final int padding;
    private int maxWidth;
    private boolean alwaysShowBorder;
    private net.minecraft.client.gui.components.FocusableTextWidget$BackgroundFill backgroundFill;
    private net.minecraft.client.gui.components.FocusableTextWidget$Builder(net.minecraft.network.chat.Component, net.minecraft.client.gui.Font);
    private net.minecraft.client.gui.components.FocusableTextWidget$Builder(net.minecraft.network.chat.Component, net.minecraft.client.gui.Font, int);
    public net.minecraft.client.gui.components.FocusableTextWidget$Builder maxWidth(int);
    public net.minecraft.client.gui.components.FocusableTextWidget$Builder textWidth(int);
    public net.minecraft.client.gui.components.FocusableTextWidget$Builder alwaysShowBorder(boolean);
    public net.minecraft.client.gui.components.FocusableTextWidget$Builder backgroundFill(net.minecraft.client.gui.components.FocusableTextWidget$BackgroundFill);
    public net.minecraft.client.gui.components.FocusableTextWidget build();
}
```
