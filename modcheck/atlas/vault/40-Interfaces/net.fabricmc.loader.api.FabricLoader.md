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
public abstract java.util.List getEntrypoints(java.lang.String, java.lang.Class)
public abstract java.util.List getEntrypointContainers(java.lang.String, java.lang.Class)
public abstract void invokeEntrypoints(java.lang.String, java.lang.Class, java.util.function.Consumer)
public abstract net.fabricmc.loader.api.ObjectShare getObjectShare()
public abstract net.fabricmc.loader.api.MappingResolver getMappingResolver()
public abstract java.util.Optional getModContainer(java.lang.String)
public abstract java.util.Collection getAllMods()
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
