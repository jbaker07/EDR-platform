---
type: "interface"
fqcn: "net.fabricmc.fabric.api.datagen.v1.provider.FabricLanguageProvider$TranslationBuilder"
module: "fabric-data-generation-api-v1"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.datagen.v1.provider.FabricLanguageProvider$TranslationBuilder

Module: [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- kind: interface

```java
public abstract boolean has(java.lang.String)
public abstract java.lang.String overwrite(java.lang.String, java.lang.String)
public void add(java.lang.String, java.lang.String)
public void add(net.minecraft.world.item.Item, java.lang.String)
public void add(net.minecraft.world.level.block.Block, java.lang.String)
public void add(net.minecraft.resources.ResourceKey, java.lang.String)
public void addCreativeModeTab(net.minecraft.resources.ResourceKey, java.lang.String)
public void add(net.minecraft.world.entity.EntityType, java.lang.String)
public void addEnchantment(net.minecraft.resources.ResourceKey, java.lang.String)
public void add(net.minecraft.core.Holder, java.lang.String)
public void addAttribute(net.minecraft.core.Holder, java.lang.String)
public void add(net.minecraft.stats.StatType, java.lang.String)
public void add(net.minecraft.world.effect.MobEffect, java.lang.String)
public void add(net.minecraft.resources.Identifier, java.lang.String)
public void add(net.minecraft.tags.TagKey, java.lang.String)
public void add(net.minecraft.sounds.SoundEvent, java.lang.String)
public void add(java.nio.file.Path)
public void overwriteWith(java.nio.file.Path)
```
