---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.PackRepository"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.PackRepository

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"([Lnet/minecraft/server/packs/repository/RepositorySource;)V` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getPack(Ljava/lang/String;)Lnet/minecraft/server/packs/repository/P` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getPack(Ljava/lang/String;)Lnet/minecraft/server/packs/repository/P` | `` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getSelectedPacks()Ljava/util/Collection;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `addPack` | `@Inject at INVOKE Ljava/util/List;add(Ljava/lang/Object;)Z` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `rebuildSelected` | `@Inject at INVOKE Lcom/google/common/collect/ImmutableList;copyOf(Ljava/util/Col` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `removePack` | `@Inject at INVOKE Ljava/util/List;remove(Ljava/lang/Object;)Z` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `sourcesLjava/util/Set;` | `` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.packs.repository.PackRepository {
    private final java.util.Set<net.minecraft.server.packs.repository.RepositorySource> sources;
    private java.util.Map<java.lang.String, net.minecraft.server.packs.repository.Pack> available;
    private java.util.List<net.minecraft.server.packs.repository.Pack> selected;
    public net.minecraft.server.packs.repository.PackRepository(net.minecraft.server.packs.repository.RepositorySource...);
    public static java.lang.String displayPackList(java.util.Collection<net.minecraft.server.packs.repository.Pack>);
    public void reload();
    private java.util.Map<java.lang.String, net.minecraft.server.packs.repository.Pack> discoverAvailable();
    public boolean isAbleToClearAnyPack();
    public void setSelected(java.util.Collection<java.lang.String>);
    public boolean addPack(java.lang.String);
    public boolean removePack(java.lang.String);
    private java.util.List<net.minecraft.server.packs.repository.Pack> rebuildSelected(java.util.Collection<java.lang.String>);
    private java.util.stream.Stream<net.minecraft.server.packs.repository.Pack> getAvailablePacks(java.util.Collection<java.lang.String>);
    public java.util.Collection<java.lang.String> getAvailableIds();
    public java.util.Collection<net.minecraft.server.packs.repository.Pack> getAvailablePacks();
    public java.util.Collection<java.lang.String> getSelectedIds();
    public net.minecraft.world.flag.FeatureFlagSet getRequestedFeatureFlags();
    public java.util.Collection<net.minecraft.server.packs.repository.Pack> getSelectedPacks();
    public net.minecraft.server.packs.repository.Pack getPack(java.lang.String);
    public boolean isAvailable(java.lang.String);
    public java.util.List<net.minecraft.server.packs.PackResources> openAllSelected();
    private static void lambda$discoverAvailable$0(java.util.Map, net.minecraft.server.packs.repository.Pack);
    private static java.lang.String lambda$displayPackList$0(net.minecraft.server.packs.repository.Pack);
}
```
