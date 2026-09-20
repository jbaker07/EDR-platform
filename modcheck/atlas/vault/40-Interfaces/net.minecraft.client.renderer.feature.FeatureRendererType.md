---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.FeatureRendererType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.FeatureRendererType

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `create` | `(Ljava/lang/String;)Lnet/minecraft/client/renderer/feature/FeatureRend` | exact | invokestatic@2 in `ExtendedBlockModelSubmit.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `create` | `(Ljava/lang/String;)Lnet/minecraft/client/renderer/feature/FeatureRend` | exact | invokestatic@2 in `ExtendedItemSubmit.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (3 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final id : I
private final name : Ljava/lang/String;
private static final NEXT_ID : Ljava/util/concurrent/atomic/AtomicInteger;
public <init>(ILjava/lang/String;)V
public static create(Ljava/lang/String;)Lnet/minecraft/client/renderer/feature/FeatureRendererType;
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public id()I
public name()Ljava/lang/String;
static <clinit>()V
```
