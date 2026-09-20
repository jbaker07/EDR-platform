---
type: "interface"
fqcn: "net.minecraft.client.gui.navigation.ScreenRectangle"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.navigation.ScreenRectangle

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(IIII)V` | exact | invokespecial@39 in `OptimizedScrollableLayout$Container.getBorderForArrowNavigation` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `<init>` | `(IIII)V` | exact | invokespecial@16 in `AdvancementTabMixin.captureWindowSize` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `bottom` | `()I` | exact | invokevirtual@46 in `OptimizedScrollableLayout$Container.setFocused` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `bottom` | `()I` | exact | invokevirtual@50 in `OptimizedScrollableLayout$Container.setFocused` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getBorder` | `(Lnet/minecraft/client/gui/navigation/ScreenDirection;)Lnet/minecraft/` | exact | invokevirtual@43 in `OptimizedScrollableLayout$Container.getBorderForArrowNavigation` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `top` | `()I` | exact | invokevirtual@35 in `OptimizedScrollableLayout$Container.setFocused` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `top` | `()I` | exact | invokevirtual@39 in `OptimizedScrollableLayout$Container.setFocused` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `top` | `()I` | exact | invokevirtual@16 in `OptimizedScrollableLayout$Container.setScrollAmount` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (4 fields, 28 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final position : Lnet/minecraft/client/gui/navigation/ScreenPosition;
private final width : I
private final height : I
private static final EMPTY : Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public <init>(IIII)V
public <init>(Lnet/minecraft/client/gui/navigation/ScreenPosition;II)V
public static empty()Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public static of(Lnet/minecraft/client/gui/navigation/ScreenAxis;IIII)Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public step(Lnet/minecraft/client/gui/navigation/ScreenDirection;)Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public getLength(Lnet/minecraft/client/gui/navigation/ScreenAxis;)I
public getBoundInDirection(Lnet/minecraft/client/gui/navigation/ScreenDirection;)I
public getBorder(Lnet/minecraft/client/gui/navigation/ScreenDirection;)Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public overlaps(Lnet/minecraft/client/gui/navigation/ScreenRectangle;)Z
public overlapsInAxis(Lnet/minecraft/client/gui/navigation/ScreenRectangle;Lnet/minecraft/client/gui/navigation/ScreenAxis;)Z
public getCenterInAxis(Lnet/minecraft/client/gui/navigation/ScreenAxis;)I
public intersection(Lnet/minecraft/client/gui/navigation/ScreenRectangle;)Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public intersects(Lnet/minecraft/client/gui/navigation/ScreenRectangle;)Z
public encompasses(Lnet/minecraft/client/gui/navigation/ScreenRectangle;)Z
public top()I
public bottom()I
public left()I
public right()I
public containsPoint(II)Z
public transformAxisAligned(Lorg/joml/Matrix3x2fc;)Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public transformMaxBounds(Lorg/joml/Matrix3x2fc;)Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public position()Lnet/minecraft/client/gui/navigation/ScreenPosition;
public width()I
public height()I
static <clinit>()V
```
