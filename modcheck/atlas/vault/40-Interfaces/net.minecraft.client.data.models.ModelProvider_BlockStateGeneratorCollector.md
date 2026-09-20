---
type: "interface"
fqcn: "net.minecraft.client.data.models.ModelProvider$BlockStateGeneratorCollector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.data.models.ModelProvider$BlockStateGeneratorCollector

System: [[20-Systems/net.minecraft.client.data|net.minecraft.client.data]]

`class` ; extends `java/lang/Object`; implements `java/util/function/Consumer`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `validate` | `()V` | name_only | @ModifyArg at ['INVOKE'] | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `generators` | `Ljava/util/Map;` | exact | getfield@33 in `ModelProviderMixin.setFabricPackOutput` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (1 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final generators : Ljava/util/Map;
private <init>()V
public accept(Lnet/minecraft/client/data/models/blockstates/BlockModelDefinitionGenerator;)V
public validate()V
public save(Lnet/minecraft/data/CachedOutput;Lnet/minecraft/data/PackOutput$PathProvider;)Ljava/util/concurrent/CompletableFuture;
public synthetic accept(Ljava/lang/Object;)V
private static synthetic lambda$save$0(Lnet/minecraft/data/PackOutput$PathProvider;Lnet/minecraft/world/level/block/Block;)Ljava/nio/file/Path;
private static synthetic lambda$validate$1(Lnet/minecraft/core/Holder$Reference;)Lnet/minecraft/resources/Identifier;
private synthetic lambda$validate$0(Lnet/minecraft/core/Holder$Reference;)Z
```
