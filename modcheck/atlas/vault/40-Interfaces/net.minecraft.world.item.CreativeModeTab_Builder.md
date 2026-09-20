---
type: "interface"
fqcn: "net.minecraft.world.item.CreativeModeTab$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.CreativeModeTab$Builder

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/item/CreativeModeTab$Row;I)V` | exact | invokespecial@3 in `FabricCreativeModeTabBuilderImpl.<init>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/item/CreativeModeTab;` | exact | invokespecial@18 in `FabricCreativeModeTabBuilderImpl.build` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `title` | `(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/world/item/Crea` | exact | invokespecial@7 in `FabricCreativeModeTabBuilderImpl.title` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |

## Declared members (11 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final EMPTY_GENERATOR : Lnet/minecraft/world/item/CreativeModeTab$DisplayItemsGenerator;
private final row : Lnet/minecraft/world/item/CreativeModeTab$Row;
private final column : I
private displayName : Lnet/minecraft/network/chat/Component;
private iconGenerator : Ljava/util/function/Supplier;
private displayItemsGenerator : Lnet/minecraft/world/item/CreativeModeTab$DisplayItemsGenerator;
private canScroll : Z
private showTitle : Z
private alignedRight : Z
private type : Lnet/minecraft/world/item/CreativeModeTab$Type;
private backgroundTexture : Lnet/minecraft/resources/Identifier;
public <init>(Lnet/minecraft/world/item/CreativeModeTab$Row;I)V
public title(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/world/item/CreativeModeTab$Builder;
public icon(Ljava/util/function/Supplier;)Lnet/minecraft/world/item/CreativeModeTab$Builder;
public displayItems(Lnet/minecraft/world/item/CreativeModeTab$DisplayItemsGenerator;)Lnet/minecraft/world/item/CreativeModeTab$Builder;
public alignedRight()Lnet/minecraft/world/item/CreativeModeTab$Builder;
public hideTitle()Lnet/minecraft/world/item/CreativeModeTab$Builder;
public noScrollBar()Lnet/minecraft/world/item/CreativeModeTab$Builder;
protected type(Lnet/minecraft/world/item/CreativeModeTab$Type;)Lnet/minecraft/world/item/CreativeModeTab$Builder;
public backgroundTexture(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/world/item/CreativeModeTab$Builder;
public build()Lnet/minecraft/world/item/CreativeModeTab;
private static synthetic lambda$new$0()Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$static$0(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;Lnet/minecraft/world/item/CreativeModeTab$Output;)V
static <clinit>()V
```
