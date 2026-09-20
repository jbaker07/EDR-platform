---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `create(Lnet/minecraft/world/inventory/tooltip/TooltipComponent;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `create(Lnet/minecraft/world/inventory/tooltip/TooltipComponent;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent {
    public static net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent create(net.minecraft.util.FormattedCharSequence);
    public static net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent create(net.minecraft.world.inventory.tooltip.TooltipComponent);
    public abstract int getHeight(net.minecraft.client.gui.Font);
    public abstract int getWidth(net.minecraft.client.gui.Font);
    public default boolean showTooltipWithItemInHand();
    public default void extractText(net.minecraft.client.gui.GuiGraphicsExtractor, net.minecraft.client.gui.Font, int, int);
    public default void extractImage(net.minecraft.client.gui.Font, int, int, int, int, net.minecraft.client.gui.GuiGraphicsExtractor);
}
```
