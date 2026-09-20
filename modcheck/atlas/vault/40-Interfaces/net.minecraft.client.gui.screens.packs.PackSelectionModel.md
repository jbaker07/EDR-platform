---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.packs.PackSelectionModel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.packs.PackSelectionModel

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<init>` | `(Ljava/util/function/Consumer;Ljava/util/function/Function;Lnet/minecr` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `findNewPacks` | `()V` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `selected` | `Ljava/util/List;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| reads | `unselected` | `Ljava/util/List;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |

## Declared members (6 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final repository : Lnet/minecraft/server/packs/repository/PackRepository;
private final selected : Ljava/util/List;
private final unselected : Ljava/util/List;
private final iconGetter : Ljava/util/function/Function;
private final onListChanged : Ljava/util/function/Consumer;
private final output : Ljava/util/function/Consumer;
public <init>(Ljava/util/function/Consumer;Ljava/util/function/Function;Lnet/minecraft/server/packs/repository/PackRepository;Ljava/util/function/Consumer;)V
public getUnselected()Ljava/util/stream/Stream;
public getSelected()Ljava/util/stream/Stream;
private updateRepoSelectedList()V
public commit()V
public findNewPacks()V
private synthetic lambda$getSelected$0(Lnet/minecraft/server/packs/repository/Pack;)Lnet/minecraft/client/gui/screens/packs/PackSelectionModel$Entry;
private synthetic lambda$getUnselected$0(Lnet/minecraft/server/packs/repository/Pack;)Lnet/minecraft/client/gui/screens/packs/PackSelectionModel$Entry;
```
