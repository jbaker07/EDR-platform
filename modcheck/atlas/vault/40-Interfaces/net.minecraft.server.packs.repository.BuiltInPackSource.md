---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.BuiltInPackSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.BuiltInPackSource

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `loadPacks` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.server.packs.repository.BuiltInPackSource implements net.minecraft.server.packs.repository.RepositorySource {
    private static final org.slf4j.Logger LOGGER;
    public static final java.lang.String VANILLA_ID;
    public static final java.lang.String TESTS_ID;
    public static final net.minecraft.server.packs.repository.KnownPack CORE_PACK_INFO;
    private final net.minecraft.server.packs.PackType packType;
    private final net.minecraft.server.packs.VanillaPackResources vanillaPack;
    private final net.minecraft.resources.Identifier packDir;
    private final net.minecraft.world.level.validation.DirectoryValidator validator;
    public net.minecraft.server.packs.repository.BuiltInPackSource(net.minecraft.server.packs.PackType, net.minecraft.server.packs.VanillaPackResources, net.minecraft.resources.Identifier, net.minecraft.world.level.validation.DirectoryValidator);
    public void loadPacks(java.util.function.Consumer<net.minecraft.server.packs.repository.Pack>);
    protected abstract net.minecraft.server.packs.repository.Pack createVanillaPack(net.minecraft.server.packs.repository.Pack$ResourcesSupplier);
    protected abstract net.minecraft.network.chat.Component getPackTitle(java.lang.String);
    public net.minecraft.server.packs.VanillaPackResources getVanillaPack();
    private void listBundledPacks(java.util.function.Consumer<net.minecraft.server.packs.repository.Pack>);
    protected void populatePackList(java.util.function.BiConsumer<java.lang.String, java.util.function.Function<java.lang.String, net.minecraft.server.packs.repository.Pack>>);
    protected void discoverPacksInPath(java.nio.file.Path, java.util.function.BiConsumer<java.lang.String, java.util.function.Function<java.lang.String, net.minecraft.server.packs.repository.Pack>>);
    private static java.lang.String pathToId(java.nio.file.Path);
    protected abstract net.minecraft.server.packs.repository.Pack createBuiltinPack(java.lang.String, net.minecraft.server.packs.repository.Pack$ResourcesSupplier, net.minecraft.network.chat.Component);
    private void lambda$discoverPacksInPath$0(java.util.function.BiConsumer, java.nio.file.Path, net.minecraft.server.packs.repository.Pack$ResourcesSupplier);
    private net.minecraft.server.packs.repository.Pack lambda$discoverPacksInPath$1(net.minecraft.server.packs.repository.Pack$ResourcesSupplier, java.lang.String);
    private void lambda$populatePackList$0(java.util.function.BiConsumer, java.nio.file.Path);
    private static void lambda$listBundledPacks$0(java.util.function.Consumer, java.lang.String, java.util.function.Function);
    static {};
}
```
