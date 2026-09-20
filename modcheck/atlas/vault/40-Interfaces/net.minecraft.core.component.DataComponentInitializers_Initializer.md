---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentInitializers$Initializer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentInitializers$Initializer

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `andThen` | `(Lnet/minecraft/core/component/DataComponentInitializers$Initializer;)` | exact | invokeinterface@11 in `FabricItem$Properties.modifyComponents` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (0 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract run(Lnet/minecraft/core/component/DataComponentMap$Builder;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/resources/ResourceKey;)V
public andThen(Lnet/minecraft/core/component/DataComponentInitializers$Initializer;)Lnet/minecraft/core/component/DataComponentInitializers$Initializer;
public add(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lnet/minecraft/core/component/DataComponentInitializers$Initializer;
private static synthetic lambda$add$0(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;Lnet/minecraft/core/component/DataComponentMap$Builder;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/resources/ResourceKey;)V
private synthetic lambda$andThen$0(Lnet/minecraft/core/component/DataComponentInitializers$Initializer;Lnet/minecraft/core/component/DataComponentMap$Builder;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/resources/ResourceKey;)V
```
