---
type: "interface"
fqcn: "net.minecraft.server.packs.metadata.MetadataSectionType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.metadata.MetadataSectionType

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/String;Lcom/mojang/serialization/Codec;)V` | exact | invokespecial@43 in `OverlayConditionsMetadata.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (2 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final name : Ljava/lang/String;
private final codec : Lcom/mojang/serialization/Codec;
public <init>(Ljava/lang/String;Lcom/mojang/serialization/Codec;)V
public withValue(Ljava/lang/Object;)Lnet/minecraft/server/packs/metadata/MetadataSectionType$WithValue;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public name()Ljava/lang/String;
public codec()Lcom/mojang/serialization/Codec;
```
