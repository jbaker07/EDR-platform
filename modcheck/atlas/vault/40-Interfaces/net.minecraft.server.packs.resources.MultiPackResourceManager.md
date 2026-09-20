---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.MultiPackResourceManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.MultiPackResourceManager

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/server/packs/resources/CloseableResourceManager`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<init>` | `(Lnet/minecraft/server/packs/PackType;Ljava/util/List;)V` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (3 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final namespacedManagers : Ljava/util/Map;
private final packs : Ljava/util/List;
public <init>(Lnet/minecraft/server/packs/PackType;Ljava/util/List;)V
private getPackFilterSection(Lnet/minecraft/server/packs/PackResources;)Lnet/minecraft/server/packs/resources/ResourceFilterSection;
public getNamespaces()Ljava/util/Set;
public getResource(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;
public getResourceStack(Lnet/minecraft/resources/Identifier;)Ljava/util/List;
public listResources(Ljava/lang/String;Lnet/minecraft/server/packs/resources/ResourceManager$Selector;)Ljava/util/Map;
public listResourceStacks(Ljava/lang/String;Lnet/minecraft/server/packs/resources/ResourceManager$Selector;)Ljava/util/Map;
private static checkTrailingDirectoryPath(Ljava/lang/String;)V
public listPacks()Ljava/util/stream/Stream;
public close()V
private static synthetic lambda$new$1(Lnet/minecraft/server/packs/resources/ResourceFilterSection;Lnet/minecraft/resources/Identifier;)Z
private static synthetic lambda$new$0(Lnet/minecraft/server/packs/PackType;Lnet/minecraft/server/packs/PackResources;)Ljava/util/stream/Stream;
static <clinit>()V
```
