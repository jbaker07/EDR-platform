---
type: "interface"
fqcn: "net.minecraft.client.gui.components.FocusableTextWidget$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.FocusableTextWidget$Builder

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `alwaysShowBorder` | `(Z)Lnet/minecraft/client/gui/components/FocusableTextWidget$Builder;` | exact | invokevirtual@64 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `alwaysShowBorder` | `(Z)Lnet/minecraft/client/gui/components/FocusableTextWidget$Builder;` | exact | invokevirtual@139 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `backgroundFill` | `(Lnet/minecraft/client/gui/components/FocusableTextWidget$BackgroundFi` | exact | invokevirtual@70 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `backgroundFill` | `(Lnet/minecraft/client/gui/components/FocusableTextWidget$BackgroundFi` | exact | invokevirtual@145 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/client/gui/components/FocusableTextWidget;` | exact | invokevirtual@73 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/client/gui/components/FocusableTextWidget;` | exact | invokevirtual@148 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `maxWidth` | `(I)Lnet/minecraft/client/gui/components/FocusableTextWidget$Builder;` | exact | invokevirtual@60 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `maxWidth` | `(I)Lnet/minecraft/client/gui/components/FocusableTextWidget$Builder;` | exact | invokevirtual@135 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (6 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final message : Lnet/minecraft/network/chat/Component;
private final font : Lnet/minecraft/client/gui/Font;
private final padding : I
private maxWidth : I
private alwaysShowBorder : Z
private backgroundFill : Lnet/minecraft/client/gui/components/FocusableTextWidget$BackgroundFill;
private <init>(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;)V
private <init>(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;I)V
public maxWidth(I)Lnet/minecraft/client/gui/components/FocusableTextWidget$Builder;
public textWidth(I)Lnet/minecraft/client/gui/components/FocusableTextWidget$Builder;
public alwaysShowBorder(Z)Lnet/minecraft/client/gui/components/FocusableTextWidget$Builder;
public backgroundFill(Lnet/minecraft/client/gui/components/FocusableTextWidget$BackgroundFill;)Lnet/minecraft/client/gui/components/FocusableTextWidget$Builder;
public build()Lnet/minecraft/client/gui/components/FocusableTextWidget;
```
