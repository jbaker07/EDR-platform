---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at INVOKE Lcom/google/common/collect/ImmutableList$Builder;add(Ljava/lan` | both | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (25, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager {
    private static final org.slf4j.Logger LOGGER;
    private static final java.lang.String STRUCTURE_FILE_EXTENSION;
    private static final java.lang.String STRUCTURE_TEXT_FILE_EXTENSION;
    public static final java.lang.String STRUCTURE_DIRECTORY_NAME;
    public static final net.minecraft.resources.FileToIdConverter WORLD_STRUCTURE_LISTER;
    private static final net.minecraft.resources.FileToIdConverter WORLD_TEXT_STRUCTURE_LISTER;
    private static final net.minecraft.resources.FileToIdConverter RESOURCE_STRUCTURE_LISTER;
    public static final net.minecraft.resources.FileToIdConverter RESOURCE_TEXT_STRUCTURE_LISTER;
    private final java.util.Map<net.minecraft.resources.Identifier, java.util.Optional<net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplate>> structureRepository;
    private final net.minecraft.world.level.levelgen.structure.templatesystem.loader.ResourceManagerTemplateSource resourceManagerSource;
    private final java.util.List<net.minecraft.world.level.levelgen.structure.templatesystem.loader.TemplateSource> sources;
    private final net.minecraft.world.level.levelgen.structure.templatesystem.loader.TemplatePathFactory worldTemplates;
    private final net.minecraft.world.level.levelgen.structure.templatesystem.loader.TemplatePathFactory testTemplates;
    public net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager(net.minecraft.server.packs.resources.ResourceManager, net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.datafixers.DataFixer, net.minecraft.core.HolderGetter<net.minecraft.world.level.block.Block>);
    public net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplate getOrCreate(net.minecraft.resources.Identifier);
    public java.util.Optional<net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplate> get(net.minecraft.resources.Identifier);
    public java.util.stream.Stream<net.minecraft.resources.Identifier> listTemplates();
    private java.util.Optional<net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplate> tryLoad(net.minecraft.resources.Identifier);
    public void onResourceManagerReload(net.minecraft.server.packs.resources.ResourceManager);
    public boolean save(net.minecraft.resources.Identifier);
    public static boolean save(java.nio.file.Path, net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplate, boolean) throws java.io.IOException;
    public net.minecraft.world.level.levelgen.structure.templatesystem.loader.TemplatePathFactory worldTemplates();
    public net.minecraft.world.level.levelgen.structure.templatesystem.loader.TemplatePathFactory testTemplates();
    public void remove(net.minecraft.resources.Identifier);
    static {};
}
```
