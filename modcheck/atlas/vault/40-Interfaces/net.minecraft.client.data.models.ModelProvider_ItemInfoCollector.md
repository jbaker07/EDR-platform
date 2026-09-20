---
type: "interface"
fqcn: "net.minecraft.client.data.models.ModelProvider$ItemInfoCollector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.data.models.ModelProvider$ItemInfoCollector

System: [[20-Systems/net.minecraft.client.data|net.minecraft.client.data]]

`class` ; extends `java/lang/Object`; implements `net/minecraft/client/data/models/ItemModelOutput`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `finalizeAndValidate` | `()V` | name_only | @ModifyArg at ['INVOKE'] | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| wraps | `lambda$finalizeAndValidate$0` | `(Lnet/minecraft/world/item/Item;)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (2 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final itemInfos : Ljava/util/Map;
private final copies : Ljava/util/Map;
private <init>()V
public accept(Lnet/minecraft/world/item/Item;Lnet/minecraft/client/renderer/item/ItemModel$Unbaked;Lnet/minecraft/client/renderer/item/ClientItem$Properties;)V
private register(Lnet/minecraft/world/item/Item;Lnet/minecraft/client/renderer/item/ClientItem;)V
public copy(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)V
public finalizeAndValidate()V
public save(Lnet/minecraft/data/CachedOutput;Lnet/minecraft/data/PackOutput$PathProvider;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$save$0(Lnet/minecraft/data/PackOutput$PathProvider;Lnet/minecraft/world/item/Item;)Ljava/nio/file/Path;
private static synthetic lambda$finalizeAndValidate$3(Lnet/minecraft/core/Holder$Reference;)Lnet/minecraft/resources/Identifier;
private synthetic lambda$finalizeAndValidate$2(Lnet/minecraft/core/Holder$Reference;)Z
private synthetic lambda$finalizeAndValidate$1(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)V
private synthetic lambda$finalizeAndValidate$0(Lnet/minecraft/world/item/Item;)V
```
