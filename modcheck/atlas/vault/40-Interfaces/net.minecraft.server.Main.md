---
type: "interface"
fqcn: "net.minecraft.server.Main"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.Main

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `main([Ljava/lang/String;)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `main` | `@Inject at NEW net/minecraft/server/dedicated/DedicatedServerSettings` | server | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `main` | `@Inject at INVOKE_ASSIGN Lnet/minecraft/server/packs/repository/ServerPacksSourc` | server | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| injects_into | `main` | `@Inject at INVOKE Lorg/slf4j/Logger;error(Lorg/slf4j/Marker;Ljava/lang/String;Lj` | server | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| injects_into | `main` | `@Inject at INVOKE Lnet/minecraft/util/Util;startTimerHackThread()V` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.Main {
    private static final org.slf4j.Logger LOGGER;
    public net.minecraft.server.Main();
    public static void main(java.lang.String[]);
    private static net.minecraft.server.WorldLoader$DataLoadOutput<net.minecraft.world.level.storage.LevelDataAndDimensions$WorldDataAndGenSettings> createNewWorldData(net.minecraft.server.dedicated.DedicatedServerSettings, net.minecraft.server.WorldLoader$DataLoadContext, net.minecraft.core.Registry<net.minecraft.world.level.dimension.LevelStem>, boolean, boolean);
    private static void writePidFile(java.nio.file.Path);
    private static net.minecraft.server.WorldLoader$InitConfig loadOrCreateConfig(net.minecraft.server.dedicated.DedicatedServerProperties, com.mojang.serialization.Dynamic<?>, boolean, net.minecraft.server.packs.repository.PackRepository);
    private static void forceUpgrade(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.datafixers.DataFixer, boolean, java.util.function.BooleanSupplier, net.minecraft.core.RegistryAccess, boolean);
    private static net.minecraft.server.dedicated.DedicatedServer lambda$main$3(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.packs.repository.PackRepository, net.minecraft.server.WorldStem, net.minecraft.server.dedicated.DedicatedServerSettings, net.minecraft.server.Services, net.minecraft.server.jsonrpc.ManagementServer, net.minecraft.server.notifications.NotificationManager, joptsimple.OptionSet, joptsimple.OptionSpec, joptsimple.OptionSpec, joptsimple.OptionSpec, joptsimple.OptionSpec, joptsimple.OptionSpec, java.lang.Thread);
    private static boolean lambda$main$2();
    private static java.util.concurrent.CompletableFuture lambda$main$0(net.minecraft.server.WorldLoader$InitConfig, com.mojang.serialization.Dynamic, net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.dedicated.DedicatedServerSettings, joptsimple.OptionSet, joptsimple.OptionSpec, joptsimple.OptionSpec, java.util.concurrent.Executor);
    private static net.minecraft.server.WorldLoader$DataLoadOutput lambda$main$1(com.mojang.serialization.Dynamic, net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.dedicated.DedicatedServerSettings, joptsimple.OptionSet, joptsimple.OptionSpec, joptsimple.OptionSpec, net.minecraft.server.WorldLoader$DataLoadContext);
    static {};
}
```
