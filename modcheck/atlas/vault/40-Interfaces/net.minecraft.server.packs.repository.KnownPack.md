---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.KnownPack"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.KnownPack

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V` | exact | invokespecial@282 in `ModNioPackResources.create` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (5 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final namespace : Ljava/lang/String;
private final id : Ljava/lang/String;
private final version : Ljava/lang/String;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final VANILLA_NAMESPACE : Ljava/lang/String;
public <init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
public static vanilla(Ljava/lang/String;)Lnet/minecraft/server/packs/repository/KnownPack;
public isVanilla()Z
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public namespace()Ljava/lang/String;
public id()Ljava/lang/String;
public version()Ljava/lang/String;
static <clinit>()V
```
