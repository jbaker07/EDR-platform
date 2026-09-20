---
type: "interface"
fqcn: "net.fabricmc.fabric.api.biome.v1.BiomeModificationContext$EffectsContext"
module: "fabric-biome-api-v1"
sha256: "ce9698da6dd365c6ee59ad5f5e85602cefaaddaebf6d80fad76925e84cda5176"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.biome.v1.BiomeModificationContext$EffectsContext

Module: [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] -- kind: interface

```java
public abstract void setFogColor(int)
public abstract void setWaterColor(int)
public abstract void setWaterFogColor(int)
public abstract void setSkyColor(int)
public abstract void setFoliageColorOverride(java.util.Optional<java.lang.Integer>)
public default void setFoliageColorOverride(int)
public default void setFoliageColorOverride(java.util.OptionalInt)
public default void clearFoliageColorOverride()
public abstract void setDryFoliageColorOverride(java.util.Optional<java.lang.Integer>)
public default void setDryFoliageColorOverride(int)
public default void setDryFoliageColorOverride(java.util.OptionalInt)
public default void clearDryFoliageColorOverride()
public abstract void setGrassColorOverride(java.util.Optional<java.lang.Integer>)
public default void setGrassColorOverride(int)
public default void setGrassColorOverride(java.util.OptionalInt)
public default void clearGrassColorOverride()
public abstract void setGrassColorModifier(net.minecraft.world.level.biome.BiomeSpecialEffects$GrassColorModifier)
public abstract void setMusicVolume(float)
```
