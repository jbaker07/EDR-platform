---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.PackSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.PackSource

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `BUILT_IN` | `Lnet/minecraft/server/packs/repository/PackSource;` | exact | getstatic@10 in `AdvancementUtil.determineSource` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| reads | `BUILT_IN` | `Lnet/minecraft/server/packs/repository/PackSource;` | exact | getstatic@10 in `ResourceUtil.determineSource` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `BUILT_IN` | `Lnet/minecraft/server/packs/repository/PackSource;` | exact | getstatic@10 in `LootUtil.determineSource` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| reads | `DEFAULT` | `Lnet/minecraft/server/packs/repository/PackSource;` | exact | getstatic@21 in `FabricResource.getFabricPackSource` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `DEFAULT` | `Lnet/minecraft/server/packs/repository/PackSource;` | exact | getstatic@4 in `PackSourceTracker.getSource` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `SERVER` | `Lnet/minecraft/server/packs/repository/PackSource;` | exact | getstatic@80 in `PackRepositoryMixin.construct` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `WORLD` | `Lnet/minecraft/server/packs/repository/PackSource;` | exact | getstatic@66 in `PackRepositoryMixin.construct` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (6 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NO_DECORATION : Ljava/util/function/UnaryOperator;
public static final DEFAULT : Lnet/minecraft/server/packs/repository/PackSource;
public static final BUILT_IN : Lnet/minecraft/server/packs/repository/PackSource;
public static final FEATURE : Lnet/minecraft/server/packs/repository/PackSource;
public static final WORLD : Lnet/minecraft/server/packs/repository/PackSource;
public static final SERVER : Lnet/minecraft/server/packs/repository/PackSource;
public abstract decorate(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
public abstract shouldAddAutomatically()Z
public static create(Ljava/util/function/UnaryOperator;Z)Lnet/minecraft/server/packs/repository/PackSource;
private static decorateWithSource(Ljava/lang/String;)Ljava/util/function/UnaryOperator;
private static synthetic lambda$decorateWithSource$0(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
static <clinit>()V
```
