---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.structure.templatesystem.loader.TemplateSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.structure.templatesystem.loader.TemplateSource

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/core/HolderGetter;)V` | exact | invokespecial@28 in `StructureTemplateManagerMixin$1.<init>` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (2 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final fixerUpper : Lcom/mojang/datafixers/DataFixer;
private final blockLookup : Lnet/minecraft/core/HolderGetter;
protected <init>(Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/core/HolderGetter;)V
public abstract load(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;
public abstract list()Ljava/util/stream/Stream;
protected load(Lnet/minecraft/server/packs/resources/IoSupplier;ZLjava/util/function/Consumer;)Ljava/util/Optional;
private static readStructure(Ljava/io/InputStream;)Lnet/minecraft/nbt/CompoundTag;
private static readTextStructure(Ljava/io/InputStream;)Lnet/minecraft/nbt/CompoundTag;
private readStructure(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate;
```
