---
type: "interface"
fqcn: "net.minecraft.client.gui.layouts.LinearLayout"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.layouts.LinearLayout

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/gui/layouts/Layout`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `addChild` | `(Lnet/minecraft/client/gui/layouts/LayoutElement;)Lnet/minecraft/clien` | exact | invokevirtual@80 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `addChild` | `(Lnet/minecraft/client/gui/layouts/LayoutElement;)Lnet/minecraft/clien` | exact | invokevirtual@155 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `arrangeElements` | `()V` | exact | invokevirtual@186 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `spacing` | `(I)Lnet/minecraft/client/gui/layouts/LinearLayout;` | exact | invokevirtual@4 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `vertical` | `()Lnet/minecraft/client/gui/layouts/LinearLayout;` | exact | invokestatic@0 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (3 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final wrapped : Lnet/minecraft/client/gui/layouts/GridLayout;
private final orientation : Lnet/minecraft/client/gui/layouts/LinearLayout$Orientation;
private nextChildIndex : I
private <init>(Lnet/minecraft/client/gui/layouts/LinearLayout$Orientation;)V
public <init>(IILnet/minecraft/client/gui/layouts/LinearLayout$Orientation;)V
public spacing(I)Lnet/minecraft/client/gui/layouts/LinearLayout;
public newCellSettings()Lnet/minecraft/client/gui/layouts/LayoutSettings;
public defaultCellSetting()Lnet/minecraft/client/gui/layouts/LayoutSettings;
public addChild(Lnet/minecraft/client/gui/layouts/LayoutElement;Lnet/minecraft/client/gui/layouts/LayoutSettings;)Lnet/minecraft/client/gui/layouts/LayoutElement;
public addChild(Lnet/minecraft/client/gui/layouts/LayoutElement;)Lnet/minecraft/client/gui/layouts/LayoutElement;
public addChild(Lnet/minecraft/client/gui/layouts/LayoutElement;Ljava/util/function/Consumer;)Lnet/minecraft/client/gui/layouts/LayoutElement;
public visitChildren(Ljava/util/function/Consumer;)V
public removeChildren()V
public arrangeElements()V
public getWidth()I
public getHeight()I
public setX(I)V
public setY(I)V
public getX()I
public getY()I
public static vertical()Lnet/minecraft/client/gui/layouts/LinearLayout;
public static horizontal()Lnet/minecraft/client/gui/layouts/LinearLayout;
```
