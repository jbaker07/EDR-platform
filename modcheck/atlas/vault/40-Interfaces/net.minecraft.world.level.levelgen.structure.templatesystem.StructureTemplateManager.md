---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<init>` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (13 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final STRUCTURE_FILE_EXTENSION : Ljava/lang/String;
private static final STRUCTURE_TEXT_FILE_EXTENSION : Ljava/lang/String;
public static final STRUCTURE_DIRECTORY_NAME : Ljava/lang/String;
public static final WORLD_STRUCTURE_LISTER : Lnet/minecraft/resources/FileToIdConverter;
private static final WORLD_TEXT_STRUCTURE_LISTER : Lnet/minecraft/resources/FileToIdConverter;
private static final RESOURCE_STRUCTURE_LISTER : Lnet/minecraft/resources/FileToIdConverter;
public static final RESOURCE_TEXT_STRUCTURE_LISTER : Lnet/minecraft/resources/FileToIdConverter;
private final structureRepository : Ljava/util/Map;
private final resourceManagerSource : Lnet/minecraft/world/level/levelgen/structure/templatesystem/loader/ResourceManagerTemplateSource;
private final sources : Ljava/util/List;
private final worldTemplates : Lnet/minecraft/world/level/levelgen/structure/templatesystem/loader/TemplatePathFactory;
private final testTemplates : Lnet/minecraft/world/level/levelgen/structure/templatesystem/loader/TemplatePathFactory;
public <init>(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/core/HolderGetter;)V
public getOrCreate(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate;
public get(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;
public listTemplates()Ljava/util/stream/Stream;
private tryLoad(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;
public onResourceManagerReload(Lnet/minecraft/server/packs/resources/ResourceManager;)V
public save(Lnet/minecraft/resources/Identifier;)Z
public static save(Ljava/nio/file/Path;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate;Z)Z
public worldTemplates()Lnet/minecraft/world/level/levelgen/structure/templatesystem/loader/TemplatePathFactory;
public testTemplates()Lnet/minecraft/world/level/levelgen/structure/templatesystem/loader/TemplatePathFactory;
public remove(Lnet/minecraft/resources/Identifier;)V
static <clinit>()V
```
