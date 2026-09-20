---
type: "interface"
fqcn: "net.fabricmc.fabric.api.biome.v1.BiomeSelectionContext"
module: "fabric-biome-api-v1"
sha256: "ce9698da6dd365c6ee59ad5f5e85602cefaaddaebf6d80fad76925e84cda5176"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.biome.v1.BiomeSelectionContext

Module: [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] -- kind: interface

```java
public abstract net.minecraft.resources.ResourceKey getBiomeKey()
public abstract net.minecraft.world.level.biome.Biome getBiome()
public abstract net.minecraft.core.Holder getBiomeHolder()
public boolean hasFeature(net.minecraft.resources.ResourceKey)
public boolean hasPlacedFeature(net.minecraft.resources.ResourceKey)
public abstract java.util.Optional getFeatureKey(net.minecraft.world.level.levelgen.feature.Feature)
public abstract java.util.Optional getPlacedFeatureKey(net.minecraft.world.level.levelgen.placement.PlacedFeature)
public abstract boolean validForStructure(net.minecraft.resources.ResourceKey)
public abstract java.util.Optional getStructureKey(net.minecraft.world.level.levelgen.structure.Structure)
public abstract boolean canGenerateIn(net.minecraft.resources.ResourceKey)
public abstract boolean hasTag(net.minecraft.tags.TagKey)
```
