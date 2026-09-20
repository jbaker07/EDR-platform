---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.PackRepository"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.PackRepository

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `([Lnet/minecraft/server/packs/repository/RepositorySource;)V` | exact | invokespecial@44 in `ModPackResourcesUtil.createModdedRepository` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getPack` | `(Ljava/lang/String;)Lnet/minecraft/server/packs/repository/Pack;` | exact | invokevirtual@23 in `MinecraftServerMixin.onCheckDisabled` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getPack` | `(Ljava/lang/String;)Lnet/minecraft/server/packs/repository/Pack;` | exact | invokevirtual@51 in `GameOptionsWriteVisitorMixin.toPackListString` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getSelectedPacks` | `()Ljava/util/Collection;` | exact | invokevirtual@1 in `DataPackCommandMixin.filterEnabledPackSuggestions` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `<init>` | `([Lnet/minecraft/server/packs/repository/RepositorySource;)V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `addPack` | `(Ljava/lang/String;)Z` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `rebuildSelected` | `(Ljava/util/Collection;)Ljava/util/List;` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `removePack` | `(Ljava/lang/String;)Z` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `available` | `Ljava/util/Map;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| reads | `sources` | `Ljava/util/Set;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| reads | `sources` | `Ljava/util/Set;` | exact | getfield@1 in `CreateWorldScreenMixin.onCreateResManagerInit` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `sources` | `Ljava/util/Set;` | exact | getfield@4 in `CreateWorldScreenMixin.onScanPacks` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (3 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final sources : Ljava/util/Set;
private available : Ljava/util/Map;
private selected : Ljava/util/List;
public <init>([Lnet/minecraft/server/packs/repository/RepositorySource;)V
public static displayPackList(Ljava/util/Collection;)Ljava/lang/String;
public reload()V
private discoverAvailable()Ljava/util/Map;
public isAbleToClearAnyPack()Z
public setSelected(Ljava/util/Collection;)V
public addPack(Ljava/lang/String;)Z
public removePack(Ljava/lang/String;)Z
private rebuildSelected(Ljava/util/Collection;)Ljava/util/List;
private getAvailablePacks(Ljava/util/Collection;)Ljava/util/stream/Stream;
public getAvailableIds()Ljava/util/Collection;
public getAvailablePacks()Ljava/util/Collection;
public getSelectedIds()Ljava/util/Collection;
public getRequestedFeatureFlags()Lnet/minecraft/world/flag/FeatureFlagSet;
public getSelectedPacks()Ljava/util/Collection;
public getPack(Ljava/lang/String;)Lnet/minecraft/server/packs/repository/Pack;
public isAvailable(Ljava/lang/String;)Z
public openAllSelected()Ljava/util/List;
private static synthetic lambda$discoverAvailable$0(Ljava/util/Map;Lnet/minecraft/server/packs/repository/Pack;)V
private static synthetic lambda$displayPackList$0(Lnet/minecraft/server/packs/repository/Pack;)Ljava/lang/String;
```
