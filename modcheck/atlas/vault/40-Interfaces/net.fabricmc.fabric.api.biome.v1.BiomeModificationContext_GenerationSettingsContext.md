---
type: "interface"
fqcn: "net.fabricmc.fabric.api.biome.v1.BiomeModificationContext$GenerationSettingsContext"
module: "fabric-biome-api-v1"
sha256: "ce9698da6dd365c6ee59ad5f5e85602cefaaddaebf6d80fad76925e84cda5176"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.biome.v1.BiomeModificationContext$GenerationSettingsContext

Module: [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] -- kind: interface

```java
public abstract boolean removeFeature(net.minecraft.world.level.levelgen.GenerationStep$Decoration, net.minecraft.resources.ResourceKey<net.minecraft.world.level.levelgen.placement.PlacedFeature>)
public default boolean removeFeature(net.minecraft.resources.ResourceKey<net.minecraft.world.level.levelgen.placement.PlacedFeature>)
public abstract void addFeature(net.minecraft.world.level.levelgen.GenerationStep$Decoration, net.minecraft.resources.ResourceKey<net.minecraft.world.level.levelgen.placement.PlacedFeature>)
public abstract void addCarver(net.minecraft.resources.ResourceKey<net.minecraft.world.level.levelgen.carver.WorldCarver>)
public abstract boolean removeCarver(net.minecraft.resources.ResourceKey<net.minecraft.world.level.levelgen.carver.WorldCarver>)
```
