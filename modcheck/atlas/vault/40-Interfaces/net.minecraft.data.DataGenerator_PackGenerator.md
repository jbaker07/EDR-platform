---
type: "interface"
fqcn: "net.minecraft.data.DataGenerator$PackGenerator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.DataGenerator$PackGenerator

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/data/DataGenerator;ZLjava/lang/String;Lnet/minecraft/d` | exact | invokespecial@16 in `FabricDataGenerator$Pack.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `addProvider` | `(Lnet/minecraft/data/DataProvider$Factory;)Lnet/minecraft/data/DataPro` | exact | invokespecial@7 in `FabricDataGenerator$Pack.addProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `addProvider` | `(Lnet/minecraft/data/DataProvider$Factory;)Lnet/minecraft/data/DataPro` | exact | invokespecial@8 in `FabricDataGenerator$Pack.addProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (4 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final toRun : Z
private final providerPrefix : Ljava/lang/String;
private final output : Lnet/minecraft/data/PackOutput;
final synthetic this$0 : Lnet/minecraft/data/DataGenerator;
private <init>(Lnet/minecraft/data/DataGenerator;ZLjava/lang/String;Lnet/minecraft/data/PackOutput;)V
public addProvider(Lnet/minecraft/data/DataProvider$Factory;)Lnet/minecraft/data/DataProvider;
```
