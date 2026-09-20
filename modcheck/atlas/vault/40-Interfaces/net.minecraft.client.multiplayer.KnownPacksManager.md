---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.KnownPacksManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.KnownPacksManager

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `trySelectingPacks` | `(Ljava/util/List;)Ljava/util/List;` | name_only | @ModifyReturnValue at ['RETURN'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| wraps | `<init>` | `()V` | name_only | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (2 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final repository : Lnet/minecraft/server/packs/repository/PackRepository;
private final knownPackToId : Ljava/util/Map;
public <init>()V
public trySelectingPacks(Ljava/util/List;)Ljava/util/List;
public createResourceManager()Lnet/minecraft/server/packs/resources/CloseableResourceManager;
private static synthetic lambda$new$0(Lcom/google/common/collect/ImmutableMap$Builder;Lnet/minecraft/server/packs/repository/Pack;)V
private static synthetic lambda$new$1(Lcom/google/common/collect/ImmutableMap$Builder;Lnet/minecraft/server/packs/PackLocationInfo;Lnet/minecraft/server/packs/repository/KnownPack;)V
```
