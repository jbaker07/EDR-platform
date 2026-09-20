---
type: "interface"
fqcn: "net.fabricmc.fabric.api.biome.v1.BiomeModificationContext$MobSpawnSettingsContext"
module: "fabric-biome-api-v1"
sha256: "ce9698da6dd365c6ee59ad5f5e85602cefaaddaebf6d80fad76925e84cda5176"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.biome.v1.BiomeModificationContext$MobSpawnSettingsContext

Module: [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] -- kind: interface

```java
public abstract void setCreatureGenerationProbability(float)
public abstract java.util.List<net.minecraft.util.random.Weighted<net.minecraft.world.level.biome.MobSpawnSettings$SpawnerData>> getMobs(net.minecraft.world.entity.MobCategory)
public abstract void addSpawn(net.minecraft.world.entity.MobCategory, net.minecraft.world.level.biome.MobSpawnSettings$SpawnerData, int)
public abstract boolean removeSpawns(java.util.function.BiPredicate<net.minecraft.world.entity.MobCategory, net.minecraft.world.level.biome.MobSpawnSettings$SpawnerData>)
public default boolean removeSpawnsOfEntityType(net.minecraft.world.entity.EntityType<?>)
public default void clearSpawns(net.minecraft.world.entity.MobCategory)
public default void clearSpawns()
public abstract void addMobCharge(net.minecraft.world.entity.EntityType<?>, double, double)
public abstract void clearMobCharge(net.minecraft.world.entity.EntityType<?>)
```
