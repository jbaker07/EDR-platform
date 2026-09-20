---
type: "interface"
fqcn: "net.fabricmc.loader.api.FabricLoader"
module: "fabric-loader"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.loader.api.FabricLoader

fabric-loader 0.19.5 -- kind: interface

```java
public static net.fabricmc.loader.api.FabricLoader getInstance()
public abstract <T> java.util.List<T> getEntrypoints(java.lang.String, java.lang.Class<T>)
public abstract <T> java.util.List<net.fabricmc.loader.api.entrypoint.EntrypointContainer<T>> getEntrypointContainers(java.lang.String, java.lang.Class<T>)
public abstract <T> void invokeEntrypoints(java.lang.String, java.lang.Class<T>, java.util.function.Consumer<? super T>)
public abstract net.fabricmc.loader.api.ObjectShare getObjectShare()
public abstract net.fabricmc.loader.api.MappingResolver getMappingResolver()
public abstract java.util.Optional<net.fabricmc.loader.api.ModContainer> getModContainer(java.lang.String)
public abstract java.util.Collection<net.fabricmc.loader.api.ModContainer> getAllMods()
public abstract boolean isModLoaded(java.lang.String)
public abstract boolean isDevelopmentEnvironment()
public abstract net.fabricmc.api.EnvType getEnvironmentType()
public abstract java.lang.String getRawGameVersion()
public abstract java.lang.Object getGameInstance()
public abstract java.nio.file.Path getGameDir()
public abstract java.io.File getGameDirectory()
public abstract java.nio.file.Path getConfigDir()
public abstract java.io.File getConfigDirectory()
public abstract java.lang.String[] getLaunchArguments(boolean)
```
