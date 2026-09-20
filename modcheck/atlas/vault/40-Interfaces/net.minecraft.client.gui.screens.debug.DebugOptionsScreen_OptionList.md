---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.debug.DebugOptionsScreen$OptionList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.debug.DebugOptionsScreen$OptionList

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `lambda$static$0` | `@Redirect at INVOKE Lnet/minecraft/resources/Identifier;compareTo(Lnet/minecraft` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.debug.DebugOptionsScreen$OptionList extends net.minecraft.client.gui.components.ContainerObjectSelectionList<net.minecraft.client.gui.screens.debug.DebugOptionsScreen$AbstractOptionEntry> {
    private static final java.util.Comparator<java.util.Map$Entry<net.minecraft.resources.Identifier, net.minecraft.client.gui.components.debug.DebugScreenEntry>> COMPARATOR;
    private static final int ITEM_HEIGHT;
    final net.minecraft.client.gui.screens.debug.DebugOptionsScreen this$0;
    public net.minecraft.client.gui.screens.debug.DebugOptionsScreen$OptionList(net.minecraft.client.gui.screens.debug.DebugOptionsScreen);
    public void extractWidgetRenderState(net.minecraft.client.gui.GuiGraphicsExtractor, int, int, float);
    public int getRowWidth();
    public void refreshEntries();
    public void updateSearch(java.lang.String);
    private void notifyListUpdated();
    private static int lambda$static$0(java.util.Map$Entry, java.util.Map$Entry);
    static {};
}
```
