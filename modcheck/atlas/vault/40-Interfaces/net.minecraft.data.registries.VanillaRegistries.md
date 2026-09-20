---
type: "interface"
fqcn: "net.minecraft.data.registries.VanillaRegistries"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.registries.VanillaRegistries

System: [[20-Systems/net.minecraft.data.registries|net.minecraft.data.registries]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `createWorldLookup()Lnet/minecraft/core/HolderLookup$Provider;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `validateThatAllBiomeFeaturesHaveBiomeFilter(Lnet/minecraft/core/HolderLookup$Provider;)V` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.registries.VanillaRegistries {
    private static final org.slf4j.Logger LOGGER;
    private static final net.minecraft.core.RegistrySetBuilder WORLD_BUILDER;
    private static final net.minecraft.core.RegistrySetBuilder RELOADABLE_BUILDER;
    public net.minecraft.data.registries.VanillaRegistries();
    public static void validateThatAllBiomeFeaturesHaveBiomeFilter(net.minecraft.core.HolderLookup$Provider);
    private static boolean validatePlacedFeature(net.minecraft.world.level.levelgen.placement.PlacedFeature);
    public static void validateLootData(net.minecraft.core.HolderLookup$Provider);
    public static net.minecraft.core.HolderLookup$Provider createWorldLookup();
    public static net.minecraft.core.HolderLookup$Provider createReloadableLookup(net.minecraft.core.HolderLookup$Provider);
    private static void lambda$validateLootData$1(java.lang.String, net.minecraft.util.ProblemReporter$Problem);
    private static void lambda$validateLootData$0(net.minecraft.world.level.storage.loot.ValidationContextSource, net.minecraft.core.HolderLookup$Provider, net.minecraft.world.level.storage.loot.LootDataType);
    private static void lambda$validateThatAllBiomeFeaturesHaveBiomeFilter$0(net.minecraft.core.HolderLookup$RegistryLookup, net.minecraft.core.Holder$Reference);
    private static void lambda$validateThatAllBiomeFeaturesHaveBiomeFilter$1(net.minecraft.core.HolderLookup$RegistryLookup, net.minecraft.resources.Identifier, net.minecraft.core.Holder$Reference, net.minecraft.core.Holder);
    private static void lambda$validateThatAllBiomeFeaturesHaveBiomeFilter$3(net.minecraft.core.Holder$Reference, net.minecraft.world.level.levelgen.placement.PlacedFeature);
    private static void lambda$validateThatAllBiomeFeaturesHaveBiomeFilter$2(net.minecraft.core.HolderLookup$RegistryLookup, net.minecraft.resources.Identifier, net.minecraft.resources.ResourceKey);
    static {};
}
```
