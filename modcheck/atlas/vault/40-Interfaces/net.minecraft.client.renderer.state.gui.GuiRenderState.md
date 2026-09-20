---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.gui.GuiRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.gui.GuiRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `reset` | `()V` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (10 fields, 28 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final DEBUG_RECTANGLE_COLOR : I
private final strata : Ljava/util/List;
private firstStratumAfterBlur : I
private current : Lnet/minecraft/client/renderer/state/gui/GuiRenderState$Node;
private final itemModelIdentities : Ljava/util/Set;
private lastElementBounds : Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public panoramaRenderState : Lnet/minecraft/client/renderer/state/gui/PanoramaRenderState;
public clearColorOverride : Lorg/joml/Vector4f;
public isHudHidden : Z
private windowRectangleForDebug : Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public <init>()V
public nextStratum()V
public blurBeforeThisStratum()V
public up()V
public addItem(Lnet/minecraft/client/renderer/state/gui/GuiItemRenderState;)V
public addText(Lnet/minecraft/client/renderer/state/gui/GuiTextRenderState;)V
public addPicturesInPictureState(Lnet/minecraft/client/renderer/state/gui/pip/PictureInPictureRenderState;)V
public addGuiElement(Lnet/minecraft/client/renderer/state/gui/GuiElementRenderState;)V
private addDebugRectangleIfEnabled(Lnet/minecraft/client/gui/navigation/ScreenRectangle;)V
private findAppropriateNode(Lnet/minecraft/client/renderer/state/gui/ScreenArea;)Z
private navigateToAboveHighestElementWithIntersectingBounds(Lnet/minecraft/client/gui/navigation/ScreenRectangle;)V
private hasIntersection(Lnet/minecraft/client/gui/navigation/ScreenRectangle;Ljava/util/List;)Z
public addBlitToCurrentLayer(Lnet/minecraft/client/renderer/state/gui/BlitRenderState;)V
public addGlyphToCurrentLayer(Lnet/minecraft/client/renderer/state/gui/GuiElementRenderState;)V
public getItemModelIdentities()Ljava/util/Set;
public forEachElement(Ljava/util/function/Consumer;Lnet/minecraft/client/renderer/state/gui/GuiRenderState$TraverseRange;)V
public forEachItem(Ljava/util/function/Consumer;)V
public forEachText(Ljava/util/function/Consumer;)V
public forEachPictureInPicture(Ljava/util/function/Consumer;)V
public sortElements(Ljava/util/Comparator;)V
private traverse(Ljava/util/function/Consumer;Lnet/minecraft/client/renderer/state/gui/GuiRenderState$TraverseRange;)V
private traverse(Lnet/minecraft/client/renderer/state/gui/GuiRenderState$Node;Ljava/util/function/Consumer;)V
public reset()V
private static synthetic lambda$sortElements$0(Ljava/util/Comparator;Lnet/minecraft/client/renderer/state/gui/GuiRenderState$Node;)V
private synthetic lambda$forEachPictureInPicture$0(Ljava/util/function/Consumer;Lnet/minecraft/client/renderer/state/gui/GuiRenderState$Node;)V
private synthetic lambda$forEachText$0(Ljava/util/function/Consumer;Lnet/minecraft/client/renderer/state/gui/GuiRenderState$Node;)V
private synthetic lambda$forEachItem$0(Ljava/util/function/Consumer;Lnet/minecraft/client/renderer/state/gui/GuiRenderState$Node;)V
private static synthetic lambda$forEachElement$0(Ljava/util/function/Consumer;Lnet/minecraft/client/renderer/state/gui/GuiRenderState$Node;)V
```
