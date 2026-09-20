---
type: "interface"
fqcn: "net.minecraft.data.recipes.BrewingProvider$ContainerTransformation"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.BrewingProvider$ContainerTransformation

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

`record` final; extends `java/lang/Record`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `output` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@29 in `BrewingProviderMixin.preventDuplicatingDefaultTransformations` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `output` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@77 in `BrewingProviderMixin.preventDuplicatingDefaultTransformations` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `reagent` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@15 in `BrewingProviderMixin.preventDuplicatingDefaultTransformations` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `reagent` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@63 in `BrewingProviderMixin.preventDuplicatingDefaultTransformations` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (3 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final container : Lnet/minecraft/world/item/Item;
private final reagent : Lnet/minecraft/world/item/Item;
private final output : Lnet/minecraft/world/item/Item;
private <init>(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public container()Lnet/minecraft/world/item/Item;
public reagent()Lnet/minecraft/world/item/Item;
public output()Lnet/minecraft/world/item/Item;
```
