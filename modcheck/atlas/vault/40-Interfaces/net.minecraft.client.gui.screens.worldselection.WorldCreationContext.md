---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.worldselection.WorldCreationContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.worldselection.WorldCreationContext

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `worldgenLoadContext()Lnet/minecraft/core/RegistryAccess$Frozen;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (25, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.gui.screens.worldselection.WorldCreationContext extends java.lang.Record {
    private final net.minecraft.world.level.levelgen.WorldOptions options;
    private final net.minecraft.core.Registry<net.minecraft.world.level.dimension.LevelStem> datapackDimensions;
    private final net.minecraft.world.level.levelgen.WorldDimensions selectedDimensions;
    private final net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer> worldgenRegistries;
    private final net.minecraft.server.ReloadableServerResources dataPackResources;
    private final net.minecraft.world.level.WorldDataConfiguration dataConfiguration;
    private final net.minecraft.client.gui.screens.worldselection.InitialWorldCreationOptions initialWorldCreationOptions;
    public net.minecraft.client.gui.screens.worldselection.WorldCreationContext(net.minecraft.world.level.levelgen.WorldGenSettings, net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>, net.minecraft.server.ReloadableServerResources, net.minecraft.world.level.WorldDataConfiguration);
    public net.minecraft.client.gui.screens.worldselection.WorldCreationContext(net.minecraft.world.level.levelgen.WorldOptions, net.minecraft.world.level.levelgen.WorldDimensions, net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>, net.minecraft.server.ReloadableServerResources, net.minecraft.world.level.WorldDataConfiguration, net.minecraft.client.gui.screens.worldselection.InitialWorldCreationOptions);
    public net.minecraft.client.gui.screens.worldselection.WorldCreationContext(net.minecraft.world.level.levelgen.WorldOptions, net.minecraft.core.Registry<net.minecraft.world.level.dimension.LevelStem>, net.minecraft.world.level.levelgen.WorldDimensions, net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>, net.minecraft.server.ReloadableServerResources, net.minecraft.world.level.WorldDataConfiguration, net.minecraft.client.gui.screens.worldselection.InitialWorldCreationOptions);
    public net.minecraft.client.gui.screens.worldselection.WorldCreationContext withSettings(net.minecraft.world.level.levelgen.WorldOptions, net.minecraft.world.level.levelgen.WorldDimensions);
    public net.minecraft.client.gui.screens.worldselection.WorldCreationContext withOptions(net.minecraft.client.gui.screens.worldselection.WorldCreationContext$OptionsModifier);
    public net.minecraft.client.gui.screens.worldselection.WorldCreationContext withDimensions(net.minecraft.client.gui.screens.worldselection.WorldCreationContext$DimensionsUpdater);
    public net.minecraft.core.RegistryAccess$Frozen worldgenLoadContext();
    public void validate();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.level.levelgen.WorldOptions options();
    public net.minecraft.core.Registry<net.minecraft.world.level.dimension.LevelStem> datapackDimensions();
    public net.minecraft.world.level.levelgen.WorldDimensions selectedDimensions();
    public net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer> worldgenRegistries();
    public net.minecraft.server.ReloadableServerResources dataPackResources();
    public net.minecraft.world.level.WorldDataConfiguration dataConfiguration();
    public net.minecraft.client.gui.screens.worldselection.InitialWorldCreationOptions initialWorldCreationOptions();
}
```
