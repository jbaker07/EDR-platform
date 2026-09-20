---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.worldselection.WorldCreationContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.worldselection.WorldCreationContext

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `worldgenLoadContext` | `()Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | invokevirtual@4 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (7 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final options : Lnet/minecraft/world/level/levelgen/WorldOptions;
private final datapackDimensions : Lnet/minecraft/core/Registry;
private final selectedDimensions : Lnet/minecraft/world/level/levelgen/WorldDimensions;
private final worldgenRegistries : Lnet/minecraft/core/LayeredRegistryAccess;
private final dataPackResources : Lnet/minecraft/server/ReloadableServerResources;
private final dataConfiguration : Lnet/minecraft/world/level/WorldDataConfiguration;
private final initialWorldCreationOptions : Lnet/minecraft/client/gui/screens/worldselection/InitialWorldCreationOptions;
public <init>(Lnet/minecraft/world/level/levelgen/WorldGenSettings;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/server/ReloadableServerResources;Lnet/minecraft/world/level/WorldDataConfiguration;)V
public <init>(Lnet/minecraft/world/level/levelgen/WorldOptions;Lnet/minecraft/world/level/levelgen/WorldDimensions;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/server/ReloadableServerResources;Lnet/minecraft/world/level/WorldDataConfiguration;Lnet/minecraft/client/gui/screens/worldselection/InitialWorldCreationOptions;)V
public <init>(Lnet/minecraft/world/level/levelgen/WorldOptions;Lnet/minecraft/core/Registry;Lnet/minecraft/world/level/levelgen/WorldDimensions;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/server/ReloadableServerResources;Lnet/minecraft/world/level/WorldDataConfiguration;Lnet/minecraft/client/gui/screens/worldselection/InitialWorldCreationOptions;)V
public withSettings(Lnet/minecraft/world/level/levelgen/WorldOptions;Lnet/minecraft/world/level/levelgen/WorldDimensions;)Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;
public withOptions(Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext$OptionsModifier;)Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;
public withDimensions(Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext$DimensionsUpdater;)Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;
public worldgenLoadContext()Lnet/minecraft/core/RegistryAccess$Frozen;
public validate()V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public options()Lnet/minecraft/world/level/levelgen/WorldOptions;
public datapackDimensions()Lnet/minecraft/core/Registry;
public selectedDimensions()Lnet/minecraft/world/level/levelgen/WorldDimensions;
public worldgenRegistries()Lnet/minecraft/core/LayeredRegistryAccess;
public dataPackResources()Lnet/minecraft/server/ReloadableServerResources;
public dataConfiguration()Lnet/minecraft/world/level/WorldDataConfiguration;
public initialWorldCreationOptions()Lnet/minecraft/client/gui/screens/worldselection/InitialWorldCreationOptions;
```
