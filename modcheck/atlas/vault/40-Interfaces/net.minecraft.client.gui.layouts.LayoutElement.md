---
type: "interface"
fqcn: "net.minecraft.client.gui.layouts.LayoutElement"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.layouts.LayoutElement

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `visitWidgets` | `(Ljava/util/function/Consumer;)V` | exact | invokeinterface@115 in `ClientGameTestContextImpl.tryClickScreenButtonImpl` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (0 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract setX(I)V
public abstract setY(I)V
public abstract getX()I
public abstract getY()I
public abstract getWidth()I
public abstract getHeight()I
public getRectangle()Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public setPosition(II)V
public abstract visitWidgets(Ljava/util/function/Consumer;)V
```
