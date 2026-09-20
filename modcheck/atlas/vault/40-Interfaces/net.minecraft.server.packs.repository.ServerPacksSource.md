---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.ServerPacksSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.ServerPacksSource

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/level/validation/DirectoryValidator;)V` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (18, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.packs.repository.ServerPacksSource extends net.minecraft.server.packs.repository.BuiltInPackSource {
    private static final net.minecraft.server.packs.metadata.pack.PackMetadataSection VERSION_METADATA_SECTION;
    private static final net.minecraft.server.packs.FeatureFlagsMetadataSection FEATURE_FLAGS_METADATA_SECTION;
    private static final net.minecraft.server.packs.resources.ResourceMetadata BUILT_IN_METADATA;
    private static final net.minecraft.server.packs.PackLocationInfo VANILLA_PACK_INFO;
    private static final net.minecraft.server.packs.PackSelectionConfig VANILLA_SELECTION_CONFIG;
    private static final net.minecraft.server.packs.PackSelectionConfig FEATURE_SELECTION_CONFIG;
    private static final net.minecraft.resources.Identifier PACKS_DIR;
    public net.minecraft.server.packs.repository.ServerPacksSource(net.minecraft.world.level.validation.DirectoryValidator);
    private static net.minecraft.server.packs.PackLocationInfo createBuiltInPackLocation(java.lang.String, net.minecraft.network.chat.Component);
    public static net.minecraft.server.packs.VanillaPackResources createVanillaPackSource();
    protected net.minecraft.network.chat.Component getPackTitle(java.lang.String);
    protected net.minecraft.server.packs.repository.Pack createVanillaPack(net.minecraft.server.packs.repository.Pack$ResourcesSupplier);
    protected net.minecraft.server.packs.repository.Pack createBuiltinPack(java.lang.String, net.minecraft.server.packs.repository.Pack$ResourcesSupplier, net.minecraft.network.chat.Component);
    public static net.minecraft.server.packs.repository.PackRepository createPackRepository(java.nio.file.Path, net.minecraft.world.level.validation.DirectoryValidator);
    public static net.minecraft.server.packs.repository.PackRepository createVanillaTrustedRepository();
    public static net.minecraft.server.packs.repository.PackRepository createPackRepository(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess);
    private static boolean lambda$createVanillaTrustedRepository$0(java.nio.file.Path);
    static {};
}
```
