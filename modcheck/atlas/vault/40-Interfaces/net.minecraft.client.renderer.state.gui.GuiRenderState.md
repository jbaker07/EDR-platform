---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.gui.GuiRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.gui.GuiRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `reset` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (38, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.state.gui.GuiRenderState {
    private static final int DEBUG_RECTANGLE_COLOR;
    private final java.util.List<net.minecraft.client.renderer.state.gui.GuiRenderState$Node> strata;
    private int firstStratumAfterBlur;
    private net.minecraft.client.renderer.state.gui.GuiRenderState$Node current;
    private final java.util.Set<java.lang.Object> itemModelIdentities;
    private net.minecraft.client.gui.navigation.ScreenRectangle lastElementBounds;
    public net.minecraft.client.renderer.state.gui.PanoramaRenderState panoramaRenderState;
    public org.joml.Vector4f clearColorOverride;
    public boolean isHudHidden;
    private net.minecraft.client.gui.navigation.ScreenRectangle windowRectangleForDebug;
    public net.minecraft.client.renderer.state.gui.GuiRenderState();
    public void nextStratum();
    public void blurBeforeThisStratum();
    public void up();
    public void addItem(net.minecraft.client.renderer.state.gui.GuiItemRenderState);
    public void addText(net.minecraft.client.renderer.state.gui.GuiTextRenderState);
    public void addPicturesInPictureState(net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState);
    public void addGuiElement(net.minecraft.client.renderer.state.gui.GuiElementRenderState);
    private void addDebugRectangleIfEnabled(net.minecraft.client.gui.navigation.ScreenRectangle);
    private boolean findAppropriateNode(net.minecraft.client.renderer.state.gui.ScreenArea);
    private void navigateToAboveHighestElementWithIntersectingBounds(net.minecraft.client.gui.navigation.ScreenRectangle);
    private boolean hasIntersection(net.minecraft.client.gui.navigation.ScreenRectangle, java.util.List<? extends net.minecraft.client.renderer.state.gui.ScreenArea>);
    public void addBlitToCurrentLayer(net.minecraft.client.renderer.state.gui.BlitRenderState);
    public void addGlyphToCurrentLayer(net.minecraft.client.renderer.state.gui.GuiElementRenderState);
    public java.util.Set<java.lang.Object> getItemModelIdentities();
    public void forEachElement(java.util.function.Consumer<net.minecraft.client.renderer.state.gui.GuiElementRenderState>, net.minecraft.client.renderer.state.gui.GuiRenderState$TraverseRange);
    public void forEachItem(java.util.function.Consumer<net.minecraft.client.renderer.state.gui.GuiItemRenderState>);
    public void forEachText(java.util.function.Consumer<net.minecraft.client.renderer.state.gui.GuiTextRenderState>);
    public void forEachPictureInPicture(java.util.function.Consumer<net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState>);
    public void sortElements(java.util.Comparator<net.minecraft.client.renderer.state.gui.GuiElementRenderState>);
    private void traverse(java.util.function.Consumer<net.minecraft.client.renderer.state.gui.GuiRenderState$Node>, net.minecraft.client.renderer.state.gui.GuiRenderState$TraverseRange);
    private void traverse(net.minecraft.client.renderer.state.gui.GuiRenderState$Node, java.util.function.Consumer<net.minecraft.client.renderer.state.gui.GuiRenderState$Node>);
    public void reset();
    private static void lambda$sortElements$0(java.util.Comparator, net.minecraft.client.renderer.state.gui.GuiRenderState$Node);
    private void lambda$forEachPictureInPicture$0(java.util.function.Consumer, net.minecraft.client.renderer.state.gui.GuiRenderState$Node);
    private void lambda$forEachText$0(java.util.function.Consumer, net.minecraft.client.renderer.state.gui.GuiRenderState$Node);
    private void lambda$forEachItem$0(java.util.function.Consumer, net.minecraft.client.renderer.state.gui.GuiRenderState$Node);
    private static void lambda$forEachElement$0(java.util.function.Consumer, net.minecraft.client.renderer.state.gui.GuiRenderState$Node);
}
```
