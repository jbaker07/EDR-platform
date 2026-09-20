---
type: "interface"
fqcn: "net.minecraft.client.gui.layouts.HeaderAndFooterLayout"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.layouts.HeaderAndFooterLayout

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/gui/layouts/Layout`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/client/gui/screens/Screen;)V` | exact | invokespecial@6 in `DetailsScreen.init` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `addTitleHeader` | `(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;` | exact | invokevirtual@12 in `DetailsScreen.addTitle` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `addToContents` | `(Lnet/minecraft/client/gui/layouts/LayoutElement;)Lnet/minecraft/clien` | exact | invokevirtual@194 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `addToFooter` | `(Lnet/minecraft/client/gui/layouts/LayoutElement;)Lnet/minecraft/clien` | exact | invokevirtual@25 in `DetailsScreen.addFooter` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `arrangeElements` | `()V` | exact | invokevirtual@4 in `DetailsScreen.repositionElements` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getContentHeight` | `()I` | exact | invokevirtual@178 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getContentHeight` | `()I` | exact | invokevirtual@38 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getContentHeight` | `()I` | exact | invokevirtual@97 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getHeaderHeight` | `()I` | exact | invokevirtual@21 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getHeaderHeight` | `()I` | exact | invokevirtual@60 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getHeaderHeight` | `()I` | exact | invokevirtual@90 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `visitWidgets` | `(Ljava/util/function/Consumer;)V` | inherited_exact | invokevirtual@36 in `DetailsScreen.init` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (9 fields, 24 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MAGIC_PADDING : I
public static final DEFAULT_HEADER_AND_FOOTER_HEIGHT : I
private static final CONTENT_MARGIN_TOP : I
private final headerFrame : Lnet/minecraft/client/gui/layouts/FrameLayout;
private final footerFrame : Lnet/minecraft/client/gui/layouts/FrameLayout;
private final contentsFrame : Lnet/minecraft/client/gui/layouts/FrameLayout;
private final screen : Lnet/minecraft/client/gui/screens/Screen;
private headerHeight : I
private footerHeight : I
public <init>(Lnet/minecraft/client/gui/screens/Screen;)V
public <init>(Lnet/minecraft/client/gui/screens/Screen;I)V
public <init>(Lnet/minecraft/client/gui/screens/Screen;II)V
public setX(I)V
public setY(I)V
public getX()I
public getY()I
public getWidth()I
public getHeight()I
public getFooterHeight()I
public setFooterHeight(I)V
public setHeaderHeight(I)V
public getHeaderHeight()I
public getContentHeight()I
public visitChildren(Ljava/util/function/Consumer;)V
public removeChildren()V
public arrangeElements()V
public addToHeader(Lnet/minecraft/client/gui/layouts/LayoutElement;)Lnet/minecraft/client/gui/layouts/LayoutElement;
public addToHeader(Lnet/minecraft/client/gui/layouts/LayoutElement;Ljava/util/function/Consumer;)Lnet/minecraft/client/gui/layouts/LayoutElement;
public addTitleHeader(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;)V
public addToFooter(Lnet/minecraft/client/gui/layouts/LayoutElement;)Lnet/minecraft/client/gui/layouts/LayoutElement;
public addToFooter(Lnet/minecraft/client/gui/layouts/LayoutElement;Ljava/util/function/Consumer;)Lnet/minecraft/client/gui/layouts/LayoutElement;
public addToContents(Lnet/minecraft/client/gui/layouts/LayoutElement;)Lnet/minecraft/client/gui/layouts/LayoutElement;
public addToContents(Lnet/minecraft/client/gui/layouts/LayoutElement;Ljava/util/function/Consumer;)Lnet/minecraft/client/gui/layouts/LayoutElement;
```
