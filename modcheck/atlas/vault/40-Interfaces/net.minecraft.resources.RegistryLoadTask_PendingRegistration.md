---
type: "interface"
fqcn: "net.minecraft.resources.RegistryLoadTask$PendingRegistration"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryLoadTask$PendingRegistration

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

`record` public final; extends `java/lang/Record`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `value` | `()Lcom/mojang/datafixers/util/Either;` | exact | invokevirtual@1 in `ResourceManagerRegistryLoadTaskMixin.load` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `value` | `()Lcom/mojang/datafixers/util/Either;` | exact | invokevirtual@14 in `ResourceManagerRegistryLoadTaskMixin.load` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| injects_into | `loadFromResource` | `(Lcom/mojang/serialization/Decoder;Lnet/minecraft/resources/RegistryOp` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (3 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final key : Lnet/minecraft/resources/ResourceKey;
private final value : Lcom/mojang/datafixers/util/Either;
private final registrationInfo : Lnet/minecraft/core/RegistrationInfo;
protected <init>(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/datafixers/util/Either;Lnet/minecraft/core/RegistrationInfo;)V
public static loadFromResource(Lcom/mojang/serialization/Decoder;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/server/packs/resources/Resource;)Lcom/mojang/datafixers/util/Either;
public static findAndLoadFromResource(Lcom/mojang/serialization/Decoder;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/FileToIdConverter;Lnet/minecraft/server/packs/resources/ResourceProvider;)Lcom/mojang/datafixers/util/Either;
public static loadFromNetwork(Lcom/mojang/serialization/Decoder;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/nbt/Tag;)Lcom/mojang/datafixers/util/Either;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public key()Lnet/minecraft/resources/ResourceKey;
public value()Lcom/mojang/datafixers/util/Either;
public registrationInfo()Lnet/minecraft/core/RegistrationInfo;
private static synthetic lambda$findAndLoadFromResource$1(Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/datafixers/util/Either;
private static synthetic lambda$findAndLoadFromResource$0(Lcom/mojang/serialization/Decoder;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/server/packs/resources/Resource;)Lcom/mojang/datafixers/util/Either;
```
