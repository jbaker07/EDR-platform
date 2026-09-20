---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.creativetab.v1.FabricCreativeModeInventoryScreen"
module: "fabric-creative-tab-api-v1"
sha256: "415e659be69014edac017bc384a96c4f2da5ce8fa2093e7d488593e05f03a0b6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.creativetab.v1.FabricCreativeModeInventoryScreen

Module: [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] -- kind: interface

```java
public boolean switchToPage(int)
public boolean switchToNextPage()
public boolean switchToPreviousPage()
public int getCurrentPage()
public int getPageCount()
public java.util.List getTabsOnPage(int)
public int getPage(net.minecraft.world.item.CreativeModeTab)
public boolean hasAdditionalPages()
public net.minecraft.world.item.CreativeModeTab getSelectedTab()
public boolean setSelectedTab(net.minecraft.world.item.CreativeModeTab)
```
