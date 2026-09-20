---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.inventory.HangingSignEditScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.inventory.HangingSignEditScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/screens/inventory/AbstractSignEditScreen`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `<init>` | `(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lnet/minecraf` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (5 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MAGIC_BACKGROUND_SCALE : F
private static final TEXT_SCALE : Lorg/joml/Vector3fc;
private static final TEXTURE_WIDTH : I
private static final TEXTURE_HEIGHT : I
private final texture : Lnet/minecraft/resources/Identifier;
public <init>(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lnet/minecraft/world/level/block/entity/SignTextSlot;Z)V
protected getSignYOffset()F
protected extractSignBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
protected getSignTextScale()Lorg/joml/Vector3fc;
static <clinit>()V
```
