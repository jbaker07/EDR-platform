---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.FolderRepositorySource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.FolderRepositorySource

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `packSourceLnet/minecraft/server/packs/repository/PackSource;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.packs.repository.FolderRepositorySource implements net.minecraft.server.packs.repository.RepositorySource {
    private static final org.slf4j.Logger LOGGER;
    private static final net.minecraft.server.packs.PackSelectionConfig DISCOVERED_PACK_SELECTION_CONFIG;
    private final java.nio.file.Path folder;
    private final net.minecraft.server.packs.PackType packType;
    private final net.minecraft.server.packs.repository.PackSource packSource;
    private final net.minecraft.world.level.validation.DirectoryValidator validator;
    public net.minecraft.server.packs.repository.FolderRepositorySource(java.nio.file.Path, net.minecraft.server.packs.PackType, net.minecraft.server.packs.repository.PackSource, net.minecraft.world.level.validation.DirectoryValidator);
    private static java.lang.String nameFromPath(java.nio.file.Path);
    public void loadPacks(java.util.function.Consumer<net.minecraft.server.packs.repository.Pack>);
    private net.minecraft.server.packs.PackLocationInfo createDiscoveredFilePackInfo(java.nio.file.Path);
    public static void discoverPacks(java.nio.file.Path, net.minecraft.world.level.validation.DirectoryValidator, java.util.function.BiConsumer<java.nio.file.Path, net.minecraft.server.packs.repository.Pack$ResourcesSupplier>) throws java.io.IOException;
    private void lambda$loadPacks$0(java.util.function.Consumer, java.nio.file.Path, net.minecraft.server.packs.repository.Pack$ResourcesSupplier);
    static {};
}
```
