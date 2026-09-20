---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.BuiltInPackSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.BuiltInPackSource

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/server/packs/repository/RepositorySource`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `loadPacks` | `(Ljava/util/function/Consumer;)V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (8 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final VANILLA_ID : Ljava/lang/String;
public static final TESTS_ID : Ljava/lang/String;
public static final CORE_PACK_INFO : Lnet/minecraft/server/packs/repository/KnownPack;
private final packType : Lnet/minecraft/server/packs/PackType;
private final vanillaPack : Lnet/minecraft/server/packs/VanillaPackResources;
private final packDir : Lnet/minecraft/resources/Identifier;
private final validator : Lnet/minecraft/world/level/validation/DirectoryValidator;
public <init>(Lnet/minecraft/server/packs/PackType;Lnet/minecraft/server/packs/VanillaPackResources;Lnet/minecraft/resources/Identifier;Lnet/minecraft/world/level/validation/DirectoryValidator;)V
public loadPacks(Ljava/util/function/Consumer;)V
protected abstract createVanillaPack(Lnet/minecraft/server/packs/repository/Pack$ResourcesSupplier;)Lnet/minecraft/server/packs/repository/Pack;
protected abstract getPackTitle(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
public getVanillaPack()Lnet/minecraft/server/packs/VanillaPackResources;
private listBundledPacks(Ljava/util/function/Consumer;)V
protected populatePackList(Ljava/util/function/BiConsumer;)V
protected discoverPacksInPath(Ljava/nio/file/Path;Ljava/util/function/BiConsumer;)V
private static pathToId(Ljava/nio/file/Path;)Ljava/lang/String;
protected abstract createBuiltinPack(Ljava/lang/String;Lnet/minecraft/server/packs/repository/Pack$ResourcesSupplier;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/server/packs/repository/Pack;
private synthetic lambda$discoverPacksInPath$0(Ljava/util/function/BiConsumer;Ljava/nio/file/Path;Lnet/minecraft/server/packs/repository/Pack$ResourcesSupplier;)V
private synthetic lambda$discoverPacksInPath$1(Lnet/minecraft/server/packs/repository/Pack$ResourcesSupplier;Ljava/lang/String;)Lnet/minecraft/server/packs/repository/Pack;
private synthetic lambda$populatePackList$0(Ljava/util/function/BiConsumer;Ljava/nio/file/Path;)V
private static synthetic lambda$listBundledPacks$0(Ljava/util/function/Consumer;Ljava/lang/String;Ljava/util/function/Function;)V
static <clinit>()V
```
