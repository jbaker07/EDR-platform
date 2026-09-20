---
type: "interface"
fqcn: "net.minecraft.client.data.models.ModelProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.data.models.ModelProvider

System: [[20-Systems/net.minecraft.client.data|net.minecraft.client.data]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/data/DataProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/data/PackOutput;)V` | exact | invokespecial@2 in `FabricModelProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/data/PackOutput;)V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `run` | `(Lnet/minecraft/data/CachedOutput;)Ljava/util/concurrent/CompletableFu` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| wraps | `run` | `(Lnet/minecraft/data/CachedOutput;)Ljava/util/concurrent/CompletableFu` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| wraps | `run` | `(Lnet/minecraft/data/CachedOutput;)Ljava/util/concurrent/CompletableFu` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (3 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final blockStatePathProvider : Lnet/minecraft/data/PackOutput$PathProvider;
private final itemInfoPathProvider : Lnet/minecraft/data/PackOutput$PathProvider;
private final modelPathProvider : Lnet/minecraft/data/PackOutput$PathProvider;
public <init>(Lnet/minecraft/data/PackOutput;)V
public run(Lnet/minecraft/data/CachedOutput;)Ljava/util/concurrent/CompletableFuture;
public getName()Ljava/lang/String;
```
