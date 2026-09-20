---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.advancements.AdvancementTabType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.advancements.AdvancementTabType

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `extractIcon` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIILnet/minecraft/worl` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (10 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final ABOVE : Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType;
public static final BELOW : Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType;
public static final LEFT : Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType;
public static final RIGHT : Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType;
private final selectedSprites : Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType$Sprites;
private final unselectedSprites : Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType$Sprites;
private final width : I
private final height : I
private final max : I
private static final synthetic $VALUES : [Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType;
public static values()[Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType;
private <init>(Ljava/lang/String;ILnet/minecraft/client/gui/screens/advancements/AdvancementTabType$Sprites;Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType$Sprites;III)V
public getWidth()I
public getHeight()I
public getMax()I
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIZI)V
public extractIcon(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIILnet/minecraft/world/item/ItemStack;)V
public getX(I)I
public getY(I)I
public isMouseOver(IIIDD)Z
private static synthetic $values()[Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType;
static <clinit>()V
```
