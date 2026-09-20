---
type: "interface"
fqcn: "net.minecraft.network.codec.IdDispatchCodec$Entry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.codec.IdDispatchCodec$Entry

System: [[20-Systems/net.minecraft.network.codec|net.minecraft.network.codec]]

`record` final; extends `java/lang/Record`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `type` | `()Ljava/lang/Object;` | exact | invokevirtual@1 in `IdDispatchCodecMixin.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Ljava/lang/Object;` | exact | invokevirtual@11 in `IdDispatchCodecMixin.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Ljava/lang/Object;` | exact | invokevirtual@72 in `IdDispatchCodecMixin.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final serializer : Lnet/minecraft/network/codec/StreamCodec;
private final type : Ljava/lang/Object;
private <init>(Lnet/minecraft/network/codec/StreamCodec;Ljava/lang/Object;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public serializer()Lnet/minecraft/network/codec/StreamCodec;
public type()Ljava/lang/Object;
```
