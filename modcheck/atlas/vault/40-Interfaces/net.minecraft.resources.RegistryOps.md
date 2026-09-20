---
type: "interface"
fqcn: "net.minecraft.resources.RegistryOps"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryOps

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

`class` public; extends `net/minecraft/resources/DelegatingOps`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `create` | `(Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/resources/Registr` | exact | invokestatic@69 in `FabricRecipeProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `retrieveGetter` | `(Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/serialization/codec` | exact | invokestatic@4 in `TheEndBiomeSourceMixin.lambda$modifyCodec$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `lookupProvider` | `Lnet/minecraft/resources/RegistryOps$RegistryInfoLookup;` | exact | getfield@25 in `RegistryLoadTaskPendingRegistrationMixin.loadFromResource` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (1 fields, 22 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final lookupProvider : Lnet/minecraft/resources/RegistryOps$RegistryInfoLookup;
public static create(Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/resources/RegistryOps;
public static create(Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/resources/RegistryOps$RegistryInfoLookup;)Lnet/minecraft/resources/RegistryOps;
public static injectRegistryContext(Lcom/mojang/serialization/Dynamic;Lnet/minecraft/core/HolderLookup$Provider;)Lcom/mojang/serialization/Dynamic;
private <init>(Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/resources/RegistryOps$RegistryInfoLookup;)V
public withParent(Lcom/mojang/serialization/DynamicOps;)Lnet/minecraft/resources/RegistryOps;
public getter(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public static retrieveGetter(Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/serialization/codecs/RecordCodecBuilder;
public static retrieveElement(Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/serialization/codecs/RecordCodecBuilder;
private static synthetic lambda$retrieveElement$5(Ljava/lang/Object;)Lnet/minecraft/core/Holder$Reference;
private static synthetic lambda$retrieveElement$0(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/DynamicOps;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$retrieveElement$4()Ljava/lang/String;
private static synthetic lambda$retrieveElement$2(Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$retrieveElement$3(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
private static synthetic lambda$retrieveElement$1(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/HolderGetter;)Ljava/util/Optional;
private static synthetic lambda$retrieveGetter$5(Ljava/lang/Object;)Lnet/minecraft/core/HolderGetter;
private static synthetic lambda$retrieveGetter$0(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/DynamicOps;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$retrieveGetter$4()Ljava/lang/String;
private static synthetic lambda$retrieveGetter$2(Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$retrieveGetter$3(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
private static synthetic lambda$retrieveGetter$1(Lnet/minecraft/core/HolderGetter;)Lcom/mojang/serialization/DataResult;
```
