---
type: "interface"
fqcn: "net.minecraft.world.item.CreativeModeTab"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.CreativeModeTab

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `column()I` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getDisplayName()Lnet/minecraft/network/chat/Component;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `isAlignedRight()Z` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `isAlignedRight()Z` | `` | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `row()Lnet/minecraft/world/item/CreativeModeTab$Row;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `shouldDisplay()Z` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `shouldDisplay()Z` | `` | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| injects_into | `buildContents` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |

## Declared members (33, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.item.CreativeModeTab {
    private static final net.minecraft.resources.Identifier DEFAULT_BACKGROUND;
    private final net.minecraft.network.chat.Component displayName;
    private net.minecraft.resources.Identifier backgroundTexture;
    private boolean canScroll;
    private boolean showTitle;
    private boolean alignedRight;
    private final net.minecraft.world.item.CreativeModeTab$Row row;
    private final int column;
    private final net.minecraft.world.item.CreativeModeTab$Type type;
    private net.minecraft.world.item.ItemStack iconItemStack;
    private java.util.Collection<net.minecraft.world.item.ItemStack> displayItems;
    private java.util.Set<net.minecraft.world.item.ItemStack> displayItemsSearchTab;
    private final java.util.function.Supplier<net.minecraft.world.item.ItemStack> iconGenerator;
    private final net.minecraft.world.item.CreativeModeTab$DisplayItemsGenerator displayItemsGenerator;
    private net.minecraft.world.item.CreativeModeTab(net.minecraft.world.item.CreativeModeTab$Row, int, net.minecraft.world.item.CreativeModeTab$Type, net.minecraft.network.chat.Component, java.util.function.Supplier<net.minecraft.world.item.ItemStack>, net.minecraft.world.item.CreativeModeTab$DisplayItemsGenerator);
    public static net.minecraft.resources.Identifier createTextureLocation(java.lang.String);
    public static net.minecraft.world.item.CreativeModeTab$Builder builder(net.minecraft.world.item.CreativeModeTab$Row, int);
    public net.minecraft.network.chat.Component getDisplayName();
    public net.minecraft.world.item.ItemStack getIconItem();
    public net.minecraft.resources.Identifier getBackgroundTexture();
    public boolean showTitle();
    public boolean canScroll();
    public int column();
    public net.minecraft.world.item.CreativeModeTab$Row row();
    public boolean hasAnyItems();
    public boolean shouldDisplay();
    public boolean isAlignedRight();
    public net.minecraft.world.item.CreativeModeTab$Type getType();
    public void buildContents(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters);
    public java.util.Collection<net.minecraft.world.item.ItemStack> getDisplayItems();
    public java.util.Collection<net.minecraft.world.item.ItemStack> getSearchTabDisplayItems();
    public boolean contains(net.minecraft.world.item.ItemStack);
    static {};
}
```
