---
type: "interface"
fqcn: "net.fabricmc.fabric.api.biome.v1.BiomeModifications"
module: "fabric-biome-api-v1"
sha256: "ce9698da6dd365c6ee59ad5f5e85602cefaaddaebf6d80fad76925e84cda5176"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.biome.v1.BiomeModifications

Module: [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] -- kind: class

```java
public static void addFeature(java.util.function.Predicate<net.fabricmc.fabric.api.biome.v1.BiomeSelectionContext>, net.minecraft.world.level.levelgen.GenerationStep$Decoration, net.minecraft.resources.ResourceKey<net.minecraft.world.level.levelgen.placement.PlacedFeature>)
public static void addCarver(java.util.function.Predicate<net.fabricmc.fabric.api.biome.v1.BiomeSelectionContext>, net.minecraft.resources.ResourceKey<net.minecraft.world.level.levelgen.carver.WorldCarver>)
public static void addSpawn(java.util.function.Predicate<net.fabricmc.fabric.api.biome.v1.BiomeSelectionContext>, net.minecraft.world.entity.MobCategory, net.minecraft.world.entity.EntityType<?>, int, int, int)
public static net.fabricmc.fabric.api.biome.v1.BiomeModification create(net.minecraft.resources.Identifier)
```
