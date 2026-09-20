---
type: "interface"
fqcn: "net.fabricmc.fabric.api.creativetab.v1.FabricCreativeModeTabOutput"
module: "fabric-creative-tab-api-v1"
sha256: "415e659be69014edac017bc384a96c4f2da5ce8fa2093e7d488593e05f03a0b6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.creativetab.v1.FabricCreativeModeTabOutput

Module: [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] -- kind: class

```java
public <init>(net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters, java.util.List, java.util.List)
public net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters getContext()
public net.minecraft.world.flag.FeatureFlagSet getEnabledFeatures()
public boolean shouldShowOpRestrictedItems()
public java.util.List getDisplayStacks()
public java.util.List getSearchTabStacks()
public void accept(net.minecraft.world.item.ItemStack, net.minecraft.world.item.CreativeModeTab$TabVisibility)
public void prepend(net.minecraft.world.item.ItemStack)
public void prepend(net.minecraft.world.item.ItemStack, net.minecraft.world.item.CreativeModeTab$TabVisibility)
public void prepend(net.minecraft.world.level.ItemLike)
public void prepend(net.minecraft.world.level.ItemLike, net.minecraft.world.item.CreativeModeTab$TabVisibility)
public void insertAfter(net.minecraft.world.level.ItemLike, net.minecraft.world.item.ItemStack[])
public void insertAfter(net.minecraft.world.item.ItemStack, net.minecraft.world.item.ItemStack[])
public void insertAfter(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike[])
public void insertAfter(net.minecraft.world.item.ItemStack, net.minecraft.world.level.ItemLike[])
public void insertAfter(net.minecraft.world.level.ItemLike, java.util.Collection)
public void insertAfter(net.minecraft.world.item.ItemStack, java.util.Collection)
public void insertAfter(net.minecraft.world.level.ItemLike, java.util.Collection, net.minecraft.world.item.CreativeModeTab$TabVisibility)
public void insertAfter(net.minecraft.world.item.ItemStack, java.util.Collection, net.minecraft.world.item.CreativeModeTab$TabVisibility)
public void insertAfter(java.util.function.Predicate, java.util.Collection, net.minecraft.world.item.CreativeModeTab$TabVisibility)
public void insertBefore(net.minecraft.world.level.ItemLike, net.minecraft.world.item.ItemStack[])
public void insertBefore(net.minecraft.world.item.ItemStack, net.minecraft.world.item.ItemStack[])
public void insertBefore(net.minecraft.world.level.ItemLike, net.minecraft.world.level.ItemLike[])
public void insertBefore(net.minecraft.world.item.ItemStack, net.minecraft.world.level.ItemLike[])
public void insertBefore(net.minecraft.world.level.ItemLike, java.util.Collection)
public void insertBefore(net.minecraft.world.item.ItemStack, java.util.Collection)
public void insertBefore(net.minecraft.world.level.ItemLike, java.util.Collection, net.minecraft.world.item.CreativeModeTab$TabVisibility)
public void insertBefore(net.minecraft.world.item.ItemStack, java.util.Collection, net.minecraft.world.item.CreativeModeTab$TabVisibility)
public void insertBefore(java.util.function.Predicate, java.util.Collection, net.minecraft.world.item.CreativeModeTab$TabVisibility)
```
