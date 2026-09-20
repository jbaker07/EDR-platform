---
type: "interface"
fqcn: "net.minecraft.world.item.CreativeModeTab$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.CreativeModeTab$Builder

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/item/CreativeModeTab$Row;I)V` | `` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `build()Lnet/minecraft/world/item/CreativeModeTab;` | `` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `title(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/world` | `` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |

## Declared members (24, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.item.CreativeModeTab$Builder {
    private static final net.minecraft.world.item.CreativeModeTab$DisplayItemsGenerator EMPTY_GENERATOR;
    private final net.minecraft.world.item.CreativeModeTab$Row row;
    private final int column;
    private net.minecraft.network.chat.Component displayName;
    private java.util.function.Supplier<net.minecraft.world.item.ItemStack> iconGenerator;
    private net.minecraft.world.item.CreativeModeTab$DisplayItemsGenerator displayItemsGenerator;
    private boolean canScroll;
    private boolean showTitle;
    private boolean alignedRight;
    private net.minecraft.world.item.CreativeModeTab$Type type;
    private net.minecraft.resources.Identifier backgroundTexture;
    public net.minecraft.world.item.CreativeModeTab$Builder(net.minecraft.world.item.CreativeModeTab$Row, int);
    public net.minecraft.world.item.CreativeModeTab$Builder title(net.minecraft.network.chat.Component);
    public net.minecraft.world.item.CreativeModeTab$Builder icon(java.util.function.Supplier<net.minecraft.world.item.ItemStack>);
    public net.minecraft.world.item.CreativeModeTab$Builder displayItems(net.minecraft.world.item.CreativeModeTab$DisplayItemsGenerator);
    public net.minecraft.world.item.CreativeModeTab$Builder alignedRight();
    public net.minecraft.world.item.CreativeModeTab$Builder hideTitle();
    public net.minecraft.world.item.CreativeModeTab$Builder noScrollBar();
    protected net.minecraft.world.item.CreativeModeTab$Builder type(net.minecraft.world.item.CreativeModeTab$Type);
    public net.minecraft.world.item.CreativeModeTab$Builder backgroundTexture(net.minecraft.resources.Identifier);
    public net.minecraft.world.item.CreativeModeTab build();
    private static net.minecraft.world.item.ItemStack lambda$new$0();
    private static void lambda$static$0(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, net.minecraft.world.item.CreativeModeTab$Output);
    static {};
}
```
