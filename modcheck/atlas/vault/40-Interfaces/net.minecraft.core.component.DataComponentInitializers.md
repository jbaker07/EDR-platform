---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentInitializers"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentInitializers

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `createInitializerForRegistry` | `(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/compone` | name_only | @WrapMethod | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (1 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final initializers : Ljava/util/List;
public <init>()V
public add(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/component/DataComponentInitializers$Initializer;)V
private runInitializers(Lnet/minecraft/core/HolderLookup$Provider;)Ljava/util/Map;
private static registryEmpty(Ljava/util/Map;Lnet/minecraft/resources/ResourceKey;)V
private static addBuilder(Ljava/util/Map;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/component/DataComponentMap$Builder;)V
public build(Lnet/minecraft/core/HolderLookup$Provider;)Ljava/util/List;
private static createInitializerForRegistry(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/component/DataComponentInitializers$PendingComponentBuilders;)Lnet/minecraft/core/component/DataComponentInitializers$PendingComponents;
private static synthetic lambda$createInitializerForRegistry$2(Ljava/util/List;Lnet/minecraft/core/Holder$Reference;)V
private static synthetic lambda$createInitializerForRegistry$1(Ljava/util/Set;Lnet/minecraft/core/Holder$Reference;)Z
private static synthetic lambda$createInitializerForRegistry$0(Lnet/minecraft/core/HolderLookup$RegistryLookup;Ljava/util/List;Ljava/util/Set;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/component/DataComponentMap$Builder;)V
private static synthetic lambda$build$2(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/component/DataComponentInitializers$PendingComponentBuilders;)Lnet/minecraft/core/component/DataComponentInitializers$PendingComponents;
private static synthetic lambda$build$1(Ljava/util/Map;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/component/DataComponentMap$Builder;)V
private static synthetic lambda$build$0(Ljava/util/Map;Lnet/minecraft/resources/ResourceKey;)V
private static synthetic lambda$runInitializers$0(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/component/DataComponentMap$Builder;
```
